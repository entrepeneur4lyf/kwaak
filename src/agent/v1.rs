use tavily::Tavily;
use uuid::Uuid;
use crate::commands::CommandResponder; // Correct import for CommandResponder

pub fn build_agent(uuid: Uuid, repository: &str, query: &str, responder: CommandResponder) -> Result<Tavily, Box<dyn std::error::Error + Send + Sync>> {
    let api_key = std::env::var("TAVILY_API_KEY")?;
    let tavily = Tavily::builder(&api_key).build()?;
    // Update logic if necessary for including the responder functionality
    Ok(tavily)
}

pub fn available_tools() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Tools Available");
    Ok(())
}
