# Cice 架构重构 - 任务清单

> 本文档列出了从 Controller-Recognizer 到 Runtime-Action 架构重构的所有任务及其完成状态。

**最后更新**: 2025-11-24
**当前阶段**: Phase 1 已完成，Phase 2 进行中

---

## 📊 总体进度

| 阶段 | 状态 | 进度 | 说明 |
|------|------|------|------|
| **Phase 1: 核心模块** | ✅ 完成 | 100% | cice-core 和测试框架 |
| **Phase 2: 外部模块** | ✅ 完成 | 100% | cice-runtimes 和 cice-actions |
| **Phase 3: 功能增强** | 🔮 计划中 | 0% | 扩展功能和优化 |

---

## ✅ Phase 1: 核心模块重构（已完成）

### 1.1 核心架构设计与实现

- [x] 设计 Runtime trait 接口
- [x] 设计 Action trait 接口
- [x] 实现 Runtime trait（`src/runtime/mod.rs`）
- [x] 实现 Action trait（`src/action/mod.rs`）
- [x] 实现 Runtime 扩展机制（`src/runtime/ext/mod.rs`）
- [x] 定义错误类型（RecognizeError, ExecError）

### 1.2 Context 和 Task 重构

- [x] 重构 `Context` 以支持 Runtime
- [x] 重构 `ContextBuilder` 以接受 Runtime 和 Action
- [x] 简化 Task 执行流程
- [x] 移除 Controller/Recognizer 查找逻辑
- [x] 更新消息处理逻辑（`src/message/task.rs`）

### 1.3 删除旧代码

- [x] 删除 `src/controller/mod.rs`（89 行）
- [x] 删除 `src/recognizer/mod.rs`（61 行）
- [x] 删除 `src/utils.rs`（87 行）
- [x] 清理相关依赖

### 1.4 测试框架适配

- [x] 创建 `TestRuntime` 实现
- [x] 创建 `SimpleAction`（总是成功）
- [x] 创建 `DenyAction`（总是识别失败）
- [x] 创建 `ConfigurableAction`（可配置成功/失败）
- [x] 更新 `TaskConfig` 结构
- [x] 实现 JSON 配置加载支持

### 1.5 集成测试迁移

- [x] 更新 `base_task.json`
- [x] 更新 `task_sequence.json`
- [x] 更新 `simple_image.json`
- [x] 更新 `controller_input_and_output_action.json`
- [x] 更新 `recognizer_simple_with_action.json`
- [x] 重写 `base_task.rs` 测试文件
- [x] 验证所有测试通过

### 1.6 文档编写

- [x] 编写 `README.md`（What, Why, How）
- [x] 编写 `QUICK_REFERENCE.md`（快速参考）
- [x] 编写 `SUMMARY.md`（完整总结）
- [x] 编写 `TODO.md`（本文档）
- [x] 编写 `runtime-refactor.md`（详细设计）
- [x] 编写测试框架使用指南
- [x] 编写测试重构说明
- [x] 提供代码示例

---

## ⏳ Phase 2: 外部模块迁移（待完成）

### 2.1 cice-controllers → cice-runtimes

#### 2.1.1 模块重构

- [x] 创建 `cice-runtimes` 目录结构
- [x] 设计 Runtime 模块架构
- [x] 定义公共 Runtime trait 扩展

#### 2.1.2 cice-controller-vnc → cice-runtime-vnc

- [x] 创建 `cice-runtime-vnc` crate
- [x] 实现 `VncRuntime` struct
- [x] 实现 `Runtime` trait for `VncRuntime`
- [x] 迁移 VNC 连接管理逻辑
- [x] 提供屏幕控制接口（截图、点击、输入等）
- [x] 提供资源访问接口
- [x] 更新 `Cargo.toml` 依赖
- [x] 编写单元测试
- [x] 编写集成测试
- [x] 编写使用文档

#### 2.1.3 其他 Controller 迁移（如有）

- [x] 识别其他需要迁移的 Controller（无其他 Controller）
- [x] 逐个迁移为 Runtime（N/A）
- [x] 更新相关测试和文档（N/A）

