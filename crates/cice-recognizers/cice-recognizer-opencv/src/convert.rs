use core::slice;

/// Inspired by https://github.com/jerry73204/rust-cv-convert. However, this library only supports opencv-rs version 0.89, and some of the abstractions are not necessary for us, so we implement it seperately
use image::{self};
use opencv::{core as cv, prelude::*};

/// Error types for image format conversion between OpenCV and image-rs
#[derive(Debug)]
pub enum ConversionError {
    /// The image color type is not supported for conversion
    UnsupportedColorType(image::ColorType),
    /// The OpenCV matrix type is not supported for conversion
    UnsupportedMatType(String),
    /// The matrix dimensions are invalid (negative or zero)
    InvalidDimensions,
    /// An error originating from OpenCV operations
    OpenCvError(opencv::Error),
}

use element_type::*;
mod element_type {
    use opencv::core as cv;

    /// Trait for OpenCV element types that can be used in image conversion
    ///
    /// Provides the OpenCV depth constant for the type
    pub trait OpenCvElement: cv::DataType {
        /// OpenCV depth constant for this type (e.g. CV_8U for u8)
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

// // &Mat -> ImageBuffer
// impl<P> TryToCv<image::ImageBuffer<P, Vec<P::Subpixel>>> for cv::Mat
// where
//     P: image::Pixel + 'static,
//     P::Subpixel: image::Primitive + OpenCvElement + cv::DataType + Default + Copy,
// {
//     type Error = ConversionError;

//     fn try_to_cv(&self) -> Result<image::ImageBuffer<P, Vec<P::Subpixel>>, Self::Error> {
//         let rows = self.rows();
//         let cols = self.cols();
//         if rows == -1 || cols == -1 {
//             return Err(ConversionError::InvalidDimensions);
//         }

//         let depth = self.depth();
//         let n_channels = self.channels();
//         let width = cols as u32;
//         let height = rows as u32;

//         if P::CHANNEL_COUNT as i32 != n_channels {
//             return Err(ConversionError::ChannelMismatch(n_channels as usize));
//         }

//         if <P::Subpixel as OpenCvElement>::DEPTH != depth {
//             return Err(ConversionError::SubpixelTypeMismatch);
//         }

//         let mut buffer = image::ImageBuffer::new(width, height);

//         for y in 0..height {
//             for x in 0..width {
//                 let pixel = match n_channels {
//                     1 => {
//                         let val: P::Subpixel = *self
//                             .at_2d(y as i32, x as i32)
//                             .map_err(ConversionError::OpenCvError)?;
//                         image::Luma([val])
//                     }
//                     3 => {
//                         let cv::Point3_ { x: b, y: g, z: r } = *self
//                             .at_2d(y as i32, x as i32)
//                             .map_err(ConversionError::OpenCvError)?;
//                         image::Rgb([r, g, b])
//                     }
//                     _ => {
//                         return Err(ConversionError::UnsupportedMatType(format!(
//                             "Unsupported channel count: {}",
//                             n_channels
//                         )))
//                     }
//                 };
//                 buffer.put_pixel(x, y, pixel);
//             }
//         }

//         Ok(buffer)
//     }
// }

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

/// Extension trait for OpenCV Mat with additional utility methods
trait MatExt {
    fn size_with_depth(&self) -> Vec<usize>;
    fn numel(&self) -> usize {
        self.size_with_depth().iter().product()
    }

    fn as_slice<T>(&self) -> Result<&[T], opencv::Error>
    where
        T: OpenCvElement;

    fn type_name(&self) -> String;
    #[cfg(test)]
    fn new_randn_2d(rows: i32, cols: i32, typ: i32) -> Result<Self, opencv::Error>
    where
        Self: Sized;

    #[cfg(test)]
    fn new_randn_nd<T>(shape: &[usize]) -> Result<Self, opencv::Error>
    where
        Self: Sized,
        T: OpenCvElement;
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

