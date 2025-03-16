#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BaseTaskConfig {
    pub task_name: String,
    pub next_task: Vec<String>,
    pub interrupt_task: Vec<String>,
    pub controller_id: String,
    pub recognizer_id: String,
}
