use async_trait::async_trait;
use cice_core::runtime::ext::ScreenshotExt;

use crate::VncRuntime;

/// 为 VncRuntime 实现 ScreenshotExt trait
#[async_trait]
impl ScreenshotExt for VncRuntime {
    async fn screenshot(&self) -> Option<Vec<u8>> {
        self.screenshot().await.ok()
    }
}
