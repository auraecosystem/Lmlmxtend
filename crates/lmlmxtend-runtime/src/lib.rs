//! Minimal in-process extension host. This is not an isolation boundary.

use lmlmxtend_api::{CapabilityId, ExecutionContext, Extension, ExtensionError, ExtensionManifest, ExtensionRequest, ExtensionResponse};
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
    Cancelled,
    DeadlineExceeded,
    OutputTooLarge,
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
            Self::Cancelled => f.write_str("execution cancelled"),
            Self::DeadlineExceeded => f.write_str("execution deadline exceeded"),
            Self::OutputTooLarge => f.write_str("extension output exceeds configured limit"),
            Self::Extension(error) => write!(f, "{error}"),
            Self::InvalidManifest(reason) => write!(f, "invalid extension manifest: {reason}"),
        }
    }
}

impl std::error::Error for RuntimeError {}

/// Read-only public metadata for a registered extension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtensionDescriptor {
    pub extension_id: String,
    pub version: String,
    pub api_version: u16,
    pub capabilities: Vec<CapabilityId>,
}

/// Result of probing one registered extension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtensionHealth {
    pub extension_id: String,
    pub healthy: bool,
    pub error: Option<String>,
}

pub struct Runtime {
    policy: ExecutionPolicy,
    max_output_bytes: usize,
    extensions: BTreeMap<String, Arc<dyn Extension>>,
    capabilities: BTreeMap<CapabilityId, String>,
}

impl Runtime {
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            policy: ExecutionPolicy::from_config(&config),
            max_output_bytes: config.max_output_bytes,
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

    /// Returns stable, read-only metadata for registered extensions.
    pub fn extensions(&self) -> Vec<ExtensionDescriptor> {
        self.extensions.iter().map(|(id, extension)| {
            let manifest = extension.manifest();
            ExtensionDescriptor {
                extension_id: id.clone(),
                version: manifest.version,
                api_version: manifest.api_version,
                capabilities: manifest.capabilities,
            }
        }).collect()
    }

    /// Finds registered providers for an exact capability identifier.
    pub fn discover_capability(&self, capability: &CapabilityId) -> Vec<ExtensionDescriptor> {
        self.extensions().into_iter()
            .filter(|extension| extension.capabilities.contains(capability))
            .collect()
    }

    /// Number of registered extensions.
    pub fn extension_count(&self) -> usize { self.extensions.len() }

    /// Dispatches a request only when the extension declared the capability
    /// and host policy permits the request.
    pub fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse, RuntimeError> {
        self.execute_with_context(request, &ExecutionContext::new(
            lmlmxtend_api::CancellationToken::new(), None, self.max_output_bytes
        ))
    }

