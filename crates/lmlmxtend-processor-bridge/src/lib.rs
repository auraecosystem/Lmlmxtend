//! Typed boundary between LMLMxtend and an independently implemented LMLM processor.
//!
//! This crate deliberately does not implement inference or depend on a model runtime.
//! A concrete LMLM backend implements `LmlmProcessor` and is injected by the host.
//! The typed API carries arbitrary modality payloads; the generic Extension adapter
//! currently exposes UTF-8 text requests through the existing string-based contract.

use lmlmxtend_api::{
    CapabilityId, ExecutionContext, Extension, ExtensionError, ExtensionManifest, ExtensionRequest,
    ExtensionResponse,
};
use std::fmt;
use std::sync::Arc;

/// Input modality understood by a processor implementation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Modality {
    Text,
    Image,
    Audio,
    Video,
    Multimodal,
}

/// A processor input. Payload bytes are opaque to the bridge and interpreted by
/// the concrete LMLM backend according to `modality` and `content_type`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessorRequest {
    pub modality: Modality,
    pub payload: Vec<u8>,
    pub content_type: String,
    pub prompt: Option<String>,
    pub correlation_id: Option<String>,
}

impl ProcessorRequest {
    pub fn text(input: impl Into<String>) -> Self {
        Self {
            modality: Modality::Text,
            payload: input.into().into_bytes(),
            content_type: "text/plain; charset=utf-8".into(),
            prompt: None,
            correlation_id: None,
        }
    }
}

/// Opaque processor output. The backend declares the media type of its payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessorResponse {
    pub payload: Vec<u8>,
    pub content_type: String,
}

/// Errors reported by a concrete processor implementation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProcessorError {
    UnsupportedModality(Modality),
    InvalidInput(String),
    Unavailable(String),
    Failed(String),
}

impl fmt::Display for ProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedModality(modality) => {
                write!(f, "processor does not support {modality:?}")
            }
            Self::InvalidInput(message) => write!(f, "invalid processor input: {message}"),
            Self::Unavailable(message) => write!(f, "processor unavailable: {message}"),
            Self::Failed(message) => write!(f, "processor failed: {message}"),
        }
    }
}

impl std::error::Error for ProcessorError {}

/// Implement this trait in the LMLM repository or in a host-owned integration crate.
///
/// Implementations must check the supplied execution context between bounded units
/// of work. The bridge cannot forcibly interrupt a backend that ignores the context.
pub trait LmlmProcessor: Send + Sync {
    fn process(
        &self,
        request: ProcessorRequest,
        context: &ExecutionContext,
    ) -> Result<ProcessorResponse, ProcessorError>;

    /// A lightweight readiness check; it should not load weights or run inference.
    fn health_check(&self) -> Result<(), ProcessorError> {
        Ok(())
    }
}

/// Adapter that exposes an injected LMLM processor to the LMLMxtend extension host.
pub struct ProcessorBridge {
    processor: Arc<dyn LmlmProcessor>,
    max_request_bytes: usize,
}

impl ProcessorBridge {
    pub const EXTENSION_ID: &'static str = "lmlm.processor.bridge";
    pub const CAPABILITY_ID: &'static str = "lmlm.process";

    pub fn new(processor: Arc<dyn LmlmProcessor>) -> Self {
        Self::with_max_request_bytes(processor, 8 * 1024 * 1024)
    }

    pub fn with_max_request_bytes(
        processor: Arc<dyn LmlmProcessor>,
        max_request_bytes: usize,
    ) -> Self {
        Self {
            processor,
            max_request_bytes,
        }
    }

    /// Calls the processor using the typed, modality-aware contract.
    pub fn process(
        &self,
        request: ProcessorRequest,
        context: &ExecutionContext,
    ) -> Result<ProcessorResponse, ProcessorBridgeError> {
        context
            .check_cancelled()
            .map_err(ProcessorBridgeError::ExecutionControl)?;
        if request.payload.len() > self.max_request_bytes {
            return Err(ProcessorBridgeError::RequestTooLarge {
                actual: request.payload.len(),
                limit: self.max_request_bytes,
            });
        }
        let response = self
            .processor
            .process(request, context)
            .map_err(ProcessorBridgeError::Processor)?;
        context
            .check_cancelled()
            .map_err(ProcessorBridgeError::ExecutionControl)?;
        if response.payload.len() > context.max_output_bytes() {
            return Err(ProcessorBridgeError::OutputTooLarge {
                actual: response.payload.len(),
                limit: context.max_output_bytes(),
            });
        }
        Ok(response)
    }
}

impl Extension for ProcessorBridge {
    fn manifest(&self) -> ExtensionManifest {
        ExtensionManifest {
            id: Self::EXTENSION_ID.into(),
            version: env!("CARGO_PKG_VERSION").into(),
            api_version: lmlmxtend_api::API_VERSION,
            capabilities: vec![CapabilityId::new(Self::CAPABILITY_ID)
                .expect("the bridge capability identifier is static and valid")],
        }
    }

