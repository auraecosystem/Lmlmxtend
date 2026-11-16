//! Capability-aware task planning and execution for LMLMxtend hosts.
//!
//! This layer routes declared capabilities; it does not implement or replace
//! the LMLM neural processor.

use lmlmxtend_api::{CapabilityId, ExecutionContext, ExtensionRequest, ExtensionResponse};
use lmlmxtend_runtime::{ExtensionDescriptor, Runtime, RuntimeError};
use std::fmt;

/// A host-authored task intent. Capability and API requirements are explicit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskIntent {
    pub capability: CapabilityId,
    pub input: String,
    pub required_api_version: u16,
    pub preferred_extension: Option<String>,
    pub correlation_id: Option<String>,
}

impl TaskIntent {
    pub fn new(capability: CapabilityId, input: impl Into<String>) -> Self {
        Self {
            capability,
            input: input.into(),
            required_api_version: lmlmxtend_api::API_VERSION,
            preferred_extension: None,
            correlation_id: None,
        }
    }

    pub fn prefer_extension(mut self, extension_id: impl Into<String>) -> Self {
        self.preferred_extension = Some(extension_id.into());
        self
    }

    pub fn with_api_version(mut self, api_version: u16) -> Self {
        self.required_api_version = api_version;
        self
    }

    pub fn with_correlation_id(mut self, correlation_id: impl Into<String>) -> Self {
        self.correlation_id = Some(correlation_id.into());
        self
    }
}

/// A deterministic route selected from the runtime's registered capabilities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskPlan {
    pub extension_id: String,
    pub extension_version: String,
    pub api_version: u16,
    pub capability: CapabilityId,
    pub selection_reason: String,
}

/// Planning failures are explicit; the router never silently substitutes a provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RoutingError {
    NoProvider(String),
    IncompatibleApi { extension_id: String, required: u16, provided: u16 },
    PreferredProviderUnavailable { requested: String, capability: String },
}

impl fmt::Display for RoutingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoProvider(capability) => write!(f, "no extension provides capability '{capability}'"),
            Self::IncompatibleApi { extension_id, required, provided } => write!(
                f, "extension '{extension_id}' provides API {provided}, but API {required} is required"
            ),
            Self::PreferredProviderUnavailable { requested, capability } => write!(
                f, "preferred extension '{requested}' does not provide capability '{capability}'"
            ),
        }
    }
}

impl std::error::Error for RoutingError {}

#[derive(Debug)]
pub enum IntelligenceError {
    Routing(RoutingError),
    Runtime(RuntimeError),
}

impl fmt::Display for IntelligenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Routing(error) => write!(f, "{error}"),
            Self::Runtime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for IntelligenceError {}

/// Discovers compatible capability providers and dispatches approved task plans.
#[derive(Clone, Copy, Debug, Default)]
pub struct IntelligenceRouter;

impl IntelligenceRouter {
    pub fn discover(&self, runtime: &Runtime, capability: &CapabilityId) -> Vec<ExtensionDescriptor> {
        runtime.discover_capability(capability)
    }

    pub fn plan(&self, runtime: &Runtime, intent: &TaskIntent) -> Result<TaskPlan, RoutingError> {
        let providers = self.discover(runtime, &intent.capability);
        if providers.is_empty() {
            return Err(RoutingError::NoProvider(intent.capability.to_string()));
        }

        let selected = if let Some(preferred) = &intent.preferred_extension {
            providers.iter().find(|provider| &provider.extension_id == preferred)
                .ok_or_else(|| RoutingError::PreferredProviderUnavailable {
                    requested: preferred.clone(),
                    capability: intent.capability.to_string(),
                })?
        } else {
            &providers[0]
        };

        if selected.api_version != intent.required_api_version {
            return Err(RoutingError::IncompatibleApi {
                extension_id: selected.extension_id.clone(),
                required: intent.required_api_version,
                provided: selected.api_version,
            });
        }

        let selection_reason = match &intent.preferred_extension {
            Some(id) => format!("explicitly preferred provider '{id}'"),
            None => "first compatible provider in stable extension-ID order".into(),
        };

        Ok(TaskPlan {
            extension_id: selected.extension_id.clone(),
            extension_version: selected.version.clone(),
            api_version: selected.api_version,
            capability: intent.capability.clone(),
            selection_reason,
        })
    }

