use async_trait::async_trait;

use crate::resource::ResourceData;

use super::{Controller, CustomControllerError};

#[async_trait]
pub trait InputController: Controller {
    async fn exec(&self, action: &ResourceData) -> Result<(), CustomControllerError>;
}
