# WebSocket Chat in Rust

Welcome to the WebSocket Chat project! This project is a simple chat application built using Rust and WebSockets.

## Features

- Real-time messaging
- Multiple clients support
- Lightweight and efficient

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/VerifiedFemboy/websocket-chat.git
    cd websocket-chat
    ```

2. Build the project:
    ```sh
    cargo build
    ```

## Usage

1. Run the server:
    ```sh
    cd server
    cargo run
    ```

2. Run the terminal chat client:
    ```sh
    cd terminal-chat
    cargo run
    ```

3. Connect to the server using a WebSocket client (e.g., a web browser or a dedicated WebSocket client).

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License.

## Contact

For any questions or suggestions, please contact the project maintainer.

## Development Status

This project is currently in development. The `server` directory contains the backend WebSocket server, while the `terminal-chat` directory contains the client for WebSocket communication.