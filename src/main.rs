use vibe::agents::developer::DeveloperAgent;
use vibe::agents::Agent;

#[tokio::main]
async fn main() {
    let agent = DeveloperAgent::new();
    println!("DeveloperAgent created");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_developer_agent_creation() {
        let agent = DeveloperAgent::new();
        assert!(agent.name() == "DeveloperAgent");
    }
}
