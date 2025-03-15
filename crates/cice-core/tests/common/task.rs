use std::collections::HashMap;

use cice_core::{
    config::{BaseTaskConfig, BaseTaskConfigContent},
    task::TaskData,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct TestTasks(HashMap<String, BaseTaskConfigContent>);

impl From<TestTasks> for Vec<TestTaskData> {
    fn from(value: TestTasks) -> Self {
        let mut vec = vec![];
        for task in value.0 {
            vec.push(TestTaskData {
                base: BaseTaskConfig {
                    task_name: task.0,
                    next_task: task.1.next_task,
                    interrupt_task: task.1.interrupt_task,
                    controller_id: task.1.controller_id,
                    recognizer_id: task.1.recognizer_id,
                },
            });
        }
        return vec;
    }
}

#[derive(Serialize, Deserialize)]
pub struct TestTaskData {
    base: BaseTaskConfig,
}

impl TaskData for TestTaskData {
    fn base_data(&self) -> BaseTaskConfig {
        return self.base.clone();
    }

    fn controller_config_ext(&self) -> Option<cice_core::resource::ResourceData> {
        return None;
    }

    fn recognizer_config_ext(&self) -> Option<cice_core::resource::ResourceData> {
        return None;
    }

    fn controller_output_action_ext(&self) -> Option<cice_core::resource::ResourceData> {
        None
    }

    fn recognizer_action(&self) -> cice_core::resource::ResourceData {
        return json!({});
    }
}