### 2.2 cice-recognizers + cice-action → cice-actions

#### 2.2.1 模块重构

- [x] 创建 `cice-actions` 目录结构
- [x] 设计 Action 模块架构
- [x] 定义公共 Action trait 实现

#### 2.2.2 cice-recognizer-opencv → cice-action-opencv

- [x] 创建 `cice-action-opencv` crate
- [x] 实现图像识别 Action
  - [x] `TemplateMatchAction`（模板匹配）
  - [ ] `FindImageAction`（查找图像）
  - [ ] `DetectObjectAction`（对象检测）
- [x] 迁移 OpenCV 相关功能
- [x] 实现 `Action` trait（通过 `ScreenshotExt`）
- [x] 更新 `Cargo.toml` 依赖
- [x] 编写单元测试
- [x] 编写集成测试
- [x] 编写使用文档

#### 2.2.3 cice-action 模块适配

- [x] 评估现有 `cice-action` 模块（不存在独立模块）
- [x] 适配新的 `Action` trait（N/A）
- [x] 迁移现有 Action 实现（N/A）
- [x] 更新相关测试和文档（N/A）

#### 2.2.4 其他 Recognizer 迁移（如有）

- [x] 识别其他需要迁移的 Recognizer（无其他 Recognizer）
- [x] 逐个迁移为 Action（N/A）
- [x] 更新相关测试和文档（N/A）

### 2.3 CI/CD 和测试更新

#### 2.3.1 CI Workflow 更新

- [ ] 更新 GitHub Actions workflow
- [ ] 更新构建脚本
- [ ] 更新测试脚本
- [ ] 更新部署脚本（如有）

#### 2.3.2 集成测试更新

- [ ] 更新端到端测试
- [ ] 更新性能测试（如有）
- [ ] 更新回归测试
- [ ] 验证所有测试通过

#### 2.3.3 文档更新

- [ ] 更新项目 README
- [ ] 更新 API 文档
- [ ] 更新用户指南
- [ ] 更新开发者指南

### 2.4 代码清理

- [ ] 删除旧的 `cice-controllers` 目录
- [ ] 删除旧的 `cice-recognizers` 目录
- [ ] 清理未使用的依赖
- [ ] 清理未使用的代码
- [ ] 运行 `cargo clippy` 并修复警告
- [ ] 运行 `cargo fmt` 格式化代码

---

## 🔮 Phase 3: 功能增强（未来计划）

### 3.1 Runtime 扩展

- [ ] 实现定时器扩展
  - [ ] `TimerExt` trait
  - [ ] 延迟执行
  - [ ] 定时任务
- [ ] 实现网络扩展
  - [ ] `NetworkExt` trait
  - [ ] HTTP 请求
  - [ ] WebSocket 支持
- [ ] 实现文件系统扩展
  - [ ] `FileSystemExt` trait
  - [ ] 文件读写
  - [ ] 目录操作
- [ ] 实现资源池化
  - [ ] 连接池
  - [ ] 对象池
  - [ ] 资源复用

### 3.2 Action 组合模式

- [ ] 实现装饰器模式
  - [ ] `LoggingAction`（日志装饰器）
  - [ ] `RetryAction`（重试装饰器）
  - [ ] `TimeoutAction`（超时装饰器）
- [ ] 实现责任链模式
  - [ ] `ActionChain`
  - [ ] 条件分支
  - [ ] 错误处理链
- [ ] 实现状态机模式
  - [ ] `StateMachine`
  - [ ] 状态转换
  - [ ] 状态持久化
- [ ] 实现并行执行
  - [ ] `ParallelAction`
  - [ ] 并发控制
  - [ ] 结果聚合

### 3.3 性能优化

- [ ] 性能基准测试
  - [ ] 建立基准测试框架
  - [ ] 测试关键路径性能
  - [ ] 生成性能报告
- [ ] 并行执行优化
  - [ ] 识别可并行的 Action
  - [ ] 实现并行调度器
  - [ ] 优化资源竞争
