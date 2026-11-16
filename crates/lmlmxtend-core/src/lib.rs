//! Shared configuration and policy primitives.

use lmlmxtend_api::CapabilityId;
use std::collections::BTreeSet;

/// Host configuration. Limits are conservative defaults, not a sandbox.
#[derive(Clone, Debug)]
pub struct RuntimeConfig {
    pub max_input_bytes: usize,
    pub allowed_capabilities: Option<BTreeSet<CapabilityId>>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self { max_input_bytes: 1_048_576, allowed_capabilities: None }
    }
}

/// Decision returned by the host policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PolicyDecision {
    Allow,
    DenyCapability,
    DenyInputTooLarge,
}

#[derive(Clone, Debug)]
pub struct ExecutionPolicy {
    max_input_bytes: usize,
    allowed_capabilities: Option<BTreeSet<CapabilityId>>,
}

impl ExecutionPolicy {
    pub fn from_config(config: &RuntimeConfig) -> Self {
        Self {
            max_input_bytes: config.max_input_bytes,
            allowed_capabilities: config.allowed_capabilities.clone(),
        }
    }

    pub fn evaluate(&self, capability: &CapabilityId, input: &str) -> PolicyDecision {
        if input.len() > self.max_input_bytes {
            return PolicyDecision::DenyInputTooLarge;
        }
        if self.allowed_capabilities.as_ref().is_some_and(|allowed| !allowed.contains(capability)) {
            return PolicyDecision::DenyCapability;
        }
        PolicyDecision::Allow
    }
}
