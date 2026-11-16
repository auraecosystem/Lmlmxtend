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
        if valid {
            Ok(Self(value))
        } else {
            Err(ContractError::InvalidCapabilityId)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
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

/// Errors that can be returned by an extension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExtensionError {
    InvalidInput(String),
    ExecutionFailed(String),
    UnsupportedOperation,
}

/// Implement this contract to expose a capability to a LMLMxtend host.
///
/// Implementations must document resource limits and must not assume that
/// caller-provided input is trusted.
pub trait Extension: Send + Sync {
    fn manifest(&self) -> ExtensionManifest;
    fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse, ExtensionError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractError {
    InvalidCapabilityId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_namespaced_capability() {
        assert_eq!(CapabilityId::new("creative.generate").unwrap().as_str(), "creative.generate");
    }

    #[test]
    fn rejects_malformed_capability() {
        assert_eq!(CapabilityId::new("Creative..generate"), Err(ContractError::InvalidCapabilityId));
    }
}
