# cice-runtime-vnc

VNC Runtime 实现，提供 VNC 连接管理和屏幕控制功能。

## 功能特性

- ✅ VNC 连接管理（连接、断开、状态检查）
- ✅ 屏幕截图
- ✅ 鼠标操作（点击、移动）
- ✅ 键盘操作（文本输入、按键）
- ✅ 屏幕尺寸管理

## 使用示例

```rust
use cice_runtime_vnc::VncRuntime;

#[tokio::main]
async fn main() {
    // 创建 VNC Runtime
    let runtime = VncRuntime::new("localhost:5900", Some("password".to_string()));

    // 连接到 VNC 服务器
    runtime.connect().await.unwrap();

    // 获取屏幕截图
    let screenshot = runtime.screenshot().await.unwrap();

    // 点击屏幕
    runtime.click(100, 100, 1).await.unwrap();

    // 输入文本
    runtime.type_text("Hello, World!").await.unwrap();

    // 断开连接
    runtime.disconnect().await.unwrap();
}
```

## 与 Action 配合使用

```rust
use cice_core::context::ContextBuilder;
use cice_runtime_vnc::VncRuntime;

#[tokio::main]
async fn main() {
    // 创建 VNC Runtime
    let runtime = VncRuntime::new("localhost:5900", None);
    runtime.connect().await.unwrap();

    // 创建 Context
    let mut builder = ContextBuilder::new(runtime);

    // 添加 Actions 和 Tasks
    // ...

    // 运行
    builder.build().run("entry".to_string()).await.unwrap();
}
```

## API 文档

### VncRuntime

#### 构造函数

- `new(address: impl Into<String>, password: Option<String>) -> Self`
  - 创建新的 VNC Runtime
  - `address`: VNC 服务器地址，格式为 "host:port"
  - `password`: VNC 连接密码（可选）

#### 连接管理

- `async fn connect(&self) -> Result<(), VncError>`
  - 连接到 VNC 服务器

- `async fn disconnect(&self) -> Result<(), VncError>`
  - 断开 VNC 连接

- `async fn is_connected(&self) -> bool`
  - 检查是否已连接

#### 屏幕操作

- `async fn screenshot(&self) -> Result<Vec<u8>, VncError>`
  - 获取屏幕截图

- `async fn get_screen_size(&self) -> (u32, u32)`
  - 获取屏幕尺寸

- `async fn set_screen_size(&self, width: u32, height: u32)`
  - 设置屏幕尺寸

#### 鼠标操作

- `async fn click(&self, x: u32, y: u32, button: u8) -> Result<(), VncError>`
  - 点击屏幕指定位置
  - `button`: 1=左键, 2=中键, 3=右键

- `async fn move_mouse(&self, x: u32, y: u32) -> Result<(), VncError>`
  - 移动鼠标到指定位置

#### 键盘操作

- `async fn type_text(&self, text: &str) -> Result<(), VncError>`
  - 输入文本

- `async fn press_key(&self, key: u32) -> Result<(), VncError>`
  - 按下键盘按键

### VncError

错误类型：

- `NotConnected`: 未连接到 VNC 服务器
- `ConnectionFailed(String)`: 连接失败
- `OperationFailed(String)`: 操作失败
- `InvalidParameter(String)`: 无效的参数

## 开发状态

⚠️ **注意**: 当前实现为框架代码，实际的 VNC 协议通信逻辑需要后续实现。

TODO:
- [ ] 实现实际的 VNC 协议通信
- [ ] 添加 VNC 客户端库依赖
- [ ] 实现真实的截图功能
- [ ] 实现真实的鼠标和键盘操作
- [ ] 添加更多错误处理
- [ ] 添加连接重试机制
- [ ] 添加性能优化

## 测试

```bash
# 运行单元测试
cargo test -p cice-runtime-vnc

# 运行测试并显示输出
cargo test -p cice-runtime-vnc -- --nocapture
```

## 依赖

- `cice-core`: 核心 Runtime trait 定义
- `async-trait`: 异步 trait 支持
- `tokio`: 异步运行时
- `serde`: 序列化支持

## 许可证

与 Cice 项目保持一致
