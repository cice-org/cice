use serde::{Deserialize, Serialize};
use snafu::Snafu;

use crate::{action::ActionId, task::TaskId};

#[derive(Debug, Clone, Serialize, Deserialize, Snafu)]
pub enum TaskMessage {
    #[snafu(display("enter task {id}"))]
    Enter { id: TaskId },
    #[snafu(display("try recognize action {id}"))]
    TryRecognize { id: ActionId },
    #[snafu(display("try exec action {id}"))]
    TryExec { id: ActionId },
    #[snafu(display("exec task {id} successfully"))]
    ExecSuccess { id: TaskId },
}
