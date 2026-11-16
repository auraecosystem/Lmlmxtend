//! Minimal in-process extension host. This is not an isolation boundary.

use lmlmxtend_api::{CapabilityId, Extension, ExtensionError, ExtensionManifest, ExtensionRequest, ExtensionResponse};
use lmlmxtend_core::{ExecutionPolicy, PolicyDecision, RuntimeConfig};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeError {
    DuplicateExtension(String),
    DuplicateCapability(String),
    ExtensionNotFound(String),
    CapabilityNotDeclared(String),
    PolicyDenied,
    Extension(ExtensionError),
    InvalidManifest(String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateExtension(id) => write!(f, "extension already registered: {id}"),
            Self::DuplicateCapability(id) => write!(f, "capability already registered: {id}"),
            Self::ExtensionNotFound(id) => write!(f, "extension not found: {id}"),
            Self::CapabilityNotDeclared(id) => write!(f, "capability not declared: {id}"),
            Self::PolicyDenied => f.write_str("request denied by runtime policy"),
            Self::Extension(error) => write!(f, "{error}"),
            Self::InvalidManifest(reason) => write!(f, "invalid extension manifest: {reason}"),
        }
    }
}

impl std::error::Error for RuntimeError {}

/// Result of probing one registered extension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtensionHealth {
    pub extension_id: String,
    pub healthy: bool,
    pub error: Option<String>,
}

pub struct Runtime {
    policy: ExecutionPolicy,
    extensions: BTreeMap<String, Arc<dyn Extension>>,
    capabilities: BTreeMap<CapabilityId, String>,
}

impl Runtime {
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            policy: ExecutionPolicy::from_config(&config),
            extensions: BTreeMap::new(),
            capabilities: BTreeMap::new(),
        }
    }

    /// Registers an extension after validating its manifest and capability ownership.
    pub fn register(&mut self, extension: Arc<dyn Extension>) -> Result<(), RuntimeError> {
        let manifest: ExtensionManifest = extension.manifest();
        if manifest.id.trim().is_empty() || manifest.version.trim().is_empty() {
            return Err(RuntimeError::InvalidManifest("id and version are required".into()));
        }
        if manifest.api_version != lmlmxtend_api::API_VERSION {
            return Err(RuntimeError::InvalidManifest("incompatible API version".into()));
        }
        if self.extensions.contains_key(&manifest.id) {
            return Err(RuntimeError::DuplicateExtension(manifest.id));
        }
        // Validate the complete manifest before mutating runtime state. This also
        // catches duplicate declarations inside a single manifest.
        let mut declared = BTreeSet::new();
        for capability in &manifest.capabilities {
            if !declared.insert(capability.clone()) || self.capabilities.contains_key(capability) {
                return Err(RuntimeError::DuplicateCapability(capability.to_string()));
            }
        }
        for capability in manifest.capabilities {
            self.capabilities.insert(capability, manifest.id.clone());
        }
        self.extensions.insert(manifest.id, extension);
        Ok(())
    }

    /// Probes registered extensions in stable ID order without changing registry state.
    pub fn health_report(&self) -> Vec<ExtensionHealth> {
        self.extensions.iter().map(|(id, extension)| {
            match extension.health_check() {
                Ok(()) => ExtensionHealth { extension_id: id.clone(), healthy: true, error: None },
                Err(error) => ExtensionHealth { extension_id: id.clone(), healthy: false, error: Some(error.to_string()) },
            }
        }).collect()
    }

    /// Number of registered extensions.
    pub fn extension_count(&self) -> usize { self.extensions.len() }

    /// Dispatches a request only when the extension declared the capability
    /// and host policy permits the request.
    pub fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse, RuntimeError> {
        match self.policy.evaluate(&request.capability, &request.input) {
            PolicyDecision::Allow => {}
            PolicyDecision::DenyCapability | PolicyDecision::DenyInputTooLarge => {
                return Err(RuntimeError::PolicyDenied);
            }
        }
        let owner = self.capabilities.get(&request.capability)
            .ok_or_else(|| RuntimeError::CapabilityNotDeclared(request.capability.to_string()))?;
        let extension = self.extensions.get(owner)
            .ok_or_else(|| RuntimeError::ExtensionNotFound(owner.clone()))?;
        extension.execute(request).map_err(RuntimeError::Extension)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lmlmxtend_api::{ExtensionManifest, ExtensionRequest};
    use std::sync::Arc;

    struct Echo;

    impl Extension for Echo {
        fn manifest(&self) -> ExtensionManifest {
            ExtensionManifest {
                id: "example.echo".into(),
                version: "0.1.0".into(),
                api_version: lmlmxtend_api::API_VERSION,
                capabilities: vec![CapabilityId::new("text.echo").unwrap()],
            }
        }

        fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse, ExtensionError> {
            Ok(ExtensionResponse::text(request.input))
        }
    }

    #[test]
    fn registers_and_dispatches_declared_capability() {
        let mut runtime = Runtime::new(RuntimeConfig::default());
        runtime.register(Arc::new(Echo)).unwrap();
        let request = ExtensionRequest::new(CapabilityId::new("text.echo").unwrap(), "hello");
        assert_eq!(runtime.execute(request).unwrap().output, "hello");
    }

    #[test]
    fn rejects_duplicate_capabilities_within_one_manifest() {
        struct DuplicateCapability;

        impl Extension for DuplicateCapability {
            fn manifest(&self) -> ExtensionManifest {
                let capability = CapabilityId::new("text.echo").unwrap();
                ExtensionManifest {
                    id: "example.duplicate".into(),
                    version: "0.1.0".into(),
                    api_version: lmlmxtend_api::API_VERSION,
                    capabilities: vec![capability.clone(), capability],
                }
            }

            fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse, ExtensionError> {
                Ok(ExtensionResponse::text(request.input))
            }
        }

        let mut runtime = Runtime::new(RuntimeConfig::default());
        assert_eq!(
            runtime.register(Arc::new(DuplicateCapability)),
            Err(RuntimeError::DuplicateCapability("text.echo".into()))
        );
        // Failed registration is atomic: the rejected extension owns no capability.
        assert!(runtime.execute(ExtensionRequest::new(
            CapabilityId::new("text.echo").unwrap(),
            "hello"
        )).is_err());
    }

    #[test]
    fn rejects_input_over_configured_limit() {
        let config = RuntimeConfig { max_input_bytes: 3, allowed_capabilities: None };
        let mut runtime = Runtime::new(config);
        runtime.register(Arc::new(Echo)).unwrap();
        let request = ExtensionRequest::new(CapabilityId::new("text.echo").unwrap(), "long");
        assert_eq!(runtime.execute(request), Err(RuntimeError::PolicyDenied));
    }
}
