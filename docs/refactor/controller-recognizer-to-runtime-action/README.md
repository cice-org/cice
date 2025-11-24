# Cice 重构文档

本目录包含 cice-core 从 Controller-Recognizer 架构到 Runtime-Action 架构重构的完整文档。

## 📚 文档索引

### 核心文档

1. **[runtime-refactor.md](runtime-refactor.md)** - 详细重构文档
   - 架构变更对比
   - 详细变更内容
   - 迁移指南
   - 设计理念

2. **[SUMMARY.md](SUMMARY.md)** - 完整修改总结
   - 修改概览
   - 代码统计
   - 架构对比
   - 使用示例

3. **[QUICK_REFERENCE.md](QUICK_REFERENCE.md)** - 快速参考
   - 快速开始
   - 常用代码片段
   - 迁移对照表
   - 常见问题

## 🎯 阅读指南

### 如果你想...

#### 📖 了解重构的背景和动机
→ 阅读 [runtime-refactor.md](runtime-refactor.md) 的"概述"和"设计理念"部分

#### 🔍 查看具体修改了哪些文件
→ 阅读 [SUMMARY.md](SUMMARY.md) 的"修改概览"部分

#### 🚀 快速上手新架构
→ 阅读 [QUICK_REFERENCE.md](QUICK_REFERENCE.md) 的"快速开始"部分

#### 🔄 迁移现有代码
→ 阅读 [runtime-refactor.md](runtime-refactor.md) 的"迁移指南"部分

#### 📝 编写测试用例
→ 阅读 [../../crates/dev/cice-tests-common/README.md](../../crates/dev/cice-tests-common/README.md)

#### 🧪 查看测试修改详情
→ 阅读 [../../crates/cice-core/tests/REFACTOR.md](../../crates/cice-core/tests/REFACTOR.md)

## 📊 重构概览

### 核心变化

```
旧架构：Controller-Recognizer 驱动
┌─────────────┐
│   Context   │
├─────────────┤
│ Controllers │ ◄─── 通过 ID 查找
│ Recognizers │ ◄─── 通过 ID 查找
│   Tasks     │
└─────────────┘

新架构：Runtime-Action 驱动
┌─────────────┐
│   Context   │
├─────────────┤
│   Runtime   │ ◄─── 提供基础设施
│   Tasks     │ ◄─── 直接引用 Action
└─────────────┘
```

### 关键改进

- ✅ **职责清晰**：Runtime 提供资源，Action 定义行为
- ✅ **高度解耦**：Action 完全独立，可自由组合
- ✅ **易于扩展**：只需实现 Action trait
- ✅ **更好的测试性**：可以 Mock Runtime
- ✅ **代码更简洁**：减少 500+ 行代码

### 统计数据

| 指标 | 数值 |
|------|------|
| 删除代码 | 237 行 |
| 新增代码 | 150 行 |
| 净减少 | 509 行 |
| 修改文件 | 16 个 |
| 新增文档 | 7 个 |

## 🗂️ 相关文档

### 测试相关

- [cice-tests-common README](../../crates/dev/cice-tests-common/README.md) - 测试框架使用指南
- [cice-core/tests REFACTOR](../../crates/cice-core/tests/REFACTOR.md) - 测试重构说明
- [Action 示例测试](../../crates/dev/cice-tests-common/tests/action_tests.rs) - 代码示例

### 代码示例

- [base_task.rs](../../crates/cice-core/tests/base_task.rs) - 集成测试示例
- [action/mod.rs](../../crates/dev/cice-tests-common/src/action/mod.rs) - Action 实现示例

## 🚀 快速开始

### 1. 创建简单测试

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

### 2. 运行测试

```bash
# 运行所有测试
cargo test -p cice-core --test base_task

# 运行单个测试
cargo test -p cice-core --test base_task config

# 查看输出
cargo test -- --nocapture
```

## 📋 迁移检查清单

### 代码迁移

- [ ] 移除 Controller 实现
- [ ] 移除 Recognizer 实现
- [ ] 创建 Runtime 实现
- [ ] 创建 Action 实现
- [ ] 更新 Context 创建代码
- [ ] 更新 Task 添加代码

### 配置迁移

- [ ] 更新 JSON 配置文件
  - [ ] 移除 `controller_id`
  - [ ] 移除 `recognizer_id`
  - [ ] 添加 `action_name`
  - [ ] 添加 `timeout_secs`
  - [ ] 添加 `max_retry`

### 测试验证

- [ ] 编译通过
- [ ] 测试通过
- [ ] 行为一致
- [ ] 文档更新

## 🎓 学习路径

### 初学者

1. 阅读 [QUICK_REFERENCE.md](QUICK_REFERENCE.md) 快速了解新架构
2. 运行 [action_tests.rs](../../crates/dev/cice-tests-common/tests/action_tests.rs) 示例
3. 尝试修改示例代码

### 进阶用户

1. 阅读 [runtime-refactor.md](runtime-refactor.md) 理解设计理念
2. 查看 [base_task.rs](../../crates/cice-core/tests/base_task.rs) 集成测试
3. 实现自定义 Action

### 迁移开发者

1. 阅读 [runtime-refactor.md](runtime-refactor.md) 的"迁移指南"
2. 参考 [SUMMARY.md](SUMMARY.md) 的"迁移对照表"
3. 使用 [QUICK_REFERENCE.md](QUICK_REFERENCE.md) 作为参考

## 💡 最佳实践

1. **先理解后实践**：先阅读文档理解架构，再动手编码
2. **参考示例**：查看现有的测试用例和 Action 实现
3. **增量迁移**：一次迁移一个模块，逐步验证
4. **保持测试**：确保迁移后测试行为一致
5. **更新文档**：及时更新相关文档

## 🐛 问题反馈

如果在使用过程中遇到问题：

1. 查看 [QUICK_REFERENCE.md](QUICK_REFERENCE.md) 的"常见问题"部分
2. 检查代码是否符合迁移指南
3. 查看示例代码寻找灵感
4. 提交 Issue 或联系维护者

## 📅 版本历史

| 版本 | 日期 | 说明 |
|------|------|------|
| v1.0 | 2025-11-24 | 初始版本，完成核心重构 |

## 🙏 致谢

感谢所有参与重构的开发者和测试人员！

---

**维护者**：Cice Team
**最后更新**：2025-11-24
