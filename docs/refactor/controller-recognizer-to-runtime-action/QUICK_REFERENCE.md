# Runtime-Action 架构快速参考

> 快速查找常用代码片段、API 和迁移对照表

---

## 🚀 5 分钟快速开始

### 1. 创建简单测试

```rust
use cice_core::context::ContextBuilder;
use cice_core::task::TaskConfig;
use cice_tests_common::action::{SimpleAction, TestParams, TestRuntime};
use std::time::Duration;

#[tokio::test]
async fn my_test() {
    // 1. 创建 Runtime
    let runtime = TestRuntime::new();

    // 2. 创建 Action
    let action = SimpleAction::new("my_action");

    // 3. 构建 Context
    let mut builder = ContextBuilder::new(runtime);
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
        TestParams,  // 传递参数
    );

    // 4. 运行
    builder.build().run("task1".to_string()).await.unwrap();
}
```

### 2. 实现自定义 Action

```rust
use async_trait::async_trait;
use cice_core::action::{Action, ActionParams, ExecError, RecognizeError};
use cice_core::runtime::Runtime;

pub struct MyAction {
    name: String,
}

#[async_trait]
impl<R: Runtime, P: ActionParams> Action<R, P> for MyAction {
    async fn recognize(&self, runtime: &R, params: &P) -> Result<(), RecognizeError> {
        // 检查前置条件
        // 如果条件不满足，返回 RecognizeError::UnRecognized
        Ok(())
    }

    async fn exec(&self, runtime: &R, params: &P) -> Result<(), ExecError> {
        // 执行具体动作
        println!("Executing action: {}", self.name);
        Ok(())
    }
}
```

### 3. 自定义参数类型

```rust
use cice_core::action::ActionParams;

#[derive(Clone)]
pub struct MyParams {
    pub target: String,
    pub timeout_ms: u64,
}

impl ActionParams for MyParams {}

// 使用自定义参数
let params = MyParams { target: "button".to_string(), timeout_ms: 5000 };
builder.add_task(config, &action, params);
```

### 4. JSON 配置格式

```json
{
  "task_name": {
    "action_name": "my_action",
    "next_task": ["next_task"],
    "interrupt_task": [],
    "timeout_secs": 30,
    "max_retry": 3
  }
}
```

---

## 📋 核心概念速查

| 概念 | 职责 | 代码示例 |
|------|------|----------|
| **Runtime** | 提供系统资源和基础设施 | `let runtime = TestRuntime::new();` |
| **Action** | 定义行为（识别 + 执行） | `let action = SimpleAction::new("name");` |
| **ActionParams** | 传递给 Action 的参数 | `TestParams` 或自定义类型 |
| **Task** | 关联 Action 和执行配置 | `TaskConfig { task_name, action_name, ... }` |
| **Context** | 管理 Runtime 和 Tasks | `ContextBuilder::new(runtime)` |

---

## 🔄 迁移对照表

### 概念映射

| 旧架构 | 新架构 | 变化说明 |
|--------|--------|----------|
| **Controller** | **Runtime** | 提供资源的部分 → Runtime |
| **Controller** | **Action** | 执行动作的部分 → Action |
| **Recognizer** | **Action** | 统一为 Action |
| `controller_id` | `action_name` | 字段重命名 |
| `recognizer_id` | ~~删除~~ | 不再需要 |
| `Controller.exec_output()` | `Action.exec(params)` | 方法简化，参数分离 |
| `Controller.exec_input()` | ~~删除~~ | 合并到 exec() |
| `Recognizer.exec()` | `Action.recognize(params)` | 方法重命名，参数分离 |

### 代码迁移示例

#### 旧架构代码

```rust
// 1. 创建 Context
let mut builder = ContextBuilder::new();

// 2. 注册 Controller 和 Recognizer
builder.add_controller((Box::new(MyController::new()), config));
builder.add_recognizer((Box::new(MyRecognizer::new()), config));

// 3. 添加 Task
builder.add_task(task_data);

// 4. 构建并运行
let context = builder.build();
context.run("entry".to_string()).await;
```

#### 新架构代码

```rust
// 1. 创建 Runtime 和 Action
let runtime = MyRuntime::new();
let action = MyAction::new();

// 2. 创建 Context
let mut builder = ContextBuilder::new(runtime);

// 3. 添加 Task（直接关联 Action 和参数）
builder.add_task(task_config, &action, params);

// 4. 构建并运行
let context = builder.build();
context.run("entry".to_string()).await;
```

### JSON 配置迁移

#### 旧格式

```json
{
  "task_name": {
    "next_task": ["next"],
    "interrupt_task": [],
    "controller_id": "my_controller",
    "recognizer_id": "my_recognizer",
    "controller_input": { "key": "value" },
    "controller_output": { "key": "value" },
    "reco": { "key": "value" }
  }
}
```

#### 新格式

```json
{
  "task_name": {
    "action_name": "my_action",
    "next_task": ["next"],
    "interrupt_task": [],
    "timeout_secs": 30,
    "max_retry": 3
  }
}
```

