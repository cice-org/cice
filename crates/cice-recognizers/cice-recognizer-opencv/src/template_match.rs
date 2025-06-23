use core::f64;

use crate::convert::TryToCv;
use async_trait::async_trait;
use cice_core::{
    controller::ControllerData,
    recognizer::{
        CustomRecognizerError, RecognizeResult, Recognizer, RecognizerError, RecognizerId,
    },
    resource::ResourceData,
};
use image::DynamicImage;
use opencv::core::{Mat, MatTraitConst};
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpenCVTemplateMatchAction {
    template: String,
    roi: Option<[u64; 4]>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpenCVTemplateMatchResult {
    matched: bool,
    position: OpenCVTemplateMatchResultPosition,
    size: OpenCVTemplateMatchResultSize,
    confidence: f64,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpenCVTemplateMatchResultPosition {
    x: i32,
    y: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpenCVTemplateMatchResultSize {
    width: i32,
    height: i32,
}

pub struct OpenCVTemplateMatchRecognizer;

impl OpenCVTemplateMatchRecognizer {
    fn template_match(
        src: Mat,
        template: Mat,
        roi: Option<[u64; 4]>,
    ) -> Result<OpenCVTemplateMatchResult, CustomRecognizerError> {
        // Apply ROI if specified
        let input_roi = if let Some([x, y, width, height]) = roi {
            let rect = opencv::core::Rect::new(x as i32, y as i32, width as i32, height as i32);
            opencv::core::Mat::roi(&src, rect).map_err(|e| CustomRecognizerError::Common {
                source: Box::new(e),
            })?
        } else {
            src.into()
        };

        // Perform template matching
        let mut result = opencv::core::Mat::default();
        opencv::imgproc::match_template(
            &input_roi,
            &template,
            &mut result,
            opencv::imgproc::TM_CCOEFF_NORMED,
            &opencv::core::no_array(),
        )
        .map_err(|e| CustomRecognizerError::Common {
            source: Box::new(e),
        })?;

        // Find best match location
        let (mut min_val, mut max_val, mut min_loc, mut max_loc) = (
            f64::MIN,
            f64::MAX,
            opencv::core::Point::new(0, 0),
            opencv::core::Point::new(0, 0),
        );
        opencv::core::min_max_loc(
            &result,
            Some(&mut min_val),
            Some(&mut max_val),
            Some(&mut min_loc),
            Some(&mut max_loc),
            &opencv::core::no_array(),
        )
        .map_err(|e| CustomRecognizerError::Common {
            source: Box::new(e),
        })?;

        Ok(OpenCVTemplateMatchResult {
            matched: max_val > 0.8, // Threshold for match confidence
            position: OpenCVTemplateMatchResultPosition {
                x: max_loc.x,
                y: max_loc.y,
            },
            size: OpenCVTemplateMatchResultSize {
                width: template.cols(),
                height: template.rows(),
            },
            confidence: max_val,
        })
    }
    fn exec(
        image: DynamicImage,
        action: OpenCVTemplateMatchAction,
    ) -> Result<OpenCVTemplateMatchResult, CustomRecognizerError> {
        let input_mat = image
            .try_to_cv()
            .map_err(|_| CustomRecognizerError::InvalidData)?;

        let template_mat =
            opencv::imgcodecs::imread(&action.template, opencv::imgcodecs::IMREAD_COLOR).map_err(
                |e| CustomRecognizerError::Common {
                    source: Box::new(e),
                },
            )?;
        OpenCVTemplateMatchRecognizer::template_match(input_mat, template_mat, action.roi)
    }
}

#[async_trait]
impl Recognizer for OpenCVTemplateMatchRecognizer {
    fn name(&self) -> RecognizerId {
        return "template_match".to_string();
    }
    /// ## Calling time
    /// Init Recognizer, would be called at the first time when the recognizer is called or any override config [recognizer_config_ext](crate::task::TaskData::recognizer_config_ext())
    /// ## Notice
    /// This is only necessary to be implemented when Recognizer supports `reinitialze` or needs `lazy initialize`. For most of the cases,
    /// keeping this function as a dummy implementation (by returning `Ok(())` directly) and passing an initialized and immutable structure (use `::new()` for example) is always the best solution
    fn init(&self, resource: &ResourceData) -> Result<(), RecognizerError> {
        return Ok(());
    }
    fn require_input(&self) -> Option<ResourceData> {
        return None;
    } //Require input from a OutputController
    async fn exec(
        &self,
        action: Option<&ResourceData>,
        data: ControllerData,
    ) -> Result<RecognizeResult, CustomRecognizerError> {
        //TODO extract these deserialization codes.
        let action: OpenCVTemplateMatchAction =
            serde_json::from_value::<OpenCVTemplateMatchAction>(
                action
                    .ok_or(CustomRecognizerError::InvalidAction {
                        action: action.cloned(),
                    })?
                    .clone(),
            )
            .map_err(|_| CustomRecognizerError::InvalidAction {
                action: action.cloned(),
            })?;

        // Convert ControllerData to image
        let input_image = match data {
            ControllerData::Image(img) => img,
            _ => return Err(CustomRecognizerError::InvalidAction { action: None }),
        };
        let result = OpenCVTemplateMatchRecognizer::exec(input_image, action)?;
        Ok(serde_json::to_value(result).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::TryToCv;
    use crate::OpenCVTemplateMatchRecognizer;
    #[test]
    fn template_match_same_file() {
        let result = OpenCVTemplateMatchRecognizer::template_match(
            image::open(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/resource/template_match/src-1.jpg",
            ))
            .unwrap()
            .try_to_cv()
            .unwrap(),
            image::open(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/resource/template_match/src-1.jpg",
            ))
            .unwrap()
            .try_to_cv()
            .unwrap(),
            None,
        )
        .unwrap();
        assert!(result.position.x == 0 && result.position.y == 0 && result.confidence > 0.95)
    }

    #[test]
    fn template_match_sub_file() {
        let result = OpenCVTemplateMatchRecognizer::template_match(
            image::open(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/resource/template_match/src-1.jpg",
            ))
            .unwrap()
            .try_to_cv()
            .unwrap(),
            image::open(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/resource/template_match/template-1.jpg",
            ))
            .unwrap()
            .try_to_cv()
            .unwrap(),
            None,
        )
        .unwrap();
        assert!(
            result.position.x == 0
                && (result.position.y - 84).abs() < 1
                && result.confidence > 0.95
        )
    }
}
