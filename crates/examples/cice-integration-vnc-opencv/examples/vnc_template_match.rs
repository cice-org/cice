use cice_action_opencv::{TemplateMatchAction, TemplateMatchConfig};
use cice_core::action::ActionParams;
use cice_core::context::ContextBuilder;
use cice_core::task::TaskConfig;
use cice_runtime_vnc::VncRuntime;
use std::time::Duration;

/// 示例用的空参数实现
#[derive(Clone, Default)]
struct EmptyParams;

impl ActionParams for EmptyParams {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== VNC + OpenCV 模板匹配示例 ===\n");

    // 1. 创建 VNC Runtime
    println!("1. 创建 VNC Runtime...");
    let runtime = VncRuntime::new("localhost:5900", None);

    // 2. 连接到 VNC 服务器
    println!("2. 连接到 VNC 服务器...");
    runtime.connect().await?;
    println!("   ✓ 已连接\n");

    // 3. 创建模板匹配 Action
    println!("3. 创建模板匹配 Action...");
    let find_button_action = TemplateMatchAction::new(
        "find_login_button",
        TemplateMatchConfig {
            template_path: "templates/login_button.png".to_string(),
            threshold: 0.8, // 匹配阈值
            roi: None,      // 全屏搜索
        },
    );

    let find_icon_action = TemplateMatchAction::new(
        "find_app_icon",
        TemplateMatchConfig {
            template_path: "templates/app_icon.png".to_string(),
            threshold: 0.9,              // 匹配阈值
            roi: Some([0, 0, 800, 600]), // 只在左上角区域搜索
        },
    );

    // 4. 构建 Context
    println!("4. 构建 Context...");
    let mut builder = ContextBuilder::new(runtime);

    // 添加任务：查找登录按钮
    builder.add_task(
        TaskConfig {
            task_name: "find_login_button".to_string(),
            action_name: "find_login_button".to_string(),
            next_task: vec!["find_app_icon".to_string()],
            interrupt_task: vec![],
            timeout: Duration::from_secs(30),
            max_retry: 5,
        },
        &find_button_action,
        EmptyParams,
    );

    // 添加任务：查找应用图标
    builder.add_task(
        TaskConfig {
            task_name: "find_app_icon".to_string(),
            action_name: "find_app_icon".to_string(),
            next_task: vec![],
            interrupt_task: vec![],
            timeout: Duration::from_secs(30),
            max_retry: 5,
        },
        &find_icon_action,
        EmptyParams,
    );

    // 5. 运行任务
    println!("5. 运行任务...\n");
    let context = builder.build();
    match context.run("find_login_button".to_string()).await {
        Ok(_) => println!("\n✓ 所有任务执行成功！"),
        Err(e) => println!("\n✗ 任务执行失败: {:?}", e),
    }

    // 6. 断开连接
    println!("\n6. 断开 VNC 连接...");
    // runtime.disconnect().await?;
    println!("   ✓ 已断开\n");

    Ok(())
}
