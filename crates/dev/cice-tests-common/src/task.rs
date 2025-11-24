use cice_core::task::TaskConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// 简化的任务配置内容，用于从 JSON 反序列化
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BaseTaskConfigContent {
    pub next_task: Vec<String>,
    pub interrupt_task: Vec<String>,
    pub action_name: String,
    #[serde(default = "default_timeout_secs")]
    pub timeout_secs: u64,
    #[serde(default = "default_max_retry")]
    pub max_retry: usize,
}

fn default_timeout_secs() -> u64 {
    30
}

fn default_max_retry() -> usize {
    3
}

/// 任务集合，用于从 JSON 反序列化
#[derive(Serialize, Deserialize)]
pub struct Tasks(pub HashMap<String, BaseTaskConfigContent>);

impl From<Tasks> for Vec<TaskConfig> {
    fn from(value: Tasks) -> Self {
        let mut vec = vec![];
        for (task_name, content) in value.0 {
            vec.push(TaskConfig {
                task_name,
                action_name: content.action_name,
                next_task: content.next_task,
                interrupt_task: content.interrupt_task,
                timeout: Duration::from_secs(content.timeout_secs),
                max_retry: content.max_retry,
            });
        }
        vec
    }
}
