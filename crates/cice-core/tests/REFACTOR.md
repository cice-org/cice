# Cice-Core Tests 重构说明

## 概述

本文档说明了 `crates/cice-core/tests` 目录下测试用例的重构内容，以适配新的 **Runtime-Action** 架构。

## 修改文件清单

### 1. JSON 配置文件

所有 JSON 配置文件都已更新，将旧的 `controller_id` 和 `recognizer_id` 字段替换为新的 `action_name` 字段，并添加了 `timeout_secs` 和 `max_retry` 字段。

#### 修改的文件：

1. **base_task.json**
   - 移除：`controller_id`, `recognizer_id`
   - 新增：`action_name: "simple_action"`, `timeout_secs: 30`, `max_retry: 3`

2. **task_sequence.json**
   - 移除：`controller_id`, `recognizer_id`
   - 新增：`action_name` (根据任务类型为 `"accept_action"` 或 `"deny_action"`)
   - 新增：`timeout_secs: 30`, `max_retry: 3`

3. **simple_image.json**
   - 移除：`controller_id`, `recognizer_id`
   - 新增：`action_name` (`"simple_action"` 或 `"simple_image_action"`)
   - 新增：`timeout_secs: 30`, `max_retry: 3`

4. **controller_input_and_output_action.json**
   - 移除：`controller_id`, `recognizer_id`, `controller_input`, `controller_output`
   - 新增：`action_name` (`"simple_action"` 或 `"input_output_action"`)
   - 新增：`timeout_secs: 30`, `max_retry: 3`

5. **recognizer_simple_with_action.json**
   - 移除：`controller_id`, `recognizer_id`, `reco`
   - 新增：`action_name` (`"simple_action"` 或 `"action_with_area"`)
   - 新增：`timeout_secs: 30`, `max_retry: 3`

### 2. 测试文件

#### base_task.rs

完全重写以使用新的 Runtime-Action 架构。

**主要变化：**

##### 旧架构代码模式：
```rust
let mut builder = ContextBuilder::new();
builder.add_controller((Box::new(MyController::new()), config));
builder.add_recognizer((Box::new(MyRecognizer::new()), config));
builder.add_task(task_data);
let context = builder.build();
```

##### 新架构代码模式：
```rust
let runtime = TestRuntime::new();
let action = SimpleAction::new("action_name");
let mut builder = ContextBuilder::new(runtime);
builder.add_task(task_config, &action);
let context = builder.build();
```

## 测试用例对照表

| 测试函数 | 旧架构使用的组件 | 新架构使用的 Action | 测试目的 |
|---------|----------------|-------------------|---------|
| `config()` | SimpleControllerWithConfig<br>SimpleRecognizerWithConfig<br>AcceptAllRecognizer | SimpleAction | 测试基本配置和任务执行 |
| `task_sequence()` | SimpleControllerWithConfig<br>AssertImageRecognizer<br>AcceptAllRecognizer<br>DenyAllRecognizer | SimpleAction (accept)<br>DenyAction (deny) | 测试任务序列执行和分支选择 |
| `simple_image()` | SimpleImageController<br>SimpleImageInputRecognizer<br>AcceptAllRecognizer | SimpleAction<br>SimpleAction (image) | 测试图像相关功能 |
| `controller_input_and_output_action()` | ControllerWithInputAndOutputAction<br>AcceptAllRecognizer | SimpleAction<br>SimpleAction (input/output) | 测试输入输出动作 |
| `recognizer_simple_with_action()` | DummyController<br>AcceptAllRecognizer<br>SimpleRecognizerWithAction | SimpleAction<br>SimpleAction (with area) | 测试带参数的识别动作 |

## 架构对比

### 旧架构（Controller-Recognizer）

```rust
// 1. 创建 Builder
let mut builder = ContextBuilder::new();

// 2. 添加 Controller
builder.add_controller((
    Box::new(SimpleControllerWithConfig::new(config)),
    serde_json::to_value(config).unwrap(),
));

// 3. 添加 Recognizer
builder.add_recognizer((
    Box::new(AcceptAllRecognizer {}),
    serde_json::json!({}),
));

// 4. 添加 Task（关联 controller_id 和 recognizer_id）
for task in task_datas {
    builder.add_task(task);
}

// 5. 构建并运行
let ret = builder.build().run("test".to_string()).await;
```

### 新架构（Runtime-Action）

