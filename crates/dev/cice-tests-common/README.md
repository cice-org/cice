# Cice Tests Common

测试和开发专用的公共模块。

## 概述

本模块提供了用于测试 cice-core 的通用工具和示例实现，包括：

- 测试用的 Runtime 实现
- 示例 Action 实现
- 测试辅助函数和工具

## 架构适配说明

本模块已适配新的 **Runtime-Action** 架构。旧的 Controller-Recognizer 模式已被移除。

### 新架构核心组件

#### 1. TestRuntime

测试用的 Runtime 实现，提供基础的运行时环境：

```rust
use cice_tests_common::action::TestRuntime;

let runtime = TestRuntime::new();
```

#### 2. Action 实现

提供了多种测试用的 Action 实现：

##### SimpleAction
总是成功的简单 Action：

```rust
use cice_tests_common::action::SimpleAction;

let action = SimpleAction::new("my_action");
```

##### DenyAction
总是识别失败的 Action（用于测试失败场景）：

```rust
use cice_tests_common::action::DenyAction;

let action = DenyAction::new("deny_action");
```

##### ConfigurableAction
可配置成功/失败的 Action：

```rust
use cice_tests_common::action::ConfigurableAction;

let action_success = ConfigurableAction::new("success", true);
let action_fail = ConfigurableAction::new("fail", false);
```

### 使用示例

#### 基本测试

```rust
use cice_core::context::ContextBuilder;
use cice_core::task::TaskConfig;
use cice_tests_common::action::{SimpleAction, TestRuntime};
use std::time::Duration;

#[tokio::test]
async fn test_simple_task() {
    // 1. 创建 Runtime
    let runtime = TestRuntime::new();

    // 2. 创建 Action
    let action = SimpleAction::new("my_action");

    // 3. 创建 Context Builder
    let mut builder = ContextBuilder::new(runtime);

    // 4. 添加任务
    builder.add_task(
        TaskConfig {
            task_name: "task1".to_string(),
            action_name: "action1".to_string(),
            next_task: vec![],
            interrupt_task: vec![],
            timeout: Duration::from_secs(30),
            max_retry: 3,
        },
        &action,
    );

    // 5. 构建并运行
    let context = builder.build();
    let result = context.run("task1".to_string()).await;

    assert!(result.is_ok());
}
```

#### 任务链测试

```rust
#[tokio::test]
async fn test_task_chain() {
    let runtime = TestRuntime::new();

    let action1 = SimpleAction::new("action1");
    let action2 = SimpleAction::new("action2");

    let mut builder = ContextBuilder::new(runtime);

    // 任务1 -> 任务2
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
    );

    builder.add_task(
        TaskConfig {
            task_name: "task2".to_string(),
            action_name: "action2".to_string(),
            next_task: vec![],
            interrupt_task: vec![],
            timeout: Duration::from_secs(30),
            max_retry: 3,
        },
        &action2,
    );

    let context = builder.build();
    let result = context.run("task1".to_string()).await;

    assert!(result.is_ok());
}
```

#### 多分支测试

```rust
#[tokio::test]
async fn test_multiple_branches() {
    let runtime = TestRuntime::new();

    let action_accept = SimpleAction::new("accept");
    let action_deny = DenyAction::new("deny");

    let mut builder = ContextBuilder::new(runtime);

    // entry -> [task_deny, task_accept]
    // task_deny 会失败，task_accept 会成功
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
    );

    let context = builder.build();
    let result = context.run("entry".to_string()).await;

    // 应该成功，因为 task_accept 会成功
    assert!(result.is_ok());
}
```

### 从 JSON 加载任务配置

```rust
use cice_tests_common::task::Tasks;
use std::fs;

let json_str = fs::read_to_string("tasks.json").unwrap();
let tasks: Tasks = serde_json::from_str(&json_str).unwrap();
let task_configs: Vec<TaskConfig> = tasks.into();

for config in task_configs {
    builder.add_task(config, &action);
}
```

JSON 格式示例：

```json
{
  "task1": {
    "action_name": "action1",
    "next_task": ["task2"],
    "interrupt_task": [],
    "timeout_secs": 30,
    "max_retry": 3
  },
  "task2": {
    "action_name": "action2",
    "next_task": [],
    "interrupt_task": [],
    "timeout_secs": 30,
    "max_retry": 3
  }
}
```

## 迁移指南

### 从旧架构迁移

如果你有使用旧的 Controller-Recognizer 架构的测试代码，需要进行以下迁移：

#### 1. 移除 Controller 和 Recognizer

**旧代码：**
```rust
builder.add_controller((Box::new(MyController::new()), config));
builder.add_recognizer((Box::new(MyRecognizer::new()), config));
```

**新代码：**
```rust
let runtime = TestRuntime::new();
let action = MyAction::new();
builder.add_task(task_config, &action);
```

#### 2. 更新 TaskConfig

**旧代码：**
```rust
TaskConfig {
    task_name: "task1".to_string(),
    next_task: vec!["task2".to_string()],
    interrupt_task: vec![],
    controller_id: "my_controller".to_string(),
    recognizer_id: "my_recognizer".to_string(),
}
```

**新代码：**
```rust
TaskConfig {
    task_name: "task1".to_string(),
    action_name: "my_action".to_string(),
    next_task: vec!["task2".to_string()],
    interrupt_task: vec![],
    timeout: Duration::from_secs(30),
    max_retry: 3,
}
```

#### 3. 实现自定义 Action

如果需要自定义 Action，实现 `Action<TestRuntime>` trait：

```rust
use async_trait::async_trait;
use cice_core::action::{Action, ExecError, RecognizeError};
use cice_tests_common::action::TestRuntime;

pub struct MyCustomAction {
    // 你的字段
}

#[async_trait]
impl Action<TestRuntime> for MyCustomAction {
    async fn recognize(&self, runtime: &TestRuntime) -> Result<(), RecognizeError> {
        // 实现识别逻辑
        Ok(())
    }

    async fn exec(&self, runtime: &TestRuntime) -> Result<(), ExecError> {
        // 实现执行逻辑
        Ok(())
    }
}
```

## 运行测试

```bash
# 运行所有测试
cargo test -p cice-tests-common

# 运行特定测试
cargo test -p cice-tests-common test_simple_action_sequence

# 查看测试输出
cargo test -p cice-tests-common -- --nocapture
```

## 参考资料

- [Runtime 重构文档](../../docs/refactor/runtime-refactor.md)
- [cice-core API 文档](../cice-core/README.md)
- [示例测试](tests/action_tests.rs)
