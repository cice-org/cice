# Cice-Core Runtime 重构文档

## 概述

本次重构将 cice-core 模块从 **Controller-Recognizer 驱动模式** 改为 **Runtime-Action 驱动模式**，这是一次架构层面的重大变更，旨在提高系统的灵活性、可扩展性和可测试性。

**重构提交**: `f4e2615` - refactor(core): basic runtime refactor
**重构日期**: 2025-11-23

---

## 架构变更对比

### 旧架构：Controller-Recognizer 驱动模式

#### 核心概念

在旧架构中，系统由两个核心组件驱动：

1. **Controller（控制器）**
   - 负责与外部系统交互（输入/输出）
   - 提供 `exec_input()` 和 `exec_output()` 方法
   - 输出数据供 Recognizer 使用

2. **Recognizer（识别器）**
   - 负责识别和判断任务状态
   - 接收 Controller 的输出数据
   - 通过 `exec()` 方法执行识别逻辑

#### 执行流程

```
Task → Controller.exec_output() → ControllerData → Recognizer.exec() → RecognizeResult
     ↓
Controller.exec_input() (如果需要)
```

#### 代码示例（旧架构）

```rust
// Controller trait
#[async_trait]
pub trait Controller {
    fn name(&self) -> ControllerId;
    fn init(&self, resource: &ResourceData) -> Result<(), ControllerError>;
    async fn exec_input(&self, input_action: &ResourceData) -> Result<(), CustomControllerError>;
    async fn exec_output(&self, output_action: &ResourceData) -> Result<ControllerData, CustomControllerError>;
}

// Recognizer trait
#[async_trait]
pub trait Recognizer {
    fn name(&self) -> String;
    fn init(&self, resource: &ResourceData) -> Result<(), RecognizerError>;
    async fn exec(&self, action: Option<&ResourceData>, data: ControllerData) -> Result<RecognizeResult, CustomRecognizerError>;
    fn require_input(&self) -> Option<ResourceData>;
}
```

#### 旧架构的问题

1. **职责耦合**：Controller 和 Recognizer 紧密耦合，难以独立测试和复用
2. **流程固定**：执行流程固化为 Controller → Recognizer，缺乏灵活性
3. **扩展困难**：添加新的执行模式需要修改核心逻辑
4. **资源管理复杂**：Controller 和 Recognizer 各自管理资源，容易出现不一致

---

### 新架构：Runtime-Action 驱动模式

#### 核心概念

新架构引入了三个核心抽象：

1. **Runtime（运行时）**
   - 提供系统运行所需的基础设施和资源
   - 是一个轻量级的 trait，可以根据需要扩展
   - 作为 Action 执行的上下文环境

2. **Action（动作）**
   - 统一的行为抽象，包含识别和执行两个阶段
   - `recognize()`: 检查前置条件是否满足
   - `exec()`: 执行具体的动作
   - 完全独立，可以自由组合

3. **Task（任务）**
   - 关联一个 Action 和配置信息
   - 通过 Runtime 执行 Action
   - 支持任务链和重试机制

#### 执行流程

```
Task → Action.recognize(runtime) → Action.exec(runtime) → Next Task
                ↑                           ↑
                └───────── Runtime ─────────┘
```

#### 代码示例（新架构）

```rust
// Runtime trait - 轻量级基础设施
pub trait Runtime: Sync + Send {}

// Action trait - 统一的行为抽象
#[async_trait]
pub trait Action<RUNTIME: Runtime>: Send + Sync {
    async fn recognize(&self, runtime: &RUNTIME) -> Result<(), RecognizeError>;
    async fn exec(&self, runtime: &RUNTIME) -> Result<(), ExecError>;
}

// Task - 关联 Action 和配置
pub struct Task<'task, RUNTIME: Runtime> {
    config: TaskConfig,
    action: &'task dyn Action<RUNTIME>,
}
```

#### 新架构的优势

1. **职责清晰**：Runtime 提供资源，Action 定义行为，Task 组织流程
2. **高度解耦**：Action 之间完全独立，可以自由组合和复用
3. **易于扩展**：通过实现 Action trait 即可添加新功能
4. **统一抽象**：所有行为都是 Action，简化了系统模型
5. **更好的测试性**：可以为测试提供 Mock Runtime

---

## 详细变更内容

### 1. 新增模块

#### `src/action/mod.rs`
定义了 Action trait 和相关错误类型：

