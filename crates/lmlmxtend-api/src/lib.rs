//! Stable, application-neutral contracts exposed by LMLMxtend.

use std::fmt;

/// Contract version implemented by this API crate.
pub const API_VERSION: u16 = 1;

/// A validated, namespaced capability identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CapabilityId(String);

impl CapabilityId {
    /// Creates an identifier such as `creative.generate` or `vision.describe`.
    pub fn new(value: impl Into<String>) -> Result<Self, ContractError> {
        let value = value.into();
        let valid = !value.is_empty()
            && value.len() <= 128
            && value.split('.').all(|part| {
                !part.is_empty()
                    && part.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
            });
        if valid { Ok(Self(value)) } else { Err(ContractError::InvalidCapabilityId) }
    }

    pub fn as_str(&self) -> &str { &self.0 }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.0) }
}

/// A request passed to an installed extension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtensionRequest {
    pub capability: CapabilityId,
    pub input: String,
    pub correlation_id: Option<String>,
}

impl ExtensionRequest {
    pub fn new(capability: CapabilityId, input: impl Into<String>) -> Self {
        Self { capability, input: input.into(), correlation_id: None }
    }
}

/// A response returned by an extension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtensionResponse {
    pub output: String,
    pub media_type: String,
}

impl ExtensionResponse {
    pub fn text(output: impl Into<String>) -> Self {
        Self { output: output.into(), media_type: "text/plain; charset=utf-8".into() }
    }
}

/// Metadata advertised by an extension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtensionManifest {
    pub id: String,
    pub version: String,
    pub api_version: u16,
    pub capabilities: Vec<CapabilityId>,
}


/// Cooperative cancellation signal shared by a host and an extension.
#[derive(Clone, Debug, Default)]
pub struct CancellationToken(std::sync::Arc<std::sync::atomic::AtomicBool>);

impl CancellationToken {
    pub fn new() -> Self { Self::default() }
    pub fn cancel(&self) {
        self.0.store(true, std::sync::atomic::Ordering::Release);
    }
    pub fn is_cancelled(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::Acquire)
    }
}

/// Per-request execution controls. Deadlines use a monotonic clock and are
/// therefore process-local; they are not intended for serialization.
#[derive(Clone, Debug)]
pub struct ExecutionContext {
    cancellation: CancellationToken,
    deadline: Option<std::time::Instant>,
    max_output_bytes: usize,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self { cancellation: CancellationToken::new(), deadline: None, max_output_bytes: usize::MAX }
    }
}

impl ExecutionContext {
    pub fn new(cancellation: CancellationToken, deadline: Option<std::time::Instant>, max_output_bytes: usize) -> Self {
        Self { cancellation, deadline, max_output_bytes }
    }
    pub fn with_timeout(timeout: std::time::Duration, max_output_bytes: usize) -> Self {
        Self::new(CancellationToken::new(), Some(std::time::Instant::now() + timeout), max_output_bytes)
    }
    pub fn cancellation_token(&self) -> CancellationToken { self.cancellation.clone() }
    pub fn is_cancelled(&self) -> bool { self.cancellation.is_cancelled() }
    pub fn is_deadline_exceeded(&self) -> bool {
        self.deadline.is_some_and(|deadline| std::time::Instant::now() >= deadline)
    }
    pub fn max_output_bytes(&self) -> usize { self.max_output_bytes }
    pub fn limited_to_output(&self, max_output_bytes: usize) -> Self {
        let mut context = self.clone();
        context.max_output_bytes = context.max_output_bytes.min(max_output_bytes);
        context
    }
    /// Extensions should call this between bounded units of work.
    pub fn check_cancelled(&self) -> Result<(), ExecutionControlError> {
        if self.is_cancelled() { return Err(ExecutionControlError::Cancelled); }
        if self.is_deadline_exceeded() { return Err(ExecutionControlError::DeadlineExceeded); }
        Ok(())
    }
}

/// Host-enforced request lifecycle failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionControlError { Cancelled, DeadlineExceeded, OutputTooLarge }

impl fmt::Display for ExecutionControlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cancelled => f.write_str("execution cancelled"),
            Self::DeadlineExceeded => f.write_str("execution deadline exceeded"),
            Self::OutputTooLarge => f.write_str("extension output exceeds configured limit"),
        }
    }
}
impl std::error::Error for ExecutionControlError {}

/// Implement this contract to expose a capability to a LMLMxtend host.
/// Implementations must document resource limits and treat caller input as untrusted.
pub trait Extension: Send + Sync {
    fn manifest(&self) -> ExtensionManifest;

    /// Performs a lightweight readiness check. Implementations should not start
    /// expensive work or mutate extension state from this method.
    fn health_check(&self) -> Result<(), ExtensionError> { Ok(()) }

    fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse, ExtensionError>;

    /// Context-aware execution. The default preserves compatibility but cannot
    /// interrupt legacy implementations; cooperative extensions should override it.
    fn execute_with_context(&self, request: ExtensionRequest, context: &ExecutionContext) -> Result<ExtensionResponse, ExtensionError> {
        context.check_cancelled().map_err(|error| ExtensionError::ExecutionFailed(error.to_string()))?;
        self.execute(request)
    }
}

/// Errors that can be returned by an extension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExtensionError {
    InvalidInput(String),
    ExecutionFailed(String),
    UnsupportedOperation,
}

impl fmt::Display for ExtensionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message) => write!(f, "invalid extension input: {message}"),
            Self::ExecutionFailed(message) => write!(f, "extension execution failed: {message}"),
            Self::UnsupportedOperation => f.write_str("extension operation is unsupported"),
        }
    }
}

impl std::error::Error for ExtensionError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractError {
    InvalidCapabilityId,
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCapabilityId => f.write_str("invalid capability identifier"),
        }
    }
}

impl std::error::Error for ContractError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_errors_have_readable_messages() {
        assert_eq!(
            ExtensionError::UnsupportedOperation.to_string(),
            "extension operation is unsupported"
        );
    }

    #[test]
    fn accepts_namespaced_capability() {
        assert_eq!(CapabilityId::new("creative.generate").unwrap().as_str(), "creative.generate");
    }

    #[test]
    fn rejects_malformed_capability() {
        assert_eq!(CapabilityId::new("Creative..generate"), Err(ContractError::InvalidCapabilityId));
    }
}
