use core::slice;

/// 图像格式转换工具，用于在 OpenCV 和 image-rs 之间转换
///
/// 灵感来自 https://github.com/jerry73204/rust-cv-convert
/// 但该库仅支持 opencv-rs 0.89 版本，且部分抽象对我们来说不必要，因此单独实现
use image::{self};
use opencv::{core as cv, prelude::*};

/// 图像格式转换错误类型
#[derive(Debug)]
pub enum ConversionError {
    /// 不支持的图像颜色类型
    UnsupportedColorType(image::ColorType),
    /// 不支持的 OpenCV 矩阵类型
    UnsupportedMatType(String),
    /// 无效的矩阵维度（负数或零）
    InvalidDimensions,
    /// OpenCV 操作错误
    OpenCvError(opencv::Error),
}

use element_type::*;
mod element_type {
    use opencv::core as cv;

    /// OpenCV 元素类型 trait
    ///
    /// 提供类型对应的 OpenCV 深度常量
    pub trait OpenCvElement: cv::DataType {
        /// OpenCV 深度常量（例如 u8 对应 CV_8U）
        const DEPTH: i32;
    }

    impl OpenCvElement for u8 {
        const DEPTH: i32 = cv::CV_8U;
    }

    impl OpenCvElement for u16 {
        const DEPTH: i32 = cv::CV_16U;
    }

    impl OpenCvElement for f32 {
        const DEPTH: i32 = cv::CV_32F;
    }
}

// &ImageBuffer -> Mat
impl<P, Container> TryToCv<cv::Mat> for image::ImageBuffer<P, Container>
where
    P: image::Pixel,
    P::Subpixel: OpenCvElement,
    Container: std::ops::Deref<Target = [P::Subpixel]> + Clone,
{
    type Error = ConversionError;

    fn try_to_cv(&self) -> Result<cv::Mat, Self::Error> {
        let (width, height) = self.dimensions();
        let cv_type = cv::CV_MAKETYPE(P::Subpixel::DEPTH, P::CHANNEL_COUNT as i32);
        let mat = unsafe {
            cv::Mat::new_rows_cols_with_data_unsafe_def(
                height as i32,
                width as i32,
                cv_type,
                self.as_ptr().cast::<core::ffi::c_void>().cast_mut(),
            )
            .map_err(ConversionError::OpenCvError)
        }?
        .try_clone()
        .map_err(ConversionError::OpenCvError)?;
        Ok(mat)
    }
}

// &DynamicImage -> Mat
impl TryToCv<cv::Mat> for image::DynamicImage {
    type Error = ConversionError;

    fn try_to_cv(&self) -> Result<cv::Mat, Self::Error> {
        use image::DynamicImage as D;

        let mat = match self {
            D::ImageLuma8(image) => image.try_to_cv()?,
            D::ImageLumaA8(image) => image.try_to_cv()?,
            D::ImageRgb8(image) => image.try_to_cv()?,
            D::ImageRgba8(image) => image.try_to_cv()?,
            D::ImageLuma16(image) => image.try_to_cv()?,
            D::ImageLumaA16(image) => image.try_to_cv()?,
            D::ImageRgb16(image) => image.try_to_cv()?,
            D::ImageRgba16(image) => image.try_to_cv()?,
            D::ImageRgb32F(image) => image.try_to_cv()?,
            D::ImageRgba32F(image) => image.try_to_cv()?,
            image => return Err(ConversionError::UnsupportedColorType(image.color())),
        };
        Ok(mat)
    }
}

// &Mat -> DynamicImage
impl TryToCv<image::DynamicImage> for cv::Mat {
    type Error = ConversionError;

