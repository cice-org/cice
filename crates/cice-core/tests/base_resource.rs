use cice_core::{context::Context, task::TaskError};
use common::task::TestTaskData;
mod common;

#[tokio::test]
async fn resource_data() {
    let mut context = Context::new();
    let task_config = include_str!("task_config/json/base_resource.json");
    let task_data: TestTaskData = serde_json::from_str(task_config).unwrap();
    context.insert_task(task_data);
    let ret: Result<(), cice_core::task::TaskError> = context.run("test".to_string()).await;
    assert!(ret.is_ok());
    let ret: Result<(), cice_core::task::TaskError> = context.run("invalid_test".to_string()).await;
    assert!(ret.is_err());
    if let Err(TaskError::UnknownTask(id)) = ret {
        assert!(id == "invalid_test");
    } else {
        assert!(false, "unexpected error type");
    }
}
