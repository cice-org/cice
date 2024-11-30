use cice_core::context::Context;
use common::TestTaskData;
mod common;

#[tokio::test]
async fn resource_data() {
    let mut context = Context::new();
    let task_config = include_str!("task_config/json/base_resourcce.json");
    let task_data: TestTaskData = serde_json::from_str(task_config).unwrap();
    context.insert_task(task_data);
    let ret = context.run("test".to_string()).await;
    assert!(ret.is_ok())
}
