use crate::runtime::Runtime;

pub trait ScreenCaptureExt: Runtime {
    fn capture_screen(&self) -> Option<&[u8]>;
}
