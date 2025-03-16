use serde::{Deserialize, Serialize};
use task::TaskMessage;

pub mod task;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    TaskMessage(TaskMessage),
}
