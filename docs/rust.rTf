2026-11-16T23:14:31.5561775Z ##[group]Run cargo fmt --all -- --check 
2026-11-16T23:14:31.5562083Z [36;1mcargo fmt --all -- --check[0m
2026-11-16T23:14:31.5600824Z shell: /usr/bin/bash -e {0}
2026-11-16T23:14:31.5601049Z env:
2026-11-16T23:14:31.5601222Z   CARGO_HOME: /home/runner/.cargo
2026-11-16T23:14:31.5601441Z   CARGO_INCREMENTAL: 0
2026-11-16T23:14:31.5601637Z   CARGO_TERM_COLOR: always
2026-11-16T23:14:31.5601826Z ##[endgroup]
2026-11-16T23:14:31.6033024Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:32:
2026-11-16T23:14:31.6033689Z  
2026-11-16T23:14:31.6033946Z  impl AgentTask {
2026-11-16T23:14:31.6034569Z      pub fn new(task_id: impl Into<String>, input: impl Into<String>) -> Self {
2026-11-16T23:14:31.6035277Z -        Self { task_id: task_id.into(), input: input.into(), correlation_id: None }
2026-11-16T23:14:31.6035789Z +        Self {
2026-11-16T23:14:31.6036093Z +            task_id: task_id.into(),
2026-11-16T23:14:31.6036462Z +            input: input.into(),
2026-11-16T23:14:31.6037018Z +            correlation_id: None,
2026-11-16T23:14:31.6037655Z +        }
2026-11-16T23:14:31.6037917Z      }
2026-11-16T23:14:31.6038167Z  
2026-11-16T23:14:31.6038545Z      pub fn with_correlation_id(mut self, id: impl Into<String>) -> Self {
2026-11-16T23:14:31.6039206Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:50:
2026-11-16T23:14:31.6039752Z  
2026-11-16T23:14:31.6040010Z  impl AgentResponse {
2026-11-16T23:14:31.6040362Z      pub fn text(output: impl Into<String>) -> Self {
2026-11-16T23:14:31.6040924Z -        Self { output: output.into(), media_type: "text/plain; charset=utf-8".into() }
2026-11-16T23:14:31.6041439Z +        Self {
2026-11-16T23:14:31.6041732Z +            output: output.into(),
2026-11-16T23:14:31.6042145Z +            media_type: "text/plain; charset=utf-8".into(),
2026-11-16T23:14:31.6042541Z +        }
2026-11-16T23:14:31.6042787Z      }
2026-11-16T23:14:31.6043027Z  }
2026-11-16T23:14:31.6043266Z  
2026-11-16T23:14:31.6043690Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:98:
2026-11-16T23:14:31.6048171Z              Self::AgentNotFound(id) => write!(f, "agent not found: {id}"),
2026-11-16T23:14:31.6048661Z              Self::InputTooLarge => f.write_str("agent task input exceeds configured limit"),
2026-11-16T23:14:31.6049126Z              Self::OutputTooLarge => f.write_str("agent output exceeds configured limit"),
2026-11-16T23:14:31.6049675Z -            Self::CapabilityDenied(capability) => write!(f, "agent is not permitted to use capability '{capability}'"),
2026-11-16T23:14:31.6050263Z -            Self::CapabilityExecutionFailed(reason) => write!(f, "capability execution failed: {reason}"),
2026-11-16T23:14:31.6050687Z +            Self::CapabilityDenied(capability) => {
2026-11-16T23:14:31.6051037Z +                write!(f, "agent is not permitted to use capability '{capability}'")
2026-11-16T23:14:31.6051348Z +            }
2026-11-16T23:14:31.6051583Z +            Self::CapabilityExecutionFailed(reason) => {
2026-11-16T23:14:31.6051911Z +                write!(f, "capability execution failed: {reason}")
2026-11-16T23:14:31.6052185Z +            }
2026-11-16T23:14:31.6052489Z              Self::DelegationCycle(id) => write!(f, "agent delegation cycle detected at '{id}'"),
2026-11-16T23:14:31.6052976Z              Self::DelegationDepthExceeded => f.write_str("maximum agent delegation depth exceeded"),
2026-11-16T23:14:31.6053424Z              Self::Cancelled => f.write_str("agent execution cancelled"),
2026-11-16T23:14:31.6053831Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:114:
2026-11-16T23:14:31.6054245Z  /// Implement this contract to register an agent with the local host.
2026-11-16T23:14:31.6054817Z  pub trait Agent: Send + Sync {
2026-11-16T23:14:31.6055065Z      fn manifest(&self) -> AgentManifest;
2026-11-16T23:14:31.6055454Z -    fn execute(&self, task: AgentTask, context: &AgentContext<'_>) -> Result<AgentResponse, AgentError>;
2026-11-16T23:14:31.6055838Z +    fn execute(
2026-11-16T23:14:31.6056249Z +        &self,
2026-11-16T23:14:31.6056441Z +        task: AgentTask,
2026-11-16T23:14:31.6056655Z +        context: &AgentContext<'_>,
2026-11-16T23:14:31.6056902Z +    ) -> Result<AgentResponse, AgentError>;
2026-11-16T23:14:31.6057133Z  }
2026-11-16T23:14:31.6057295Z  
2026-11-16T23:14:31.6057461Z  #[derive(Clone)]
2026-11-16T23:14:31.6057764Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:145:
2026-11-16T23:14:31.6058091Z  }
2026-11-16T23:14:31.6058248Z  
2026-11-16T23:14:31.6058428Z  impl<'a> AgentContext<'a> {
2026-11-16T23:14:31.6058684Z -    pub fn agent_id(&self) -> &str { &self.agent_id }
2026-11-16T23:14:31.6058953Z +    pub fn agent_id(&self) -> &str {
2026-11-16T23:14:31.6059263Z +        &self.agent_id
2026-11-16T23:14:31.6059549Z +    }
2026-11-16T23:14:31.6059775Z  
2026-11-16T23:14:31.6060188Z -    pub fn execution_context(&self) -> &ExecutionContext { self.execution }
2026-11-16T23:14:31.6060795Z +    pub fn execution_context(&self) -> &ExecutionContext {
2026-11-16T23:14:31.6061380Z +        self.execution
2026-11-16T23:14:31.6061695Z +    }
2026-11-16T23:14:31.6061942Z  
2026-11-16T23:14:31.6062378Z      /// Execute an extension capability only if this agent declared it in its manifest.
2026-11-16T23:14:31.6062960Z      pub fn execute_capability(
2026-11-16T23:14:31.6063473Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:158:
2026-11-16T23:14:31.6064091Z          if !self.allowed_capabilities.contains(&capability) {
2026-11-16T23:14:31.6065571Z              return Err(AgentError::CapabilityDenied(capability.to_string()));
2026-11-16T23:14:31.6065927Z          }
2026-11-16T23:14:31.6066192Z -        self.execution.check_cancelled().map_err(map_lifecycle_error)?;
2026-11-16T23:14:31.6066508Z +        self.execution
2026-11-16T23:14:31.6066721Z +            .check_cancelled()
2026-11-16T23:14:31.6066956Z +            .map_err(map_lifecycle_error)?;
2026-11-16T23:14:31.6067253Z          let mut intent = TaskIntent::new(capability, input);
2026-11-16T23:14:31.6067569Z          if let Some(id) = &self.correlation_id {
2026-11-16T23:14:31.6067858Z              intent = intent.with_correlation_id(id.clone());
2026-11-16T23:14:31.6068237Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:165:
2026-11-16T23:14:31.6068574Z          }
2026-11-16T23:14:31.6068865Z -        IntelligenceRouter.execute_with_context(self.runtime, intent, self.execution)
2026-11-16T23:14:31.6069219Z +        IntelligenceRouter
2026-11-16T23:14:31.6069489Z +            .execute_with_context(self.runtime, intent, self.execution)
2026-11-16T23:14:31.6069883Z              .map_err(|error| AgentError::CapabilityExecutionFailed(error.to_string()))
2026-11-16T23:14:31.6070204Z      }
2026-11-16T23:14:31.6070370Z  
2026-11-16T23:14:31.6070654Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:199:
2026-11-16T23:14:31.6070984Z  }
2026-11-16T23:14:31.6071141Z  
2026-11-16T23:14:31.6071325Z  impl Default for AgentRuntime {
2026-11-16T23:14:31.6071626Z -    fn default() -> Self { Self::new(AgentRuntimeConfig::default()) }
2026-11-16T23:14:31.6071929Z +    fn default() -> Self {
2026-11-16T23:14:31.6072169Z +        Self::new(AgentRuntimeConfig::default())
2026-11-16T23:14:31.6072425Z +    }
2026-11-16T23:14:31.6072581Z  }
2026-11-16T23:14:31.6072736Z  
2026-11-16T23:14:31.6072897Z  impl AgentRuntime {
2026-11-16T23:14:31.6073196Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:206:
2026-11-16T23:14:31.6073575Z      pub fn new(config: AgentRuntimeConfig) -> Self {
2026-11-16T23:14:31.6073869Z -        Self { config, agents: BTreeMap::new() }
2026-11-16T23:14:31.6074135Z +        Self {
2026-11-16T23:14:31.6074378Z +            config,
2026-11-16T23:14:31.6074737Z +            agents: BTreeMap::new(),
2026-11-16T23:14:31.6074960Z +        }
2026-11-16T23:14:31.6075130Z      }
2026-11-16T23:14:31.6075394Z  
2026-11-16T23:14:31.6075813Z      /// Validate and register an agent. The manifest is snapshotted at registration.
2026-11-16T23:14:31.6076702Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:212:
2026-11-16T23:14:31.6077263Z          let manifest = agent.manifest();
2026-11-16T23:14:31.6077762Z          if !valid_agent_id(&manifest.id) || manifest.version.trim().is_empty() {
2026-11-16T23:14:31.6078306Z              return Err(AgentError::InvalidManifest(
2026-11-16T23:14:31.6078861Z -                "id must be a lowercase namespaced identifier and version is required".into()
2026-11-16T23:14:31.6079552Z +                "id must be a lowercase namespaced identifier and version is required".into(),
2026-11-16T23:14:31.6079892Z              ));
2026-11-16T23:14:31.6080079Z          }
2026-11-16T23:14:31.6080309Z          if manifest.api_version != AGENT_API_VERSION {
2026-11-16T23:14:31.6080673Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:219:
2026-11-16T23:14:31.6081130Z -            return Err(AgentError::InvalidManifest("incompatible agent API version".into()));
2026-11-16T23:14:31.6081640Z +            return Err(AgentError::InvalidManifest(
2026-11-16T23:14:31.6081915Z +                "incompatible agent API version".into(),
2026-11-16T23:14:31.6082164Z +            ));
2026-11-16T23:14:31.6082337Z          }
2026-11-16T23:14:31.6082534Z          if self.agents.contains_key(&manifest.id) {
2026-11-16T23:14:31.6082840Z              return Err(AgentError::DuplicateAgent(manifest.id));
2026-11-16T23:14:31.6083233Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:229:
2026-11-16T23:14:31.6083565Z                  )));
2026-11-16T23:14:31.6083754Z              }
2026-11-16T23:14:31.6083926Z          }
2026-11-16T23:14:31.6084200Z -        self.agents.insert(manifest.id.clone(), AgentEntry { manifest, agent });
2026-11-16T23:14:31.6085089Z +        self.agents
2026-11-16T23:14:31.6085484Z +            .insert(manifest.id.clone(), AgentEntry { manifest, agent });
2026-11-16T23:14:31.6085884Z          Ok(())
2026-11-16T23:14:31.6086066Z      }
2026-11-16T23:14:31.6086231Z  
2026-11-16T23:14:31.6086514Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:236:
2026-11-16T23:14:31.6086902Z -    pub fn agent_count(&self) -> usize { self.agents.len() }
2026-11-16T23:14:31.6087193Z +    pub fn agent_count(&self) -> usize {
2026-11-16T23:14:31.6087432Z +        self.agents.len()
2026-11-16T23:14:31.6087631Z +    }
2026-11-16T23:14:31.6087787Z  
2026-11-16T23:14:31.6087988Z      pub fn agents(&self) -> Vec<AgentDescriptor> {
2026-11-16T23:14:31.6088294Z -        self.agents.values().map(|entry| AgentDescriptor {
2026-11-16T23:14:31.6088588Z -            id: entry.manifest.id.clone(),
2026-11-16T23:14:31.6088856Z -            version: entry.manifest.version.clone(),
2026-11-16T23:14:31.6089144Z -            api_version: entry.manifest.api_version,
2026-11-16T23:14:31.6089448Z -            capabilities: entry.manifest.capabilities.clone(),
2026-11-16T23:14:31.6089733Z -        }).collect()
2026-11-16T23:14:31.6089937Z +        self.agents
2026-11-16T23:14:31.6090124Z +            .values()
2026-11-16T23:14:31.6090330Z +            .map(|entry| AgentDescriptor {
2026-11-16T23:14:31.6090586Z +                id: entry.manifest.id.clone(),
2026-11-16T23:14:31.6090880Z +                version: entry.manifest.version.clone(),
2026-11-16T23:14:31.6091165Z +                api_version: entry.manifest.api_version,
2026-11-16T23:14:31.6091474Z +                capabilities: entry.manifest.capabilities.clone(),
2026-11-16T23:14:31.6091751Z +            })
2026-11-16T23:14:31.6091927Z +            .collect()
2026-11-16T23:14:31.6092113Z      }
2026-11-16T23:14:31.6092272Z  
2026-11-16T23:14:31.6092441Z      pub fn execute(
2026-11-16T23:14:31.6092745Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:272:
2026-11-16T23:14:31.6093157Z          if delegation_stack.len() > self.config.max_delegation_depth {
2026-11-16T23:14:31.6093509Z              return Err(AgentError::DelegationDepthExceeded);
2026-11-16T23:14:31.6093938Z          }
2026-11-16T23:14:31.6094136Z -        let entry = self.agents.get(agent_id)
2026-11-16T23:14:31.6094385Z +        let entry = self
2026-11-16T23:14:31.6094759Z +            .agents
2026-11-16T23:14:31.6094945Z +            .get(agent_id)
2026-11-16T23:14:31.6095242Z              .ok_or_else(|| AgentError::AgentNotFound(agent_id.to_owned()))?;
2026-11-16T23:14:31.6095573Z          let agent = Arc::clone(&entry.agent);
2026-11-16T23:14:31.6095979Z          let allowed: BTreeSet<CapabilityId> = entry.manifest.capabilities.iter().cloned().collect();
2026-11-16T23:14:31.6096487Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:306:
2026-11-16T23:14:31.6096835Z          && id.len() <= 128
2026-11-16T23:14:31.6097053Z          && id.split('.').all(|part| {
2026-11-16T23:14:31.6097288Z              !part.is_empty()
2026-11-16T23:14:31.6097520Z -                && part.chars().all(|c| {
2026-11-16T23:14:31.6097850Z -                    c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_'
2026-11-16T23:14:31.6098277Z -                })
2026-11-16T23:14:31.6098469Z +                && part
2026-11-16T23:14:31.6098671Z +                    .chars()
2026-11-16T23:14:31.6098975Z +                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
2026-11-16T23:14:31.6099295Z          })
2026-11-16T23:14:31.6099471Z  }
2026-11-16T23:14:31.6099631Z  
2026-11-16T23:14:31.6099916Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:323:
2026-11-16T23:14:31.6100255Z  #[cfg(test)]
2026-11-16T23:14:31.6100545Z  mod tests {
2026-11-16T23:14:31.6100829Z      use super::*;
2026-11-16T23:14:31.6101430Z -    use lmlmxtend_api::{CancellationToken, Extension, ExtensionError, ExtensionManifest, ExtensionRequest};
2026-11-16T23:14:31.6102090Z +    use lmlmxtend_api::{
2026-11-16T23:14:31.6102430Z +        CancellationToken, Extension, ExtensionError, ExtensionManifest, ExtensionRequest,
2026-11-16T23:14:31.6102802Z +    };
2026-11-16T23:14:31.6102988Z      use lmlmxtend_core::RuntimeConfig;
2026-11-16T23:14:31.6103229Z      use std::sync::Arc;
2026-11-16T23:14:31.6103421Z  
2026-11-16T23:14:31.6103702Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:330:
2026-11-16T23:14:31.6104061Z -    struct EchoAgent { id: &'static str }
2026-11-16T23:14:31.6104302Z +    struct EchoAgent {
2026-11-16T23:14:31.6104681Z +        id: &'static str,
2026-11-16T23:14:31.6104993Z +    }
2026-11-16T23:14:31.6105252Z  
2026-11-16T23:14:31.6105510Z      impl Agent for EchoAgent {
2026-11-16T23:14:31.6105869Z          fn manifest(&self) -> AgentManifest {
2026-11-16T23:14:31.6106337Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:339:
2026-11-16T23:14:31.6106677Z              }
2026-11-16T23:14:31.6106850Z          }
2026-11-16T23:14:31.6107026Z  
2026-11-16T23:14:31.6107357Z -        fn execute(&self, task: AgentTask, _context: &AgentContext<'_>) -> Result<AgentResponse, AgentError> {
2026-11-16T23:14:31.6107759Z +        fn execute(
2026-11-16T23:14:31.6107945Z +            &self,
2026-11-16T23:14:31.6108138Z +            task: AgentTask,
2026-11-16T23:14:31.6108369Z +            _context: &AgentContext<'_>,
2026-11-16T23:14:31.6108634Z +        ) -> Result<AgentResponse, AgentError> {
2026-11-16T23:14:31.6108916Z              Ok(AgentResponse::text(task.input))
2026-11-16T23:14:31.6109156Z          }
2026-11-16T23:14:31.6109326Z      }
2026-11-16T23:14:31.6109609Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:356:
2026-11-16T23:14:31.6109940Z              }
2026-11-16T23:14:31.6110112Z          }
2026-11-16T23:14:31.6110275Z  
2026-11-16T23:14:31.6110595Z -        fn execute(&self, task: AgentTask, context: &AgentContext<'_>) -> Result<AgentResponse, AgentError> {
2026-11-16T23:14:31.6110977Z +        fn execute(
2026-11-16T23:14:31.6111167Z +            &self,
2026-11-16T23:14:31.6111353Z +            task: AgentTask,
2026-11-16T23:14:31.6111769Z +            context: &AgentContext<'_>,
2026-11-16T23:14:31.6112043Z +        ) -> Result<AgentResponse, AgentError> {
2026-11-16T23:14:31.6112412Z              context.delegate("agent.echo", AgentTask::new(task.task_id, task.input))
2026-11-16T23:14:31.6112734Z          }
2026-11-16T23:14:31.6112901Z      }
2026-11-16T23:14:31.6113182Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:373:
2026-11-16T23:14:31.6113517Z              }
2026-11-16T23:14:31.6113694Z          }
2026-11-16T23:14:31.6113852Z  
2026-11-16T23:14:31.6114179Z -        fn execute(&self, task: AgentTask, _context: &AgentContext<'_>) -> Result<AgentResponse, AgentError> {
2026-11-16T23:14:31.6114754Z +        fn execute(
2026-11-16T23:14:31.6114939Z +            &self,
2026-11-16T23:14:31.6115127Z +            task: AgentTask,
2026-11-16T23:14:31.6115351Z +            _context: &AgentContext<'_>,
2026-11-16T23:14:31.6115603Z +        ) -> Result<AgentResponse, AgentError> {
2026-11-16T23:14:31.6115939Z              Ok(AgentResponse::text(task.correlation_id.unwrap_or_default()))
2026-11-16T23:14:31.6116369Z          }
2026-11-16T23:14:31.6116531Z      }
2026-11-16T23:14:31.6116812Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:390:
2026-11-16T23:14:31.6117139Z              }
2026-11-16T23:14:31.6117312Z          }
2026-11-16T23:14:31.6117478Z  
2026-11-16T23:14:31.6117797Z -        fn execute(&self, task: AgentTask, context: &AgentContext<'_>) -> Result<AgentResponse, AgentError> {
2026-11-16T23:14:31.6118307Z -            context.delegate("agent.correlation", AgentTask::new(task.task_id, task.input))
2026-11-16T23:14:31.6118650Z +        fn execute(
2026-11-16T23:14:31.6118836Z +            &self,
2026-11-16T23:14:31.6119031Z +            task: AgentTask,
2026-11-16T23:14:31.6119310Z +            context: &AgentContext<'_>,
2026-11-16T23:14:31.6119718Z +        ) -> Result<AgentResponse, AgentError> {
2026-11-16T23:14:31.6120119Z +            context.delegate(
2026-11-16T23:14:31.6120440Z +                "agent.correlation",
2026-11-16T23:14:31.6120699Z +                AgentTask::new(task.task_id, task.input),
2026-11-16T23:14:31.6120948Z +            )
2026-11-16T23:14:31.6121126Z          }
2026-11-16T23:14:31.6121286Z      }
2026-11-16T23:14:31.6121446Z  
2026-11-16T23:14:31.6121719Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:407:
2026-11-16T23:14:31.6122049Z              }
2026-11-16T23:14:31.6122220Z          }
2026-11-16T23:14:31.6122382Z  
2026-11-16T23:14:31.6122703Z -        fn execute(&self, task: AgentTask, context: &AgentContext<'_>) -> Result<AgentResponse, AgentError> {
2026-11-16T23:14:31.6123076Z +        fn execute(
2026-11-16T23:14:31.6123265Z +            &self,
2026-11-16T23:14:31.6123453Z +            task: AgentTask,
2026-11-16T23:14:31.6123680Z +            context: &AgentContext<'_>,
2026-11-16T23:14:31.6123939Z +        ) -> Result<AgentResponse, AgentError> {
2026-11-16T23:14:31.6124213Z              context.delegate("agent.loop", task)
2026-11-16T23:14:31.6124650Z          }
2026-11-16T23:14:31.6124829Z      }
2026-11-16T23:14:31.6125120Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:441:
2026-11-16T23:14:31.6125458Z              }
2026-11-16T23:14:31.6125639Z          }
2026-11-16T23:14:31.6125799Z  
2026-11-16T23:14:31.6126119Z -        fn execute(&self, task: AgentTask, context: &AgentContext<'_>) -> Result<AgentResponse, AgentError> {
2026-11-16T23:14:31.6126537Z -            let result = context.execute_capability(
2026-11-16T23:14:31.6126844Z -                CapabilityId::new("text.transform").unwrap(),
2026-11-16T23:14:31.6127118Z -                task.input,
2026-11-16T23:14:31.6127329Z -            )?;
2026-11-16T23:14:31.6127520Z +        fn execute(
2026-11-16T23:14:31.6127707Z +            &self,
2026-11-16T23:14:31.6127895Z +            task: AgentTask,
2026-11-16T23:14:31.6128118Z +            context: &AgentContext<'_>,
2026-11-16T23:14:31.6128513Z +        ) -> Result<AgentResponse, AgentError> {
2026-11-16T23:14:31.6128774Z +            let result = context
2026-11-16T23:14:31.6129125Z +                .execute_capability(CapabilityId::new("text.transform").unwrap(), task.input)?;
2026-11-16T23:14:31.6129515Z              Ok(AgentResponse::text(result.output))
2026-11-16T23:14:31.6129763Z          }
2026-11-16T23:14:31.6129932Z      }
2026-11-16T23:14:31.6130212Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:461:
2026-11-16T23:14:31.6130548Z      #[test]
2026-11-16T23:14:31.6130733Z      fn registers_and_runs_agent() {
2026-11-16T23:14:31.6130981Z          let mut agents = AgentRuntime::default();
2026-11-16T23:14:31.6131320Z -        agents.register(Arc::new(EchoAgent { id: "agent.echo" })).unwrap();
2026-11-16T23:14:31.6131824Z -        let result = agents.execute(&empty_runtime(), "agent.echo", AgentTask::new("t1", "hello"), &execution()).unwrap();
2026-11-16T23:14:31.6132238Z +        agents
2026-11-16T23:14:31.6132477Z +            .register(Arc::new(EchoAgent { id: "agent.echo" }))
2026-11-16T23:14:31.6132880Z +            .unwrap();
2026-11-16T23:14:31.6133085Z +        let result = agents
2026-11-16T23:14:31.6133291Z +            .execute(
2026-11-16T23:14:31.6133487Z +                &empty_runtime(),
2026-11-16T23:14:31.6133710Z +                "agent.echo",
2026-11-16T23:14:31.6133943Z +                AgentTask::new("t1", "hello"),
2026-11-16T23:14:31.6134188Z +                &execution(),
2026-11-16T23:14:31.6134384Z +            )
2026-11-16T23:14:31.6134760Z +            .unwrap();
2026-11-16T23:14:31.6134983Z          assert_eq!(result.output, "hello");
2026-11-16T23:14:31.6135238Z          assert_eq!(agents.agent_count(), 1);
2026-11-16T23:14:31.6135465Z      }
2026-11-16T23:14:31.6135748Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:470:
2026-11-16T23:14:31.6136079Z      #[test]
2026-11-16T23:14:31.6136269Z      fn delegates_to_registered_agent() {
2026-11-16T23:14:31.6136558Z          let mut agents = AgentRuntime::default();
2026-11-16T23:14:31.6136886Z -        agents.register(Arc::new(EchoAgent { id: "agent.echo" })).unwrap();
2026-11-16T23:14:31.6137187Z +        agents
2026-11-16T23:14:31.6137418Z +            .register(Arc::new(EchoAgent { id: "agent.echo" }))
2026-11-16T23:14:31.6137700Z +            .unwrap();
2026-11-16T23:14:31.6137951Z          agents.register(Arc::new(DelegatingAgent)).unwrap();
2026-11-16T23:14:31.6138433Z -        let result = agents.execute(&empty_runtime(), "agent.parent", AgentTask::new("t2", "delegated"), &execution()).unwrap();
2026-11-16T23:14:31.6138881Z +        let result = agents
2026-11-16T23:14:31.6139092Z +            .execute(
2026-11-16T23:14:31.6139294Z +                &empty_runtime(),
2026-11-16T23:14:31.6139518Z +                "agent.parent",
2026-11-16T23:14:31.6139757Z +                AgentTask::new("t2", "delegated"),
2026-11-16T23:14:31.6140006Z +                &execution(),
2026-11-16T23:14:31.6140222Z +            )
2026-11-16T23:14:31.6140407Z +            .unwrap();
2026-11-16T23:14:31.6140644Z          assert_eq!(result.output, "delegated");
2026-11-16T23:14:31.6140882Z      }
2026-11-16T23:14:31.6141050Z  
2026-11-16T23:14:31.6141332Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:482:
2026-11-16T23:14:31.6141726Z          agents.register(Arc::new(CorrelationAgent)).unwrap();
2026-11-16T23:14:31.6142066Z          agents.register(Arc::new(CorrelationParent)).unwrap();
2026-11-16T23:14:31.6142458Z          let task = AgentTask::new("t-correlation", "payload").with_correlation_id("corr-42");
2026-11-16T23:14:31.6142972Z -        let result = agents.execute(&empty_runtime(), "agent.correlation-parent", task, &execution()).unwrap();
2026-11-16T23:14:31.6143376Z +        let result = agents
2026-11-16T23:14:31.6143578Z +            .execute(
2026-11-16T23:14:31.6143778Z +                &empty_runtime(),
2026-11-16T23:14:31.6144019Z +                "agent.correlation-parent",
2026-11-16T23:14:31.6144368Z +                task,
2026-11-16T23:14:31.6144644Z +                &execution(),
2026-11-16T23:14:31.6144847Z +            )
2026-11-16T23:14:31.6145024Z +            .unwrap();
2026-11-16T23:14:31.6145236Z          assert_eq!(result.output, "corr-42");
2026-11-16T23:14:31.6145468Z      }
2026-11-16T23:14:31.6145642Z  
2026-11-16T23:14:31.6145913Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:490:
2026-11-16T23:14:31.6146266Z      fn rejects_invalid_agent_identity() {
2026-11-16T23:14:31.6146532Z          let mut agents = AgentRuntime::default();
2026-11-16T23:14:31.6146778Z          assert_eq!(
2026-11-16T23:14:31.6147037Z -            agents.register(Arc::new(EchoAgent { id: "Invalid Agent" })),
2026-11-16T23:14:31.6147365Z +            agents.register(Arc::new(EchoAgent {
2026-11-16T23:14:31.6147614Z +                id: "Invalid Agent"
2026-11-16T23:14:31.6147831Z +            })),
2026-11-16T23:14:31.6148052Z              Err(AgentError::InvalidManifest(
2026-11-16T23:14:31.6148523Z                  "id must be a lowercase namespaced identifier and version is required".into()
2026-11-16T23:14:31.6148856Z              ))
2026-11-16T23:14:31.6149139Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:502:
2026-11-16T23:14:31.6149510Z          let mut agents = AgentRuntime::default();
2026-11-16T23:14:31.6149791Z          agents.register(Arc::new(LoopAgent)).unwrap();
2026-11-16T23:14:31.6150044Z          assert_eq!(
2026-11-16T23:14:31.6150376Z -            agents.execute(&empty_runtime(), "agent.loop", AgentTask::new("t3", "loop"), &execution()),
2026-11-16T23:14:31.6150745Z +            agents.execute(
2026-11-16T23:14:31.6150968Z +                &empty_runtime(),
2026-11-16T23:14:31.6151200Z +                "agent.loop",
2026-11-16T23:14:31.6151428Z +                AgentTask::new("t3", "loop"),
2026-11-16T23:14:31.6151669Z +                &execution()
2026-11-16T23:14:31.6151871Z +            ),
2026-11-16T23:14:31.6152115Z              Err(AgentError::DelegationCycle("agent.loop".into()))
2026-11-16T23:14:31.6152393Z          );
2026-11-16T23:14:31.6152557Z      }
2026-11-16T23:14:31.6152833Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:514:
2026-11-16T23:14:31.6153183Z              max_task_output_bytes: 1024,
2026-11-16T23:14:31.6153429Z              max_delegation_depth: 0,
2026-11-16T23:14:31.6153650Z          });
2026-11-16T23:14:31.6153903Z -        agents.register(Arc::new(EchoAgent { id: "agent.echo" })).unwrap();
2026-11-16T23:14:31.6154206Z +        agents
2026-11-16T23:14:31.6154538Z +            .register(Arc::new(EchoAgent { id: "agent.echo" }))
2026-11-16T23:14:31.6154814Z +            .unwrap();
2026-11-16T23:14:31.6155065Z          agents.register(Arc::new(DelegatingAgent)).unwrap();
2026-11-16T23:14:31.6155336Z          assert_eq!(
2026-11-16T23:14:31.6155687Z -            agents.execute(&empty_runtime(), "agent.parent", AgentTask::new("t-depth", "hello"), &execution()),
2026-11-16T23:14:31.6156088Z +            agents.execute(
2026-11-16T23:14:31.6156308Z +                &empty_runtime(),
2026-11-16T23:14:31.6156542Z +                "agent.parent",
2026-11-16T23:14:31.6157165Z +                AgentTask::new("t-depth", "hello"),
2026-11-16T23:14:31.6157655Z +                &execution()
2026-11-16T23:14:31.6158156Z +            ),
2026-11-16T23:14:31.6158715Z              Err(AgentError::DelegationDepthExceeded)
2026-11-16T23:14:31.6159162Z          );
2026-11-16T23:14:31.6170198Z      }
2026-11-16T23:14:31.6170732Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:528:
2026-11-16T23:14:31.6171401Z          agents.register(Arc::new(CapabilityAgent)).unwrap();
2026-11-16T23:14:31.6171810Z          let mut runtime = empty_runtime();
2026-11-16T23:14:31.6172230Z          runtime.register(Arc::new(TextExtension)).unwrap();
2026-11-16T23:14:31.6172763Z -        let result = agents.execute(&runtime, "agent.writer", AgentTask::new("t4", "hello"), &execution()).unwrap();
2026-11-16T23:14:31.6173409Z +        let result = agents
2026-11-16T23:14:31.6173633Z +            .execute(
2026-11-16T23:14:31.6173840Z +                &runtime,
2026-11-16T23:14:31.6174049Z +                "agent.writer",
2026-11-16T23:14:31.6174298Z +                AgentTask::new("t4", "hello"),
2026-11-16T23:14:31.6174747Z +                &execution(),
2026-11-16T23:14:31.6174959Z +            )
2026-11-16T23:14:31.6175144Z +            .unwrap();
2026-11-16T23:14:31.6175370Z          assert_eq!(result.output, "HELLO");
2026-11-16T23:14:31.6175610Z      }
2026-11-16T23:14:31.6175786Z  
2026-11-16T23:14:31.6176095Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:537:
2026-11-16T23:14:31.6176457Z          struct Unprivileged;
2026-11-16T23:14:31.6176686Z          impl Agent for Unprivileged {
2026-11-16T23:14:31.6176945Z              fn manifest(&self) -> AgentManifest {
2026-11-16T23:14:31.6177422Z -                AgentManifest { id: "agent.unprivileged".into(), version: "1.0.0".into(), api_version: AGENT_API_VERSION, capabilities: vec![] }
2026-11-16T23:14:31.6178017Z +                AgentManifest {
2026-11-16T23:14:31.6178269Z +                    id: "agent.unprivileged".into(),
2026-11-16T23:14:31.6178533Z +                    version: "1.0.0".into(),
2026-11-16T23:14:31.6178798Z +                    api_version: AGENT_API_VERSION,
2026-11-16T23:14:31.6179060Z +                    capabilities: vec![],
2026-11-16T23:14:31.6179291Z +                }
2026-11-16T23:14:31.6179473Z              }
2026-11-16T23:14:31.6179824Z -            fn execute(&self, _task: AgentTask, context: &AgentContext<'_>) -> Result<AgentResponse, AgentError> {
2026-11-16T23:14:31.6180371Z -                context.execute_capability(CapabilityId::new("text.transform").unwrap(), "no")
2026-11-16T23:14:31.6180728Z +            fn execute(
2026-11-16T23:14:31.6180932Z +                &self,
2026-11-16T23:14:31.6181136Z +                _task: AgentTask,
2026-11-16T23:14:31.6181382Z +                context: &AgentContext<'_>,
2026-11-16T23:14:31.6181647Z +            ) -> Result<AgentResponse, AgentError> {
2026-11-16T23:14:31.6181900Z +                context
2026-11-16T23:14:31.6182199Z +                    .execute_capability(CapabilityId::new("text.transform").unwrap(), "no")
2026-11-16T23:14:31.6182589Z                      .map(|response| AgentResponse::text(response.output))
2026-11-16T23:14:31.6182882Z              }
2026-11-16T23:14:31.6183063Z          }
2026-11-16T23:14:31.6183349Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:547:
2026-11-16T23:14:31.6183720Z          let mut agents = AgentRuntime::default();
2026-11-16T23:14:31.6184014Z          agents.register(Arc::new(Unprivileged)).unwrap();
2026-11-16T23:14:31.6184285Z          assert_eq!(
2026-11-16T23:14:31.6184800Z -            agents.execute(&empty_runtime(), "agent.unprivileged", AgentTask::new("t5", "x"), &execution()),
2026-11-16T23:14:31.6185197Z +            agents.execute(
2026-11-16T23:14:31.6185429Z +                &empty_runtime(),
2026-11-16T23:14:31.6185674Z +                "agent.unprivileged",
2026-11-16T23:14:31.6185930Z +                AgentTask::new("t5", "x"),
2026-11-16T23:14:31.6186173Z +                &execution()
2026-11-16T23:14:31.6186383Z +            ),
2026-11-16T23:14:31.6186639Z              Err(AgentError::CapabilityDenied("text.transform".into()))
2026-11-16T23:14:31.6186923Z          );
2026-11-16T23:14:31.6187098Z      }
2026-11-16T23:14:31.6187380Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:559:
2026-11-16T23:14:31.6187730Z              max_task_output_bytes: 2,
2026-11-16T23:14:31.6187974Z              max_delegation_depth: 1,
2026-11-16T23:14:31.6188197Z          });
2026-11-16T23:14:31.6188467Z -        agents.register(Arc::new(EchoAgent { id: "agent.echo" })).unwrap();
2026-11-16T23:14:31.6188774Z +        agents
2026-11-16T23:14:31.6189007Z +            .register(Arc::new(EchoAgent { id: "agent.echo" }))
2026-11-16T23:14:31.6189394Z +            .unwrap();
2026-11-16T23:14:31.6189603Z          assert_eq!(
2026-11-16T23:14:31.6189939Z -            agents.execute(&empty_runtime(), "agent.echo", AgentTask::new("t6", "long"), &execution()),
2026-11-16T23:14:31.6190312Z +            agents.execute(
2026-11-16T23:14:31.6190539Z +                &empty_runtime(),
2026-11-16T23:14:31.6190769Z +                "agent.echo",
2026-11-16T23:14:31.6191002Z +                AgentTask::new("t6", "long"),
2026-11-16T23:14:31.6191262Z +                &execution()
2026-11-16T23:14:31.6191466Z +            ),
2026-11-16T23:14:31.6191668Z              Err(AgentError::InputTooLarge)
2026-11-16T23:14:31.6191912Z          );
2026-11-16T23:14:31.6192084Z          assert_eq!(
2026-11-16T23:14:31.6192388Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:568:
2026-11-16T23:14:31.6192863Z -            agents.execute(&empty_runtime(), "agent.echo", AgentTask::new("t7", "abc"), &execution()),
2026-11-16T23:14:31.6193237Z +            agents.execute(
2026-11-16T23:14:31.6193550Z +                &empty_runtime(),
2026-11-16T23:14:31.6193767Z +                "agent.echo",
2026-11-16T23:14:31.6194004Z +                AgentTask::new("t7", "abc"),
2026-11-16T23:14:31.6194243Z +                &execution()
2026-11-16T23:14:31.6194624Z +            ),
2026-11-16T23:14:31.6194934Z              Err(AgentError::OutputTooLarge)
2026-11-16T23:14:31.6195300Z          );
2026-11-16T23:14:31.6195556Z      }
2026-11-16T23:14:31.6196000Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:573:
2026-11-16T23:14:31.6196539Z      #[test]
2026-11-16T23:14:31.6196883Z      fn rejects_cancelled_tasks_before_agent_execution() {
2026-11-16T23:14:31.6197353Z          let mut agents = AgentRuntime::default();
2026-11-16T23:14:31.6197723Z -        agents.register(Arc::new(EchoAgent { id: "agent.echo" })).unwrap();
2026-11-16T23:14:31.6198032Z +        agents
2026-11-16T23:14:31.6198263Z +            .register(Arc::new(EchoAgent { id: "agent.echo" }))
2026-11-16T23:14:31.6198552Z +            .unwrap();
2026-11-16T23:14:31.6198778Z          let token = CancellationToken::new();
2026-11-16T23:14:31.6199029Z          token.cancel();
2026-11-16T23:14:31.6199304Z          let context = ExecutionContext::new(token, None, usize::MAX);
2026-11-16T23:14:31.6199710Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:580:
2026-11-16T23:14:31.6200043Z          assert_eq!(
2026-11-16T23:14:31.6200367Z -            agents.execute(&empty_runtime(), "agent.echo", AgentTask::new("t8", "hello"), &context),
2026-11-16T23:14:31.6200736Z +            agents.execute(
2026-11-16T23:14:31.6200956Z +                &empty_runtime(),
2026-11-16T23:14:31.6201171Z +                "agent.echo",
2026-11-16T23:14:31.6201402Z +                AgentTask::new("t8", "hello"),
2026-11-16T23:14:31.6201640Z +                &context
2026-11-16T23:14:31.6201862Z +            ),
2026-11-16T23:14:31.6202055Z              Err(AgentError::Cancelled)
2026-11-16T23:14:31.6202298Z          );
2026-11-16T23:14:31.6202462Z      }
2026-11-16T23:14:31.6202748Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/lib.rs:585:
2026-11-16T23:14:31.6203073Z  }
2026-11-16T23:14:31.6203264Z -
2026-11-16T23:14:31.6203430Z  
2026-11-16T23:14:31.6203595Z  mod orchestration;
2026-11-16T23:14:31.6203919Z  pub use orchestration::{AgentPlan, PlanError, PlanNode, PlanResult, MAX_PLAN_NODES};
2026-11-16T23:14:31.6204755Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:24:
2026-11-16T23:14:31.6205131Z  }
2026-11-16T23:14:31.6205292Z  
2026-11-16T23:14:31.6205462Z  impl PlanNode {
2026-11-16T23:14:31.6205694Z -    pub fn new(
2026-11-16T23:14:31.6205962Z -        id: impl Into<String>,
2026-11-16T23:14:31.6206305Z -        agent_id: impl Into<String>,
2026-11-16T23:14:31.6206666Z -        task: AgentTask,
2026-11-16T23:14:31.6206998Z -    ) -> Self {
2026-11-16T23:14:31.6207435Z -        Self { id: id.into(), agent_id: agent_id.into(), task, depends_on: Vec::new() }
2026-11-16T23:14:31.6208080Z +    pub fn new(id: impl Into<String>, agent_id: impl Into<String>, task: AgentTask) -> Self {
2026-11-16T23:14:31.6208425Z +        Self {
2026-11-16T23:14:31.6208625Z +            id: id.into(),
2026-11-16T23:14:31.6208841Z +            agent_id: agent_id.into(),
2026-11-16T23:14:31.6209069Z +            task,
2026-11-16T23:14:31.6209266Z +            depends_on: Vec::new(),
2026-11-16T23:14:31.6209484Z +        }
2026-11-16T23:14:31.6209652Z      }
2026-11-16T23:14:31.6209813Z  
2026-11-16T23:14:31.6210305Z      pub fn after(mut self, dependency: impl Into<String>) -> Self {
2026-11-16T23:14:31.6210991Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:45:
2026-11-16T23:14:31.6211520Z  }
2026-11-16T23:14:31.6211758Z  
2026-11-16T23:14:31.6212008Z  impl AgentPlan {
2026-11-16T23:14:31.6212302Z -    pub fn new() -> Self { Self::default() }
2026-11-16T23:14:31.6212555Z +    pub fn new() -> Self {
2026-11-16T23:14:31.6212899Z +        Self::default()
2026-11-16T23:14:31.6213102Z +    }
2026-11-16T23:14:31.6213267Z  
2026-11-16T23:14:31.6213509Z      pub fn add_node(&mut self, node: PlanNode) -> Result<(), PlanError> {
2026-11-16T23:14:31.6213828Z          if self.nodes.len() >= MAX_PLAN_NODES {
2026-11-16T23:14:31.6214210Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:52:
2026-11-16T23:14:31.6214848Z              return Err(PlanError::TooManyNodes);
2026-11-16T23:14:31.6215107Z          }
2026-11-16T23:14:31.6215371Z          if node.id.trim().is_empty() || node.agent_id.trim().is_empty() {
2026-11-16T23:14:31.6215790Z -            return Err(PlanError::InvalidNode("node and agent IDs are required".into()));
2026-11-16T23:14:31.6216162Z +            return Err(PlanError::InvalidNode(
2026-11-16T23:14:31.6216450Z +                "node and agent IDs are required".into(),
2026-11-16T23:14:31.6216702Z +            ));
2026-11-16T23:14:31.6216890Z          }
2026-11-16T23:14:31.6217130Z          if self.nodes.iter().any(|existing| existing.id == node.id) {
2026-11-16T23:14:31.6217471Z              return Err(PlanError::DuplicateNode(node.id));
2026-11-16T23:14:31.6217872Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:61:
2026-11-16T23:14:31.6218245Z          Ok(())
2026-11-16T23:14:31.6218416Z      }
2026-11-16T23:14:31.6218583Z  
2026-11-16T23:14:31.6218795Z -    pub fn nodes(&self) -> &[PlanNode] { &self.nodes }
2026-11-16T23:14:31.6219077Z +    pub fn nodes(&self) -> &[PlanNode] {
2026-11-16T23:14:31.6219318Z +        &self.nodes
2026-11-16T23:14:31.6219506Z +    }
2026-11-16T23:14:31.6219672Z  
2026-11-16T23:14:31.6219934Z      /// Validate references and cycles, returning a stable topological order.
2026-11-16T23:14:31.6220326Z      /// Among simultaneously ready nodes, lexical node ID order is used.
2026-11-16T23:14:31.6220778Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:76:
2026-11-16T23:14:31.6221169Z          let mut by_id = BTreeMap::new();
2026-11-16T23:14:31.6221405Z          for node in &self.nodes {
2026-11-16T23:14:31.6221701Z              if node.id.trim().is_empty() || node.agent_id.trim().is_empty() {
2026-11-16T23:14:31.6222108Z -                return Err(PlanError::InvalidNode("node and agent IDs are required".into()));
2026-11-16T23:14:31.6222489Z +                return Err(PlanError::InvalidNode(
2026-11-16T23:14:31.6222773Z +                    "node and agent IDs are required".into(),
2026-11-16T23:14:31.6223033Z +                ));
2026-11-16T23:14:31.6223217Z              }
2026-11-16T23:14:31.6223444Z              if by_id.insert(node.id.as_str(), node).is_some() {
2026-11-16T23:14:31.6223783Z                  return Err(PlanError::DuplicateNode(node.id.clone()));
2026-11-16T23:14:31.6224196Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:83:
2026-11-16T23:14:31.6224718Z              }
2026-11-16T23:14:31.6225026Z          }
2026-11-16T23:14:31.6225199Z  
2026-11-16T23:14:31.6225398Z -        let mut indegree: BTreeMap<&str, usize> =
2026-11-16T23:14:31.6225687Z -            by_id.keys().map(|id| (*id, 0)).collect();
2026-11-16T23:14:31.6226046Z +        let mut indegree: BTreeMap<&str, usize> = by_id.keys().map(|id| (*id, 0)).collect();
2026-11-16T23:14:31.6226455Z          let mut outgoing: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
2026-11-16T23:14:31.6226746Z  
2026-11-16T23:14:31.6226927Z          for node in &self.nodes {
2026-11-16T23:14:31.6227284Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:105:
2026-11-16T23:14:31.6227683Z                          dependency: dependency.clone(),
2026-11-16T23:14:31.6227938Z                      });
2026-11-16T23:14:31.6228132Z                  }
2026-11-16T23:14:31.6228407Z -                *indegree.get_mut(node.id.as_str()).expect("node was indexed") += 1;
2026-11-16T23:14:31.6228824Z -                outgoing.entry(dependency.as_str()).or_default().push(node.id.as_str());
2026-11-16T23:14:31.6229297Z +                *indegree
2026-11-16T23:14:31.6229525Z +                    .get_mut(node.id.as_str())
2026-11-16T23:14:31.6229788Z +                    .expect("node was indexed") += 1;
2026-11-16T23:14:31.6230037Z +                outgoing
2026-11-16T23:14:31.6230254Z +                    .entry(dependency.as_str())
2026-11-16T23:14:31.6230502Z +                    .or_default()
2026-11-16T23:14:31.6230732Z +                    .push(node.id.as_str());
2026-11-16T23:14:31.6230962Z              }
2026-11-16T23:14:31.6231135Z          }
2026-11-16T23:14:31.6231303Z  
2026-11-16T23:14:31.6231608Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:113:
2026-11-16T23:14:31.6232017Z -        let mut ready: BTreeSet<&str> = indegree.iter()
2026-11-16T23:14:31.6232310Z +        let mut ready: BTreeSet<&str> = indegree
2026-11-16T23:14:31.6232551Z +            .iter()
2026-11-16T23:14:31.6232805Z              .filter_map(|(id, degree)| (*degree == 0).then_some(*id))
2026-11-16T23:14:31.6233095Z              .collect();
2026-11-16T23:14:31.6233352Z          let mut order = Vec::with_capacity(self.nodes.len());
2026-11-16T23:14:31.6233769Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:166:
2026-11-16T23:14:31.6234232Z              Self::InvalidNode(reason) => write!(f, "invalid plan node: {reason}"),
2026-11-16T23:14:31.6234808Z              Self::DuplicateNode(id) => write!(f, "duplicate plan node: {id}"),
2026-11-16T23:14:31.6235229Z              Self::SelfDependency(id) => write!(f, "plan node '{id}' depends on itself"),
2026-11-16T23:14:31.6235617Z -            Self::DuplicateDependency { node_id, dependency } =>
2026-11-16T23:14:31.6235990Z -                write!(f, "plan node '{node_id}' repeats dependency '{dependency}'"),
2026-11-16T23:14:31.6236357Z -            Self::MissingDependency { node_id, dependency } =>
2026-11-16T23:14:31.6236730Z -                write!(f, "plan node '{node_id}' references missing dependency '{dependency}'"),
2026-11-16T23:14:31.6237104Z +            Self::DuplicateDependency {
2026-11-16T23:14:31.6237341Z +                node_id,
2026-11-16T23:14:31.6237553Z +                dependency,
2026-11-16T23:14:31.6237843Z +            } => write!(f, "plan node '{node_id}' repeats dependency '{dependency}'"),
2026-11-16T23:14:31.6238177Z +            Self::MissingDependency {
2026-11-16T23:14:31.6238410Z +                node_id,
2026-11-16T23:14:31.6238604Z +                dependency,
2026-11-16T23:14:31.6238809Z +            } => write!(
2026-11-16T23:14:31.6239000Z +                f,
2026-11-16T23:14:31.6239266Z +                "plan node '{node_id}' references missing dependency '{dependency}'"
2026-11-16T23:14:31.6239565Z +            ),
2026-11-16T23:14:31.6239857Z              Self::CycleDetected => f.write_str("agent plan contains a dependency cycle"),
2026-11-16T23:14:31.6240241Z -            Self::AgentNotRegistered { node_id, agent_id } =>
2026-11-16T23:14:31.6240760Z -                write!(f, "plan node '{node_id}' references unregistered agent '{agent_id}'"),
2026-11-16T23:14:31.6241126Z -            Self::AgentFailed { node_id, source } =>
2026-11-16T23:14:31.6241436Z -                write!(f, "agent task '{node_id}' failed: {source}"),
2026-11-16T23:14:31.6241765Z +            Self::AgentNotRegistered { node_id, agent_id } => write!(
2026-11-16T23:14:31.6242036Z +                f,
2026-11-16T23:14:31.6242292Z +                "plan node '{node_id}' references unregistered agent '{agent_id}'"
2026-11-16T23:14:31.6242590Z +            ),
2026-11-16T23:14:31.6242802Z +            Self::AgentFailed { node_id, source } => {
2026-11-16T23:14:31.6243098Z +                write!(f, "agent task '{node_id}' failed: {source}")
2026-11-16T23:14:31.6243362Z +            }
2026-11-16T23:14:31.6243536Z          }
2026-11-16T23:14:31.6243702Z      }
2026-11-16T23:14:31.6243870Z  }
2026-11-16T23:14:31.6244188Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:192:
2026-11-16T23:14:31.6244858Z          execution: &ExecutionContext,
2026-11-16T23:14:31.6245108Z      ) -> Result<PlanResult, PlanError> {
2026-11-16T23:14:31.6245368Z          let order = plan.execution_order()?;
2026-11-16T23:14:31.6245635Z -        let nodes: BTreeMap<&str, &PlanNode> =
2026-11-16T23:14:31.6245953Z -            plan.nodes().iter().map(|node| (node.id.as_str(), node)).collect();
2026-11-16T23:14:31.6246299Z +        let nodes: BTreeMap<&str, &PlanNode> = plan
2026-11-16T23:14:31.6246550Z +            .nodes()
2026-11-16T23:14:31.6246742Z +            .iter()
2026-11-16T23:14:31.6246949Z +            .map(|node| (node.id.as_str(), node))
2026-11-16T23:14:31.6247197Z +            .collect();
2026-11-16T23:14:31.6247466Z          // Preflight every agent reference before running any user code.
2026-11-16T23:14:31.6247774Z          for node_id in &order {
2026-11-16T23:14:31.6248068Z              let node = nodes.get(node_id.as_str()).expect("validated node exists");
2026-11-16T23:14:31.6248530Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:209:
2026-11-16T23:14:31.6248885Z  
2026-11-16T23:14:31.6249058Z          for node_id in &order {
2026-11-16T23:14:31.6249353Z              let node = nodes.get(node_id.as_str()).expect("validated node exists");
2026-11-16T23:14:31.6249673Z -            let response = self.execute(
2026-11-16T23:14:31.6249902Z -                runtime,
2026-11-16T23:14:31.6250105Z -                &node.agent_id,
2026-11-16T23:14:31.6250321Z -                node.task.clone(),
2026-11-16T23:14:31.6250545Z -                execution,
2026-11-16T23:14:31.6250793Z -            ).map_err(|source| PlanError::AgentFailed {
2026-11-16T23:14:31.6251070Z -                node_id: node.id.clone(),
2026-11-16T23:14:31.6251305Z -                source,
2026-11-16T23:14:31.6251497Z -            })?;
2026-11-16T23:14:31.6251683Z +            let response = self
2026-11-16T23:14:31.6251975Z +                .execute(runtime, &node.agent_id, node.task.clone(), execution)
2026-11-16T23:14:31.6252321Z +                .map_err(|source| PlanError::AgentFailed {
2026-11-16T23:14:31.6252602Z +                    node_id: node.id.clone(),
2026-11-16T23:14:31.6252840Z +                    source,
2026-11-16T23:14:31.6253044Z +                })?;
2026-11-16T23:14:31.6253265Z              outputs.insert(node.id.clone(), response);
2026-11-16T23:14:31.6253512Z          }
2026-11-16T23:14:31.6253682Z  
2026-11-16T23:14:31.6253982Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:224:
2026-11-16T23:14:31.6254516Z -        Ok(PlanResult { execution_order: order, outputs })
2026-11-16T23:14:31.6254851Z +        Ok(PlanResult {
2026-11-16T23:14:31.6255062Z +            execution_order: order,
2026-11-16T23:14:31.6255288Z +            outputs,
2026-11-16T23:14:31.6255473Z +        })
2026-11-16T23:14:31.6255640Z      }
2026-11-16T23:14:31.6255810Z  }
2026-11-16T23:14:31.6256093Z  
2026-11-16T23:14:31.6256392Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:261:
2026-11-16T23:14:31.6256745Z      #[test]
2026-11-16T23:14:31.6256957Z      fn produces_stable_topological_order() {
2026-11-16T23:14:31.6257226Z          let mut plan = AgentPlan::new();
2026-11-16T23:14:31.6257605Z -        plan.add_node(PlanNode::new("c", "agent.echo", AgentTask::new("c", "C")).after("a")).unwrap();
2026-11-16T23:14:31.6258070Z -        plan.add_node(PlanNode::new("b", "agent.echo", AgentTask::new("b", "B"))).unwrap();
2026-11-16T23:14:31.6258489Z -        plan.add_node(PlanNode::new("a", "agent.echo", AgentTask::new("a", "A"))).unwrap();
2026-11-16T23:14:31.6258909Z +        plan.add_node(PlanNode::new("c", "agent.echo", AgentTask::new("c", "C")).after("a"))
2026-11-16T23:14:31.6259236Z +            .unwrap();
2026-11-16T23:14:31.6259517Z +        plan.add_node(PlanNode::new("b", "agent.echo", AgentTask::new("b", "B")))
2026-11-16T23:14:31.6259838Z +            .unwrap();
2026-11-16T23:14:31.6260215Z +        plan.add_node(PlanNode::new("a", "agent.echo", AgentTask::new("a", "A")))
2026-11-16T23:14:31.6260523Z +            .unwrap();
2026-11-16T23:14:31.6260780Z          assert_eq!(plan.execution_order().unwrap(), vec!["a", "b", "c"]);
2026-11-16T23:14:31.6261073Z      }
2026-11-16T23:14:31.6261236Z  
2026-11-16T23:14:31.6261537Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:270:
2026-11-16T23:14:31.6261897Z      #[test]
2026-11-16T23:14:31.6262114Z      fn rejects_missing_dependency_before_execution() {
2026-11-16T23:14:31.6262389Z          let mut plan = AgentPlan::new();
2026-11-16T23:14:31.6262777Z -        plan.add_node(PlanNode::new("child", "agent.echo", AgentTask::new("t", "x")).after("missing")).unwrap();
2026-11-16T23:14:31.6263159Z +        plan.add_node(
2026-11-16T23:14:31.6263445Z +            PlanNode::new("child", "agent.echo", AgentTask::new("t", "x")).after("missing"),
2026-11-16T23:14:31.6263775Z +        )
2026-11-16T23:14:31.6263950Z +        .unwrap();
2026-11-16T23:14:31.6264144Z          assert!(matches!(
2026-11-16T23:14:31.6264364Z              plan.execution_order(),
2026-11-16T23:14:31.6264810Z              Err(PlanError::MissingDependency { .. })
2026-11-16T23:14:31.6265195Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:280:
2026-11-16T23:14:31.6265549Z      #[test]
2026-11-16T23:14:31.6265752Z      fn rejects_dependency_cycles() {
2026-11-16T23:14:31.6266004Z          let mut plan = AgentPlan::new();
2026-11-16T23:14:31.6266362Z -        plan.add_node(PlanNode::new("a", "agent.echo", AgentTask::new("a", "A")).after("b")).unwrap();
2026-11-16T23:14:31.6266845Z -        plan.add_node(PlanNode::new("b", "agent.echo", AgentTask::new("b", "B")).after("a")).unwrap();
2026-11-16T23:14:31.6267291Z +        plan.add_node(PlanNode::new("a", "agent.echo", AgentTask::new("a", "A")).after("b"))
2026-11-16T23:14:31.6267624Z +            .unwrap();
2026-11-16T23:14:31.6267919Z +        plan.add_node(PlanNode::new("b", "agent.echo", AgentTask::new("b", "B")).after("a"))
2026-11-16T23:14:31.6268247Z +            .unwrap();
2026-11-16T23:14:31.6268509Z          assert_eq!(plan.execution_order(), Err(PlanError::CycleDetected));
2026-11-16T23:14:31.6268807Z      }
2026-11-16T23:14:31.6268976Z  
2026-11-16T23:14:31.6269265Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:290:
2026-11-16T23:14:31.6269659Z          let mut agents = AgentRuntime::default();
2026-11-16T23:14:31.6269948Z          agents.register(Arc::new(EchoAgent)).unwrap();
2026-11-16T23:14:31.6270221Z          let mut plan = AgentPlan::new();
2026-11-16T23:14:31.6270601Z -        plan.add_node(PlanNode::new("second", "agent.echo", AgentTask::new("t2", "two")).after("first")).unwrap();
2026-11-16T23:14:31.6271107Z -        plan.add_node(PlanNode::new("first", "agent.echo", AgentTask::new("t1", "one"))).unwrap();
2026-11-16T23:14:31.6271462Z +        plan.add_node(
2026-11-16T23:14:31.6271899Z +            PlanNode::new("second", "agent.echo", AgentTask::new("t2", "two")).after("first"),
2026-11-16T23:14:31.6272225Z +        )
2026-11-16T23:14:31.6272403Z +        .unwrap();
2026-11-16T23:14:31.6272600Z +        plan.add_node(PlanNode::new(
2026-11-16T23:14:31.6272835Z +            "first",
2026-11-16T23:14:31.6273047Z +            "agent.echo",
2026-11-16T23:14:31.6273262Z +            AgentTask::new("t1", "one"),
2026-11-16T23:14:31.6273481Z +        ))
2026-11-16T23:14:31.6273655Z +        .unwrap();
2026-11-16T23:14:31.6273829Z  
2026-11-16T23:14:31.6274046Z          let runtime = Runtime::new(RuntimeConfig::default());
2026-11-16T23:14:31.6274562Z          let result = agents.execute_plan(&runtime, &plan, &context()).unwrap();
2026-11-16T23:14:31.6275011Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:305:
2026-11-16T23:14:31.6275404Z          let mut agents = AgentRuntime::default();
2026-11-16T23:14:31.6275688Z          agents.register(Arc::new(EchoAgent)).unwrap();
2026-11-16T23:14:31.6276071Z          let mut plan = AgentPlan::new();
2026-11-16T23:14:31.6276420Z -        plan.add_node(PlanNode::new("first", "agent.echo", AgentTask::new("t1", "one"))).unwrap();
2026-11-16T23:14:31.6276946Z -        plan.add_node(PlanNode::new("second", "agent.missing", AgentTask::new("t2", "two")).after("first")).unwrap();
2026-11-16T23:14:31.6277367Z +        plan.add_node(PlanNode::new(
2026-11-16T23:14:31.6277593Z +            "first",
2026-11-16T23:14:31.6277784Z +            "agent.echo",
2026-11-16T23:14:31.6278003Z +            AgentTask::new("t1", "one"),
2026-11-16T23:14:31.6278226Z +        ))
2026-11-16T23:14:31.6278396Z +        .unwrap();
2026-11-16T23:14:31.6278583Z +        plan.add_node(
2026-11-16T23:14:31.6278907Z +            PlanNode::new("second", "agent.missing", AgentTask::new("t2", "two")).after("first"),
2026-11-16T23:14:31.6279256Z +        )
2026-11-16T23:14:31.6279430Z +        .unwrap();
2026-11-16T23:14:31.6279615Z  
2026-11-16T23:14:31.6279834Z          let runtime = Runtime::new(RuntimeConfig::default());
2026-11-16T23:14:31.6280108Z          assert_eq!(
2026-11-16T23:14:31.6280435Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/agent-runtime/src/orchestration.rs:320:
2026-11-16T23:14:31.6280797Z  
2026-11-16T23:14:31.6280954Z      #[test]
2026-11-16T23:14:31.6281145Z      fn rejects_empty_plan() {
2026-11-16T23:14:31.6281457Z -        assert_eq!(AgentPlan::new().execution_order(), Err(PlanError::EmptyPlan));
2026-11-16T23:14:31.6281776Z +        assert_eq!(
2026-11-16T23:14:31.6281988Z +            AgentPlan::new().execution_order(),
2026-11-16T23:14:31.6282248Z +            Err(PlanError::EmptyPlan)
2026-11-16T23:14:31.6282465Z +        );
2026-11-16T23:14:31.6282631Z      }
2026-11-16T23:14:31.6282792Z  }
2026-11-16T23:14:31.6282949Z  
2026-11-16T23:14:31.6283226Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/extension-forge/src/main.rs:3:
2026-11-16T23:14:31.6283576Z  use std::io::Write;
2026-11-16T23:14:31.6283797Z  use std::path::{Component, Path, PathBuf};
2026-11-16T23:14:31.6284039Z  
2026-11-16T23:14:31.6284322Z -const CATEGORIES: &[&str] = &["creative", "language", "vision", "audio", "spatial", "agents"];
2026-11-16T23:14:31.6284824Z +const CATEGORIES: &[&str] = &[
2026-11-16T23:14:31.6285104Z +    "creative", "language", "vision", "audio", "spatial", "agents",
2026-11-16T23:14:31.6285387Z +];
2026-11-16T23:14:31.6285547Z  
2026-11-16T23:14:31.6285704Z  fn main() {
2026-11-16T23:14:31.6285893Z      if let Err(error) = run() {
2026-11-16T23:14:31.6286234Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/extension-forge/src/main.rs:16:
2026-11-16T23:14:31.6286602Z      let mut args = env::args().skip(1);
2026-11-16T23:14:31.6286851Z      match args.next().as_deref() {
2026-11-16T23:14:31.6287119Z          Some("new") => create_extension(args.collect()),
2026-11-16T23:14:31.6287464Z -        Some("help") | Some("--help") | Some("-h") | None => { print_help(); Ok(()) }
2026-11-16T23:14:31.6288033Z -        Some(command) => Err(format!("unknown command '{command}'. Run 'lmlmxtend-forge help'.")),
2026-11-16T23:14:31.6288440Z +        Some("help") | Some("--help") | Some("-h") | None => {
2026-11-16T23:14:31.6288717Z +            print_help();
2026-11-16T23:14:31.6288926Z +            Ok(())
2026-11-16T23:14:31.6289106Z +        }
2026-11-16T23:14:31.6289294Z +        Some(command) => Err(format!(
2026-11-16T23:14:31.6289576Z +            "unknown command '{command}'. Run 'lmlmxtend-forge help'."
2026-11-16T23:14:31.6289853Z +        )),
2026-11-16T23:14:31.6290022Z      }
2026-11-16T23:14:31.6290188Z  }
2026-11-16T23:14:31.6290346Z  
2026-11-16T23:14:31.6290625Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/extension-forge/src/main.rs:43:
2026-11-16T23:14:31.6291011Z      let mut root = PathBuf::from("extensions");
2026-11-16T23:14:31.6291274Z      let mut iter = args.into_iter();
2026-11-16T23:14:31.6291515Z      while let Some(flag) = iter.next() {
2026-11-16T23:14:31.6291833Z -        let value = iter.next().ok_or_else(|| format!("missing value for {flag}"))?;
2026-11-16T23:14:31.6292273Z +        let value = iter
2026-11-16T23:14:31.6292474Z +            .next()
2026-11-16T23:14:31.6292712Z +            .ok_or_else(|| format!("missing value for {flag}"))?;
2026-11-16T23:14:31.6292992Z          match flag.as_str() {
2026-11-16T23:14:31.6293218Z              "--name" => name = Some(value),
2026-11-16T23:14:31.6293473Z              "--category" => category = Some(value),
2026-11-16T23:14:31.6293825Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/extension-forge/src/main.rs:57:
2026-11-16T23:14:31.6294243Z      let category = category.ok_or("required option: --category")?;
2026-11-16T23:14:31.6294782Z      let capability = capability.ok_or("required option: --capability")?;
2026-11-16T23:14:31.6295099Z      validate_slug(&name)?;
2026-11-16T23:14:31.6295443Z -    if root.is_absolute() || root.components().any(|part| matches!(part, Component::ParentDir)) {
2026-11-16T23:14:31.6295809Z +    if root.is_absolute()
2026-11-16T23:14:31.6296018Z +        || root
2026-11-16T23:14:31.6296202Z +            .components()
2026-11-16T23:14:31.6296443Z +            .any(|part| matches!(part, Component::ParentDir))
2026-11-16T23:14:31.6296706Z +    {
2026-11-16T23:14:31.6297004Z          return Err("--root must be a repository-relative path without '..' components".into());
2026-11-16T23:14:31.6297346Z      }
2026-11-16T23:14:31.6297541Z      if !CATEGORIES.contains(&category.as_str()) {
2026-11-16T23:14:31.6297906Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/extension-forge/src/main.rs:64:
2026-11-16T23:14:31.6298387Z -        return Err(format!("unsupported category '{category}'. Choose: {}", CATEGORIES.join(", ")));
2026-11-16T23:14:31.6298754Z +        return Err(format!(
2026-11-16T23:14:31.6299001Z +            "unsupported category '{category}'. Choose: {}",
2026-11-16T23:14:31.6299272Z +            CATEGORIES.join(", ")
2026-11-16T23:14:31.6299486Z +        ));
2026-11-16T23:14:31.6299654Z      }
2026-11-16T23:14:31.6299845Z      validate_capability(&capability)?;
2026-11-16T23:14:31.6300077Z  
2026-11-16T23:14:31.6300348Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/extension-forge/src/main.rs:68:
2026-11-16T23:14:31.6300719Z      let extension_id = format!("{category}.{name}");
2026-11-16T23:14:31.6301007Z      let destination = root.join(&category).join(&name);
2026-11-16T23:14:31.6301276Z      if destination.exists() {
2026-11-16T23:14:31.6301647Z -        return Err(format!("{} already exists; refusing to overwrite it", destination.display()));
2026-11-16T23:14:31.6302011Z +        return Err(format!(
2026-11-16T23:14:31.6302249Z +            "{} already exists; refusing to overwrite it",
2026-11-16T23:14:31.6302522Z +            destination.display()
2026-11-16T23:14:31.6302739Z +        ));
2026-11-16T23:14:31.6302908Z      }
2026-11-16T23:14:31.6303103Z      let api_path = relative_api_path(&destination);
2026-11-16T23:14:31.6303360Z      let manifest = format!(
2026-11-16T23:14:31.6303781Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/extension-forge/src/main.rs:77:
2026-11-16T23:14:31.6304139Z      let cargo = format!(
2026-11-16T23:14:31.6304968Z          "[package]\nname = \"lmlmxtend-{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\nlicense = \"MIT\"\ndescription = \"LMLMxtend {category} extension: {name}\"\n\n[dependencies]\nlmlmxtend-api = {{ path = \"{api_path}\" }}\n\n[workspace]\n"
2026-11-16T23:14:31.6305683Z      );
2026-11-16T23:14:31.6306053Z -    let source = format!(r#"//! Generated LMLMxtend extension. Replace the example behavior with your implementation.
2026-11-16T23:14:31.6306483Z +    let source = format!(
2026-11-16T23:14:31.6306829Z +        r#"//! Generated LMLMxtend extension. Replace the example behavior with your implementation.
2026-11-16T23:14:31.6307183Z  
2026-11-16T23:14:31.6307360Z  use lmlmxtend_api::{{
2026-11-16T23:14:31.6307675Z      CapabilityId, Extension, ExtensionError, ExtensionManifest, ExtensionRequest,
2026-11-16T23:14:31.6308147Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/extension-forge/src/main.rs:130:
2026-11-16T23:14:31.6308637Z          assert!(response.output.contains("5 bytes"));
2026-11-16T23:14:31.6308898Z      }}
2026-11-16T23:14:31.6309060Z  }}
2026-11-16T23:14:31.6309239Z -"#, type_name = to_type_name(&name));
2026-11-16T23:14:31.6309469Z +"#,
2026-11-16T23:14:31.6309652Z +        type_name = to_type_name(&name)
2026-11-16T23:14:31.6309879Z +    );
2026-11-16T23:14:31.6310037Z  
2026-11-16T23:14:31.6310300Z      fs::create_dir_all(destination.join("src")).map_err(|e| e.to_string())?;
2026-11-16T23:14:31.6310658Z      write_new(&destination.join("Cargo.toml"), &cargo)?;
2026-11-16T23:14:31.6311043Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/extension-forge/src/main.rs:143:
2026-11-16T23:14:31.6311450Z      println!("Created extension: {}", destination.display());
2026-11-16T23:14:31.6311785Z      println!("Extension ID: {extension_id}");
2026-11-16T23:14:31.6312054Z      println!("Capability: {capability}");
2026-11-16T23:14:31.6312407Z -    println!("Next: cargo test --manifest-path {}/Cargo.toml", destination.display());
2026-11-16T23:14:31.6312751Z +    println!(
2026-11-16T23:14:31.6312979Z +        "Next: cargo test --manifest-path {}/Cargo.toml",
2026-11-16T23:14:31.6313262Z +        destination.display()
2026-11-16T23:14:31.6313477Z +    );
2026-11-16T23:14:31.6313643Z      Ok(())
2026-11-16T23:14:31.6313813Z  }
2026-11-16T23:14:31.6313974Z  
2026-11-16T23:14:31.6314256Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/extension-forge/src/main.rs:150:
2026-11-16T23:14:31.6314831Z  fn write_new(path: &Path, content: &str) -> Result<(), String> {
2026-11-16T23:14:31.6315227Z -    let mut file = OpenOptions::new().write(true).create_new(true).open(path)
2026-11-16T23:14:31.6315564Z +    let mut file = OpenOptions::new()
2026-11-16T23:14:31.6315799Z +        .write(true)
2026-11-16T23:14:31.6315999Z +        .create_new(true)
2026-11-16T23:14:31.6316206Z +        .open(path)
2026-11-16T23:14:31.6316468Z          .map_err(|e| format!("cannot create {}: {e}", path.display()))?;
2026-11-16T23:14:31.6316823Z -    file.write_all(content.as_bytes()).map_err(|e| e.to_string())
2026-11-16T23:14:31.6317123Z +    file.write_all(content.as_bytes())
2026-11-16T23:14:31.6317362Z +        .map_err(|e| e.to_string())
2026-11-16T23:14:31.6317578Z  }
2026-11-16T23:14:31.6317734Z  
2026-11-16T23:14:31.6317937Z  fn validate_slug(value: &str) -> Result<(), String> {
2026-11-16T23:14:31.6318310Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/extension-forge/src/main.rs:157:
2026-11-16T23:14:31.6318690Z -    let valid = !value.is_empty() && value.len() <= 48
2026-11-16T23:14:31.6318953Z +    let valid = !value.is_empty()
2026-11-16T23:14:31.6319172Z +        && value.len() <= 48
2026-11-16T23:14:31.6319401Z          && value.split('-').all(|part| {
2026-11-16T23:14:31.6319636Z              !part.is_empty()
2026-11-16T23:14:31.6319876Z                  && part.as_bytes()[0].is_ascii_lowercase()
2026-11-16T23:14:31.6320361Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/extension-forge/src/main.rs:161:
2026-11-16T23:14:31.6320798Z -                && part.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
2026-11-16T23:14:31.6321109Z +                && part
2026-11-16T23:14:31.6321314Z +                    .bytes()
2026-11-16T23:14:31.6321567Z +                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
2026-11-16T23:14:31.6321834Z          });
2026-11-16T23:14:31.6322013Z -    if valid { Ok(()) } else {
2026-11-16T23:14:31.6322216Z +    if valid {
2026-11-16T23:14:31.6322394Z +        Ok(())
2026-11-16T23:14:31.6322564Z +    } else {
2026-11-16T23:14:31.6322905Z          Err("name must be 1-48 lowercase ASCII letters, digits, or hyphens; it must start with a letter".into())
2026-11-16T23:14:31.6323294Z      }
2026-11-16T23:14:31.6323461Z  }
2026-11-16T23:14:31.6323737Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/extension-forge/src/main.rs:172:
2026-11-16T23:14:31.6324069Z  }
2026-11-16T23:14:31.6324245Z  
2026-11-16T23:14:31.6324666Z  fn to_type_name(value: &str) -> String {
2026-11-16T23:14:31.6324916Z -    value.split('-').map(|part| {
2026-11-16T23:14:31.6325156Z -        let mut chars = part.chars();
2026-11-16T23:14:31.6325397Z -        match chars.next() {
2026-11-16T23:14:31.6325691Z -            Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
2026-11-16T23:14:31.6326027Z -            None => String::new(),
2026-11-16T23:14:31.6326246Z -        }
2026-11-16T23:14:31.6326419Z -    }).collect()
2026-11-16T23:14:31.6326599Z +    value
2026-11-16T23:14:31.6326775Z +        .split('-')
2026-11-16T23:14:31.6326971Z +        .map(|part| {
2026-11-16T23:14:31.6327170Z +            let mut chars = part.chars();
2026-11-16T23:14:31.6327407Z +            match chars.next() {
2026-11-16T23:14:31.6327702Z +                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
2026-11-16T23:14:31.6328030Z +                None => String::new(),
2026-11-16T23:14:31.6328261Z +            }
2026-11-16T23:14:31.6328447Z +        })
2026-11-16T23:14:31.6328617Z +        .collect()
2026-11-16T23:14:31.6328789Z  }
2026-11-16T23:14:31.6328943Z  
2026-11-16T23:14:31.6329149Z  fn relative_api_path(destination: &Path) -> String {
2026-11-16T23:14:31.6329518Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/extension-forge/src/main.rs:213:
2026-11-16T23:14:31.6329848Z  
2026-11-16T23:14:31.6330004Z      #[test]
2026-11-16T23:14:31.6330226Z      fn computes_api_path_from_default_extension_root() {
2026-11-16T23:14:31.6330692Z -        assert_eq!(relative_api_path(Path::new("extensions/creative/scene-composer")), "../../../crates/lmlmxtend-api");
2026-11-16T23:14:31.6331120Z +        assert_eq!(
2026-11-16T23:14:31.6331401Z +            relative_api_path(Path::new("extensions/creative/scene-composer")),
2026-11-16T23:14:31.6331728Z +            "../../../crates/lmlmxtend-api"
2026-11-16T23:14:31.6331961Z +        );
2026-11-16T23:14:31.6332127Z      }
2026-11-16T23:14:31.6332293Z  }
2026-11-16T23:14:31.6332452Z  
2026-11-16T23:14:31.6332720Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/intelligence/src/lib.rs:58:
2026-11-16T23:14:31.6333064Z  #[derive(Clone, Debug, Eq, PartialEq)]
2026-11-16T23:14:31.6333298Z  pub enum RoutingError {
2026-11-16T23:14:31.6333502Z      NoProvider(String),
2026-11-16T23:14:31.6333788Z -    IncompatibleApi { extension_id: String, required: u16, provided: u16 },
2026-11-16T23:14:31.6334213Z -    PreferredProviderUnavailable { requested: String, capability: String },
2026-11-16T23:14:31.6334707Z +    IncompatibleApi {
2026-11-16T23:14:31.6334915Z +        extension_id: String,
2026-11-16T23:14:31.6335125Z +        required: u16,
2026-11-16T23:14:31.6335322Z +        provided: u16,
2026-11-16T23:14:31.6335505Z +    },
2026-11-16T23:14:31.6335690Z +    PreferredProviderUnavailable {
2026-11-16T23:14:31.6335929Z +        requested: String,
2026-11-16T23:14:31.6336144Z +        capability: String,
2026-11-16T23:14:31.6336348Z +    },
2026-11-16T23:14:31.6336638Z  }
2026-11-16T23:14:31.6336807Z  
2026-11-16T23:14:31.6337000Z  impl fmt::Display for RoutingError {
2026-11-16T23:14:31.6337345Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/intelligence/src/lib.rs:100:
2026-11-16T23:14:31.6337696Z  pub struct IntelligenceRouter;
2026-11-16T23:14:31.6337907Z  
2026-11-16T23:14:31.6338081Z  impl IntelligenceRouter {
2026-11-16T23:14:31.6338440Z -    pub fn discover(&self, runtime: &Runtime, capability: &CapabilityId) -> Vec<ExtensionDescriptor> {
2026-11-16T23:14:31.6338820Z +    pub fn discover(
2026-11-16T23:14:31.6339013Z +        &self,
2026-11-16T23:14:31.6339202Z +        runtime: &Runtime,
2026-11-16T23:14:31.6339429Z +        capability: &CapabilityId,
2026-11-16T23:14:31.6339660Z +    ) -> Vec<ExtensionDescriptor> {
2026-11-16T23:14:31.6339914Z          runtime.discover_capability(capability)
2026-11-16T23:14:31.6340159Z      }
2026-11-16T23:14:31.6340325Z  
2026-11-16T23:14:31.6340592Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/intelligence/src/lib.rs:111:
2026-11-16T23:14:31.6341034Z          }
2026-11-16T23:14:31.6341203Z  
2026-11-16T23:14:31.6341451Z          let selected = if let Some(preferred) = &intent.preferred_extension {
2026-11-16T23:14:31.6341855Z -            providers.iter().find(|provider| &provider.extension_id == preferred)
2026-11-16T23:14:31.6342178Z +            providers
2026-11-16T23:14:31.6342375Z +                .iter()
2026-11-16T23:14:31.6342629Z +                .find(|provider| &provider.extension_id == preferred)
2026-11-16T23:14:31.6342987Z                  .ok_or_else(|| RoutingError::PreferredProviderUnavailable {
2026-11-16T23:14:31.6343306Z                      requested: preferred.clone(),
2026-11-16T23:14:31.6343591Z                      capability: intent.capability.to_string(),
2026-11-16T23:14:31.6343963Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/intelligence/src/lib.rs:143:
2026-11-16T23:14:31.6344286Z      }
2026-11-16T23:14:31.6344598Z  
2026-11-16T23:14:31.6344869Z      /// Plans first, then dispatches through runtime policy and lifecycle controls.
2026-11-16T23:14:31.6345399Z -    pub fn execute(&self, runtime: &Runtime, intent: TaskIntent) -> Result<ExtensionResponse, IntelligenceError> {
2026-11-16T23:14:31.6345911Z -        let plan = self.plan(runtime, &intent).map_err(IntelligenceError::Routing)?;
2026-11-16T23:14:31.6345993Z +    pub fn execute(
2026-11-16T23:14:31.6346075Z +        &self,
2026-11-16T23:14:31.6346157Z +        runtime: &Runtime,
2026-11-16T23:14:31.6346244Z +        intent: TaskIntent,
2026-11-16T23:14:31.6346371Z +    ) -> Result<ExtensionResponse, IntelligenceError> {
2026-11-16T23:14:31.6346451Z +        let plan = self
2026-11-16T23:14:31.6346543Z +            .plan(runtime, &intent)
2026-11-16T23:14:31.6346649Z +            .map_err(IntelligenceError::Routing)?;
2026-11-16T23:14:31.6346828Z          let mut request = ExtensionRequest::new(plan.capability, intent.input);
2026-11-16T23:14:31.6346945Z          request.correlation_id = intent.correlation_id;
2026-11-16T23:14:31.6347105Z          runtime.execute(request).map_err(IntelligenceError::Runtime)
2026-11-16T23:14:31.6347293Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/intelligence/src/lib.rs:157:
2026-11-16T23:14:31.6347372Z          intent: TaskIntent,
2026-11-16T23:14:31.6347466Z          context: &ExecutionContext,
2026-11-16T23:14:31.6347574Z      ) -> Result<ExtensionResponse, IntelligenceError> {
2026-11-16T23:14:31.6347753Z -        let plan = self.plan(runtime, &intent).map_err(IntelligenceError::Routing)?;
2026-11-16T23:14:31.6347831Z +        let plan = self
2026-11-16T23:14:31.6347919Z +            .plan(runtime, &intent)
2026-11-16T23:14:31.6348024Z +            .map_err(IntelligenceError::Routing)?;
2026-11-16T23:14:31.6348185Z          let mut request = ExtensionRequest::new(plan.capability, intent.input);
2026-11-16T23:14:31.6348302Z          request.correlation_id = intent.correlation_id;
2026-11-16T23:14:31.6348506Z -        runtime.execute_with_context(request, context).map_err(IntelligenceError::Runtime)
2026-11-16T23:14:31.6348706Z +        runtime
2026-11-16T23:14:31.6348813Z +            .execute_with_context(request, context)
2026-11-16T23:14:31.6348922Z +            .map_err(IntelligenceError::Runtime)
2026-11-16T23:14:31.6349002Z      }
2026-11-16T23:14:31.6349077Z  }
2026-11-16T23:14:31.6349156Z  
2026-11-16T23:14:31.6349343Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/intelligence/src/lib.rs:211:
2026-11-16T23:14:31.6349500Z          let plan = IntelligenceRouter.plan(&runtime, &intent).unwrap();
2026-11-16T23:14:31.6349618Z          assert_eq!(plan.extension_id, "example.echo");
2026-11-16T23:14:31.6349729Z          assert_eq!(plan.extension_version, "1.2.0");
2026-11-16T23:14:31.6349932Z -        assert_eq!(IntelligenceRouter.execute(&runtime, intent).unwrap().output, "hello");
2026-11-16T23:14:31.6350009Z +        assert_eq!(
2026-11-16T23:14:31.6350166Z +            IntelligenceRouter.execute(&runtime, intent).unwrap().output,
2026-11-16T23:14:31.6350243Z +            "hello"
2026-11-16T23:14:31.6350416Z +        );
2026-11-16T23:14:31.6350489Z      }
2026-11-16T23:14:31.6350565Z  
2026-11-16T23:14:31.6350637Z      #[test]
2026-11-16T23:14:31.6350828Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-api/src/lib.rs:17:
2026-11-16T23:14:31.6350922Z              && value.len() <= 128
2026-11-16T23:14:31.6351018Z              && value.split('.').all(|part| {
2026-11-16T23:14:31.6351107Z                  !part.is_empty()
2026-11-16T23:14:31.6351316Z -                    && part.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
2026-11-16T23:14:31.6351419Z +                    && part.chars().all(|c| {
2026-11-16T23:14:31.6351580Z +                        c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_'
2026-11-16T23:14:31.6351663Z +                    })
2026-11-16T23:14:31.6351738Z              });
2026-11-16T23:14:31.6351915Z -        if valid { Ok(Self(value)) } else { Err(ContractError::InvalidCapabilityId) }
2026-11-16T23:14:31.6352010Z +        if valid {
2026-11-16T23:14:31.6352091Z +            Ok(Self(value))
2026-11-16T23:14:31.6352169Z +        } else {
2026-11-16T23:14:31.6352278Z +            Err(ContractError::InvalidCapabilityId)
2026-11-16T23:14:31.6352357Z +        }
2026-11-16T23:14:31.6352431Z      }
2026-11-16T23:14:31.6352502Z  
2026-11-16T23:14:31.6352608Z -    pub fn as_str(&self) -> &str { &self.0 }
2026-11-16T23:14:31.6352694Z +    pub fn as_str(&self) -> &str {
2026-11-16T23:14:31.6352770Z +        &self.0
2026-11-16T23:14:31.6352847Z +    }
2026-11-16T23:14:31.6352920Z  }
2026-11-16T23:14:31.6352987Z  
2026-11-16T23:14:31.6353086Z  impl fmt::Display for CapabilityId {
2026-11-16T23:14:31.6353270Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-api/src/lib.rs:29:
2026-11-16T23:14:31.6353439Z -    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.0) }
2026-11-16T23:14:31.6353567Z +    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
2026-11-16T23:14:31.6353652Z +        f.write_str(&self.0)
2026-11-16T23:14:31.6353731Z +    }
2026-11-16T23:14:31.6353798Z  }
2026-11-16T23:14:31.6353870Z  
2026-11-16T23:14:31.6353974Z  /// A request passed to an installed extension.
2026-11-16T23:14:31.6354151Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-api/src/lib.rs:39:
2026-11-16T23:14:31.6354355Z  
2026-11-16T23:14:31.6354637Z  impl ExtensionRequest {
2026-11-16T23:14:31.6354843Z      pub fn new(capability: CapabilityId, input: impl Into<String>) -> Self {
2026-11-16T23:14:31.6355186Z -        Self { capability, input: input.into(), correlation_id: None }
2026-11-16T23:14:31.6355295Z +        Self {
2026-11-16T23:14:31.6355442Z +            capability,
2026-11-16T23:14:31.6355599Z +            input: input.into(),
2026-11-16T23:14:31.6355704Z +            correlation_id: None,
2026-11-16T23:14:31.6355897Z +        }
2026-11-16T23:14:31.6356002Z      }
2026-11-16T23:14:31.6356169Z  }
2026-11-16T23:14:31.6356272Z  
2026-11-16T23:14:31.6356628Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-api/src/lib.rs:52:
2026-11-16T23:14:31.6356825Z  
2026-11-16T23:14:31.6356945Z  impl ExtensionResponse {
2026-11-16T23:14:31.6357124Z      pub fn text(output: impl Into<String>) -> Self {
2026-11-16T23:14:31.6357341Z -        Self { output: output.into(), media_type: "text/plain; charset=utf-8".into() }
2026-11-16T23:14:31.6357466Z +        Self {
2026-11-16T23:14:31.6357619Z +            output: output.into(),
2026-11-16T23:14:31.6357845Z +            media_type: "text/plain; charset=utf-8".into(),
2026-11-16T23:14:31.6357978Z +        }
2026-11-16T23:14:31.6358080Z      }
2026-11-16T23:14:31.6366121Z  }
2026-11-16T23:14:31.6366261Z  
2026-11-16T23:14:31.6366537Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-api/src/lib.rs:65:
2026-11-16T23:14:31.6366660Z      pub capabilities: Vec<CapabilityId>,
2026-11-16T23:14:31.6366731Z  }
2026-11-16T23:14:31.6366799Z  
2026-11-16T23:14:31.6366864Z -
2026-11-16T23:14:31.6367042Z  /// Cooperative cancellation signal shared by a host and an extension.
2026-11-16T23:14:31.6367294Z  #[derive(Clone, Debug, Default)]
2026-11-16T23:14:31.6367500Z  pub struct CancellationToken(std::sync::Arc<std::sync::atomic::AtomicBool>);
2026-11-16T23:14:31.6367700Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-api/src/lib.rs:72:
2026-11-16T23:14:31.6367771Z  
2026-11-16T23:14:31.6367865Z  impl CancellationToken {
2026-11-16T23:14:31.6367971Z -    pub fn new() -> Self { Self::default() }
2026-11-16T23:14:31.6368057Z +    pub fn new() -> Self {
2026-11-16T23:14:31.6368134Z +        Self::default()
2026-11-16T23:14:31.6368207Z +    }
2026-11-16T23:14:31.6368284Z      pub fn cancel(&self) {
2026-11-16T23:14:31.6368431Z          self.0.store(true, std::sync::atomic::Ordering::Release);
2026-11-16T23:14:31.6368502Z      }
2026-11-16T23:14:31.6368683Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-api/src/lib.rs:91:
2026-11-16T23:14:31.6368751Z  
2026-11-16T23:14:31.6368847Z  impl Default for ExecutionContext {
2026-11-16T23:14:31.6368936Z      fn default() -> Self {
2026-11-16T23:14:31.6369166Z -        Self { cancellation: CancellationToken::new(), deadline: None, max_output_bytes: usize::MAX }
2026-11-16T23:14:31.6369243Z +        Self {
2026-11-16T23:14:31.6369357Z +            cancellation: CancellationToken::new(),
2026-11-16T23:14:31.6369440Z +            deadline: None,
2026-11-16T23:14:31.6369539Z +            max_output_bytes: usize::MAX,
2026-11-16T23:14:31.6369607Z +        }
2026-11-16T23:14:31.6369679Z      }
2026-11-16T23:14:31.6369743Z  }
2026-11-16T23:14:31.6369815Z  
2026-11-16T23:14:31.6369997Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-api/src/lib.rs:98:
2026-11-16T23:14:31.6370085Z  impl ExecutionContext {
2026-11-16T23:14:31.6370372Z -    pub fn new(cancellation: CancellationToken, deadline: Option<std::time::Instant>, max_output_bytes: usize) -> Self {
2026-11-16T23:14:31.6370490Z -        Self { cancellation, deadline, max_output_bytes }
2026-11-16T23:14:31.6370569Z +    pub fn new(
2026-11-16T23:14:31.6370665Z +        cancellation: CancellationToken,
2026-11-16T23:14:31.6370771Z +        deadline: Option<std::time::Instant>,
2026-11-16T23:14:31.6370860Z +        max_output_bytes: usize,
2026-11-16T23:14:31.6370936Z +    ) -> Self {
2026-11-16T23:14:31.6371007Z +        Self {
2026-11-16T23:14:31.6371084Z +            cancellation,
2026-11-16T23:14:31.6371161Z +            deadline,
2026-11-16T23:14:31.6371240Z +            max_output_bytes,
2026-11-16T23:14:31.6371309Z +        }
2026-11-16T23:14:31.6371377Z      }
2026-11-16T23:14:31.6371571Z      pub fn with_timeout(timeout: std::time::Duration, max_output_bytes: usize) -> Self {
2026-11-16T23:14:31.6371792Z -        Self::new(CancellationToken::new(), Some(std::time::Instant::now() + timeout), max_output_bytes)
2026-11-16T23:14:31.6371869Z +        Self::new(
2026-11-16T23:14:31.6371966Z +            CancellationToken::new(),
2026-11-16T23:14:31.6372074Z +            Some(std::time::Instant::now() + timeout),
2026-11-16T23:14:31.6372270Z +            max_output_bytes,
2026-11-16T23:14:31.6372335Z +        )
2026-11-16T23:14:31.6372405Z      }
2026-11-16T23:14:31.6372599Z -    pub fn cancellation_token(&self) -> CancellationToken { self.cancellation.clone() }
2026-11-16T23:14:31.6372765Z -    pub fn is_cancelled(&self) -> bool { self.cancellation.is_cancelled() }
2026-11-16T23:14:31.6372899Z +    pub fn cancellation_token(&self) -> CancellationToken {
2026-11-16T23:14:31.6372989Z +        self.cancellation.clone()
2026-11-16T23:14:31.6373058Z +    }
2026-11-16T23:14:31.6373146Z +    pub fn is_cancelled(&self) -> bool {
2026-11-16T23:14:31.6373236Z +        self.cancellation.is_cancelled()
2026-11-16T23:14:31.6373303Z +    }
2026-11-16T23:14:31.6373409Z      pub fn is_deadline_exceeded(&self) -> bool {
2026-11-16T23:14:31.6373587Z -        self.deadline.is_some_and(|deadline| std::time::Instant::now() >= deadline)
2026-11-16T23:14:31.6373661Z +        self.deadline
2026-11-16T23:14:31.6373810Z +            .is_some_and(|deadline| std::time::Instant::now() >= deadline)
2026-11-16T23:14:31.6373955Z      }
2026-11-16T23:14:31.6374098Z -    pub fn max_output_bytes(&self) -> usize { self.max_output_bytes }
2026-11-16T23:14:31.6374197Z +    pub fn max_output_bytes(&self) -> usize {
2026-11-16T23:14:31.6374284Z +        self.max_output_bytes
2026-11-16T23:14:31.6374358Z +    }
2026-11-16T23:14:31.6374697Z      pub fn limited_to_output(&self, max_output_bytes: usize) -> Self {
2026-11-16T23:14:31.6374790Z          let mut context = self.clone();
2026-11-16T23:14:31.6374963Z          context.max_output_bytes = context.max_output_bytes.min(max_output_bytes);
2026-11-16T23:14:31.6375156Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-api/src/lib.rs:115:
2026-11-16T23:14:31.6375225Z      }
2026-11-16T23:14:31.6375360Z      /// Extensions should call this between bounded units of work.
2026-11-16T23:14:31.6375512Z      pub fn check_cancelled(&self) -> Result<(), ExecutionControlError> {
2026-11-16T23:14:31.6375700Z -        if self.is_cancelled() { return Err(ExecutionControlError::Cancelled); }
2026-11-16T23:14:31.6375926Z -        if self.is_deadline_exceeded() { return Err(ExecutionControlError::DeadlineExceeded); }
2026-11-16T23:14:31.6376011Z +        if self.is_cancelled() {
2026-11-16T23:14:31.6376136Z +            return Err(ExecutionControlError::Cancelled);
2026-11-16T23:14:31.6376205Z +        }
2026-11-16T23:14:31.6376295Z +        if self.is_deadline_exceeded() {
2026-11-16T23:14:31.6376429Z +            return Err(ExecutionControlError::DeadlineExceeded);
2026-11-16T23:14:31.6376495Z +        }
2026-11-16T23:14:31.6376567Z          Ok(())
2026-11-16T23:14:31.6376637Z      }
2026-11-16T23:14:31.6376708Z  }
2026-11-16T23:14:31.6376896Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-api/src/lib.rs:123:
2026-11-16T23:14:31.6376970Z  
2026-11-16T23:14:31.6377070Z  /// Host-enforced request lifecycle failures.
2026-11-16T23:14:31.6377177Z  #[derive(Clone, Copy, Debug, Eq, PartialEq)]
2026-11-16T23:14:31.6377376Z -pub enum ExecutionControlError { Cancelled, DeadlineExceeded, OutputTooLarge }
2026-11-16T23:14:31.6377465Z +pub enum ExecutionControlError {
2026-11-16T23:14:31.6377540Z +    Cancelled,
2026-11-16T23:14:31.6377617Z +    DeadlineExceeded,
2026-11-16T23:14:31.6377700Z +    OutputTooLarge,
2026-11-16T23:14:31.6377768Z +}
2026-11-16T23:14:31.6377839Z  
2026-11-16T23:14:31.6377949Z  impl fmt::Display for ExecutionControlError {
2026-11-16T23:14:31.6378088Z      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
2026-11-16T23:14:31.6378272Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-api/src/lib.rs:143:
2026-11-16T23:14:31.6378340Z  
2026-11-16T23:14:31.6378519Z      /// Performs a lightweight readiness check. Implementations should not start
2026-11-16T23:14:31.6378654Z      /// expensive work or mutate extension state from this method.
2026-11-16T23:14:31.6378794Z -    fn health_check(&self) -> Result<(), ExtensionError> { Ok(()) }
2026-11-16T23:14:31.6379026Z +    fn health_check(&self) -> Result<(), ExtensionError> {
2026-11-16T23:14:31.6379102Z +        Ok(())
2026-11-16T23:14:31.6379172Z +    }
2026-11-16T23:14:31.6379236Z  
2026-11-16T23:14:31.6379449Z      fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse, ExtensionError>;
2026-11-16T23:14:31.6379514Z  
2026-11-16T23:14:31.6379701Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-api/src/lib.rs:150:
2026-11-16T23:14:31.6379878Z      /// Context-aware execution. The default preserves compatibility but cannot
2026-11-16T23:14:31.6380064Z      /// interrupt legacy implementations; cooperative extensions should override it.
2026-11-16T23:14:31.6380385Z -    fn execute_with_context(&self, request: ExtensionRequest, context: &ExecutionContext) -> Result<ExtensionResponse, ExtensionError> {
2026-11-16T23:14:31.6380622Z -        context.check_cancelled().map_err(|error| ExtensionError::ExecutionFailed(error.to_string()))?;
2026-11-16T23:14:31.6380716Z +    fn execute_with_context(
2026-11-16T23:14:31.6380880Z +        &self,
2026-11-16T23:14:31.6380973Z +        request: ExtensionRequest,
2026-11-16T23:14:31.6381061Z +        context: &ExecutionContext,
2026-11-16T23:14:31.6381180Z +    ) -> Result<ExtensionResponse, ExtensionError> {
2026-11-16T23:14:31.6381251Z +        context
2026-11-16T23:14:31.6381330Z +            .check_cancelled()
2026-11-16T23:14:31.6381500Z +            .map_err(|error| ExtensionError::ExecutionFailed(error.to_string()))?;
2026-11-16T23:14:31.6381582Z          self.execute(request)
2026-11-16T23:14:31.6381652Z      }
2026-11-16T23:14:31.6381716Z  }
2026-11-16T23:14:31.6381901Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-api/src/lib.rs:204:
2026-11-16T23:14:31.6381975Z  
2026-11-16T23:14:31.6382044Z      #[test]
2026-11-16T23:14:31.6382140Z      fn accepts_namespaced_capability() {
2026-11-16T23:14:31.6382355Z -        assert_eq!(CapabilityId::new("creative.generate").unwrap().as_str(), "creative.generate");
2026-11-16T23:14:31.6382440Z +        assert_eq!(
2026-11-16T23:14:31.6382583Z +            CapabilityId::new("creative.generate").unwrap().as_str(),
2026-11-16T23:14:31.6382669Z +            "creative.generate"
2026-11-16T23:14:31.6382740Z +        );
2026-11-16T23:14:31.6382805Z      }
2026-11-16T23:14:31.6382874Z  
2026-11-16T23:14:31.6382941Z      #[test]
2026-11-16T23:14:31.6383119Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-api/src/lib.rs:211:
2026-11-16T23:14:31.6383208Z      fn rejects_malformed_capability() {
2026-11-16T23:14:31.6383432Z -        assert_eq!(CapabilityId::new("Creative..generate"), Err(ContractError::InvalidCapabilityId));
2026-11-16T23:14:31.6383504Z +        assert_eq!(
2026-11-16T23:14:31.6383615Z +            CapabilityId::new("Creative..generate"),
2026-11-16T23:14:31.6383727Z +            Err(ContractError::InvalidCapabilityId)
2026-11-16T23:14:31.6383795Z +        );
2026-11-16T23:14:31.6383863Z      }
2026-11-16T23:14:31.6383929Z  }
2026-11-16T23:14:31.6383998Z  
2026-11-16T23:14:31.6384173Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-core/src/lib.rs:13:
2026-11-16T23:14:31.6384250Z  
2026-11-16T23:14:31.6384337Z  impl Default for RuntimeConfig {
2026-11-16T23:14:31.6384557Z      fn default() -> Self {
2026-11-16T23:14:31.6384775Z -        Self { max_input_bytes: 1_048_576, max_output_bytes: 1_048_576, allowed_capabilities: None }
2026-11-16T23:14:31.6384846Z +        Self {
2026-11-16T23:14:31.6384938Z +            max_input_bytes: 1_048_576,
2026-11-16T23:14:31.6385024Z +            max_output_bytes: 1_048_576,
2026-11-16T23:14:31.6385116Z +            allowed_capabilities: None,
2026-11-16T23:14:31.6385186Z +        }
2026-11-16T23:14:31.6385258Z      }
2026-11-16T23:14:31.6385330Z  }
2026-11-16T23:14:31.6385395Z  
2026-11-16T23:14:31.6385575Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-core/src/lib.rs:43:
2026-11-16T23:14:31.6385671Z          if input.len() > self.max_input_bytes {
2026-11-16T23:14:31.6385789Z              return PolicyDecision::DenyInputTooLarge;
2026-11-16T23:14:31.6385972Z          }
2026-11-16T23:14:31.6386190Z -        if self.allowed_capabilities.as_ref().is_some_and(|allowed| !allowed.contains(capability)) {
2026-11-16T23:14:31.6386265Z +        if self
2026-11-16T23:14:31.6386349Z +            .allowed_capabilities
2026-11-16T23:14:31.6386424Z +            .as_ref()
2026-11-16T23:14:31.6386549Z +            .is_some_and(|allowed| !allowed.contains(capability))
2026-11-16T23:14:31.6386619Z +        {
2026-11-16T23:14:31.6386722Z              return PolicyDecision::DenyCapability;
2026-11-16T23:14:31.6386792Z          }
2026-11-16T23:14:31.6386879Z          PolicyDecision::Allow
2026-11-16T23:14:31.6387119Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/examples/basic_extension.rs:1:
2026-11-16T23:14:31.6387416Z -use lmlmxtend_api::{CapabilityId, Extension, ExtensionError, ExtensionManifest, ExtensionRequest, ExtensionResponse};
2026-11-16T23:14:31.6387496Z +use lmlmxtend_api::{
2026-11-16T23:14:31.6387729Z +    CapabilityId, Extension, ExtensionError, ExtensionManifest, ExtensionRequest, ExtensionResponse,
2026-11-16T23:14:31.6387888Z +};
2026-11-16T23:14:31.6387984Z  use lmlmxtend_core::RuntimeConfig;
2026-11-16T23:14:31.6388072Z  use lmlmxtend_runtime::Runtime;
2026-11-16T23:14:31.6388148Z  use std::sync::Arc;
2026-11-16T23:14:31.6388387Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/examples/basic_extension.rs:16:
2026-11-16T23:14:31.6388453Z      }
2026-11-16T23:14:31.6388524Z  
2026-11-16T23:14:31.6388736Z      fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse, ExtensionError> {
2026-11-16T23:14:31.6388899Z -        Ok(ExtensionResponse::text(format!("Hello, {}!", request.input)))
2026-11-16T23:14:31.6388998Z +        Ok(ExtensionResponse::text(format!(
2026-11-16T23:14:31.6389080Z +            "Hello, {}!",
2026-11-16T23:14:31.6389161Z +            request.input
2026-11-16T23:14:31.6389231Z +        )))
2026-11-16T23:14:31.6389301Z      }
2026-11-16T23:14:31.6389369Z  }
2026-11-16T23:14:31.6389443Z  
2026-11-16T23:14:31.6389628Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:1:
2026-11-16T23:14:31.6389791Z  //! Minimal in-process extension host. This is not an isolation boundary.
2026-11-16T23:14:31.6389858Z  
2026-11-16T23:14:31.6390187Z -use lmlmxtend_api::{CapabilityId, ExecutionContext, Extension, ExtensionError, ExtensionManifest, ExtensionRequest, ExtensionResponse};
2026-11-16T23:14:31.6390272Z +use lmlmxtend_api::{
2026-11-16T23:14:31.6390499Z +    CapabilityId, ExecutionContext, Extension, ExtensionError, ExtensionManifest, ExtensionRequest,
2026-11-16T23:14:31.6390581Z +    ExtensionResponse,
2026-11-16T23:14:31.6390647Z +};
2026-11-16T23:14:31.6390814Z  use lmlmxtend_core::{ExecutionPolicy, PolicyDecision, RuntimeConfig};
2026-11-16T23:14:31.6390916Z  use std::collections::{BTreeMap, BTreeSet};
2026-11-16T23:14:31.6390991Z -use std::sync::Arc;
2026-11-16T23:14:31.6391067Z  use std::fmt;
2026-11-16T23:14:31.6391144Z +use std::sync::Arc;
2026-11-16T23:14:31.6391217Z  
2026-11-16T23:14:31.6391308Z  #[derive(Clone, Debug, Eq, PartialEq)]
2026-11-16T23:14:31.6391393Z  pub enum RuntimeError {
2026-11-16T23:14:31.6391582Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:79:
2026-11-16T23:14:31.6391778Z      pub fn register(&mut self, extension: Arc<dyn Extension>) -> Result<(), RuntimeError> {
2026-11-16T23:14:31.6391913Z          let manifest: ExtensionManifest = extension.manifest();
2026-11-16T23:14:31.6392074Z          if manifest.id.trim().is_empty() || manifest.version.trim().is_empty() {
2026-11-16T23:14:31.6392276Z -            return Err(RuntimeError::InvalidManifest("id and version are required".into()));
2026-11-16T23:14:31.6392385Z +            return Err(RuntimeError::InvalidManifest(
2026-11-16T23:14:31.6392489Z +                "id and version are required".into(),
2026-11-16T23:14:31.6392555Z +            ));
2026-11-16T23:14:31.6392626Z          }
2026-11-16T23:14:31.6392835Z          if manifest.api_version != lmlmxtend_api::API_VERSION {
2026-11-16T23:14:31.6393029Z -            return Err(RuntimeError::InvalidManifest("incompatible API version".into()));
2026-11-16T23:14:31.6393135Z +            return Err(RuntimeError::InvalidManifest(
2026-11-16T23:14:31.6393234Z +                "incompatible API version".into(),
2026-11-16T23:14:31.6393308Z +            ));
2026-11-16T23:14:31.6393376Z          }
2026-11-16T23:14:31.6393497Z          if self.extensions.contains_key(&manifest.id) {
2026-11-16T23:14:31.6393641Z              return Err(RuntimeError::DuplicateExtension(manifest.id));
2026-11-16T23:14:31.6393832Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:111:
2026-11-16T23:14:31.6393903Z  
2026-11-16T23:14:31.6394096Z      /// Probes registered extensions in stable ID order without changing registry state.
2026-11-16T23:14:31.6394230Z      pub fn health_report(&self) -> Vec<ExtensionHealth> {
2026-11-16T23:14:31.6394354Z -        self.extensions.iter().map(|(id, extension)| {
2026-11-16T23:14:31.6394719Z -            match extension.health_check() {
2026-11-16T23:14:31.6394918Z -                Ok(()) => ExtensionHealth { extension_id: id.clone(), healthy: true, error: None },
2026-11-16T23:14:31.6395178Z -                Err(error) => ExtensionHealth { extension_id: id.clone(), healthy: false, error: Some(error.to_string()) },
2026-11-16T23:14:31.6395253Z -            }
2026-11-16T23:14:31.6395324Z -        }).collect()
2026-11-16T23:14:31.6395406Z +        self.extensions
2026-11-16T23:14:31.6395473Z +            .iter()
2026-11-16T23:14:31.6395611Z +            .map(|(id, extension)| match extension.health_check() {
2026-11-16T23:14:31.6395709Z +                Ok(()) => ExtensionHealth {
2026-11-16T23:14:31.6395802Z +                    extension_id: id.clone(),
2026-11-16T23:14:31.6395889Z +                    healthy: true,
2026-11-16T23:14:31.6395966Z +                    error: None,
2026-11-16T23:14:31.6396044Z +                },
2026-11-16T23:14:31.6396150Z +                Err(error) => ExtensionHealth {
2026-11-16T23:14:31.6396240Z +                    extension_id: id.clone(),
2026-11-16T23:14:31.6396329Z +                    healthy: false,
2026-11-16T23:14:31.6396429Z +                    error: Some(error.to_string()),
2026-11-16T23:14:31.6396503Z +                },
2026-11-16T23:14:31.6396572Z +            })
2026-11-16T23:14:31.6396652Z +            .collect()
2026-11-16T23:14:31.6396720Z      }
2026-11-16T23:14:31.6396792Z  
2026-11-16T23:14:31.6396938Z      /// Returns stable, read-only metadata for registered extensions.
2026-11-16T23:14:31.6397144Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:126:
2026-11-16T23:14:31.6397215Z  
2026-11-16T23:14:31.6397359Z      /// Finds registered providers for an exact capability identifier.
2026-11-16T23:14:31.6397581Z      pub fn discover_capability(&self, capability: &CapabilityId) -> Vec<ExtensionDescriptor> {
2026-11-16T23:14:31.6397675Z -        self.descriptors.values()
2026-11-16T23:14:31.6397767Z +        self.descriptors
2026-11-16T23:14:31.6397840Z +            .values()
2026-11-16T23:14:31.6398002Z              .filter(|extension| extension.capabilities.contains(capability))
2026-11-16T23:14:31.6398076Z              .cloned()
2026-11-16T23:14:31.6398146Z              .collect()
2026-11-16T23:14:31.6398343Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:133:
2026-11-16T23:14:31.6398409Z      }
2026-11-16T23:14:31.6398479Z  
2026-11-16T23:14:31.6398571Z      /// Number of registered extensions.
2026-11-16T23:14:31.6398720Z -    pub fn extension_count(&self) -> usize { self.extensions.len() }
2026-11-16T23:14:31.6398823Z +    pub fn extension_count(&self) -> usize {
2026-11-16T23:14:31.6398909Z +        self.extensions.len()
2026-11-16T23:14:31.6398978Z +    }
2026-11-16T23:14:31.6399042Z  
2026-11-16T23:14:31.6399208Z      /// Dispatches a request only when the extension declared the capability
2026-11-16T23:14:31.6399407Z      /// and host policy permits the request.
2026-11-16T23:14:31.6399607Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:140:
2026-11-16T23:14:31.6399834Z      pub fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse, RuntimeError> {
2026-11-16T23:14:31.6399980Z -        self.execute_with_context(request, &ExecutionContext::new(
2026-11-16T23:14:31.6400156Z -            lmlmxtend_api::CancellationToken::new(), None, self.max_output_bytes
2026-11-16T23:14:31.6400226Z -        ))
2026-11-16T23:14:31.6400309Z +        self.execute_with_context(
2026-11-16T23:14:31.6400384Z +            request,
2026-11-16T23:14:31.6400467Z +            &ExecutionContext::new(
2026-11-16T23:14:31.6400579Z +                lmlmxtend_api::CancellationToken::new(),
2026-11-16T23:14:31.6400650Z +                None,
2026-11-16T23:14:31.6400734Z +                self.max_output_bytes,
2026-11-16T23:14:31.6400803Z +            ),
2026-11-16T23:14:31.6400872Z +        )
2026-11-16T23:14:31.6401024Z      }
2026-11-16T23:14:31.6401089Z  
2026-11-16T23:14:31.6401316Z      /// Dispatches with cooperative cancellation/deadline controls and a host-enforced output cap.
2026-11-16T23:14:31.6401550Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:152:
2026-11-16T23:14:31.6401658Z      ) -> Result<ExtensionResponse, RuntimeError> {
2026-11-16T23:14:31.6401789Z          context.check_cancelled().map_err(|error| match error {
2026-11-16T23:14:31.6401977Z              lmlmxtend_api::ExecutionControlError::Cancelled => RuntimeError::Cancelled,
2026-11-16T23:14:31.6402213Z -            lmlmxtend_api::ExecutionControlError::DeadlineExceeded => RuntimeError::DeadlineExceeded,
2026-11-16T23:14:31.6402357Z +            lmlmxtend_api::ExecutionControlError::DeadlineExceeded => {
2026-11-16T23:14:31.6402459Z +                RuntimeError::DeadlineExceeded
2026-11-16T23:14:31.6402534Z +            }
2026-11-16T23:14:31.6402744Z              lmlmxtend_api::ExecutionControlError::OutputTooLarge => RuntimeError::OutputTooLarge,
2026-11-16T23:14:31.6402822Z          })?;
2026-11-16T23:14:31.6402976Z          match self.policy.evaluate(&request.capability, &request.input) {
2026-11-16T23:14:31.6403163Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:161:
2026-11-16T23:14:31.6403272Z                  return Err(RuntimeError::PolicyDenied);
2026-11-16T23:14:31.6403344Z              }
2026-11-16T23:14:31.6403409Z          }
2026-11-16T23:14:31.6403539Z -        let owner = self.capabilities.get(&request.capability)
2026-11-16T23:14:31.6403621Z +        let owner = self
2026-11-16T23:14:31.6403699Z +            .capabilities
2026-11-16T23:14:31.6403790Z +            .get(&request.capability)
2026-11-16T23:14:31.6403994Z              .ok_or_else(|| RuntimeError::CapabilityNotDeclared(request.capability.to_string()))?;
2026-11-16T23:14:31.6404096Z -        let extension = self.extensions.get(owner)
2026-11-16T23:14:31.6404183Z +        let extension = self
2026-11-16T23:14:31.6404259Z +            .extensions
2026-11-16T23:14:31.6404330Z +            .get(owner)
2026-11-16T23:14:31.6404615Z              .ok_or_else(|| RuntimeError::ExtensionNotFound(owner.clone()))?;
2026-11-16T23:14:31.6404783Z          let bounded_context = context.limited_to_output(self.max_output_bytes);
2026-11-16T23:14:31.6404967Z          let response = match extension.execute_with_context(request, &bounded_context) {
2026-11-16T23:14:31.6405156Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:172:
2026-11-16T23:14:31.6405335Z                  // Prefer lifecycle errors when cooperative extension work observes them.
2026-11-16T23:14:31.6405477Z                  context.check_cancelled().map_err(|control| match control {
2026-11-16T23:14:31.6405672Z                      lmlmxtend_api::ExecutionControlError::Cancelled => RuntimeError::Cancelled,
2026-11-16T23:14:31.6405909Z -                    lmlmxtend_api::ExecutionControlError::DeadlineExceeded => RuntimeError::DeadlineExceeded,
2026-11-16T23:14:31.6406278Z -                    lmlmxtend_api::ExecutionControlError::OutputTooLarge => RuntimeError::OutputTooLarge,
2026-11-16T23:14:31.6406431Z +                    lmlmxtend_api::ExecutionControlError::DeadlineExceeded => {
2026-11-16T23:14:31.6406533Z +                        RuntimeError::DeadlineExceeded
2026-11-16T23:14:31.6406603Z +                    }
2026-11-16T23:14:31.6406744Z +                    lmlmxtend_api::ExecutionControlError::OutputTooLarge => {
2026-11-16T23:14:31.6406851Z +                        RuntimeError::OutputTooLarge
2026-11-16T23:14:31.6406922Z +                    }
2026-11-16T23:14:31.6406994Z                  })?;
2026-11-16T23:14:31.6407109Z                  return Err(RuntimeError::Extension(error));
2026-11-16T23:14:31.6407184Z              }
2026-11-16T23:14:31.6407382Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:180:
2026-11-16T23:14:31.6407451Z          };
2026-11-16T23:14:31.6407584Z          context.check_cancelled().map_err(|error| match error {
2026-11-16T23:14:31.6407867Z              lmlmxtend_api::ExecutionControlError::Cancelled => RuntimeError::Cancelled,
2026-11-16T23:14:31.6408097Z -            lmlmxtend_api::ExecutionControlError::DeadlineExceeded => RuntimeError::DeadlineExceeded,
2026-11-16T23:14:31.6408243Z +            lmlmxtend_api::ExecutionControlError::DeadlineExceeded => {
2026-11-16T23:14:31.6408343Z +                RuntimeError::DeadlineExceeded
2026-11-16T23:14:31.6408412Z +            }
2026-11-16T23:14:31.6408630Z              lmlmxtend_api::ExecutionControlError::OutputTooLarge => RuntimeError::OutputTooLarge,
2026-11-16T23:14:31.6408703Z          })?;
2026-11-16T23:14:31.6408889Z          if response.output.len() > self.max_output_bytes.min(context.max_output_bytes()) {
2026-11-16T23:14:31.6409079Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:213:
2026-11-16T23:14:31.6409149Z          }
2026-11-16T23:14:31.6409219Z      }
2026-11-16T23:14:31.6409293Z  
2026-11-16T23:14:31.6409357Z -
2026-11-16T23:14:31.6409432Z      struct Unhealthy;
2026-11-16T23:14:31.6409495Z  
2026-11-16T23:14:31.6409584Z      impl Extension for Unhealthy {
2026-11-16T23:14:31.6409769Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:249:
2026-11-16T23:14:31.6409859Z          assert_eq!(report[0].error, None);
2026-11-16T23:14:31.6409992Z          assert_eq!(report[1].extension_id, "example.unhealthy");
2026-11-16T23:14:31.6410076Z          assert!(!report[1].healthy);
2026-11-16T23:14:31.6410270Z -        assert_eq!(report[1].error.as_deref(), Some("extension execution failed: not ready"));
2026-11-16T23:14:31.6410341Z +        assert_eq!(
2026-11-16T23:14:31.6410429Z +            report[1].error.as_deref(),
2026-11-16T23:14:31.6410540Z +            Some("extension execution failed: not ready")
2026-11-16T23:14:31.6410606Z +        );
2026-11-16T23:14:31.6410671Z      }
2026-11-16T23:14:31.6410739Z  
2026-11-16T23:14:31.6410810Z      #[test]
2026-11-16T23:14:31.6410996Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:295:
2026-11-16T23:14:31.6411067Z                  }
2026-11-16T23:14:31.6411134Z              }
2026-11-16T23:14:31.6411200Z  
2026-11-16T23:14:31.6411423Z -            fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse, ExtensionError> {
2026-11-16T23:14:31.6411501Z +            fn execute(
2026-11-16T23:14:31.6411589Z +                &self,
2026-11-16T23:14:31.6411676Z +                request: ExtensionRequest,
2026-11-16T23:14:31.6411797Z +            ) -> Result<ExtensionResponse, ExtensionError> {
2026-11-16T23:14:31.6411912Z                  Ok(ExtensionResponse::text(request.input))
2026-11-16T23:14:31.6411980Z              }
2026-11-16T23:14:31.6412046Z          }
2026-11-16T23:14:31.6412229Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:306:
2026-11-16T23:14:31.6412371Z              Err(RuntimeError::DuplicateCapability("text.echo".into()))
2026-11-16T23:14:31.6412518Z          );
2026-11-16T23:14:31.6412690Z          // Failed registration is atomic: the rejected extension owns no capability.
2026-11-16T23:14:31.6412805Z -        assert!(runtime.execute(ExtensionRequest::new(
2026-11-16T23:14:31.6412912Z -            CapabilityId::new("text.echo").unwrap(),
2026-11-16T23:14:31.6412982Z -            "hello"
2026-11-16T23:14:31.6413050Z -        )).is_err());
2026-11-16T23:14:31.6413124Z +        assert!(runtime
2026-11-16T23:14:31.6413220Z +            .execute(ExtensionRequest::new(
2026-11-16T23:14:31.6413330Z +                CapabilityId::new("text.echo").unwrap(),
2026-11-16T23:14:31.6413401Z +                "hello"
2026-11-16T23:14:31.6413470Z +            ))
2026-11-16T23:14:31.6413538Z +            .is_err());
2026-11-16T23:14:31.6413605Z      }
2026-11-16T23:14:31.6413668Z  
2026-11-16T23:14:31.6413734Z -
2026-11-16T23:14:31.6413802Z      #[test]
2026-11-16T23:14:31.6413924Z      fn rejects_pre_cancelled_execution_before_dispatch() {
2026-11-16T23:14:31.6414139Z          let mut runtime = Runtime::new(RuntimeConfig::default());
2026-11-16T23:14:31.6414323Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:321:
2026-11-16T23:14:31.6414523Z          token.cancel();
2026-11-16T23:14:31.6414723Z          let context = ExecutionContext::new(token, None, usize::MAX);
2026-11-16T23:14:31.6414923Z          let request = ExtensionRequest::new(CapabilityId::new("text.echo").unwrap(), "hello");
2026-11-16T23:14:31.6415137Z -        assert_eq!(runtime.execute_with_context(request, &context), Err(RuntimeError::Cancelled));
2026-11-16T23:14:31.6415209Z +        assert_eq!(
2026-11-16T23:14:31.6415330Z +            runtime.execute_with_context(request, &context),
2026-11-16T23:14:31.6415421Z +            Err(RuntimeError::Cancelled)
2026-11-16T23:14:31.6415489Z +        );
2026-11-16T23:14:31.6415553Z      }
2026-11-16T23:14:31.6415618Z  
2026-11-16T23:14:31.6415691Z      #[test]
2026-11-16T23:14:31.6415875Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:330:
2026-11-16T23:14:31.6415981Z          runtime.register(Arc::new(Echo)).unwrap();
2026-11-16T23:14:31.6416178Z          let context = ExecutionContext::with_timeout(std::time::Duration::ZERO, usize::MAX);
2026-11-16T23:14:31.6416370Z          let request = ExtensionRequest::new(CapabilityId::new("text.echo").unwrap(), "hello");
2026-11-16T23:14:31.6416601Z -        assert_eq!(runtime.execute_with_context(request, &context), Err(RuntimeError::DeadlineExceeded));
2026-11-16T23:14:31.6416671Z +        assert_eq!(
2026-11-16T23:14:31.6416783Z +            runtime.execute_with_context(request, &context),
2026-11-16T23:14:31.6416883Z +            Err(RuntimeError::DeadlineExceeded)
2026-11-16T23:14:31.6416948Z +        );
2026-11-16T23:14:31.6417012Z      }
2026-11-16T23:14:31.6417078Z  
2026-11-16T23:14:31.6417147Z      #[test]
2026-11-16T23:14:31.6417328Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:337:
2026-11-16T23:14:31.6417424Z      fn enforces_host_output_limit() {
2026-11-16T23:14:31.6417653Z -        let config = RuntimeConfig { max_input_bytes: 32, max_output_bytes: 2, allowed_capabilities: None };
2026-11-16T23:14:31.6417743Z +        let config = RuntimeConfig {
2026-11-16T23:14:31.6417824Z +            max_input_bytes: 32,
2026-11-16T23:14:31.6417903Z +            max_output_bytes: 2,
2026-11-16T23:14:31.6417991Z +            allowed_capabilities: None,
2026-11-16T23:14:31.6418057Z +        };
2026-11-16T23:14:31.6418155Z          let mut runtime = Runtime::new(config);
2026-11-16T23:14:31.6418256Z          runtime.register(Arc::new(Echo)).unwrap();
2026-11-16T23:14:31.6418451Z          let request = ExtensionRequest::new(CapabilityId::new("text.echo").unwrap(), "hey");
2026-11-16T23:14:31.6418633Z Diff in /home/runner/work/Lmlmxtend/Lmlmxtend/crates/lmlmxtend-runtime/src/lib.rs:344:
2026-11-16T23:14:31.6418700Z  
2026-11-16T23:14:31.6418900Z      #[test]
2026-11-16T23:14:31.6419003Z      fn rejects_input_over_configured_limit() {
2026-11-16T23:14:31.6419247Z -        let config = RuntimeConfig { max_input_bytes: 3, max_output_bytes: 1_048_576, allowed_capabilities: None };
2026-11-16T23:14:31.6419329Z +        let config = RuntimeConfig {
2026-11-16T23:14:31.6419411Z +            max_input_bytes: 3,
2026-11-16T23:14:31.6419492Z +            max_output_bytes: 1_048_576,
2026-11-16T23:14:31.6419578Z +            allowed_capabilities: None,
2026-11-16T23:14:31.6419645Z +        };
2026-11-16T23:14:31.6419739Z          let mut runtime = Runtime::new(config);
2026-11-16T23:14:31.6419838Z          runtime.register(Arc::new(Echo)).unwrap();
2026-11-16T23:14:31.6420032Z          let request = ExtensionRequest::new(CapabilityId::new("text.echo").unwrap(), "long");
2026-11-16T23:14:31.6428684Z ##[error]Process completed with exit code 1.
