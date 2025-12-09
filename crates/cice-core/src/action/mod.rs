use async_trait::async_trait;
use snafu::Snafu;

use crate::runtime::Runtime;

pub type ActionId = String;

pub trait ActionParams: Send + Sync {}

/// Action is the behavior that a Task will perform. Basically it contains two stages:
/// 1. recognize: check whether the precondition of this action is satisfied. Such as whether a specific UI element appears on the screen.
/// 2. exec: execute the action.
///
/// An action object has one concept:
/// implementation: the actual implementation of the action, containing deserializing logic for the parameters.
///
#[async_trait]
pub trait Action<RUNTIME: Runtime, PARAMS: ActionParams>: Send + Sync {
    async fn recognize(&self, runtime: &RUNTIME, params: &PARAMS) -> Result<(), RecognizeError>;
    async fn exec(&self, runtime: &RUNTIME, params: &PARAMS) -> Result<(), ExecError>;
}

#[derive(Debug, Snafu)]
pub enum RecognizeError {
    #[snafu(display("action unrecognized"))]
    UnRecognized, // Different from RecognizeFailed, this means the action is not recognized, but it's not an error.
    #[snafu(display("action recognize failed reason:{reason}"))]
    RecognizeFailed { reason: String }, // Different from UnRecognized, this means an error occurred during the recognition process.
}

#[derive(Debug, Snafu)]
pub enum ExecError {
    #[snafu(display("action exec failed reason:{reason}"))]
    ExecFailed { reason: String }, // An error occurred during the execution of the action.
}

#[derive(Debug, Snafu)]
pub enum ActionError {
    #[snafu(display("action recognize error {source}"))]
    RecognizeError { source: RecognizeError },
    #[snafu(display("action exec error {source}"))]
    ExecError { source: ExecError },
}

impl From<RecognizeError> for ActionError {
    fn from(source: RecognizeError) -> Self {
        ActionError::RecognizeError { source }
    }
}

impl From<ExecError> for ActionError {
    fn from(source: ExecError) -> Self {
        ActionError::ExecError { source }
    }
}
