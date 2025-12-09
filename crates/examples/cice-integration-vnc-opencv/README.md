# VNC + OpenCV 集成示例

展示如何将 `cice-runtime-vnc` 和 `cice-action-opencv` 配合使用。

## 运行示例

```bash
cargo run --example vnc_template_match
```

## 示例说明

该示例演示了：

1. 创建 VNC Runtime 并连接到 VNC 服务器
2. 为 VncRuntime 实现 ScreenshotExt trait
3. 创建模板匹配 Actions
4. 构建 Context 并添加任务
5. 运行任务序列
6. 断开 VNC 连接

## 前置条件

- VNC 服务器运行在 `localhost:5900`
- 准备好模板图像文件：
  - `templates/login_button.png`
  - `templates/app_icon.png`

## 代码结构

```rust
// 1. 为 VncRuntime 实现 ScreenshotExt
impl ScreenshotExt for VncRuntime {
    async fn get_screenshot(&self) -> Result<DynamicImage, String> {
        // 实现截图逻辑
    }
}

// 2. 创建 Runtime 和 Actions
let runtime = VncRuntime::new("localhost:5900", None);
let action = TemplateMatchAction::new(...);

// 3. 构建 Context
let mut builder = ContextBuilder::new(runtime);
builder.add_task(config, &action, params);  // params 为 Action 所需的参数

// 4. 运行
builder.build().run("entry").await;
```
