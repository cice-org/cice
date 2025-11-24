use async_trait::async_trait;
use cice_core::action::{Action, ExecError, RecognizeError};
use cice_core::runtime::ext::ScreenshotExt;
use opencv::core::{Mat, MatTraitConst};
use opencv::imgcodecs;
use serde::{Deserialize, Serialize};

/// 模板匹配 Action 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMatchConfig {
    /// 模板图像路径
    pub template_path: String,
    /// 匹配置信度阈值（0.0 - 1.0）
    #[serde(default = "default_threshold")]
    pub threshold: f64,
    /// 感兴趣区域 [x, y, width, height]
    #[serde(default)]
    pub roi: Option<[u64; 4]>,
}

fn default_threshold() -> f64 {
    0.8
}

/// 模板匹配结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMatchResult {
    /// 是否匹配成功
    pub matched: bool,
    /// 匹配位置
    pub position: Position,
    /// 模板尺寸
    pub size: Size,
    /// 匹配置信度
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

/// 模板匹配 Action
///
/// 在屏幕截图中查找指定的模板图像
///
/// # 泛型参数
/// - `R`: Runtime 类型，必须实现 `ScreenshotExt` 扩展
///
/// # 示例
///
/// ```rust
/// use cice_action_opencv::TemplateMatchAction;
///
/// let action = TemplateMatchAction::new(
///     "find_button",
///     "/path/to/template.png",
///     0.8,
///     None,
/// );
/// ```
pub struct TemplateMatchAction {
    name: String,
    config: TemplateMatchConfig,
}

impl TemplateMatchAction {
    /// 创建新的模板匹配 Action
    ///
    /// # 参数
    /// - `name`: Action 名称
    /// - `config`: 模板匹配配置
    pub fn new(name: impl Into<String>, config: TemplateMatchConfig) -> Self {
        Self {
            name: name.into(),
            config,
        }
    }

    /// 执行模板匹配
    fn template_match(
        src: Mat,
        template: &Mat,
        roi: Option<[u64; 4]>,
        threshold: f64,
    ) -> Result<TemplateMatchResult, String> {
        // 应用 ROI（如果指定）
        let input_roi = if let Some([x, y, width, height]) = roi {
            let rect = opencv::core::Rect::new(x as i32, y as i32, width as i32, height as i32);
            opencv::core::Mat::roi(&src, rect).map_err(|e| format!("Failed to apply ROI: {}", e))?
        } else {
            src.into()
        };

        // 执行模板匹配
        let mut result = opencv::core::Mat::default();
        opencv::imgproc::match_template(
            &input_roi,
            template,
            &mut result,
            opencv::imgproc::TM_CCOEFF_NORMED,
            &opencv::core::no_array(),
        )
        .map_err(|e| format!("Template matching failed: {}", e))?;

        // 查找最佳匹配位置
        let (mut min_val, mut max_val, mut min_loc, mut max_loc) = (
            f64::MIN,
            f64::MAX,
            opencv::core::Point::new(0, 0),
            opencv::core::Point::new(0, 0),
        );
        opencv::core::min_max_loc(
            &result,
            Some(&mut min_val),
            Some(&mut max_val),
            Some(&mut min_loc),
            Some(&mut max_loc),
            &opencv::core::no_array(),
        )
        .map_err(|e| format!("Failed to find match location: {}", e))?;

        Ok(TemplateMatchResult {
            matched: max_val > threshold,
            position: Position {
                x: max_loc.x,
                y: max_loc.y,
            },
            size: Size {
                width: template.cols(),
                height: template.rows(),
            },
            confidence: max_val,
        })
    }
}

#[async_trait]
impl<R> Action<R> for TemplateMatchAction
where
    R: ScreenshotExt,
{
    async fn recognize(&self, runtime: &R) -> Result<(), RecognizeError> {
        // 获取屏幕截图
        let screenshot = runtime
            .screenshot()
            .await
            .ok_or(RecognizeError::RecognizeFailed {
                reason: "Failed to get screenshot".to_string(),
            })?;
        let src_mat = imgcodecs::imdecode(&screenshot.as_slice(), imgcodecs::IMREAD_COLOR)
            .map_err(|e| RecognizeError::RecognizeFailed {
                reason: format!("Failed to decode screenshot: {}", e),
            })?;
        let template_mat =
            opencv::imgcodecs::imread(&self.config.template_path, opencv::imgcodecs::IMREAD_COLOR)
                .map_err(|e| RecognizeError::RecognizeFailed {
                    reason: format!("Failed to load template: {}", e),
                })?;

        // 执行模板匹配
        let result = Self::template_match(
            src_mat,
            &template_mat,
            self.config.roi,
            self.config.threshold,
        )
        .map_err(|e| RecognizeError::RecognizeFailed { reason: e })?;

        // 检查是否匹配
        if result.matched {
            Ok(())
        } else {
            Err(RecognizeError::UnRecognized)
        }
    }

    async fn exec(&self, _runtime: &R) -> Result<(), ExecError> {
        // 模板匹配 Action 只需要识别，不需要执行额外操作
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::TryToCv;

    use super::*;

    #[test]
    fn test_template_match_config_default() {
        let config = TemplateMatchConfig {
            template_path: "test.png".to_string(),
            threshold: default_threshold(),
            roi: None,
        };

        assert_eq!(config.threshold, 0.8);
        assert!(config.roi.is_none());
    }

    #[test]
    fn test_template_match_action_creation() {
        let action = TemplateMatchAction::new(
            "test",
            TemplateMatchConfig {
                template_path: "template.png".to_string(),
                threshold: 0.9,
                roi: None,
            },
        );
        assert_eq!(action.name, "test");
        assert_eq!(action.config.template_path, "template.png");
        assert_eq!(action.config.threshold, 0.9);
    }

    #[test]
    fn test_template_match_same_file() {
        // 测试相同文件的模板匹配，应该完全匹配
        let src_image = image::open(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/resource/template_match/src-1.jpg",
        ))
        .unwrap();

        let template_image = image::open(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/resource/template_match/src-1.jpg",
        ))
        .unwrap();

        let src_mat = src_image.try_to_cv().unwrap();
        let template_mat = template_image.try_to_cv().unwrap();

        let result =
            TemplateMatchAction::template_match(src_mat, &template_mat, None, 0.8).unwrap();

        // 相同文件应该在 (0, 0) 位置完全匹配，置信度应该非常高
        assert_eq!(result.position.x, 0);
        assert_eq!(result.position.y, 0);
        assert!(result.confidence > 0.95);
        assert!(result.matched);
    }

    #[test]
    fn test_template_match_sub_file() {
        // 测试子图像的模板匹配
        let src_image = image::open(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/resource/template_match/src-1.jpg",
        ))
        .unwrap();

        let template_image = image::open(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/resource/template_match/template-1.jpg",
        ))
        .unwrap();

        let src_mat = src_image.try_to_cv().unwrap();
        let template_mat = template_image.try_to_cv().unwrap();

        let result =
            TemplateMatchAction::template_match(src_mat, &template_mat, None, 0.8).unwrap();

        // 子图像应该在特定位置匹配，置信度应该很高
        assert_eq!(result.position.x, 0);
        assert!((result.position.y - 84).abs() < 1);
        assert!(result.confidence > 0.95);
        assert!(result.matched);
    }
}
