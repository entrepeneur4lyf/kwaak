use tavily::Tavily;
use uuid::Uuid;
use crate::commands::CommandResponder; // Correct import for CommandResponder

pub fn build_agent(uuid: Uuid, repository: &str, query: &str, responder: CommandResponder) -> Result<Tavily, Box<dyn std::error::Error>> {
    let api_key = std::env::var("TAVILY_API_KEY")?;
    let tavily = Tavily::builder(&api_key).build()?;
    // Update logic if necessary for including the responder functionality
    Ok(tavily)
}

pub fn available_tools() -> Result<(), Box<dyn std::error::Error>>{
    println!("Tools Available");
    Ok(())
}

// Ensure function logic and parameter handling aligns with revised APIs and patterns.
// The CommandResponder is now explicitly included as intended. Apply necessary updates as testing proceeds.
