use std::any::Any;
use std::error::Error;
use dioxus_html::{HasResizeData, ResizeError, ResizeResult};
use dioxus_html::geometry::PixelsSize;
use crate::event::EventType;

pub trait TestResizeOperationFailedError : Error {
    fn clone_dyn(&self) -> Box<dyn TestResizeOperationFailedError>;
}

pub enum TestResizeError {
    NotSupported,
    OperationFailed(Box<dyn TestResizeOperationFailedError>)
}

impl Clone for TestResizeError {
    fn clone(&self) -> TestResizeError {
        match self {
            TestResizeError::NotSupported => TestResizeError::NotSupported,
            TestResizeError::OperationFailed(err) =>
                TestResizeError::OperationFailed(err.clone_dyn()),
        }
    }
}

impl From<TestResizeError> for ResizeError {
    fn from(value: TestResizeError) -> ResizeError {
        match value {
            TestResizeError::NotSupported => ResizeError::NotSupported,
            TestResizeError::OperationFailed(err) => ResizeError::OperationFailed(err),
        }
    }
}

#[derive(Clone)]
pub struct TestResizeData {
    pub border_box_size: Result<PixelsSize, TestResizeError>,
    pub content_box_size: Result<PixelsSize, TestResizeError>,
}

impl Default for TestResizeData {
    fn default() -> Self {
        TestResizeData {
            border_box_size: Ok(PixelsSize::default()),
            content_box_size: Ok(PixelsSize::default()),
        }
    }
}

impl HasResizeData for TestResizeData {
    fn get_border_box_size(&self) -> ResizeResult<PixelsSize> {
        Ok(self.border_box_size.clone()?)
    }

    fn get_content_box_size(&self) -> ResizeResult<PixelsSize> {
        Ok(self.content_box_size.clone()?)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ResizeEventType {
    Resize,
}

impl EventType for ResizeEventType {
    type Data = TestResizeData;

    fn name(self) -> &'static str {
        match self {
            ResizeEventType::Resize => "resize",
        }
    }
}

