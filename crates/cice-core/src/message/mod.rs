use serde::{Deserialize, Serialize};
use task::TaskMessage;

pub mod task;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive] //It's likely to be extended at any time. So keep this for compatibility
pub enum Message {
    TaskMessage(TaskMessage),
}
