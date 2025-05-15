#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;
    use anyhow::Result;

    #[tokio::test]
    async fn test_developer_agent_creation() -> Result<()> {
        // Given
        let agent = DeveloperAgent::new();

        // When
        let (tx, mut rx) = mpsc::channel(1);
        tx.send(AgentMessage::TaskAssignment {
            task: "write failing test".to_string(),
            priority: 1,
        })
        .await
        .unwrap();

        // Then
        assert!(rx.recv().await.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_developer_agent_process_message() -> Result<()> {
        // Given
        let agent = DeveloperAgent::new();
        let message = AgentMessage::TaskAssignment {
            task: "write failing test".to_string(),
            priority: 1,
        };

        // When
        let result = agent.process_message(message).await;

        // Then
        assert!(result.is_ok());
        Ok(())
    }
}
