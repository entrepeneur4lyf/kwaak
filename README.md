# Kwaak

Kwaak is a versatile tool designed to execute autonomous agents on your code repositories. It offers an array of features spanning code indexing, interactive terminal interfaces, and robust integrations with various tools and platforms like Docker and OpenAI.

## Features

- **Autonomous Agents**: Run intelligent agents on your code to automate various tasks.
- **Code Indexing**: Efficiently index and query code repositories for faster operations.
- **Interactive UI**: Engage with an interactive terminal-based UI for enhanced user experiences.
- **Docker Integration**: Seamlessly interact with Docker for managing isolated environments.
- **OpenAI Integration**: Leverage AI capabilities through OpenAI's powerful models.

## Installation

Ensure you have Rust installed. Clone the repository and navigate to the project directory.

```sh
git clone https://github.com/bosun-ai/kwaak.git
cd kwaak
```

You can build the project using Cargo:

```sh
cargo build --release
```

For Docker users, simply use the Dockerfile provided to build an image.

## Usage

Execute the main program to interact with the agents or the terminal UI.

```sh
cargo run -- --config your_config_file.toml
```

Refer to `compose.yml` for Docker-compose based usage, including setting up services like Jaeger for tracing.

## Configuration

The configuration file (`kwaak.toml`) is crucial for setting up API keys, Docker settings, and LLM configurations. Ensure you populate the necessary environment variables for API keys and tokens.

## Contributing

Contributions are welcome! Please fork the repository and submit pull requests. Ensure that your changes include relevant tests and documentation updates.

## License

Licensed under the MIT License.

## Acknowledgements

This project integrates with several libraries and platforms including:
- OpenAI for AI functionalities.
- Docker for environment management.
- Tokio for asynchronous runtime.
- The Rust ecosystem for providing essential libraries and tools.
