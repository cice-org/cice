use std::collections::HashMap;

use cice_core::task::TaskData;
use cice_core::{resource::ResourceData, task::BaseTaskConfig};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BaseTaskConfigContent {
    pub next_task: Vec<String>,
    pub interrupt_task: Vec<String>,
    pub controller_id: String,
    pub recognizer_id: String,
    pub controller_input: Option<ResourceData>,
    pub controller_output: Option<ResourceData>,
    #[serde(rename = "reco")]
    pub recognizer_action: Option<ResourceData>,
}

#[derive(Serialize, Deserialize)]
pub struct Tasks(HashMap<String, BaseTaskConfigContent>);

impl From<Tasks> for Vec<TestTaskData> {
    fn from(value: Tasks) -> Self {
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
                controller_input: task.1.controller_input,
                controller_output: task.1.controller_output,
                recognizer_action: task.1.recognizer_action,
            });
        }
        vec
    }
}

#[derive(Serialize, Deserialize)]
pub struct TestTaskData {
    base: BaseTaskConfig,
    controller_input: Option<ResourceData>,
    controller_output: Option<ResourceData>,
    recognizer_action: Option<ResourceData>,
}

impl TaskData for TestTaskData {
    fn base_data(&self) -> BaseTaskConfig {
        self.base.clone()
    }

    fn controller_config_ext(&self) -> Option<cice_core::resource::ResourceData> {
        None
    }

    fn recognizer_config_ext(&self) -> Option<cice_core::resource::ResourceData> {
        None
    }

    fn controller_output_action_ext(&self) -> Option<cice_core::resource::ResourceData> {
        self.controller_output.clone()
    }

    fn controller_input_action_ext(&self) -> Option<cice_core::resource::ResourceData> {
        self.controller_input.clone()
    }

    fn recognizer_action(&self) -> Option<cice_core::resource::ResourceData> {
        self.recognizer_action.clone()
    }
}
