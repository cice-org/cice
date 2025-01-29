use cice_core::context::Context;
use common::{controller::TestController, task::TestTaskData, TestConfig};

mod common;

#[tokio::test]
async fn config() {
    let mut context = Context::new();
    let config_str = include_str!("task_config/json/base_config.json");
    let base_config: TestConfig = serde_json::from_str(config_str).unwrap();
    context.insert_controller((
        Box::new(TestController {}),
        serde_json::to_value(base_config.controller.unwrap()).unwrap(),
    ));
    let task_config = include_str!("task_config/json/base_resource.json");
    let task_data: TestTaskData = serde_json::from_str(task_config).unwrap();
    context.insert_task(task_data);
    let ret: Result<(), cice_core::task::TaskError> = context.run("test".to_string()).await;
    assert!(ret.is_ok())
}
