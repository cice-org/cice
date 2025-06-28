use std::net::{IpAddr, SocketAddr};

use adb_client::{ADBDeviceExt, ADBTcpDevice, RustADBError};

use async_trait::async_trait;
use cice_action::{
    controller::{
        input::{ClickPostion, ControllerInputAction},
        output::ControllerOutputAction,
    },
    types::Point,
};
use cice_core::{
    controller::{Controller, ControllerData, CustomControllerError},
    resource::ResourceData,
};
use futures::lock::Mutex;

pub struct AdbController {
    device: Mutex<ADBTcpDevice>,
}

impl AdbController {
    pub fn new(ip: IpAddr, port: u16) -> Result<Self, RustADBError> {
        ADBTcpDevice::new(SocketAddr::new(ip, port)).map(|device| Self {
            device: Mutex::new(device),
        })
    }

    async fn tap(&self, pos: ClickPostion) -> Result<(), RustADBError> {
        let mut device = self.device.lock().await;
        let coordinates = match pos {
            ClickPostion::Point(point2_d) => point2_d,
            ClickPostion::Rect(box2_d) => box2_d.center(),
        };
        device.shell_command(
            &[
                "input",
                "tap",
                coordinates.x.to_string().as_str(),
                coordinates.y.to_string().as_str(),
            ],
            &mut std::io::stdout(),
        )
    }

    async fn swipe(&self, from: Point, to: Point) -> Result<(), RustADBError> {
        let mut device = self.device.lock().await;
        device.shell_command(
            &[
                "input",
                "swipe",
                from.x.to_string().as_str(),
                from.y.to_string().as_str(),
                to.x.to_string().as_str(),
                to.y.to_string().as_str(),
            ],
            &mut std::io::stdout(),
        )
    }

    async fn keyevent(&self, key: usize) -> Result<(), RustADBError> {
        let mut device = self.device.lock().await;
        device.shell_command(
            &["input", "keyevent", key.to_string().as_str()],
            &mut std::io::stdout(),
        )
    }

    async fn screen_capture(
        &self,
    ) -> Result<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, RustADBError> {
        self.device.lock().await.framebuffer_inner()
    }
}

#[async_trait]
impl Controller for AdbController {
    fn name(&self) -> cice_core::controller::ControllerId {
        "adb".to_string()
    }

    fn init(
        &self,
        _init_cofig: &cice_core::resource::ResourceData,
    ) -> Result<(), cice_core::controller::ControllerError> {
        //TODO support lazy init or reinit here
        Ok(())
    }

    /// Fetch data from target device. Such as screen capture
    async fn exec_output(
        &self,
        action: &ResourceData,
    ) -> Result<ControllerData, CustomControllerError> {
        let action: AdbControllerImageOutputAction = serde_json::from_value(action.clone())
            .map_err(|e| CustomControllerError::Fatal { source: e.into() })?;
        match action {
            ControllerOutputAction::ScreenCapture(_screen_capture_target) => self
                .screen_capture()
                .await
                .map(image::DynamicImage::ImageRgba8),
            _ => todo!(),
        }
        .map_err(|e| CustomControllerError::Common { source: e.into() })
        .map(Into::into)
    }
    /// Input events or data to the target device. Such as click at a position, or input text to the device
    async fn exec_input(&self, action: &ResourceData) -> Result<(), CustomControllerError> {
        let action: AdbControllerInputAction = serde_json::from_value(action.clone())
            .map_err(|e| CustomControllerError::Fatal { source: e.into() })?;
        match action {
            ControllerInputAction::Click { pos } => self.tap(pos).await,
            ControllerInputAction::Swipe { from, to } => self.swipe(from, to).await,
            ControllerInputAction::KeyEvent { id } => self.keyevent(id).await,
            _ => todo!(),
        }
        .map_err(|e| CustomControllerError::Common { source: e.into() })
    }
}

// #[async_trait]
// impl InputController for AdbController {
//     async fn exec(&self, action: &ResourceData) -> Result<(), CustomControllerError> {
//         let action: AdbControllerInputAction = serde_json::from_value(action.clone())
//             .map_err(|e| CustomControllerError::Fatal { source: e.into() })?;
//         match action {
//             ControllerInputAction::Click { pos } => self.tap(pos).await,
//             ControllerInputAction::Swipe { from, to } => self.swipe(from, to).await,
//             ControllerInputAction::KeyEvent { id } => self.keyevent(id).await,
//             _ => todo!(),
//         }
//         .map_err(|e| CustomControllerError::Common { source: e.into() })
//     }
// }

// impl OutputController for AdbController {
//     fn ext_image(&self) -> Option<cice_core::controller::output::ImageOutputControllerOps> {
//         Some(self)
//     }
// }

// #[async_trait]
// impl ImageOutputController for AdbController {
//     async fn exec(&self, action: &ResourceData) -> Result<ImageOutput, CustomControllerError> {
//         let action: AdbControllerImageOutputAction = serde_json::from_value(action.clone())
//             .map_err(|e| CustomControllerError::Fatal { source: e.into() })?;
//         match action {
//             ControllerOutputAction::ScreenCapture(_screen_capture_target) => self
//                 .screen_capture()
//                 .await
//                 .map(image::DynamicImage::ImageRgba8),
//             _ => todo!(),
//         }
//         .map_err(|e| CustomControllerError::Common { source: e.into() })
//     }
// }

type AdbControllerInputAction = ControllerInputAction;
type AdbControllerImageOutputAction = ControllerOutputAction;
