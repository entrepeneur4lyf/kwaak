# Kwaak: TUI for AI-Assisted Code Management

Kwaak is a Terminal User Interface (TUI) application built in Rust, designed for seamless interaction with AI agents to aid in code management and development tasks. It offers developers an innovative way to leverage AI in code generation, testing, and repository management directly within a terminal environment.

## Features

- **AI Agent Interaction**: Interact with AI agents that understand your codebase, provide solutions, and help in executing various development tasks.
- **Comprehensive TUI**: Navigate through different application modes and manage interactions using a simple keyboard-driven interface.
- **Command Execution in Containers**: Ensures isolated and safe execution of tasks by running all operations within Docker containers.
- **GitHub Integration**: Facilitates seamless interaction with GitHub repositories, including updates and pull request creation.

## Installation

Kwaak functions as a standalone binary and utilizes Docker to manage the execution of commands in secure environments.

To build the project locally for development:

```bash
docker build -t kwaak .
```

## Configuration

Kwaak requires a configuration file named `kwaak.toml` to specify settings for the application. This file should be placed in the root of your project directory.

Here's an example of what `kwaak.toml` might look like:

```toml
[api]
tavily_api_key = "your-tavily-api-key"
openai_api_key = "your-openai-api-key"

github_token = "your-github-token"

[repositories]
# Define paths, ignored files, or other specific settings

[agents]
# Configuration for agent behavior, timeouts, retries, etc.
```

Set this up to ensure Kwaak operates with the correct parameters for agent queries, repository interactions, and GitHub actions.

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
