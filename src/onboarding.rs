use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

// Function to handle the onboarding process
pub fn init_onboarding() {
    println!("Welcome to Kwaak initialization!");
    println!("Let's set up your kwaak.toml configuration file.");

    // Collect inputs from the user
    let language = prompt(
        "Enter the programming language (default is 'rust'):",
        "rust",
    );
    let tavily_api_key = prompt(
        "Enter your Tavily API key (or leave blank to use environment variable 'TAVILY_API_KEY'):",
        "env:TAVILY_API_KEY",
    );
    let tool_executor = prompt("Enter the tool executor (default is 'docker'):", "docker");
    let github_owner = prompt("Enter your GitHub owner/organization:", "bosun-ai");
    let github_repo = prompt("Enter your GitHub repository name:", "kwaak");
    let github_token = prompt(
        "Enter your GitHub token (or leave blank to use environment variable 'GITHUB_TOKEN'):",
        "env:GITHUB_TOKEN",
    );

    let openai_api_key = prompt("Enter your OpenAI API key (or leave blank to use environment variable 'KWAAK_OPENAI_API_KEY'):", "env:KWAAK_OPENAI_API_KEY");
    let dockerfile = prompt(
        "Enter your Dockerfile path (default is 'Dockerfile'):",
        "Dockerfile",
    );

    // Create the kwaak.toml content
    let config_content = format!(
        "language = \"{language}\"
        tavily_api_key = \"{tavily_api_key}\"
        tool_executor = \"{tool_executor}\"

        [commands]
        test = \"cargo test --no-fail-fast --color=never\"
        coverage = \"cargo tarpaulin --skip-clean\"
        lint_and_fix = \"cargo clippy --fix --allow-dirty --allow-staged && cargo fmt\"

        [github]
        owner = \"{github_owner}\"
        repository = \"{github_repo}\"
        main_branch = \"master\"
        token = \"{github_token}\"

        [llm.indexing]
        api_key = \"{openai_api_key}\"
        provider = \"OpenAI\"
        prompt_model = \"gpt-4o-mini\"

        [llm.query]
        api_key = \"{openai_api_key}\"
        provider = \"OpenAI\"
        prompt_model = \"gpt-4o\"

        [llm.embedding]
        api_key = \"{openai_api_key}\"
        provider = \"OpenAI\"
        embedding_model = \"text-embedding-3-large\"

        [docker]
        dockerfile = \"{dockerfile}\"
        "
    );

    // Write the content to kwaak.toml
    match write_config_file("kwaak.toml", &config_content) {
        Ok(()) => println!("Successfully created kwaak.toml."),
        Err(e) => println!("Error writing kwaak.toml: {e}"),
    }
}

// Helper function to prompt user for input
fn prompt(message: &str, default: &str) -> String {
    println!("{message} [{default}]");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    if input.is_empty() {
        default.to_string()
    } else {
        input.to_string()
    }
}

// Function to write the configuration file
fn write_config_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())
}
