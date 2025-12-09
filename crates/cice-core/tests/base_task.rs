use cice_core::context::ContextBuilder;
use cice_core::message::{task::TaskMessage, Message};
use cice_core::task::TaskConfig;
use cice_tests_common::action::{DenyAction, SimpleAction, TestParams, TestRuntime};
use cice_tests_common::task::Tasks;

#[tokio::test]
async fn config() {
    // 创建 Runtime
    let runtime = TestRuntime::new();

    // 创建 Action
    let simple_action = SimpleAction::new("simple_action");

    // 创建 Context Builder
    let mut builder = ContextBuilder::new(runtime);

    // 从 JSON 加载任务配置
    let task_config = include_str!("task_config/json/base_task.json");
    let tasks: Tasks = serde_json::from_str(task_config).unwrap();
    let task_configs: Vec<TaskConfig> = tasks.into();

    // 添加任务
    for config in task_configs {
        builder.add_task(config, &simple_action, TestParams);
    }

    // 构建并运行
    let ret = builder.build().run("test".to_string()).await;
    println!("{ret:?}");
    assert!(ret.is_ok())
}

#[tokio::test]
async fn task_sequence() {
    // 创建 Runtime
    let runtime = TestRuntime::new();

    // 创建 Actions
    let accept_action = SimpleAction::new("accept_action");
    let deny_action = DenyAction::new("deny_action");

    // 创建 Context Builder
    let mut builder = ContextBuilder::new(runtime);

    // 从 JSON 加载任务配置
    let task_config = include_str!("task_config/json/task_sequence.json");
    let tasks: Tasks = serde_json::from_str(task_config).unwrap();
    let task_configs: Vec<TaskConfig> = tasks.into();

    // 添加任务 - 根据 action_name 分配对应的 action
    for config in task_configs {
        match config.action_name.as_str() {
            "accept_action" => builder.add_task(config, &accept_action, TestParams),
            "deny_action" => builder.add_task(config, &deny_action, TestParams),
            _ => panic!("Unknown action name: {}", config.action_name),
        };
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
    // 创建 Runtime
    let runtime = TestRuntime::new();

    // 创建 Actions
    let simple_action = SimpleAction::new("simple_action");
    let simple_image_action = SimpleAction::new("simple_image_action");

    // 创建 Context Builder
    let mut builder = ContextBuilder::new(runtime);

    // 从 JSON 加载任务配置
    let task_config = include_str!("task_config/json/simple_image.json");
    let tasks: Tasks = serde_json::from_str(task_config).unwrap();
    let task_configs: Vec<TaskConfig> = tasks.into();

    // 添加任务
    for config in task_configs {
        match config.action_name.as_str() {
            "simple_action" => builder.add_task(config, &simple_action, TestParams),
            "simple_image_action" => builder.add_task(config, &simple_image_action, TestParams),
            _ => panic!("Unknown action name: {}", config.action_name),
        };
    }

    let ret = builder.build().run("entry".to_string()).await;
    println!("{ret:?}");
    assert!(ret.is_ok())
}

#[tokio::test]
async fn controller_input_and_output_action() {
    // 创建 Runtime
    let runtime = TestRuntime::new();

    // 创建 Actions
    let simple_action = SimpleAction::new("simple_action");
    let input_output_action = SimpleAction::new("input_output_action");

    // 创建 Context Builder
    let mut builder = ContextBuilder::new(runtime);

    // 从 JSON 加载任务配置
    let task_config = include_str!("task_config/json/controller_input_and_output_action.json");
    let tasks: Tasks = serde_json::from_str(task_config).unwrap();
    let task_configs: Vec<TaskConfig> = tasks.into();

    // 添加任务
    for config in task_configs {
        match config.action_name.as_str() {
            "simple_action" => builder.add_task(config, &simple_action, TestParams),
            "input_output_action" => builder.add_task(config, &input_output_action, TestParams),
            _ => panic!("Unknown action name: {}", config.action_name),
        };
    }

    let ret = builder.build().run("entry".to_string()).await;
    println!("{ret:?}");
    assert!(ret.is_ok())
}

#[tokio::test]
async fn recognizer_simple_with_action() {
    // 创建 Runtime
    let runtime = TestRuntime::new();

    // 创建 Actions
    let simple_action = SimpleAction::new("simple_action");
    let action_with_area = SimpleAction::new("action_with_area");

    // 创建 Context Builder
    let mut builder = ContextBuilder::new(runtime);

    // 从 JSON 加载任务配置
    let task_config = include_str!("task_config/json/recognizer_simple_with_action.json");
    let tasks: Tasks = serde_json::from_str(task_config).unwrap();
    let task_configs: Vec<TaskConfig> = tasks.into();

    // 添加任务
    for config in task_configs {
        match config.action_name.as_str() {
            "simple_action" => builder.add_task(config, &simple_action, TestParams),
            "action_with_area" => builder.add_task(config, &action_with_area, TestParams),
            _ => panic!("Unknown action name: {}", config.action_name),
        };
    }

    let ret = builder.build().run("entry".to_string()).await;
    println!("{ret:?}");
    assert!(ret.is_ok())
}
