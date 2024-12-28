use cice_core::{context::Context, resource::ResourceData};
use common::{controller::TestController, task::TestTaskData, TestConfig};
use prost::Message;
mod common;

mod config {
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

#[tokio::test]
async fn config() {
    let mut context = Context::new();
    let config_str = include_str!("task_config/json/base_config.json");
    let base_config: TestConfig = serde_json::from_str(config_str).unwrap();
    context.insert_controller((
        Box::new(TestController {}),
        ResourceData::Proto(
            base_config
                .controller
                .unwrap()
                .encode_length_delimited_to_vec(),
        ),
    ));
    let task_config = include_str!("task_config/json/base_resource.json");
    let task_data: TestTaskData = serde_json::from_str(task_config).unwrap();
    context.insert_task(task_data);
    let ret: Result<(), cice_core::task::TaskError> = context.run("test".to_string()).await;
    assert!(ret.is_ok())
}
