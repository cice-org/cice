use cice_core::{config::BaseTaskConfig, context::ContextBuilder};
use common::{
    controller::TestController,
    recognizer::TestImageRecognizer,
    task::{TestTaskData, TestTasks},
    TestConfig,
};

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
    builder.add_recognizer((
        Box::new(TestImageRecognizer {}),
        serde_json::to_value(base_config.recognizer.unwrap()).unwrap(),
    ));
    let task_config = include_str!("task_config/json/tasks.json");
    let task_datas: TestTasks = serde_json::from_str(task_config).unwrap();
    let task_datas: Vec<TestTaskData> = task_datas.into();
    for task in task_datas {
        builder.add_task(task);
    }
    let ret: Result<(), cice_core::task::TaskError> = builder.build().run("test".to_string()).await;
    println!("{ret:?}");
    assert!(ret.is_ok())
}
