use async_trait::async_trait;
use cice_core::action::{Action, ExecError, RecognizeError};
use cice_core::runtime::Runtime;
use std::sync::Arc;

/// 测试用的简单 Runtime 实现
#[derive(Clone)]
pub struct TestRuntime {
    _inner: Arc<TestRuntimeInner>,
}

struct TestRuntimeInner {
    // 可以在这里添加共享状态
}

impl TestRuntime {
    pub fn new() -> Self {
        Self {
            _inner: Arc::new(TestRuntimeInner {}),
        }
    }
}

impl Default for TestRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime for TestRuntime {}

/// 简单的 Action 实现 - 总是成功
pub struct SimpleAction {
    name: String,
}

impl SimpleAction {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[async_trait]
impl Action<TestRuntime> for SimpleAction {
    async fn recognize(&self, _runtime: &TestRuntime) -> Result<(), RecognizeError> {
        log::debug!("SimpleAction {} recognize", self.name);
        Ok(())
    }

    async fn exec(&self, _runtime: &TestRuntime) -> Result<(), ExecError> {
        log::debug!("SimpleAction {} exec", self.name);
        Ok(())
    }
}

/// 总是识别失败的 Action
pub struct DenyAction {
    name: String,
}

impl DenyAction {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[async_trait]
impl Action<TestRuntime> for DenyAction {
    async fn recognize(&self, _runtime: &TestRuntime) -> Result<(), RecognizeError> {
        log::debug!("DenyAction {} recognize - denied", self.name);
        Err(RecognizeError::UnRecognized)
    }

    async fn exec(&self, _runtime: &TestRuntime) -> Result<(), ExecError> {
        log::debug!("DenyAction {} exec", self.name);
        Ok(())
    }
}

/// 带配置的 Action
pub struct ConfigurableAction {
    name: String,
    should_succeed: bool,
}

impl ConfigurableAction {
    pub fn new(name: impl Into<String>, should_succeed: bool) -> Self {
        Self {
            name: name.into(),
            should_succeed,
        }
    }
}

#[async_trait]
impl Action<TestRuntime> for ConfigurableAction {
    async fn recognize(&self, _runtime: &TestRuntime) -> Result<(), RecognizeError> {
        log::debug!("ConfigurableAction {} recognize", self.name);
        if self.should_succeed {
            Ok(())
        } else {
            Err(RecognizeError::UnRecognized)
        }
    }

    async fn exec(&self, _runtime: &TestRuntime) -> Result<(), ExecError> {
        log::debug!("ConfigurableAction {} exec", self.name);
        if self.should_succeed {
            Ok(())
        } else {
            Err(ExecError::ExecFailed {
                reason: "configured to fail".to_string(),
            })
        }
    }
}
