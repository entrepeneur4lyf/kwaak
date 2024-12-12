# Kwaak: Your Autonomous AI Agent for Code Management

Kwaak is a cutting-edge software project written in Rust designed to automate and assist in managing code projects. By leveraging AI-driven capabilities, it provides an intelligent interface for developers, enhancing productivity and streamlining development workflows. With Kwaak, developers can efficiently automate repetitive tasks, integrate large language models, and manage code repositories seamlessly.

## Key Features

- **Autonomous AI Agent**: Runs intelligent agents that assist in code query and interaction, utilizing OpenAI models.
- **Rich TUI Interface**: Provides a terminal-based user interface for interaction using `ratatui`.
- **Comprehensive Code Indexing**: Efficiently indexes code repositories, enhancing the search and query process.
- **Seamless Integration**: Integrates with GitHub, Docker, and other tools for comprehensive project management.
- **Modern Rust Integration**: Utilizes the latest Rust libraries and frameworks for high performance and safety.
- **Task Automation**: Automates tasks like testing, coverage reporting, and cache management to enhance the development process.

## Installation

Kwaak can be set up in a containerized environment using Docker. A Dockerfile is provided to streamline the build process:

```bash
docker build -t kwaak .
```

To run Kwaak with Docker Compose for monitoring and tracing with Jaeger:

```bash
docker-compose up
```

## Configuration

Kwaak requires certain API keys and configuration settings which can be specified in a `kwaak.toml` file or as environment variables:

- **TAVILY_API_KEY**: The API key for Tavily services.
- **KWAAK_OPENAI_API_KEY**: The API key for OpenAI integrations.
- **GITHUB_TOKEN**: GitHub token for repository interactions.

Add these to your environment before running Kwaak:

```sh
export TAVILY_API_KEY="your-tavily-api-key"
export KWAAK_OPENAI_API_KEY="your-openai-api-key"
export GITHUB_TOKEN="your-github-token"
```

## Usage

Kwaak supports two primary modes of operation:

- **Agent Mode**: Utilize autonomous agents to query and interact with your codebase.
  ```bash
  cargo run -- --mode run-agent
  ```

- **TUI Mode**: Launch the terminal user interface to interact and manage tasks easily.
  ```bash
  cargo run -- --mode tui
  ```

Use command-line flags to manage configurations and caches, such as `--clear-cache` and `--print-config`.

## Contributing

We welcome contributions! Please adhere to the guidelines defined in the GitHub repository and maintain code quality using provided linting tools. Ensure all tests pass before submitting a pull request.

## License

Kwaak is licensed under the MIT license. See `LICENSE` for more details.

## Contact

For further questions, feel free to contact us via the repository's GitHub issues page or reach out directly at our support email listed on the GitHub page.
