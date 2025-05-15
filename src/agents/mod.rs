pub mod developer;

use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

/// Base message type for agent communication
#[derive(Debug, Serialize, Deserialize)]
pub enum AgentMessage {
    TaskAssignment { task: String, priority: u8 },
    StatusUpdate { progress: f32, message: String },
    DecisionRequest { question: String, options: Vec<String> },
    KnowledgeShare { topic: String, content: String },
}

/// Base trait for all agents
#[async_trait]
pub trait Agent {
    fn name(&self) -> &'static str;
    fn role(&self) -> &'static str;
    async fn process_message(&self, message: AgentMessage) -> Result<(), anyhow::Error>;
}