    fn try_to_cv(&self) -> Result<image::DynamicImage, Self::Error> {
        let rows = self.rows();
        let cols = self.cols();
        if rows == -1 || cols == -1 {
            return Err(ConversionError::InvalidDimensions);
        }

        let depth = self.depth();
        let n_channels = self.channels();
        let width = cols as u32;
        let height = rows as u32;

        let image: image::DynamicImage = match (depth, n_channels) {
            (cv::CV_8U, 1) => mat_to_image_buffer_gray::<u8>(self, width, height).into(),
            (cv::CV_16U, 1) => mat_to_image_buffer_gray::<u16>(self, width, height).into(),
            (cv::CV_8U, 3) => mat_to_image_buffer_rgb::<u8>(self, width, height).into(),
            (cv::CV_16U, 3) => mat_to_image_buffer_rgb::<u16>(self, width, height).into(),
            (cv::CV_32F, 3) => mat_to_image_buffer_rgb::<f32>(self, width, height).into(),
            _ => {
                return Err(ConversionError::UnsupportedMatType(format!(
                    "depth: {}, channels: {}, type: {}",
                    depth,
                    n_channels,
                    self.type_name()
                )))
            }
        };

        Ok(image)
    }
}

/// OpenCV Mat 扩展 trait
trait MatExt {
    fn size_with_depth(&self) -> Vec<usize>;
    fn numel(&self) -> usize {
        self.size_with_depth().iter().product()
    }

    fn as_slice<T>(&self) -> Result<&[T], opencv::Error>
    where
        T: OpenCvElement;

    fn type_name(&self) -> String;
}

impl MatExt for cv::Mat {
    fn size_with_depth(&self) -> Vec<usize> {
        let size = self.mat_size();
        let size = size.iter().map(|&dim| dim as usize);
        let channels = self.channels() as usize;
        size.chain([channels]).collect()
    }

    fn as_slice<T>(&self) -> Result<&[T], opencv::Error>
    where
        T: OpenCvElement,
    {
        debug_assert!(self.depth() == T::DEPTH, "element type mismatch");
        debug_assert!(self.is_continuous(), "Mat data must be continuous");

        let numel = self.numel();
        let ptr = self.ptr(0)? as *const T;

        let slice = unsafe { slice::from_raw_parts(ptr, numel) };
        Ok(slice)
    }

    fn type_name(&self) -> String {
        cv::type_to_string(self.typ()).unwrap()
    }
}

/// 将 OpenCV 矩阵转换为灰度图像缓冲区
fn mat_to_image_buffer_gray<T>(
    mat: &cv::Mat,
    width: u32,
    height: u32,
) -> image::ImageBuffer<image::Luma<T>, Vec<T>>
where
    T: image::Primitive + OpenCvElement + Default + Copy,
{
    type Image<T> = image::ImageBuffer<image::Luma<T>, Vec<T>>;

    match mat.as_slice::<T>() {
        Ok(slice) => Image::<T>::from_vec(width, height, slice.to_vec()).unwrap(),
        Err(_) => Image::<T>::from_fn(width, height, |col, row| {
            let pixel: T = *mat.at_2d(row as i32, col as i32).unwrap();
            image::Luma([pixel])
        }),
    }
}

/// 将 OpenCV 矩阵转换为 RGB 图像缓冲区
fn mat_to_image_buffer_rgb<T>(
    mat: &cv::Mat,
    width: u32,
    height: u32,
) -> image::ImageBuffer<image::Rgb<T>, Vec<T>>
where
    T: image::Primitive + OpenCvElement + Default + Copy,
    image::Rgb<T>: image::Pixel<Subpixel = T>,
{
    type Image<T> = image::ImageBuffer<image::Rgb<T>, Vec<T>>;

    match mat.as_slice::<T>() {
        Ok(slice) => Image::<T>::from_vec(width, height, slice.to_vec()).unwrap(),
        Err(_) => Image::<T>::from_fn(width, height, |col, row| {
            let cv::Point3_::<T> { x, y, z } = *mat.at_2d(row as i32, col as i32).unwrap();
            image::Rgb([x, y, z])
        }),
    }
}

/// 在 OpenCV 和 image-rs 格式之间转换的 trait
pub trait TryToCv<T> {
    type Error;
    fn try_to_cv(&self) -> Result<T, Self::Error>;
}