**变化说明**：
- ❌ 移除：`controller_id`, `recognizer_id`
- ❌ 移除：`controller_input`, `controller_output`, `reco`（配置现在通过 `params` 参数传递）
- ✅ 新增：`action_name`（标识 Action）
- ✅ 新增：`timeout_secs`（超时时间，可选）
- ✅ 新增：`max_retry`（最大重试次数，可选）

---

## 🎯 测试用 Actions

### SimpleAction - 总是成功

```rust
use cice_tests_common::action::{SimpleAction, TestParams};

let action = SimpleAction::new("my_action");
builder.add_task(config, &action, TestParams);
// recognize() 和 exec() 都会成功
```

### DenyAction - 总是识别失败

```rust
use cice_tests_common::action::{DenyAction, TestParams};

let action = DenyAction::new("my_action");
builder.add_task(config, &action, TestParams);
// recognize() 会返回 RecognizeError::UnRecognized
```

### ConfigurableAction - 可配置成功/失败

```rust
use cice_tests_common::action::{ConfigurableAction, TestParams};

// 成功的 Action
let action = ConfigurableAction::new("my_action", true);

// 失败的 Action
let action = ConfigurableAction::new("my_action", false);
```

### TestParams - 测试用空参数

```rust
use cice_tests_common::action::TestParams;

// 用于不需要特定参数的测试场景
builder.add_task(config, &action, TestParams);
```

---

## 📝 TaskConfig 字段说明

| 字段 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| `task_name` | String | ✅ | - | 任务唯一标识符 |
| `action_name` | String | ✅ | - | Action 标识符 |
| `next_task` | Vec\<String\> | ✅ | - | 成功后的下一个任务列表 |
| `interrupt_task` | Vec\<String\> | ✅ | - | 中断时的任务列表 |
| `timeout` | Duration | ❌ | 30s | 任务超时时间 |
| `max_retry` | u32 | ❌ | 3 | 最大重试次数 |

**JSON 格式**：

```json
{
  "task_name": {
    "action_name": "my_action",
    "next_task": ["task2", "task3"],
    "interrupt_task": ["error_handler"],
    "timeout_secs": 30,
    "max_retry": 3
  }
}
```

---

## 🛠️ 常用命令

### 编译检查

```bash
# 检查核心模块
cargo check -p cice-core

# 检查测试模块
cargo check -p cice-tests-common

# 检查所有模块
cargo check --workspace
```

### 运行测试

```bash
# 运行所有核心测试
cargo test -p cice-core --test base_task

# 运行单个测试
cargo test -p cice-core --test base_task test_name

# 运行测试并显示输出
cargo test -p cice-core --test base_task -- --nocapture

# 运行测试框架测试
cargo test -p cice-tests-common
```

### 代码格式化和检查

```bash
# 格式化代码
cargo fmt

# 运行 clippy
cargo clippy --workspace

# 构建文档
cargo doc --open
```

---

## 📚 相关文档

| 文档 | 说明 |
|------|------|
| [README.md](README.md) | 重构概述（What, Why, How） |
| [SUMMARY.md](SUMMARY.md) | 完整修改总结和统计 |
| [TODO.md](TODO.md) | 重构任务清单 |
| [runtime-refactor.md](runtime-refactor.md) | 详细设计文档 |
| [测试框架指南](../../crates/dev/cice-tests-common/README.md) | 测试框架使用说明 |
| [测试重构说明](../../crates/cice-core/tests/REFACTOR.md) | 测试用例修改详情 |

---

## ⚠️ 重要注意事项

### 1. Action 生命周期
Action 必须在 Context 生命周期内有效。通常在函数开始时创建 Action。

```rust
// ✅ 正确：Action 在整个函数作用域内有效
let action = MyAction::new();
let mut builder = ContextBuilder::new(runtime);
builder.add_task(config, &action, params);
let context = builder.build();
context.run("task1".to_string()).await;

// ❌ 错误：Action 在 add_task 后被销毁
builder.add_task(config, &MyAction::new(), params);
```

### 2. 引用传递
使用 `&action` 而非 `action`，避免所有权转移。

```rust
// ✅ 正确
builder.add_task(config, &action, params);

// ❌ 错误
builder.add_task(config, action, params);
```

### 3. Runtime 和 Params 类型匹配
Action、Runtime 和 Params 的泛型类型必须匹配。

```rust
// ✅ 正确
impl<P: ActionParams> Action<TestRuntime, P> for MyAction { /* ... */ }
let runtime = TestRuntime::new();

// ❌ 错误：类型不匹配
impl<P: ActionParams> Action<VncRuntime, P> for MyAction { /* ... */ }
let runtime = TestRuntime::new();
```

### 4. 参数传递
参数在 `add_task` 时传入，会在 Action 执行时传递给 `recognize` 和 `exec` 方法。

