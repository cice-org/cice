# cice-action-opencv

OpenCV 图像识别 Actions 实现。

## 功能特性

- ✅ 模板匹配（Template Matching）
- ✅ 图像格式转换（OpenCV ↔ image-rs）
- ✅ ROI（感兴趣区域）支持
- ✅ 可配置的匹配阈值

## 使用示例

### 基本用法

```rust
use cice_action_opencv::{TemplateMatchAction, ScreenshotExt};
use cice_core::context::ContextBuilder;
use cice_core::runtime::Runtime;
use image::DynamicImage;

// 1. 实现 ScreenshotExt trait
struct MyRuntime;

impl Runtime for MyRuntime {}

#[async_trait::async_trait]
impl ScreenshotExt for MyRuntime {
    async fn get_screenshot(&self) -> Result<DynamicImage, String> {
        // 实现截图逻辑
        todo!()
    }
}

// 2. 创建 Action
let action = TemplateMatchAction::new(
    "find_button",
    "/path/to/button_template.png",
    0.8,  // 匹配阈值
    None, // ROI（可选）
);

// 3. 使用 Action
let runtime = MyRuntime;
let mut builder = ContextBuilder::new(runtime);
builder.add_task(task_config, &action, params);  // params 为 Action 所需的参数
```

### 使用 ROI

```rust
// 只在屏幕的特定区域查找模板
let action = TemplateMatchAction::new(
    "find_icon",
    "/path/to/icon.png",
    0.9,
    Some([100, 100, 800, 600]), // [x, y, width, height]
);
```

### 从配置创建

```rust
use cice_action_opencv::TemplateMatchConfig;

let config = TemplateMatchConfig {
    template_path: "/path/to/template.png".to_string(),
    threshold: 0.85,
    roi: Some([0, 0, 1920, 1080]),
};

let action = TemplateMatchAction::from_config("my_action", config);
```

## 与 VncRuntime 配合使用

```rust
use cice_runtime_vnc::VncRuntime;
use cice_action_opencv::{TemplateMatchAction, ScreenshotExt};
use image::DynamicImage;

// 为 VncRuntime 实现 ScreenshotExt
#[async_trait::async_trait]
impl ScreenshotExt for VncRuntime {
    async fn get_screenshot(&self) -> Result<DynamicImage, String> {
        let data = self.screenshot().await
            .map_err(|e| format!("Screenshot failed: {}", e))?;

        // 将字节数据转换为 DynamicImage
        image::load_from_memory(&data)
            .map_err(|e| format!("Failed to load image: {}", e))
    }
}

// 使用
#[tokio::main]
async fn main() {
    let runtime = VncRuntime::new("localhost:5900", None);
    runtime.connect().await.unwrap();

    let action = TemplateMatchAction::new(
        "find_login_button",
        "templates/login_button.png",
        0.8,
        None,
    );

    // 识别
    if action.recognize(&runtime).await.is_ok() {
        println!("找到登录按钮！");
    }
}
```

## API 文档

### TemplateMatchAction

模板匹配 Action，在屏幕截图中查找指定的模板图像。

#### 构造函数

- `new(name, template_path, threshold, roi) -> Self`
  - `name`: Action 名称
  - `template_path`: 模板图像路径
  - `threshold`: 匹配置信度阈值（0.0 - 1.0）
  - `roi`: 感兴趣区域 [x, y, width, height]（可选）

- `from_config(name, config) -> Self`
  - 从配置对象创建 Action

#### Action Trait 实现

- `async fn recognize(&self, runtime: &R, params: &P) -> Result<(), RecognizeError>`
  - 在屏幕截图中查找模板
  - 如果找到（置信度 > threshold），返回 `Ok(())`
  - 如果未找到，返回 `RecognizeError::UnRecognized`
  - 如果出错，返回 `RecognizeError::RecognizeFailed`

- `async fn exec(&self, runtime: &R, params: &P) -> Result<(), ExecError>`
  - 模板匹配不需要执行操作，直接返回 `Ok(())`

### TemplateMatchConfig

模板匹配配置。

```rust
pub struct TemplateMatchConfig {
    pub template_path: String,
    pub threshold: f64,        // 默认 0.8
    pub roi: Option<[u64; 4]>, // 默认 None
}
```

### TemplateMatchResult

模板匹配结果。

```rust
pub struct TemplateMatchResult {
    pub matched: bool,
    pub position: Position,  // { x, y }
    pub size: Size,          // { width, height }
    pub confidence: f64,
}
```

### ScreenshotExt Trait

Runtime 扩展 trait，用于获取屏幕截图。

```rust
pub trait ScreenshotExt: Runtime {
    async fn get_screenshot(&self) -> Result<DynamicImage, String>;
}
```

任何想要使用 `TemplateMatchAction` 的 Runtime 都需要实现此 trait。

## 图像格式转换

内部使用 `convert` 模块在 OpenCV 和 image-rs 之间转换图像格式。

支持的格式：
- Luma8, Luma16
- Rgb8, Rgb16, Rgb32F
- Rgba8, Rgba16, Rgba32F

## 测试

```bash
# 运行单元测试
cargo test -p cice-action-opencv

# 运行测试并显示输出
cargo test -p cice-action-opencv -- --nocapture
```

## 依赖

- `cice-core`: 核心 Action trait 定义
- `opencv`: OpenCV 绑定
- `image`: 图像处理库
- `async-trait`: 异步 trait 支持
- `serde`: 序列化支持

## 开发状态

✅ 已完成：
- [x] 模板匹配 Action
- [x] 图像格式转换
- [x] ROI 支持
- [x] 基本测试

🚧 待完成：
- [ ] 更多图像识别 Actions（边缘检测、特征匹配等）
- [ ] 性能优化
- [ ] 更完善的错误处理
- [ ] 集成测试

## 许可证

与 Cice 项目保持一致
