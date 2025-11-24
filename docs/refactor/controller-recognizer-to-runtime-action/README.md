# Cice 架构重构：从 Controller-Recognizer 到 Runtime-Action

> **重构状态**: 🚧 进行中 | **核心模块**: ✅ 已完成 | **外部模块**: ⏳ 待迁移

本文档介绍 Cice 项目从 Controller-Recognizer 架构到 Runtime-Action 架构的重构工作。

---

## 📖 What - 这是什么重构？

### 核心变化

这是一次**架构层面的重大重构**，将 Cice 的核心执行模型从紧耦合的 Controller-Recognizer 双组件模式，转变为松耦合的 Runtime-Action 单一抽象模式。

### 架构对比

```
旧架构：Controller-Recognizer 双组件驱动
┌─────────────────────────────────┐
│          Context                │
├─────────────────────────────────┤
│  Controllers (通过 ID 查找)     │
│  Recognizers (通过 ID 查找)     │
│  Tasks                          │
└─────────────────────────────────┘
         │
         ▼
Task → Controller.exec_output()
    → Recognizer.exec()
    → Controller.exec_input()

新架构：Runtime-Action 统一抽象
┌─────────────────────────────────┐
│          Context                │
├─────────────────────────────────┤
│  Runtime (提供基础设施)         │
│  Tasks (直接引用 Action)        │
└─────────────────────────────────┘
         │
         ▼
Task → Action.recognize(runtime)
    → Action.exec(runtime)
```

### 关键概念

| 概念 | 说明 | 职责 |
|------|------|------|
| **Runtime** | 运行时环境 | 提供系统资源和基础设施（如屏幕控制、输入设备等） |
| **Action** | 行为抽象 | 定义具体的行为逻辑（识别 + 执行） |
| **Task** | 任务配置 | 关联 Action 和执行配置（超时、重试等） |

---

## 🤔 Why - 为什么要重构？

### 旧架构的问题

#### 1. **职责不清晰**
- Controller 既负责输入又负责输出，职责混乱
- Recognizer 名为"识别器"，但也可能执行动作
- 两者边界模糊，难以理解和维护

#### 2. **紧耦合**
- Controller 和 Recognizer 通过字符串 ID 关联
- Task 依赖于 Controller 和 Recognizer 的存在
- 运行时查找，容易出错且难以调试

#### 3. **扩展困难**
- 添加新功能需要同时考虑 Controller 和 Recognizer
- 执行流程固化，缺乏灵活性
- 资源管理分散，容易出现不一致

#### 4. **测试复杂**
- 需要同时 Mock Controller 和 Recognizer
- 测试代码冗长且难以维护
- 难以进行单元测试

### 新架构的优势

#### 1. **职责清晰** ✅
- **Runtime**: 只提供资源和基础设施
- **Action**: 只定义行为（识别 + 执行）
- **Task**: 只组织执行流程

#### 2. **高度解耦** ✅
- Action 完全独立，无需注册
- 编译时类型检查，类型安全
- 可以自由组合和复用

#### 3. **易于扩展** ✅
- 只需实现 `Action` trait 即可添加新功能
- 通过 Runtime 扩展提供新能力
- 支持 Action 组合模式

#### 4. **更好的测试性** ✅
- 可以轻松 Mock Runtime
- Action 可以独立测试
- 测试代码简洁明了

#### 5. **代码更简洁** ✅
- 删除 237 行旧代码
- 新增 150 行核心代码
- 净减少 509 行代码

---

## 🛠️ How - 如何实现重构？

### 重构范围

#### ✅ 已完成（Phase 1: 核心模块）

- [x] **cice-core 模块重构**
  - [x] 新增 `Runtime` trait
  - [x] 新增 `Action` trait
  - [x] 重构 `Context` 和 `Task`
  - [x] 删除 `Controller` 和 `Recognizer`

- [x] **测试框架适配**
  - [x] 创建 `TestRuntime` 实现
  - [x] 创建测试用 Action（SimpleAction, DenyAction, ConfigurableAction）
  - [x] 更新所有测试用例
  - [x] 更新 JSON 配置文件

- [x] **文档完善**
  - [x] 编写重构文档
  - [x] 编写迁移指南
  - [x] 编写快速参考

#### ⏳ 待完成（Phase 2: 外部模块）

详见 [TODO.md](TODO.md) 获取完整的任务清单。

**主要任务**：
1. **cice-controllers → cice-runtimes**
   - 将 `cice-controller-vnc` 迁移为 `cice-runtime-vnc`
   - 实现 `Runtime` trait

2. **cice-recognizers + cice-action → cice-actions**
   - 将 `cice-recognizer-opencv` 迁移为 `cice-action-opencv`
   - 实现 `Action` trait

3. **CI 和测试更新**
   - 更新 CI workflow
   - 更新集成测试

### 核心 API

#### Runtime Trait

```rust
/// 运行时环境，提供系统资源和基础设施
pub trait Runtime: Sync + Send {}
```

#### Action Trait

```rust
/// 行为抽象，定义识别和执行逻辑
#[async_trait]
pub trait Action<RUNTIME: Runtime>: Send + Sync {
    /// 识别阶段：检查前置条件是否满足
    async fn recognize(&self, runtime: &RUNTIME) -> Result<(), RecognizeError>;

    /// 执行阶段：执行具体的动作
    async fn exec(&self, runtime: &RUNTIME) -> Result<(), ExecError>;
}
```

