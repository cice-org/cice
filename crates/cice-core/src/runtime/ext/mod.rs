use async_trait::async_trait;

use crate::runtime::Runtime;

#[async_trait]
pub trait ScreenshotExt: Runtime {
    async fn screenshot(&self) -> Option<Vec<u8>>;
}
