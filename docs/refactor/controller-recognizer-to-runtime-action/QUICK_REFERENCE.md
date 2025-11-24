# Runtime-Action 架构快速参考

## 🚀 快速开始

### 1. 创建测试

```rust
use cice_core::context::ContextBuilder;
use cice_core::task::TaskConfig;
use cice_tests_common::action::{SimpleAction, TestRuntime};
use std::time::Duration;

#[tokio::test]
async fn my_test() {
    let runtime = TestRuntime::new();
    let action = SimpleAction::new("my_action");
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
    );

    builder.build().run("task1".to_string()).await.unwrap();
}
```

### 2. 自定义 Action

```rust
use async_trait::async_trait;
use cice_core::action::{Action, ExecError, RecognizeError};
use cice_tests_common::action::TestRuntime;

pub struct MyAction {}

#[async_trait]
impl Action<TestRuntime> for MyAction {
    async fn recognize(&self, _runtime: &TestRuntime) -> Result<(), RecognizeError> {
        // 检查前置条件
        Ok(())
    }

    async fn exec(&self, _runtime: &TestRuntime) -> Result<(), ExecError> {
        // 执行动作
        Ok(())
    }
}
```

### 3. JSON 配置

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

## 📋 核心概念

| 概念 | 说明 | 示例 |
|------|------|------|
| **Runtime** | 提供运行时资源和基础设施 | `TestRuntime::new()` |
| **Action** | 定义行为（识别 + 执行） | `SimpleAction::new("name")` |
| **Task** | 关联 Action 和配置 | `TaskConfig { ... }` |
| **Context** | 管理 Runtime 和 Tasks | `ContextBuilder::new(runtime)` |

## 🔄 迁移对照

### 旧架构 → 新架构

| 旧概念 | 新概念 | 说明 |
|--------|--------|------|
| Controller | Action | 统一为 Action |
| Recognizer | Action | 统一为 Action |
| controller_id | action_name | 字段重命名 |
| recognizer_id | ~~删除~~ | 不再需要 |
| exec_output() | exec() | 方法简化 |
| exec_input() | ~~删除~~ | 合并到 exec() |
| exec() (Recognizer) | recognize() | 方法重命名 |

### 代码迁移

#### 旧代码
```rust
let mut builder = ContextBuilder::new();
builder.add_controller((Box::new(MyController::new()), config));
builder.add_recognizer((Box::new(MyRecognizer::new()), config));
builder.add_task(task_data);
```

#### 新代码
```rust
let runtime = TestRuntime::new();
let action = MyAction::new();
let mut builder = ContextBuilder::new(runtime);
builder.add_task(task_config, &action);
```

## 🎯 常用 Actions

### SimpleAction
总是成功的 Action
```rust
let action = SimpleAction::new("name");
```

### DenyAction
总是识别失败的 Action
```rust
let action = DenyAction::new("name");
```

### ConfigurableAction
可配置成功/失败的 Action
```rust
let action = ConfigurableAction::new("name", true); // 成功
let action = ConfigurableAction::new("name", false); // 失败
```

## 📝 JSON 字段说明

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `action_name` | String | ✅ | Action 标识符 |
| `next_task` | Array | ✅ | 下一个任务列表 |
| `interrupt_task` | Array | ✅ | 中断任务列表 |
| `timeout_secs` | Number | ❌ | 超时时间（默认 30） |
| `max_retry` | Number | ❌ | 最大重试次数（默认 3） |

## 🛠️ 常用命令

```bash
# 编译检查
cargo check -p cice-core
cargo check -p cice-tests-common

# 运行测试
cargo test -p cice-core --test base_task
cargo test -p cice-tests-common

# 查看输出
cargo test -- --nocapture

# 运行单个测试
cargo test -p cice-core --test base_task config
```

## 📚 文档链接

- [详细重构文档](runtime-refactor.md)
- [测试重构说明](../../crates/cice-core/tests/REFACTOR.md)
- [测试框架指南](../../crates/dev/cice-tests-common/README.md)
- [完整总结](SUMMARY.md)

## ⚠️ 注意事项

1. **Action 生命周期**：Action 必须在 Context 生命周期内有效
2. **引用传递**：使用 `&action` 而非 `action`
3. **Runtime 类型**：Action 和 Runtime 类型必须匹配
4. **错误处理**：区分 `UnRecognized`（非错误）和 `RecognizeFailed`（错误）

## 🐛 常见问题

### Q: 编译错误：lifetime mismatch
**A**: 确保 Action 的生命周期足够长，通常在函数开始时创建

### Q: 运行时找不到 Action
**A**: 检查 `action_name` 是否与代码中的匹配逻辑一致

### Q: 测试一直超时
**A**: 检查 Action 的 `recognize()` 是否正确返回，避免死循环

### Q: JSON 解析失败
**A**: 确保 JSON 格式正确，包含所有必填字段

## 💡 最佳实践

1. **命名规范**：Action 名称使用 `snake_case`
2. **错误处理**：明确区分识别失败和执行失败
3. **日志记录**：在 Action 中添加适当的日志
4. **测试覆盖**：为每个 Action 编写单元测试
5. **文档注释**：为自定义 Action 添加文档注释

---

**版本**：v1.0
**更新日期**：2025-11-24