#### 使用示例

```rust
// 1. 创建 Runtime
let runtime = TestRuntime::new();

// 2. 创建 Action
let action = SimpleAction::new("my_action");

// 3. 创建 Context
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

// 5. 运行
let context = builder.build();
context.run("task1".to_string()).await.unwrap();
```

### 迁移步骤

#### 从 Controller 迁移到 Runtime

```rust
// 旧代码：Controller
pub struct VncController { /* ... */ }

impl Controller for VncController {
    async fn exec_output(&self, action: &ResourceData) -> Result<ControllerData, Error> {
        // 执行输出逻辑
    }
}

// 新代码：Runtime
pub struct VncRuntime { /* ... */ }

impl Runtime for VncRuntime {}

// 在 Runtime 上添加扩展方法
impl VncRuntime {
    pub fn click(&self, x: i32, y: i32) -> Result<(), Error> {
        // 点击逻辑
    }
}
```

#### 从 Recognizer 迁移到 Action

```rust
// 旧代码：Recognizer
pub struct OpenCVRecognizer { /* ... */ }

impl Recognizer for OpenCVRecognizer {
    async fn exec(&self, action: Option<&ResourceData>, data: ControllerData)
        -> Result<RecognizeResult, Error> {
        // 识别逻辑
    }
}

// 新代码：Action
pub struct OpenCVAction { /* ... */ }

impl Action<VncRuntime> for OpenCVAction {
    async fn recognize(&self, runtime: &VncRuntime) -> Result<(), RecognizeError> {
        // 识别逻辑
        // 如果未识别到，返回 RecognizeError::UnRecognized
    }

    async fn exec(&self, runtime: &VncRuntime) -> Result<(), ExecError> {
        // 执行逻辑（如果需要）
    }
}
```

---

## 📚 文档索引

### 核心文档

| 文档 | 说明 | 适合人群 |
|------|------|----------|
| **[README.md](README.md)** (本文档) | 重构概述（What, Why, How） | 所有人 |
| **[QUICK_REFERENCE.md](QUICK_REFERENCE.md)** | 快速参考和代码片段 | 开发者 |
| **[SUMMARY.md](SUMMARY.md)** | 完整修改总结和统计 | 技术负责人 |
| **[TODO.md](TODO.md)** | 重构任务清单 | 项目管理者 |
| **[runtime-refactor.md](runtime-refactor.md)** | 详细设计文档 | 架构师 |

### 相关文档

- [测试框架使用指南](../../crates/dev/cice-tests-common/README.md)
- [测试重构说明](../../crates/cice-core/tests/REFACTOR.md)
- [Action 示例测试](../../crates/dev/cice-tests-common/tests/action_tests.rs)

---

## 🚀 快速开始

### 1. 了解新架构

阅读 [QUICK_REFERENCE.md](QUICK_REFERENCE.md) 快速了解核心概念和 API。

### 2. 查看示例

运行测试示例：

```bash
# 运行核心测试
cargo test -p cice-core --test base_task

# 运行 Action 示例测试
cargo test -p cice-tests-common
```

### 3. 开始迁移

参考 [runtime-refactor.md](runtime-refactor.md) 的迁移指南，逐步迁移你的代码。

---

## 📊 重构进度

| 模块 | 状态 | 进度 | 说明 |
|------|------|------|------|
| cice-core | ✅ 完成 | 100% | 核心架构重构完成 |
| cice-tests-common | ✅ 完成 | 100% | 测试框架适配完成 |
| cice-runtimes | ⏳ 待开始 | 0% | 从 cice-controllers 迁移 |
| cice-actions | ⏳ 待开始 | 0% | 从 cice-recognizers 迁移 |
| CI/CD | ⏳ 待开始 | 0% | 更新 workflow 和测试 |

详细任务清单请查看 [TODO.md](TODO.md)。

---

## 💡 设计理念

### 单一职责原则
- Runtime 只负责提供资源
- Action 只负责定义行为
- Task 只负责组织流程

### 开闭原则
- 对扩展开放：通过实现 Action trait 添加新功能
- 对修改封闭：核心框架无需修改

### 依赖倒置原则
- Action 依赖于 Runtime 抽象，而非具体实现
- 便于测试和 Mock

### 组合优于继承
- Action 可以自由组合
- 通过 Task 链实现复杂流程

---

## 🐛 问题反馈

如果在使用过程中遇到问题：

1. 查看 [QUICK_REFERENCE.md](QUICK_REFERENCE.md) 的"常见问题"部分
2. 查看 [runtime-refactor.md](runtime-refactor.md) 的"迁移指南"
3. 查看示例代码寻找灵感
4. 提交 Issue 或联系维护者

---

## 📅 版本历史

| 版本 | 日期 | 说明 |
|------|------|------|
| v1.0 | 2025-11-24 | 完成核心模块重构（Phase 1） |
| v2.0 | TBD | 完成外部模块迁移（Phase 2） |

---

**维护者**: Cice Team
**最后更新**: 2025-11-24
**重构提交**: `f4e2615` - refactor(core): basic runtime refactor
