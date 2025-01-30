use cice_core::{config::BaseTaskConfig, task::TaskData};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct TestTaskData {
    base: BaseTaskConfig,
}

impl TaskData for TestTaskData {
    fn base_data(&self) -> BaseTaskConfig {
        return self.base.clone();
    }

    fn controller_config(&self) -> cice_core::resource::ResourceData {
        return json!({});
    }

    fn recognizer_config(&self) -> Option<cice_core::resource::ResourceData> {
        return None;
    }

    fn controller_output_action(&self) -> Option<cice_core::resource::ResourceData> {
        None
    }

    fn recognizer_action(&self) -> cice_core::resource::ResourceData {
        return json!({});
    }
}
