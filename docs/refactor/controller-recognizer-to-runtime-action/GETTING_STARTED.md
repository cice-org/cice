# 快速开始指南 - VNC + OpenCV

> 5 分钟快速上手 cice-runtime-vnc 和 cice-action-opencv

---

## 🎯 目标

本指南将帮助你：
1. 创建一个 VNC Runtime
2. 创建一个模板匹配 Action
3. 运行一个简单的图像识别任务

---

## 📋 前置条件

1. Rust 工具链（1.70+）
2. OpenCV 库（通过 vcpkg 或系统包管理器安装）
3. VNC 服务器（可选，用于实际测试）

---

## 🚀 步骤 1: 添加依赖

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
cice-core = { path = "path/to/cice-core" }
cice-runtime-vnc = { path = "path/to/cice-runtime-vnc" }
cice-action-opencv = { path = "path/to/cice-action-opencv" }
async-trait = "0.1"
tokio = { version = "1", features = ["full"] }
image = "0.25"
```

---

## 🚀 步骤 2: 实现 ScreenshotExt

为 VncRuntime 实现截图功能：

```rust
use async_trait::async_trait;
use cice_action_opencv::ScreenshotExt;
use cice_runtime_vnc::VncRuntime;
use image::DynamicImage;

#[async_trait]
impl ScreenshotExt for VncRuntime {
    async fn get_screenshot(&self) -> Result<DynamicImage, String> {
        // 获取 VNC 截图
        let data = self.screenshot().await
            .map_err(|e| format!("Screenshot failed: {}", e))?;

        // 转换为 DynamicImage
        image::load_from_memory(&data)
            .map_err(|e| format!("Failed to load image: {}", e))
    }
}
```

---

## 🚀 步骤 3: 创建 Runtime 和 Action

```rust
use cice_runtime_vnc::VncRuntime;
use cice_action_opencv::TemplateMatchAction;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 创建 VNC Runtime
    let runtime = VncRuntime::new("localhost:5900", None);

    // 2. 连接到 VNC 服务器
    runtime.connect().await?;

    // 3. 创建模板匹配 Action
    let action = TemplateMatchAction::new(
        "find_button",
        "templates/button.png",  // 模板图像路径
        0.8,                      // 匹配阈值
        None,                     // ROI（可选）
    );

    // 4. 执行识别
    match action.recognize(&runtime).await {
        Ok(_) => println!("✓ 找到按钮！"),
        Err(e) => println!("✗ 未找到按钮: {:?}", e),
    }

    // 5. 断开连接
    runtime.disconnect().await?;

    Ok(())
}
```

---

## 🚀 步骤 4: 使用 Context 运行任务序列

```rust
use cice_core::context::ContextBuilder;
use cice_core::task::TaskConfig;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 创建 Runtime
    let runtime = VncRuntime::new("localhost:5900", None);
    runtime.connect().await?;

    // 2. 创建 Actions
    let find_login = TemplateMatchAction::new(
        "find_login",
        "templates/login_button.png",
        0.8,
        None,
    );

    let find_submit = TemplateMatchAction::new(
        "find_submit",
        "templates/submit_button.png",
        0.8,
        None,
    );

    // 3. 构建 Context
    let mut builder = ContextBuilder::new(runtime);

    // 添加任务 1: 查找登录按钮
    builder.add_task(
        TaskConfig {
            task_name: "find_login".to_string(),
            action_name: "find_login".to_string(),
            next_task: vec!["find_submit".to_string()],
            interrupt_task: vec![],
            timeout: Duration::from_secs(30),
            max_retry: 5,
        },
        &find_login,
    );

    // 添加任务 2: 查找提交按钮
    builder.add_task(
        TaskConfig {
            task_name: "find_submit".to_string(),
            action_name: "find_submit".to_string(),
            next_task: vec![],
            interrupt_task: vec![],
            timeout: Duration::from_secs(30),
            max_retry: 5,
        },
        &find_submit,
    );

    // 4. 运行任务序列
    let context = builder.build();
    context.run("find_login".to_string()).await?;

    println!("✓ 所有任务完成！");

    Ok(())
}
```

---

## 🎨 高级用法

### 使用 ROI（感兴趣区域）

只在屏幕的特定区域查找模板：

```rust
let action = TemplateMatchAction::new(
    "find_icon",
    "templates/icon.png",
    0.9,
    Some([100, 100, 800, 600]), // [x, y, width, height]
);
```

### 从配置创建 Action

```rust
use cice_action_opencv::TemplateMatchConfig;

let config = TemplateMatchConfig {
    template_path: "templates/button.png".to_string(),
    threshold: 0.85,
    roi: Some([0, 0, 1920, 1080]),
};

let action = TemplateMatchAction::from_config("my_action", config);
```

### 调整匹配阈值

```rust
// 高阈值 = 更严格的匹配
let strict_action = TemplateMatchAction::new(
    "strict_match",
    "template.png",
    0.95,  // 95% 相似度
    None,
);

// 低阈值 = 更宽松的匹配
let loose_action = TemplateMatchAction::new(
    "loose_match",
    "template.png",
    0.7,   // 70% 相似度
    None,
);
```

---

## 🐛 常见问题

### Q1: 编译错误 - 找不到 OpenCV

**解决方案**: 安装 OpenCV 库

```bash
# macOS (Homebrew)
brew install opencv

# Ubuntu
sudo apt-get install libopencv-dev

# Windows (vcpkg)
vcpkg install opencv4
```

### Q2: 运行时错误 - 连接 VNC 失败

**解决方案**: 确保 VNC 服务器正在运行

```bash
# 检查 VNC 服务器状态
netstat -an | grep 5900
```

### Q3: 识别失败 - 总是返回 UnRecognized

**可能原因**:
1. 模板图像路径错误
2. 匹配阈值太高
3. 屏幕分辨率不匹配

**解决方案**:
1. 检查模板图像路径
2. 降低阈值（如 0.7）
3. 使用 ROI 缩小搜索范围

### Q4: 性能问题 - 识别太慢

**优化建议**:
1. 使用 ROI 减少搜索区域
2. 降低模板图像分辨率
3. 使用更快的匹配算法

---

## 📚 下一步

- 阅读 [cice-runtime-vnc README](../../crates/cice-runtimes/cice-runtime-vnc/README.md)
- 阅读 [cice-action-opencv README](../../crates/cice-actions/cice-action-opencv/README.md)
- 查看 [集成示例](../../crates/examples/cice-integration-vnc-opencv/examples/vnc_template_match.rs)
- 阅读 [QUICK_REFERENCE.md](QUICK_REFERENCE.md)

---

## 💡 提示

1. **开发时**: 使用较低的阈值（0.7-0.8）以便快速验证
2. **生产时**: 使用较高的阈值（0.85-0.95）以确保准确性
3. **调试时**: 保存截图和模板图像以便分析
4. **性能优化**: 优先使用 ROI 而非全屏搜索

---

**版本**: v1.0
**最后更新**: 2025-11-24
