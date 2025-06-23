use cice_core::context::ContextBuilder;
use cice_core::message::{task::TaskMessage, Message};

use cice_tests_common::controller::{
    ControllerWithInputAndOutputAction, DummyController, SimpleImageController,
};
use cice_tests_common::recognizer::SimpleImageInputRecognizer;
use cice_tests_common::recognizer::{
    AcceptAllRecognizer, DenyAllRecognizer, SimpleRecognizerWithAction, SimpleRecognizerWithConfig,
};
use cice_tests_common::{
    controller::SimpleControllerWithConfig,
    recognizer::AssertImageRecognizer,
    task::{Tasks, TestTaskData},
    Config,
};

#[tokio::test]
async fn config() {
    let mut builder = ContextBuilder::new();
    let config_str = include_str!("task_config/json/base_config.json");
    let base_config: Config = serde_json::from_str(config_str).unwrap();
    builder.add_controller((
        Box::new(SimpleControllerWithConfig::new(
            base_config.controller.clone().unwrap().try_into().unwrap(),
        )),
        serde_json::to_value(base_config.controller.unwrap()).unwrap(),
    ));
    builder.add_recognizer((
        Box::new(SimpleRecognizerWithConfig::new(
            base_config.recognizer.clone().unwrap().try_into().unwrap(),
        )),
        serde_json::to_value(base_config.recognizer.unwrap()).unwrap(),
    ));
    builder.add_recognizer((Box::new(AcceptAllRecognizer {}), serde_json::json!({})));
    let task_config = include_str!("task_config/json/base_task.json");
    let task_datas: Tasks = serde_json::from_str(task_config).unwrap();
    let task_datas: Vec<TestTaskData> = task_datas.into();
    for task in task_datas {
        builder.add_task(task);
    }
    let ret = builder.build().run("test".to_string()).await;
    println!("{ret:?}");
    assert!(ret.is_ok())
}

#[tokio::test]
async fn task_sequence() {
    let mut builder = ContextBuilder::new();
    let config_str = include_str!("task_config/json/base_config.json");
    let base_config: Config = serde_json::from_str(config_str).unwrap();
    builder.add_controller((
        Box::new(SimpleControllerWithConfig::new(
            base_config.controller.clone().unwrap().try_into().unwrap(),
        )),
        serde_json::to_value(base_config.controller.unwrap()).unwrap(),
    ));
    builder.add_recognizer((
        Box::new(AssertImageRecognizer {}),
        serde_json::to_value(base_config.recognizer.unwrap()).unwrap(),
    ));
    builder.add_recognizer((Box::new(AcceptAllRecognizer {}), serde_json::json!({})));
    builder.add_recognizer((Box::new(DenyAllRecognizer {}), serde_json::json!({})));
    // Load task sequence config
    let task_config = include_str!("task_config/json/task_sequence.json");
    let task_datas: Tasks = serde_json::from_str(task_config).unwrap();
    let task_datas: Vec<TestTaskData> = task_datas.into();

    for task in task_datas {
        builder.add_task(task);
    }

    let context = builder.build();
    let handler = context.get_handler();

    // Spawn task to collect messages
    let message_task = tokio::spawn(async move {
        let mut messages = Vec::new();
        while let Ok(msg) = handler.try_recv() {
            if let Message::TaskMessage(task_msg) = msg {
                messages.push(task_msg);
            }
        }
        messages
    });

    // Run the task sequence
    let ret = context.run("Task1".to_string()).await;
    println!("ret: {ret:?}");
    assert!(ret.is_ok());

    // Get collected messages
    let messages = message_task.await.unwrap();
    println!("message :{messages:?}");
    // Verify task execution order
    let expected_order = vec![
        "Task1".to_string(),
        "Task2".to_string(),
        "Task3".to_string(),
    ];

    let mut executed_tasks = Vec::new();
    for msg in messages {
        match msg {
            TaskMessage::Enter { id } => executed_tasks.push(id),
            _ => continue,
        }
    }

    assert_eq!(executed_tasks, expected_order);
}

#[tokio::test]
async fn simple_image() {
    let mut builder = ContextBuilder::new();
    let config_str = include_str!("task_config/json/base_config.json");
    let base_config: Config = serde_json::from_str(config_str).unwrap();
    builder.add_controller((
        Box::new(SimpleImageController::new()),
        serde_json::to_value(base_config.controller.unwrap()).unwrap(),
    ));
    builder.add_recognizer((
        Box::new(SimpleImageInputRecognizer {}),
        serde_json::to_value(base_config.recognizer.unwrap()).unwrap(),
    ));
    builder.add_recognizer((Box::new(AcceptAllRecognizer {}), serde_json::json!({})));
    let task_config = include_str!("task_config/json/simple_image.json");
    let task_datas: Tasks = serde_json::from_str(task_config).unwrap();
    let task_datas: Vec<TestTaskData> = task_datas.into();
    for task in task_datas {
        builder.add_task(task);
    }
    let ret = builder.build().run("entry".to_string()).await;
    println!("{ret:?}");
    assert!(ret.is_ok())
}

#[tokio::test]
async fn controller_input_and_output_action() {
    let mut builder = ContextBuilder::new();
    let config_str = include_str!("task_config/json/base_config.json");
    let base_config: Config = serde_json::from_str(config_str).unwrap();
    builder.add_controller((
        Box::new(ControllerWithInputAndOutputAction::new()),
        serde_json::to_value(base_config.controller.unwrap()).unwrap(),
    ));
    builder.add_recognizer((Box::new(AcceptAllRecognizer {}), serde_json::json!({})));
    let task_config = include_str!("task_config/json/controller_input_and_output_action.json");
    let task_datas: Tasks = serde_json::from_str(task_config).unwrap();
    let task_datas: Vec<TestTaskData> = task_datas.into();
    for task in task_datas {
        builder.add_task(task);
    }
    let ret = builder.build().run("entry".to_string()).await;
    println!("{ret:?}");
    assert!(ret.is_ok())
}

#[tokio::test]
async fn recognizer_simple_with_action() {
    let mut builder = ContextBuilder::new();
    let config_str = include_str!("task_config/json/base_config.json");
    let base_config: Config = serde_json::from_str(config_str).unwrap();
    builder.add_controller((
        Box::new(DummyController::new()),
        serde_json::to_value(base_config.controller.unwrap()).unwrap(),
    ));
    builder.add_recognizer((Box::new(AcceptAllRecognizer {}), serde_json::json!({})));
    builder.add_recognizer((
        Box::new(SimpleRecognizerWithAction {}),
        serde_json::json!({}),
    ));
    let task_config = include_str!("task_config/json/recognizer_simple_with_action.json");
    let task_datas: Tasks = serde_json::from_str(task_config).unwrap();
    let task_datas: Vec<TestTaskData> = task_datas.into();
    for task in task_datas {
        builder.add_task(task);
    }
    let ret = builder.build().run("entry".to_string()).await;
    println!("{ret:?}");
    assert!(ret.is_ok())
}