```rust
// 参数在 add_task 时指定
builder.add_task(config, &action, MyParams { value: 42 });

// Action 实现中使用参数
async fn exec(&self, runtime: &R, params: &MyParams) -> Result<(), ExecError> {
    println!("Value: {}", params.value);
    Ok(())
}
```

### 5. 错误处理
区分 `UnRecognized`（非错误，继续重试）和 `RecognizeFailed`（错误，停止执行）。

```rust
async fn recognize(&self, runtime: &R, params: &P) -> Result<(), RecognizeError> {
    if !condition_met() {
        // 条件未满足，继续重试
        return Err(RecognizeError::UnRecognized);
    }

    if error_occurred() {
        // 发生错误，停止执行
        return Err(RecognizeError::RecognizeFailed {
            reason: "Error occurred".to_string()
        });
    }

    Ok(())
}
```

---

## 🐛 常见问题

### Q1: 编译错误：lifetime mismatch

```
error[E0597]: `action` does not live long enough
```

**原因**：Action 的生命周期不够长。

**解决方案**：在函数开始时创建 Action，确保其在整个 Context 生命周期内有效。

```rust
// ✅ 正确
let action = MyAction::new();
let mut builder = ContextBuilder::new(runtime);
builder.add_task(config, &action, params);
```

### Q2: 运行时找不到 Action

**原因**：`action_name` 与代码中的匹配逻辑不一致。

**解决方案**：确保 JSON 配置中的 `action_name` 与代码中的 Action 名称匹配。

```json
{
  "task1": {
    "action_name": "my_action"  // 确保这个名称与代码中一致
  }
}
```

### Q3: 测试一直超时

**原因**：Action 的 `recognize()` 一直返回 `UnRecognized`，导致无限重试。

**解决方案**：检查 `recognize()` 的逻辑，确保在合理的时间内返回 `Ok(())`。

```rust
async fn recognize(&self, runtime: &R, params: &P) -> Result<(), RecognizeError> {
    // 确保这里的条件能够被满足
    if self.check_condition(runtime, params) {
        Ok(())
    } else {
        Err(RecognizeError::UnRecognized)
    }
}
```

### Q4: JSON 解析失败

**原因**：JSON 格式不正确或缺少必填字段。

**解决方案**：确保 JSON 格式正确，包含所有必填字段。

```json
{
  "task_name": {
    "action_name": "my_action",     // 必填
    "next_task": [],                // 必填
    "interrupt_task": [],           // 必填
    "timeout_secs": 30,             // 可选
    "max_retry": 3                  // 可选
  }
}
```

### Q5: 参数类型不匹配

**原因**：`add_task` 传入的参数类型与 Action 实现的泛型参数不匹配。

**解决方案**：确保参数类型实现了 `ActionParams` trait，且类型一致。

```rust
// ✅ 正确：使用 TestParams
builder.add_task(config, &action, TestParams);

// 或自定义参数类型
#[derive(Clone)]
struct MyParams;
impl ActionParams for MyParams {}
builder.add_task(config, &action, MyParams);
```

---

## 💡 最佳实践

### 1. 命名规范
- **Action 名称**：使用 `snake_case`，如 `click_button`, `find_image`
- **Task 名称**：使用描述性名称，如 `login_task`, `verify_result`
- **Params 类型**：使用 `PascalCase` + `Params` 后缀，如 `ClickParams`, `SearchParams`

### 2. 错误处理
- **识别失败**：使用 `RecognizeError::UnRecognized`（会重试）
- **识别错误**：使用 `RecognizeError::RecognizeFailed`（停止执行）
- **执行错误**：使用 `ExecError::ExecFailed`（停止执行）

### 3. 日志记录
在 Action 中添加适当的日志，便于调试。

```rust
async fn exec(&self, runtime: &R, params: &P) -> Result<(), ExecError> {
    log::info!("Executing action: {}", self.name);
    // 执行逻辑
    log::info!("Action completed successfully");
    Ok(())
}
```

### 4. 测试覆盖
为每个 Action 编写单元测试。

```rust
#[tokio::test]
async fn test_my_action() {
    let runtime = TestRuntime::new();
    let action = MyAction::new();
    let params = TestParams;

    // 测试 recognize
    assert!(action.recognize(&runtime, &params).await.is_ok());

    // 测试 exec
    assert!(action.exec(&runtime, &params).await.is_ok());
}
```

### 5. 文档注释
为自定义 Action 和 Params 添加文档注释。

```rust
/// 点击按钮的 Action
///
/// # 示例
///
/// ```
/// let action = ClickButtonAction::new("submit");
/// builder.add_task(config, &action, ClickParams { x: 100, y: 200 });
/// ```
pub struct ClickButtonAction {
    button_name: String,
}

/// 点击操作的参数
#[derive(Clone)]
pub struct ClickParams {
    pub x: i32,
    pub y: i32,
}

impl ActionParams for ClickParams {}
```

---

**版本**: v1.1
**最后更新**: 2025-12-09
