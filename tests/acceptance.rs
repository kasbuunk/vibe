#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::path::Path;
    use std::fs;
    use vibe::agents::AgentMessage;
    use vibe::agents::developer::DeveloperAgent;

    #[tokio::test]
    async fn test_developer_agent_writes_test() -> Result<()> {
        // Given
        let test_name = "test_addition";
        let test_file = Path::new("tests/developer_agent_test.rs");
        let agent = DeveloperAgent::new();
        
        // Clean up any previous test file
        if test_file.exists() {
            fs::remove_file(test_file)?;
        }

        // When
        agent.write_failing_test(test_name).await?;
        
        // Then
        // Verify the test file exists
        assert!(test_file.exists(), "Test file was not created");
        
        // Verify the test file contains a basic test
        let content = fs::read_to_string(test_file)?;
        assert!(content.contains("#[test]"), "Test file does not contain a test function");
        assert!(content.contains("assert_eq!"), "Test file does not contain an assertion");
        
        // Clean up
        fs::remove_file(test_file)?;
        
        Ok(())
    }
}
