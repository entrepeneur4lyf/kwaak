# Kwaak

Kwaak is a Rust-based Terminal User Interface (TUI) designed for developers to engage interactively with AI agents in managing and querying their codebase.

## Key Features

- **AI Agents:** Automate code management tasks using integrated AI agents.
- **TUI Chat Interface:** Engage with agents through a seamless chat interface right in your terminal.
- **Code Indexing:** Efficiently index your codebase and persist metadata for enhanced accessibility.
- **Docker Integration:** Executes actions in an isolated Docker environment for consistent results.

## Main Components

- **DockerExecutor:** Manages execution within Docker containers to ensure environment consistency.
- **EnvSetup:** Maintains the necessary environment configurations for agent operations.
- **App:** The core TUI application interface, handling commands and interaction modes (Chat/Logs).
- **Config:** Handles configuration parsing and storage including cache and log directories.

## Architecture

- **Modular Design:** Composed of independent Rust modules focusing on distinct functionalities like configuration, execution, and user interactions.
- **Docker Utilization:** Employs Docker for an isolated execution environment. A custom image is built based on project-specific configurations.

## Usage

1. **Setup Environment Variables:** Ensure `OPENAI_API_KEY` and `GITHUB_TOKEN` are available in your environment.
2. **Configuration:** Customize `kwaak.toml` in your project directory to configure details like language and LLM settings.
3. **Run:** Start using Kwaak through `cargo run` and begin interacting with AI agents.

## Available Commands

- **/chat &lt;message&gt;:** Interact with the AI querying or managing your codebase.
- **/index_repository:** Initiate repository indexing for metadata enrichment.
- **/show_config:** View current configuration settings.
- **/quit:** Exit the Kwaak application.

## Contribution

1. **Clone the Repo:** Start working by cloning the repository.
2. **Feature Branch:** Develop new features or fix bugs on a separate branch.
3. **Pull Request:** Once finalized, submit a pull request with detailed information on your changes.

## License

Licensed under the MIT License. See the LICENSE file for details.

---

For support or to report issues, reach out to project maintainers. Contributions to improve Kwaak are always welcome!
