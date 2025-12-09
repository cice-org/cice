use cice_core::context::ContextBuilder;
use cice_core::task::TaskConfig;
use cice_tests_common::action::{
    ConfigurableAction, DenyAction, SimpleAction, TestParams, TestRuntime,
};
use std::time::Duration;

#[tokio::test]
async fn test_simple_action_sequence() {
    // 创建 Runtime
    let runtime = TestRuntime::new();

    // 创建 Actions
    let action1 = SimpleAction::new("action1");
    let action2 = SimpleAction::new("action2");
    let action3 = SimpleAction::new("action3");

    // 创建 Context Builder
    let mut builder = ContextBuilder::new(runtime);

    // 添加任务
    builder.add_task(
        TaskConfig {
            task_name: "task1".to_string(),
            action_name: "action1".to_string(),
            next_task: vec!["task2".to_string()],
            interrupt_task: vec![],
            timeout: Duration::from_secs(30),
            max_retry: 3,
        },
        &action1,
        TestParams,
    );

    builder.add_task(
        TaskConfig {
            task_name: "task2".to_string(),
            action_name: "action2".to_string(),
            next_task: vec!["task3".to_string()],
            interrupt_task: vec![],
            timeout: Duration::from_secs(30),
            max_retry: 3,
        },
        &action2,
        TestParams,
    );

    builder.add_task(
        TaskConfig {
            task_name: "task3".to_string(),
            action_name: "action3".to_string(),
            next_task: vec![],
            interrupt_task: vec![],
            timeout: Duration::from_secs(30),
            max_retry: 3,
        },
        &action3,
        TestParams,
    );

    // 构建并运行
    let context = builder.build();
    let result = context.run("task1".to_string()).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_action_with_deny() {
    let runtime = TestRuntime::new();

    let action_accept = SimpleAction::new("accept");
    let action_deny = DenyAction::new("deny");

    let mut builder = ContextBuilder::new(runtime);

    // 添加一个会失败的任务和一个会成功的任务
    builder.add_task(
        TaskConfig {
            task_name: "entry".to_string(),
            action_name: "entry_action".to_string(),
            next_task: vec!["task_deny".to_string(), "task_accept".to_string()],
            interrupt_task: vec![],
            timeout: Duration::from_secs(30),
            max_retry: 3,
        },
        &action_accept,
        TestParams,
    );

    builder.add_task(
        TaskConfig {
            task_name: "task_deny".to_string(),
            action_name: "deny_action".to_string(),
            next_task: vec![],
            interrupt_task: vec![],
            timeout: Duration::from_secs(30),
            max_retry: 3,
        },
        &action_deny,
        TestParams,
    );

    builder.add_task(
        TaskConfig {
            task_name: "task_accept".to_string(),
            action_name: "accept_action".to_string(),
            next_task: vec![],
            interrupt_task: vec![],
            timeout: Duration::from_secs(30),
            max_retry: 3,
        },
        &action_accept,
        TestParams,
    );

    let context = builder.build();
    let result = context.run("entry".to_string()).await;

    // 应该成功，因为有一个 accept 任务
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_configurable_action() {
    let runtime = TestRuntime::new();

    let action_success = ConfigurableAction::new("success", true);
    let action_fail = ConfigurableAction::new("fail", false);

    let mut builder = ContextBuilder::new(runtime);

    builder.add_task(
        TaskConfig {
            task_name: "entry".to_string(),
            action_name: "entry_action".to_string(),
            next_task: vec!["task_fail".to_string(), "task_success".to_string()],
            interrupt_task: vec![],
            timeout: Duration::from_secs(30),
            max_retry: 3,
        },
        &action_success,
        TestParams,
    );

    builder.add_task(
        TaskConfig {
            task_name: "task_fail".to_string(),
            action_name: "fail_action".to_string(),
            next_task: vec![],
            interrupt_task: vec![],
            timeout: Duration::from_secs(30),
            max_retry: 3,
        },
        &action_fail,
        TestParams,
    );

    builder.add_task(
        TaskConfig {
            task_name: "task_success".to_string(),
            action_name: "success_action".to_string(),
            next_task: vec![],
            interrupt_task: vec![],
            timeout: Duration::from_secs(30),
            max_retry: 3,
        },
        &action_success,
        TestParams,
    );

    let context = builder.build();
    let result = context.run("entry".to_string()).await;

    assert!(result.is_ok());
}