```rust
// 1. 创建 Runtime
let runtime = TestRuntime::new();

// 2. 创建 Actions
let simple_action = SimpleAction::new("simple_action");

// 3. 创建 Builder
let mut builder = ContextBuilder::new(runtime);

// 4. 添加 Task（直接关联 Action 引用）
for config in task_configs {
    builder.add_task(config, &simple_action);
}

// 5. 构建并运行
let ret = builder.build().run("test".to_string()).await;
```

## 关键改进

### 1. 简化的依赖管理

**旧架构：**
- 需要先注册 Controller 和 Recognizer
- Task 通过字符串 ID 引用它们
- 运行时查找和匹配

**新架构：**
- 直接传递 Action 引用
- 编译时类型检查
- 无需运行时查找

### 2. 更清晰的测试意图

**旧架构：**
```rust
// 不清楚这个任务会做什么
"controller_id": "controller_simple_with_config",
"recognizer_id": "recognizer_AcceptAll"
```

**新架构：**
```rust
// 清楚地表明这个任务使用 simple_action
"action_name": "simple_action"
```

### 3. 更好的代码复用

**旧架构：**
- 每个测试都需要创建和注册多个 Controller 和 Recognizer
- 大量重复代码

**新架构：**
- 创建一次 Action，多次使用
- 代码更简洁

### 4. 统一的配置格式

所有 JSON 配置文件现在使用统一的格式：

```json
{
  "task_name": {
    "action_name": "action_identifier",
    "next_task": ["next_task_name"],
    "interrupt_task": [],
    "timeout_secs": 30,
    "max_retry": 3
  }
}
```

## 测试执行

### 运行所有测试

```bash
cargo test -p cice-core --test base_task
```

### 运行单个测试

```bash
cargo test -p cice-core --test base_task config
cargo test -p cice-core --test base_task task_sequence
cargo test -p cice-core --test base_task simple_image
```

### 查看测试输出

```bash
cargo test -p cice-core --test base_task -- --nocapture
```

## 迁移检查清单

如果你需要迁移其他测试文件，请遵循以下步骤：

- [ ] 更新 JSON 配置文件
  - [ ] 移除 `controller_id` 和 `recognizer_id`
  - [ ] 添加 `action_name`
  - [ ] 添加 `timeout_secs` 和 `max_retry`
  - [ ] 移除 `controller_input`, `controller_output`, `reco` 等旧字段

- [ ] 更新测试代码
  - [ ] 创建 `TestRuntime` 实例
  - [ ] 创建 `Action` 实例（替代 Controller 和 Recognizer）
  - [ ] 使用 `ContextBuilder::new(runtime)` 而非 `ContextBuilder::new()`
  - [ ] 使用 `builder.add_task(config, &action)` 而非 `builder.add_task(task_data)`
  - [ ] 移除 `add_controller` 和 `add_recognizer` 调用

- [ ] 验证测试
  - [ ] 编译通过
  - [ ] 测试通过
  - [ ] 测试行为与之前一致

## 注意事项

1. **Action 生命周期**：Action 必须在 Context 的整个生命周期内保持有效，因此使用引用传递。

2. **Action 名称匹配**：JSON 中的 `action_name` 用于标识，但实际执行时使用的是传递给 `add_task` 的 Action 引用。

3. **测试行为一致性**：虽然架构改变了，但测试的行为和预期结果保持不变。

4. **简化的实现**：新架构下的测试 Action 实现更简单，因为不需要区分 Controller 和 Recognizer 的职责。

## 相关文档

- [Runtime 重构文档](../../../docs/refactor/runtime-refactor.md)
- [cice-tests-common README](../../dev/cice-tests-common/README.md)
- [Action 示例测试](../../dev/cice-tests-common/tests/action_tests.rs)

## 总结

本次重构成功将 `cice-core/tests` 下的所有测试用例从旧的 Controller-Recognizer 架构迁移到新的 Runtime-Action 架构。主要成果包括：

- ✅ 更新了 5 个 JSON 配置文件
- ✅ 重写了 `base_task.rs` 测试文件（5 个测试用例）
- ✅ 保持了测试行为的一致性
- ✅ 简化了测试代码结构
- ✅ 提高了代码可读性和可维护性

所有测试用例现在使用统一的 Runtime-Action 模式，为未来的扩展和维护提供了更好的基础。