```rust
pub trait Action<RUNTIME: Runtime>: Send + Sync {
    async fn recognize(&self, runtime: &RUNTIME) -> Result<(), RecognizeError>;
    async fn exec(&self, runtime: &RUNTIME) -> Result<(), ExecError>;
}

pub enum RecognizeError {
    UnRecognized,      // 未识别（非错误）
    RecognizeFailed { reason: String },  // 识别失败
}

pub enum ExecError {
    ExecFailed { reason: String },  // 执行失败
}
```

#### `src/runtime/mod.rs` 和 `src/runtime/ext/mod.rs`
定义了 Runtime trait 及其扩展机制：

```rust
pub trait Runtime: Sync + Send {}
```

### 2. 删除模块

- **删除 `src/controller/mod.rs`** (89 行)
  - Controller trait 及其相关实现被移除
  - 功能被 Action trait 替代

- **删除 `src/recognizer/mod.rs`** (61 行)
  - Recognizer trait 及其相关实现被移除
  - 功能被 Action trait 替代

- **删除 `src/utils.rs`** (87 行)
  - 旧架构的工具函数被移除

### 3. 重构模块

#### `src/context.rs` (大幅简化)
- **变更前**: 173 行，管理 Controller 和 Recognizer
- **变更后**: 简化为管理 Runtime 和 Task
- **主要变化**:
  - `ContextBuilder` 现在接受 `Runtime` 和 `Action`
  - 移除了 Controller/Recognizer 的注册和管理逻辑
  - 简化了资源管理

```rust
// 新的 ContextBuilder
pub struct ContextBuilder<'task, RUNTIME: Runtime> {
    runtime: RUNTIME,
    tasks: HashMap<TaskId, Task<'task, RUNTIME>>,
    // ...
}

impl<'task, RUNTIME: Runtime> ContextBuilder<'task, RUNTIME> {
    pub fn new(runtime: RUNTIME) -> Self { /* ... */ }

    pub fn add_task(
        &mut self,
        task_config: TaskConfig,
        action: &'task impl Action<RUNTIME>,
    ) -> &mut Self { /* ... */ }
}
```

#### `src/task/mod.rs` (大幅重构)
- **变更前**: 371 行，复杂的 Controller-Recognizer 协调逻辑
- **变更后**: 简化为 Action 的执行流程
- **主要变化**:
  - Task 现在直接关联 Action
  - 执行流程简化为 `recognize() → exec()`
  - 通过 Runtime 提供执行上下文

```rust
impl<'task, RUNTIME: Runtime> Task<'task, RUNTIME> {
    async fn try_recognize(&self, context: &Context<'task, RUNTIME>) -> Result<(), RecognizeError> {
        self.action.recognize(context.get_runtime()).await?;
        Ok(())
    }

    async fn try_exec(&self, context: &Context<'task, RUNTIME>) -> Result<(), ExecError> {
        self.action.exec(context.get_runtime()).await?;
        Ok(())
    }
}
```

#### `src/message/task.rs`
- 消息类型进行了调整以适配新的执行流程
- 移除了 Controller/Recognizer 相关的消息

### 4. 依赖变更

#### `Cargo.toml`
移除了不再需要的依赖：
- 移除了与旧架构相关的依赖项

---

## 迁移指南

### 从 Controller 迁移到 Action

#### 旧代码（Controller）
```rust
pub struct MyController {}

#[async_trait]
impl Controller for MyController {
    fn name(&self) -> ControllerId {
        "my_controller".into()
    }

    async fn exec_output(&self, action: &ResourceData) -> Result<ControllerData, CustomControllerError> {
        // 执行输出逻辑
        Ok(data)
    }
}
```

#### 新代码（Action）
```rust
pub struct MyAction {}

#[async_trait]
impl<RUNTIME: Runtime> Action<RUNTIME> for MyAction {
    async fn recognize(&self, runtime: &RUNTIME) -> Result<(), RecognizeError> {
        // 检查前置条件
        Ok(())
    }

    async fn exec(&self, runtime: &RUNTIME) -> Result<(), ExecError> {
        // 执行动作
        Ok(())
    }
}
```

### 从 Recognizer 迁移到 Action

