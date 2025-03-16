use serde::{Deserialize, Serialize};
use snafu::Snafu;

use crate::{controller::ControllerId, recognizer::RecognizerId, task::TaskId};

#[derive(Debug, Clone, Serialize, Deserialize, Snafu)]
pub enum TaskMessage {
    #[snafu(display("enter task {id}"))]
    Enter { id: TaskId },
    #[snafu(display("try exec task {id}"))]
    TryExec { id: TaskId },
    #[snafu(display("exec controller {controller_id} in task {task_id}"))]
    ExecController {
        task_id: TaskId,
        controller_id: ControllerId,
    },
    #[snafu(display("exec recognizer {recognizer_id} in task {task_id}"))]
    ExecRecognizer {
        task_id: TaskId,
        recognizer_id: RecognizerId,
    },
    #[snafu(display("exec task {id} successfully"))]
    ExecSuccess { id: TaskId },
}