    /// Plans first, then dispatches through runtime policy and lifecycle controls.
    pub fn execute(&self, runtime: &Runtime, intent: TaskIntent) -> Result<ExtensionResponse, IntelligenceError> {
        let plan = self.plan(runtime, &intent).map_err(IntelligenceError::Routing)?;
        let mut request = ExtensionRequest::new(plan.capability, intent.input);
        request.correlation_id = intent.correlation_id;
        runtime.execute(request).map_err(IntelligenceError::Runtime)
    }

    /// Executes a planned task with caller-supplied cancellation and deadline controls.
    pub fn execute_with_context(
        &self,
        runtime: &Runtime,
        intent: TaskIntent,
        context: &ExecutionContext,
    ) -> Result<ExtensionResponse, IntelligenceError> {
        let plan = self.plan(runtime, &intent).map_err(IntelligenceError::Routing)?;
        let mut request = ExtensionRequest::new(plan.capability, intent.input);
        request.correlation_id = intent.correlation_id;
        runtime.execute_with_context(request, context).map_err(IntelligenceError::Runtime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lmlmxtend_api::{Extension, ExtensionError, ExtensionManifest};
    use lmlmxtend_core::RuntimeConfig;
    use std::sync::Arc;

    struct Echo;

    impl Extension for Echo {
        fn manifest(&self) -> ExtensionManifest {
            ExtensionManifest {
                id: "example.echo".into(),
                version: "1.2.0".into(),
                api_version: lmlmxtend_api::API_VERSION,
                capabilities: vec![CapabilityId::new("text.echo").unwrap()],
            }
        }

        fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse, ExtensionError> {
            Ok(ExtensionResponse::text(request.input))
        }
    }

    fn runtime() -> Runtime {
        let mut runtime = Runtime::new(RuntimeConfig::default());
        runtime.register(Arc::new(Echo)).unwrap();
        runtime
    }

    #[test]
    fn discovers_capability_provider() {
        let runtime = runtime();
        let capability = CapabilityId::new("text.echo").unwrap();
        let found = IntelligenceRouter.discover(&runtime, &capability);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].extension_id, "example.echo");
    }

    #[test]
    fn plans_and_executes_task() {
        let runtime = runtime();
        let intent = TaskIntent::new(CapabilityId::new("text.echo").unwrap(), "hello")
            .with_correlation_id("task-42");
        let plan = IntelligenceRouter.plan(&runtime, &intent).unwrap();
        assert_eq!(plan.extension_id, "example.echo");
        assert_eq!(plan.extension_version, "1.2.0");
        assert_eq!(IntelligenceRouter.execute(&runtime, intent).unwrap().output, "hello");
    }

    #[test]
    fn rejects_missing_provider() {
        let runtime = runtime();
        let intent = TaskIntent::new(CapabilityId::new("vision.describe").unwrap(), "image");
        assert_eq!(
            IntelligenceRouter.plan(&runtime, &intent),
            Err(RoutingError::NoProvider("vision.describe".into()))
        );
    }

    #[test]
    fn rejects_incompatible_api_version() {
        let runtime = runtime();
        let intent = TaskIntent::new(CapabilityId::new("text.echo").unwrap(), "hello")
            .with_api_version(lmlmxtend_api::API_VERSION + 1);
        assert!(matches!(
            IntelligenceRouter.plan(&runtime, &intent),
            Err(RoutingError::IncompatibleApi { .. })
        ));
    }

    #[test]
    fn does_not_ignore_explicit_provider_preference() {
        let runtime = runtime();
        let intent = TaskIntent::new(CapabilityId::new("text.echo").unwrap(), "hello")
            .prefer_extension("example.other");
        assert!(matches!(
            IntelligenceRouter.plan(&runtime, &intent),
            Err(RoutingError::PreferredProviderUnavailable { .. })
        ));
    }
}
