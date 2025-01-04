use tavily::Tavily;
use uuid::Uuid;

pub fn build_agent(uuid: Uuid, repository: &str, query: &str, responder: Responder) -> Result<Tavily, Box<dyn std::error::Error>> {
    let api_key = std::env::var("TAVILY_API_KEY")?;
    let tavily = Tavily::builder(&api_key).build()?;
    Ok(tavily)
}

pub fn available_tools() -> Result<(), Box<dyn std::error::Error>>{
    println!("Tools Available");
    Ok(())
}

// Correct function signatures and arguments based on recent changes to align with caller expectations.
// Implement actual logic, if needed, to fulfill the intended operations.