    #[cfg(test)]
    fn new_randn_2d(rows: i32, cols: i32, typ: i32) -> Result<Self, opencv::Error>
    where
        Self: Sized,
    {
        let mut mat = Self::zeros(rows, cols, typ)?.to_mat()?;
        cv::randn(&mut mat, &0.0, &1.0)?;
        Ok(mat)
    }

    #[cfg(test)]
    fn new_randn_nd<T>(shape: &[usize]) -> Result<Self, opencv::Error>
    where
        T: OpenCvElement,
    {
        let shape: Vec<_> = shape.iter().map(|&val| val as i32).collect();
        let mut mat = Self::zeros_nd(&shape, T::DEPTH)?.to_mat()?;
        cv::randn(&mut mat, &0.0, &1.0)?;
        Ok(mat)
    }
}

/// Convert an OpenCV matrix to a grayscale image buffer
///
/// # Arguments
/// * `mat` - OpenCV matrix containing grayscale image data
/// * `width` - Width of the resulting image
/// * `height` - Height of the resulting image
///
/// # Returns
/// ImageBuffer containing the converted grayscale image
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

/// Convert an OpenCV matrix to an RGB image buffer
///
/// # Arguments
/// * `mat` - OpenCV matrix containing RGB image data
/// * `width` - Width of the resulting image
/// * `height` - Height of the resulting image
///
/// # Returns
/// ImageBuffer containing the converted RGB image
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
/// Trait for converting between OpenCV and image-rs formats
///
/// Implemented for various image types to enable conversion to/from OpenCV matrices
pub trait TryToCv<T> {
    type Error;
    fn try_to_cv(&self) -> Result<T, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::{element_type::OpenCvElement, MatExt, TryToCv};
    use core::fmt::Debug;
    use image::{DynamicImage, ImageBuffer, Luma, Rgb};
    use itertools::iproduct;
    use opencv::{
        core::{self as cv},
        prelude::*,
    };
    const WIDTH: usize = 100;
    const HEIGHT: usize = 50;

    fn test_image_conversion<
        P: image::Pixel + Debug + core::cmp::PartialEq<P>,
        CVDATA: cv::DataType + core::cmp::PartialEq<CVDATA> + core::fmt::Debug,
    >(
        create_image: impl Fn(u32, u32) -> ImageBuffer<P, Vec<P::Subpixel>>,
        from_dynamic_image_to_buffer: impl Fn(&DynamicImage) -> ImageBuffer<P, Vec<P::Subpixel>>,
        mat_type: i32,
    ) where
        P::Subpixel: OpenCvElement + Debug + Default + Copy,
        ImageBuffer<P, Vec<P::Subpixel>>: Into<DynamicImage>,
    {
        assert_eq!(
            cv::CV_MAKETYPE(P::Subpixel::DEPTH, P::CHANNEL_COUNT as i32),
            mat_type
        );
        // Test Image -> Mat -> Image roundtrip
        let image = create_image(WIDTH as u32, HEIGHT as u32);
        let mat: cv::Mat = image.try_to_cv().unwrap();
        let image2: ImageBuffer<P, Vec<P::Subpixel>> =
            from_dynamic_image_to_buffer(&mat.try_to_cv().unwrap());

        iproduct!(0..HEIGHT, 0..WIDTH).for_each(|(row, col)| {
            let p1 = image.get_pixel(col as u32, row as u32);
            let p2 = image2.get_pixel(col as u32, row as u32);
            assert_eq!(*p1, *p2, "{row}, {col}");
        });

        // Test Mat -> Image -> Mat roundtrip
        let mat = Mat::new_randn_2d(HEIGHT as i32, WIDTH as i32, mat_type).unwrap();
        let image: DynamicImage = mat.try_to_cv().unwrap();
        let mat2: cv::Mat = image.try_to_cv().unwrap();
        iproduct!(0..HEIGHT, 0..WIDTH).for_each(|(row, col)| {
            let p1: &CVDATA = mat.at_2d(row as i32, col as i32).unwrap();
            let p2: &CVDATA = mat2.at_2d(row as i32, col as i32).unwrap();
            assert_eq!(*p1, *p2);
        });
    }

