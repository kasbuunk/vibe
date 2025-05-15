#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use std::time::Duration;
    use anyhow::Result;

    #[tokio::test]
    async fn test_cluster_lifecycle() -> Result<()> {
        // Given
        let cluster = KubernetesCluster::new().await?;

        // When
        let result = cluster.shutdown().await;

        // Then
        assert!(result.is_ok(), "Cluster shutdown failed");
        Ok(())
    }

    #[tokio::test]
    async fn test_cluster_readiness() -> Result<()> {
        // Given
        let cluster = KubernetesCluster::new().await?;
        let timeout = Duration::from_secs(30);

        // When
        let result = cluster.wait_for_ready(timeout).await;

        // Then
        assert!(result.is_ok(), "Cluster readiness check failed");
        cluster.shutdown().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_namespace_operations() -> Result<()> {
        // Given
        let cluster = KubernetesCluster::new().await?;
        let namespace = "test-namespace";

        // When
        let create_result = cluster.create_namespace(namespace).await;
        let delete_result = cluster.delete_namespace(namespace).await;

        // Then
        assert!(create_result.is_ok(), "Namespace creation failed");
        assert!(delete_result.is_ok(), "Namespace deletion failed");
        cluster.shutdown().await?;
        Ok(())
    }
}
