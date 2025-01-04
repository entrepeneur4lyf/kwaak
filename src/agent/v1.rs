use tavily::Tavily;

pub fn build_agent() -> Result<Tavily, Box<dyn std::error::Error>> {
    let api_key = std::env::var("TAVILY_API_KEY")?;
    let tavily = Tavily::builder(&api_key).build()?;
    Ok(tavily)
}

pub fn available_tools() {
    // Implementation for available_tools
    println!("Tools Available");
}

// Other V1 handler implementations and existing, relevant code...