    fn health_check(&self) -> Result<(), ExtensionError> {
        self.processor
            .health_check()
            .map_err(|error| ExtensionError::ExecutionFailed(error.to_string()))
    }

    fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse, ExtensionError> {
        self.execute_with_context(request, &ExecutionContext::default())
    }

    fn execute_with_context(
        &self,
        request: ExtensionRequest,
        context: &ExecutionContext,
    ) -> Result<ExtensionResponse, ExtensionError> {
        if request.capability.as_str() != Self::CAPABILITY_ID {
            return Err(ExtensionError::UnsupportedOperation);
        }
        let text = request.input;
        let mut processor_request = ProcessorRequest::text(text);
        processor_request.correlation_id = request.correlation_id;
        let response = self
            .process(processor_request, context)
            .map_err(|error| ExtensionError::ExecutionFailed(error.to_string()))?;
        let output = String::from_utf8(response.payload)
            .map_err(|_| ExtensionError::ExecutionFailed(
                "binary processor output cannot be represented by the text extension contract; use ProcessorBridge::process".into(),
            ))?;
        Ok(ExtensionResponse {
            output,
            media_type: response.content_type,
        })
    }
}

/// Failures enforced by the bridge before or after delegating to the processor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProcessorBridgeError {
    ExecutionControl(lmlmxtend_api::ExecutionControlError),
    RequestTooLarge { actual: usize, limit: usize },
    OutputTooLarge { actual: usize, limit: usize },
    Processor(ProcessorError),
}

impl fmt::Display for ProcessorBridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExecutionControl(error) => write!(f, "{error}"),
            Self::RequestTooLarge { actual, limit } => {
                write!(
                    f,
                    "processor request is too large: {actual} bytes (limit {limit})"
                )
            }
            Self::OutputTooLarge { actual, limit } => {
                write!(
                    f,
                    "processor output is too large: {actual} bytes (limit {limit})"
                )
            }
            Self::Processor(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for ProcessorBridgeError {}

#[cfg(test)]
mod tests {
    use super::*;
    use lmlmxtend_api::{CancellationToken, CapabilityId, ExecutionContext, ExtensionRequest};
    use std::sync::Arc;

    struct TestProcessor;

    impl LmlmProcessor for TestProcessor {
        fn process(
            &self,
            request: ProcessorRequest,
            _context: &ExecutionContext,
        ) -> Result<ProcessorResponse, ProcessorError> {
            Ok(ProcessorResponse {
                payload: request.payload,
                content_type: request.content_type,
            })
        }
    }

    fn bridge() -> ProcessorBridge {
        ProcessorBridge::new(Arc::new(TestProcessor))
    }

    #[test]
    fn typed_bridge_preserves_non_text_payloads() {
        let bridge = bridge();
        let request = ProcessorRequest {
            modality: Modality::Image,
            payload: vec![0, 1, 2, 255],
            content_type: "image/png".into(),
            prompt: Some("describe this image".into()),
            correlation_id: Some("test-1".into()),
        };
        let response = bridge
            .process(request.clone(), &ExecutionContext::default())
            .unwrap();
        assert_eq!(response.payload, request.payload);
        assert_eq!(response.content_type, "image/png");
    }

    #[test]
    fn extension_adapter_dispatches_text_to_processor() {
        let bridge = bridge();
        let request = ExtensionRequest::new(
            CapabilityId::new(ProcessorBridge::CAPABILITY_ID).unwrap(),
            "hello LMLM",
        );
        let response = bridge.execute(request).unwrap();
        assert_eq!(response.output, "hello LMLM");
        assert_eq!(response.media_type, "text/plain; charset=utf-8");
    }

    #[test]
    fn request_size_is_checked_before_processor_execution() {
        let bridge = ProcessorBridge::with_max_request_bytes(Arc::new(TestProcessor), 2);
        let error = bridge
            .process(
                ProcessorRequest::text("too long"),
                &ExecutionContext::default(),
            )
            .unwrap_err();
        assert!(matches!(
            error,
            ProcessorBridgeError::RequestTooLarge { limit: 2, .. }
        ));
    }

    #[test]
    fn cancelled_request_never_reaches_processor() {
        let bridge = bridge();
        let token = CancellationToken::new();
        token.cancel();
        let context = ExecutionContext::new(token, None, 1024);
        let error = bridge
            .process(ProcessorRequest::text("cancel me"), &context)
            .unwrap_err();
        assert_eq!(
            error,
            ProcessorBridgeError::ExecutionControl(lmlmxtend_api::ExecutionControlError::Cancelled)
        );
    }
}
