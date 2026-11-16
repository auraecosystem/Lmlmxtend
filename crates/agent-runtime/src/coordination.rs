//! Shared state and bounded in-process messaging for one agent execution plan.

use crate::AgentError;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::{Arc, Mutex};

pub const MAX_SHARED_STATE_ENTRIES: usize = 256;
pub const MAX_SHARED_VALUE_BYTES: usize = 64 * 1024;
pub const MAX_MESSAGE_BYTES: usize = 64 * 1024;
pub const MAX_QUEUED_MESSAGES: usize = 1024;

#[derive(Clone, Default)]
pub struct AgentSharedState {
    values: Arc<Mutex<BTreeMap<String, String>>>,
}

impl AgentSharedState {
    /// Store a UTF-8 value under a bounded key. State is shared by agents in
    /// the same root execution and is discarded when that execution ends.
    pub fn set(&self, key: impl Into<String>, value: impl Into<String>) -> Result<(), AgentError> {
        let key = key.into();
        let value = value.into();
        if key.trim().is_empty()
            || key.len() > 256
            || value.len() > MAX_SHARED_VALUE_BYTES
        {
            return Err(AgentError::SharedStateLimitExceeded);
        }

        let mut values = self.values.lock().map_err(|_| {
            AgentError::ExecutionFailed("shared task state lock poisoned".into())
        })?;
        if !values.contains_key(&key) && values.len() >= MAX_SHARED_STATE_ENTRIES {
            return Err(AgentError::SharedStateLimitExceeded);
        }
        values.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, AgentError> {
        let values = self.values.lock().map_err(|_| {
            AgentError::ExecutionFailed("shared task state lock poisoned".into())
        })?;
        Ok(values.get(key).cloned())
    }

    pub fn snapshot(&self) -> Result<BTreeMap<String, String>, AgentError> {
        let values = self.values.lock().map_err(|_| {
            AgentError::ExecutionFailed("shared task state lock poisoned".into())
        })?;
        Ok(values.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentMessage {
    pub from_agent_id: String,
    pub to_agent_id: String,
    pub task_id: String,
    pub correlation_id: Option<String>,
    pub payload: String,
}

#[derive(Default)]
struct MessageQueues {
    by_agent: BTreeMap<String, VecDeque<AgentMessage>>,
    queued: usize,
}

#[derive(Clone)]
pub struct AgentMessageBus {
    known_agents: Arc<BTreeSet<String>>,
    queues: Arc<Mutex<MessageQueues>>,
}

impl AgentMessageBus {
    pub(crate) fn new(known_agents: BTreeSet<String>) -> Self {
        Self {
            known_agents: Arc::new(known_agents),
            queues: Arc::new(Mutex::new(MessageQueues::default())),
        }
    }

    pub(crate) fn send(&self, message: AgentMessage) -> Result<(), AgentError> {
        if !self.known_agents.contains(&message.to_agent_id) {
            return Err(AgentError::MessageTargetNotFound(
                message.to_agent_id.clone(),
            ));
        }
        if message.payload.len() > MAX_MESSAGE_BYTES {
            return Err(AgentError::MessageTooLarge);
        }
        let mut queues = self.queues.lock().map_err(|_| {
            AgentError::ExecutionFailed("agent message queue lock poisoned".into())
        })?;
        if queues.queued >= MAX_QUEUED_MESSAGES {
            return Err(AgentError::MessageQueueFull);
        }
        queues
            .by_agent
            .entry(message.to_agent_id.clone())
            .or_default()
            .push_back(message);
        queues.queued += 1;
        Ok(())
    }

    pub(crate) fn receive(&self, agent_id: &str) -> Result<Vec<AgentMessage>, AgentError> {
        let mut queues = self.queues.lock().map_err(|_| {
            AgentError::ExecutionFailed("agent message queue lock poisoned".into())
        })?;
        let messages = queues.by_agent.remove(agent_id).unwrap_or_default();
        queues.queued = queues.queued.saturating_sub(messages.len());
        Ok(messages.into_iter().collect())
    }
}

#[derive(Clone)]
pub(crate) struct AgentCoordination {
    pub shared_state: AgentSharedState,
    pub message_bus: AgentMessageBus,
}

impl AgentCoordination {
    pub(crate) fn new(known_agents: BTreeSet<String>) -> Self {
        Self {
            shared_state: AgentSharedState::default(),
            message_bus: AgentMessageBus::new(known_agents),
        }
    }
}