- [ ] 资源管理优化
  - [ ] 实现资源池
  - [ ] 优化资源分配
  - [ ] 减少资源开销
- [ ] 内存使用优化
  - [ ] 分析内存使用
  - [ ] 减少内存分配
  - [ ] 优化数据结构

### 3.4 开发者体验

- [ ] 实现 Action 宏
  - [ ] `#[action]` 属性宏
  - [ ] 简化 Action 定义
  - [ ] 自动生成样板代码
- [ ] 实现 Runtime 宏
  - [ ] `#[runtime]` 属性宏
  - [ ] 简化 Runtime 定义
  - [ ] 自动生成扩展方法
- [ ] 改进错误消息
  - [ ] 更详细的错误信息
  - [ ] 错误上下文
  - [ ] 错误恢复建议
- [ ] 改进调试支持
  - [ ] 添加调试日志
  - [ ] 实现调试工具
  - [ ] 可视化执行流程

---

## 📋 检查清单

### Phase 2 开始前检查

- [ ] Phase 1 所有任务已完成
- [ ] 所有测试通过
- [ ] 文档已更新
- [ ] 代码已审查
- [ ] 性能无明显退化

### Phase 2 完成后检查

- [ ] 所有外部模块已迁移
- [ ] 所有测试通过
- [ ] CI/CD 正常工作
- [ ] 文档已更新
- [ ] 代码已审查
- [ ] 性能无明显退化
- [ ] 旧代码已清理

### Phase 3 开始前检查

- [ ] Phase 2 所有任务已完成
- [ ] 系统稳定运行
- [ ] 用户反馈良好
- [ ] 技术债务已清理

---

## 🎯 里程碑

| 里程碑 | 目标日期 | 状态 | 说明 |
|--------|----------|------|------|
| **M1: 核心重构完成** | 2025-11-24 | ✅ 完成 | Phase 1 完成 |
| **M2: Runtime 迁移完成** | 2025-11-24 | ✅ 完成 | cice-runtimes 完成 |
| **M3: Action 迁移完成** | 2025-11-24 | ✅ 完成 | cice-actions 完成 |
| **M4: 外部模块迁移完成** | TBD | ⏳ 待开始 | Phase 2 完成 |
| **M5: 功能增强完成** | TBD | 🔮 计划中 | Phase 3 完成 |

---

## 📝 注意事项

### 迁移优先级

1. **高优先级**: cice-runtime-vnc（核心 Runtime）
2. **高优先级**: cice-action-opencv（核心 Action）
3. **中优先级**: CI/CD 更新
4. **低优先级**: 功能增强

### 风险和挑战

1. **兼容性风险**: 确保迁移后行为与旧版本一致
2. **性能风险**: 监控性能变化，避免性能退化
3. **测试覆盖**: 确保测试覆盖率不降低
4. **文档同步**: 及时更新文档，避免文档过时

### 最佳实践

1. **增量迁移**: 一次迁移一个模块，逐步验证
2. **保持测试**: 确保每次迁移后测试都通过
3. **及时文档**: 迁移完成后立即更新文档
4. **代码审查**: 每次迁移都进行代码审查
5. **性能监控**: 持续监控性能指标

---

## 🔗 相关资源

### 文档

- [README.md](README.md) - 重构概述
- [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - 快速参考
- [SUMMARY.md](SUMMARY.md) - 完整总结
- [runtime-refactor.md](runtime-refactor.md) - 详细设计

### 代码示例

- [TestRuntime](../../crates/dev/cice-tests-common/src/action/mod.rs)
- [SimpleAction](../../crates/dev/cice-tests-common/src/action/mod.rs)
- [集成测试](../../crates/cice-core/tests/base_task.rs)

### 工具

- `cargo test` - 运行测试
- `cargo clippy` - 代码检查
- `cargo fmt` - 代码格式化
- `cargo doc` - 生成文档

---

**维护者**: Cice Team
**最后更新**: 2025-11-24
**文档版本**: v1.0