    #[test]
    fn test_luma8_conversion() {
        test_image_conversion::<_, u8>(
            |w, h| ImageBuffer::from_fn(w, h, |x, y| Luma([(x + y) as u8])),
            DynamicImage::to_luma8,
            cv::CV_8UC1,
        );
    }

    #[test]
    fn test_luma16_conversion() {
        test_image_conversion::<_, u16>(
            |w, h| ImageBuffer::from_fn(w, h, |x, y| Luma([(x + y) as u16])),
            DynamicImage::to_luma16,
            cv::CV_16UC1,
        );
    }

    #[test]
    fn test_rgb8_conversion() {
        test_image_conversion::<_, cv::Point3_<u8>>(
            |w, h| ImageBuffer::from_fn(w, h, |x, y| Rgb([x as u8, y as u8, (x + y) as u8])),
            DynamicImage::to_rgb8,
            cv::CV_8UC3,
        );
    }

    #[test]
    fn test_rgb16_conversion() {
        test_image_conversion::<_, cv::Point3_<u16>>(
            |w, h| ImageBuffer::from_fn(w, h, |x, y| Rgb([x as u16, y as u16, (x + y) as u16])),
            DynamicImage::to_rgb16,
            cv::CV_16UC3,
        );
    }

    //TODO rgba test is not usable for now since there isn't such a type like `Point4_<u8>` to easily represents a rgba pixel in opencv matrix
    // #[test]
    // fn test_rgba8_conversion() {
    //     test_image_conversion(
    //         |w, h| ImageBuffer::from_fn(w, h, |x, y| Rgba([x as u8, y as u8, (x + y) as u8, 255])),
    //         DynamicImage::to_rgba8,
    //         cv::CV_8UC4,
    //     );
    // }

    #[test]
    fn test_rgb32f_conversion() {
        test_image_conversion::<_, cv::Point3_<f32>>(
            |w, h| {
                ImageBuffer::from_fn(w, h, |x, y| {
                    Rgb([
                        (x as f32 / w as f32),
                        (y as f32 / h as f32),
                        ((x + y) as f32 / (w + h) as f32),
                    ])
                })
            },
            DynamicImage::to_rgb32f,
            cv::CV_32FC3,
        );
    }

    //NOTE Skip this test for now since `try_clone` a `BoxedRef<Mat>` would convert it from a `non-continuous` matrix to a `continuous` matrix and it seems to be necessary to use `clone` in this transformation
    // #[test]
    // fn test_non_continuous_mat() {
    //     // Create a non-continuous mat by taking a ROI
    //     let mut mat = Mat::new_randn_2d(HEIGHT as i32 * 2, WIDTH as i32 * 2, cv::CV_8UC3).unwrap();
    //     let roi = cv::Mat::roi(&mat, cv::Rect::new(10, 10, WIDTH as i32, HEIGHT as i32)).unwrap();
    //     assert!(!roi.is_continuous());

    //     // Test conversion
    //     let image: DynamicImage = roi.try_clone().unwrap().try_to_cv().unwrap();
    //     let mat2: cv::Mat = image.try_to_cv().unwrap();
    //     assert!(mat2.is_continuous());

    //     // Verify pixels match
    //     iproduct!(0..HEIGHT, 0..WIDTH).for_each(|(row, col)| {
    //         let p1 = roi
    //             .at_2d::<cv::Point3_<u8>>(row as i32, col as i32)
    //             .unwrap();
    //         let p2 = mat2
    //             .at_2d::<cv::Point3_<u8>>(row as i32, col as i32)
    //             .unwrap();
    //         assert_eq!(*p1, *p2);
    //     });
    // }
}
