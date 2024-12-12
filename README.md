# Kwaak: Terminal User Interface for AI-Assisted Code Management

Kwaak is a Terminal User Interface (TUI) application developed in Rust that integrates AI agents for managing and interacting with code projects. By leveraging AI-driven capabilities, Kwaak offers a unique platform for developers to engage with intelligent agents that can assist in writing code, testing, and even creating pull requests, all within a terminal environment.

## Features

- **AI Agent Interaction**: Run and chat with AI agents that can understand your codebase, generate solutions, and assist with various development tasks.
- **TUI Interface**: Provides a straightforward terminal-based interface for managing interactions.
- **Code Execution in Containers**: Each agent operates in a Docker container, allowing them to run tools and make changes confidently.
- **GitHub Integration**: Agents can interact with repositories, making it easy to integrate updates or create pull requests directly.

## Installation

Kwaak is primarily run as a binary. Docker is used internally by the agents to execute tools within isolated environments.

To build the project locally for development:

```bash
docker build -t kwaak .
```

## Configuration

Before running Kwaak, make sure to set up the environment with necessary API keys:

- **TAVILY_API_KEY**: For Tavily services.
- **KWAAK_OPENAI_API_KEY**: For AI integrations using OpenAI.
- **GITHUB_TOKEN**: To enable GitHub repository interactions.

Set these as environment variables:

```sh
export TAVILY_API_KEY="your-tavily-api-key"
export KWAAK_OPENAI_API_KEY="your-openai-api-key"
export GITHUB_TOKEN="your-github-token"
```

## Usage

Kwaak can be operated in different modes using command-line arguments:

- **Run the AI Agent**: Start AI agents to assist with code tasks.
  ```bash
  ./kwaak --mode run-agent
  ```

- **Launch the TUI**: Open the terminal interface to interact with agents and tasks.
  ```bash
  ./kwaak --mode tui
  ```

Command-line options like `--clear-cache` and `--print-config` are available to manage settings and cache efficiently.

## Contributing

Contributions are welcome! Please ensure code quality with linting and testing tools before submitting pull requests. Follow the contribution guidelines documented in the repository.

## License

Kwaak is open-source software licensed under the MIT License. See `LICENSE` for more details.

## Support

For additional support or to report issues, visit our GitHub repository or contact us through provided channels there.
