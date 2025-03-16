use async_trait::async_trait;

use crate::{controller::output::image::ImageOutput, resource::ResourceData};

use super::{CustomRecognizerError, RecognizeResult, Recognizer};

#[async_trait]
pub trait ImageRecognizer: Recognizer {
    async fn exec(
        &self,
        action: &ResourceData,
        data: ImageOutput,
    ) -> Result<RecognizeResult, CustomRecognizerError>;
}
