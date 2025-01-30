use cice_core::context::ContextBuilder;
use common::{controller::TestController, task::TestTaskData, TestConfig};

mod common;

#[tokio::test]
async fn config() {
    let mut builder = ContextBuilder::new();
    let config_str = include_str!("task_config/json/base_config.json");
    let base_config: TestConfig = serde_json::from_str(config_str).unwrap();
    builder.add_controller((
        Box::new(TestController {}),
        serde_json::to_value(base_config.controller.unwrap()).unwrap(),
    ));
    let task_config = include_str!("task_config/json/base_resource.json");
    let task_data: TestTaskData = serde_json::from_str(task_config).unwrap();
    builder.add_task(task_data);
    let ret: Result<(), cice_core::task::TaskError> = builder.build().run("test".to_string()).await;
    assert!(ret.is_ok())
}