#### 旧代码（Recognizer）
```rust
pub struct MyRecognizer {}

#[async_trait]
impl Recognizer for MyRecognizer {
    fn name(&self) -> String {
        "my_recognizer".into()
    }

    async fn exec(&self, action: Option<&ResourceData>, data: ControllerData) -> Result<RecognizeResult, CustomRecognizerError> {
        // 识别逻辑
        Ok(result)
    }
}
```

#### 新代码（Action）
```rust
pub struct MyRecognizeAction {}

#[async_trait]
impl<RUNTIME: Runtime> Action<RUNTIME> for MyRecognizeAction {
    async fn recognize(&self, runtime: &RUNTIME) -> Result<(), RecognizeError> {
        // 识别逻辑
        // 如果识别失败，返回 RecognizeError::UnRecognized
        Ok(())
    }

    async fn exec(&self, runtime: &RUNTIME) -> Result<(), ExecError> {
        // 执行后续动作（如果需要）
        Ok(())
    }
}
```

### 测试代码迁移

#### 旧的测试代码
```rust
let mut builder = ContextBuilder::new();
builder.add_controller((Box::new(MyController::new()), config));
builder.add_recognizer((Box::new(MyRecognizer::new()), config));
builder.add_task(task_data);
let context = builder.build();
context.run("entry".to_string()).await;
```

#### 新的测试代码
```rust
let runtime = MyRuntime::new();
let mut builder = ContextBuilder::new(runtime);
let action = MyAction::new();
builder.add_task(task_config, &action);
let context = builder.build();
context.run("entry".to_string()).await;
```

---

## 影响范围

### 核心模块
- ✅ `cice-core`: 完全重构
- ⚠️ `cice-action`: 需要适配新的 Action trait
- ⚠️ `cice-controllers/*`: 需要迁移到 Action 模式
- ⚠️ `cice-recognizers/*`: 需要迁移到 Action 模式

### 测试模块
- ⚠️ `cice-tests-common`: 需要更新测试用例以适配新架构
- ⚠️ `cice-core/tests`: 需要更新集成测试

### 外部依赖
- `cice-recognizer-opencv`: 需要适配新的 Action 接口

---

## 待办事项

### 高优先级
- [ ] 更新 `cice-tests-common` 中的所有测试用例
- [ ] 迁移 `cice-recognizer-opencv` 到新架构
- [ ] 更新 `cice-controllers` 到新架构

### 中优先级
- [ ] 编写 Runtime 扩展示例
- [ ] 完善 Action 组合模式文档
- [ ] 添加更多单元测试

### 低优先级
- [ ] 性能基准测试
- [ ] 迁移指南完善
- [ ] API 文档更新

---

## 设计理念

### 为什么选择 Runtime-Action 模式？

1. **单一职责原则**
   - Runtime 只负责提供资源和基础设施
   - Action 只负责定义具体的行为
   - Task 只负责组织执行流程

2. **开闭原则**
   - 对扩展开放：通过实现 Action trait 添加新功能
   - 对修改封闭：核心框架无需修改

3. **依赖倒置原则**
   - Action 依赖于 Runtime 抽象，而非具体实现
   - 便于测试和 Mock

4. **组合优于继承**
   - Action 可以自由组合
   - 通过 Task 链实现复杂流程

### 未来扩展方向

1. **Runtime 扩展**
   - 添加定时器支持
   - 添加网络支持
   - 添加文件系统支持

2. **Action 组合**
   - 实现 Action 装饰器模式
   - 实现 Action 责任链模式
   - 实现 Action 状态机模式

3. **性能优化**
   - Action 并行执行
   - Runtime 资源池化
   - 异步流水线优化

---

## 总结

本次重构是 cice-core 架构演进的重要里程碑，从紧耦合的 Controller-Recognizer 模式转向松耦合的 Runtime-Action 模式，为系统的长期发展奠定了坚实的基础。

**核心变化**：
- 删除了 Controller 和 Recognizer trait（共 150 行）
- 新增了 Runtime 和 Action trait（共 50 行）
- 简化了 Context 和 Task 实现（减少 422 行代码）
- 总体代码量减少约 428 行，同时提高了灵活性和可维护性

**收益**：
- 更清晰的职责划分
- 更好的可测试性
- 更强的扩展能力
- 更简洁的代码结构

**代价**：
- 需要迁移现有的 Controller 和 Recognizer 实现
- 需要更新所有测试用例
- 需要更新相关文档

总体而言，这是一次成功的重构，为项目的未来发展提供了更好的架构基础。
