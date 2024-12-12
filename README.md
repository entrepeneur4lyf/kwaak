# Kwaak: TUI for AI-Assisted Code Management

Kwaak is a Terminal User Interface (TUI) application built in Rust, designed for seamless interaction with AI agents to aid in code management and development tasks. It offers developers an innovative way to leverage AI in code generation, testing, and repository management directly within a terminal environment.

## Features

- **AI Agent Interaction**: Interact with AI agents that understand your codebase, provide solutions, and help in executing various development tasks.
- **Comprehensive TUI**: Navigate through different application modes and manage interactions using a simple keyboard-driven interface.
- **Command Execution in Containers**: Ensures isolated and safe execution of tasks by running all operations within Docker containers.
- **GitHub Integration**: Facilitates seamless interaction with GitHub repositories, including updates and pull request creation.

## Installation

Kwaak leverages Docker for running its agents within containers. Additionally, Docker Compose is used to start Jaeger for tracing purposes.

To set up the application:

1. **Run Agents in Docker:**
   The `Dockerfile` is used to create an environment for executing agents.
   ```bash
   docker build -t kwaak .
   ````

2. **Start Jaeger with Docker Compose:**
   The `compose.yml` file sets up Jaeger, a tracing system, for monitoring and debugging.
   ```bash
   docker-compose up jaeger
   ```

## Configuration

Kwaak requires a configuration file named `kwaak.toml` for its settings. This file should be placed in the root of your project directory.

Example `kwaak.toml`:

```toml
language = "rust"
tavily_api_key = "env:TAVILY_API_KEY"
tool_executor = "docker"

[commands]
test = "cargo test --no-fail-fast --color=never"
coverage = "cargo tarpaulin --skip-clean"
lint_and_fix = "cargo clippy --fix --allow-dirty --allow-staged && cargo fmt"

[github]
owner = "bosun-ai"
repository = "kwaak"
main_branch = "master"
token = "env:GITHUB_TOKEN"

[llm.indexing]
api_key = "env:KWAAK_OPENAI_API_KEY"
provider = "OpenAI"
prompt_model = "gpt-4o-mini"

[llm.query]
api_key = "env:KWAAK_OPENAI_API_KEY"
provider = "OpenAI"
prompt_model = "gpt-4o"

[llm.embedding]
api_key = "env:KWAAK_OPENAI_API_KEY"
provider = "OpenAI"
embedding_model = "text-embedding-3-large"

[docker]
dockerfile = "Dockerfile"
```

Set this file up to ensure Kwaak operates with the correct parameters for agent queries, command executions, and GitHub interactions.

## Usage

Kwaak offers multiple operation modes:

- **AI Agent Mode**: Directly engage with AI agents for code assistance.
  ```bash
  ./kwaak --mode run-agent
  ```

- **TUI Mode**: Launch the user interface for managing agent interactions and tasks.
  ```bash
  ./kwaak --mode tui
  ```

Additional command-line options include `--clear-cache` to clean up caches and `--print-config` to display the current configuration.

## Contribution

Contributions are welcome. Please ensure adherence to coding standards and comprehensive testing before submitting pull requests. Refer to the contribution guide in the repository for detailed information.

## License

Kwaak is distributed under the MIT License. See the `LICENSE` file for more information.

## Support

For support or to report issues, please visit our [GitHub repository](https://github.com/user/repo).
