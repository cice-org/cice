use cice_core::task::{BaseTaskData, TaskData};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TestTaskData {
    base_data: BaseTaskData,
}

impl TaskData for TestTaskData {
    fn base_data(&self) -> cice_core::task::BaseTaskData {
        return self.base_data.clone();
    }

    fn controller_config(&self) -> cice_core::resource::ResourceData {
        return cice_core::resource::ResourceData::Json("".to_string());
    }

    fn recognizer_config(&self) -> Option<cice_core::resource::ResourceData> {
        return None;
    }
}