    /// Dispatches with cooperative cancellation/deadline controls and a host-enforced output cap.
    /// Legacy extensions remain callable but cannot be interrupted while inside execute().
    pub fn execute_with_context(
        &self,
        request: ExtensionRequest,
        context: &ExecutionContext,
    ) -> Result<ExtensionResponse, RuntimeError> {
        context.check_cancelled().map_err(|error| match error {
            lmlmxtend_api::ExecutionControlError::Cancelled => RuntimeError::Cancelled,
            lmlmxtend_api::ExecutionControlError::DeadlineExceeded => RuntimeError::DeadlineExceeded,
            lmlmxtend_api::ExecutionControlError::OutputTooLarge => RuntimeError::OutputTooLarge,
        })?;
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
        let bounded_context = context.limited_to_output(self.max_output_bytes);
        let response = match extension.execute_with_context(request, &bounded_context) {
            Ok(response) => response,
            Err(error) => {
                // Prefer lifecycle errors when cooperative extension work observes them.
                context.check_cancelled().map_err(|control| match control {
                    lmlmxtend_api::ExecutionControlError::Cancelled => RuntimeError::Cancelled,
                    lmlmxtend_api::ExecutionControlError::DeadlineExceeded => RuntimeError::DeadlineExceeded,
                    lmlmxtend_api::ExecutionControlError::OutputTooLarge => RuntimeError::OutputTooLarge,
                })?;
                return Err(RuntimeError::Extension(error));
            }
        };
        context.check_cancelled().map_err(|error| match error {
            lmlmxtend_api::ExecutionControlError::Cancelled => RuntimeError::Cancelled,
            lmlmxtend_api::ExecutionControlError::DeadlineExceeded => RuntimeError::DeadlineExceeded,
            lmlmxtend_api::ExecutionControlError::OutputTooLarge => RuntimeError::OutputTooLarge,
        })?;
        if response.output.len() > self.max_output_bytes.min(context.max_output_bytes()) {
            return Err(RuntimeError::OutputTooLarge);
        }
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lmlmxtend_api::{CancellationToken, ExecutionContext, ExtensionManifest, ExtensionRequest};
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


    struct Unhealthy;

    impl Extension for Unhealthy {
        fn manifest(&self) -> ExtensionManifest {
            ExtensionManifest {
                id: "example.unhealthy".into(),
                version: "0.1.0".into(),
                api_version: lmlmxtend_api::API_VERSION,
                capabilities: vec![],
            }
        }

        fn health_check(&self) -> Result<(), ExtensionError> {
            Err(ExtensionError::ExecutionFailed("not ready".into()))
        }

        fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse, ExtensionError> {
            Ok(ExtensionResponse::text(request.input))
        }
    }

    #[test]
    fn health_report_includes_healthy_and_unhealthy_extensions() {
        let mut runtime = Runtime::new(RuntimeConfig::default());
        runtime.register(Arc::new(Echo)).unwrap();
        runtime.register(Arc::new(Unhealthy)).unwrap();

        let report = runtime.health_report();
        assert_eq!(runtime.extension_count(), 2);
        assert_eq!(report.len(), 2);
        assert_eq!(report[0].extension_id, "example.echo");
        assert!(report[0].healthy);
        assert_eq!(report[0].error, None);
        assert_eq!(report[1].extension_id, "example.unhealthy");
        assert!(!report[1].healthy);
        assert_eq!(report[1].error.as_deref(), Some("extension execution failed: not ready"));
    }

    #[test]
    fn runtime_errors_have_readable_messages() {
        assert_eq!(
            RuntimeError::PolicyDenied.to_string(),
            "request denied by runtime policy"
        );
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
    fn rejects_pre_cancelled_execution_before_dispatch() {
        let mut runtime = Runtime::new(RuntimeConfig::default());
        runtime.register(Arc::new(Echo)).unwrap();
        let token = CancellationToken::new();
        token.cancel();
        let context = ExecutionContext::new(token, None, usize::MAX);
        let request = ExtensionRequest::new(CapabilityId::new("text.echo").unwrap(), "hello");
        assert_eq!(runtime.execute_with_context(request, &context), Err(RuntimeError::Cancelled));
    }

    #[test]
    fn rejects_expired_deadline_before_dispatch() {
        let mut runtime = Runtime::new(RuntimeConfig::default());
        runtime.register(Arc::new(Echo)).unwrap();
        let context = ExecutionContext::with_timeout(std::time::Duration::ZERO, usize::MAX);
        let request = ExtensionRequest::new(CapabilityId::new("text.echo").unwrap(), "hello");
        assert_eq!(runtime.execute_with_context(request, &context), Err(RuntimeError::DeadlineExceeded));
    }

    #[test]
    fn enforces_host_output_limit() {
        let config = RuntimeConfig { max_input_bytes: 32, max_output_bytes: 2, allowed_capabilities: None };
        let mut runtime = Runtime::new(config);
        runtime.register(Arc::new(Echo)).unwrap();
        let request = ExtensionRequest::new(CapabilityId::new("text.echo").unwrap(), "hey");
        assert_eq!(runtime.execute(request), Err(RuntimeError::OutputTooLarge));
    }

    #[test]
    fn rejects_input_over_configured_limit() {
        let config = RuntimeConfig { max_input_bytes: 3, max_output_bytes: 1_048_576, allowed_capabilities: None };
        let mut runtime = Runtime::new(config);
        runtime.register(Arc::new(Echo)).unwrap();
        let request = ExtensionRequest::new(CapabilityId::new("text.echo").unwrap(), "long");
        assert_eq!(runtime.execute(request), Err(RuntimeError::PolicyDenied));
    }
}
