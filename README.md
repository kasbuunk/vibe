# Vibe - A Simple Rust HTTP Server

A minimal HTTP server built with Rust and Axum, following Test-Driven Development (TDD) principles.

## Features

- Basic HTTP server with a single endpoint
- Test-driven development approach
- Proper error handling and testing
- Modern async/await Rust patterns

## Setup

1. Install Rust and Cargo if you haven't already:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository:
   ```bash
   git clone git@github.com:kasbuunk/vibe.git
   cd vibe
   ```

3. Run the server:
   ```bash
   cargo run
   ```

4. Test the server:
   ```bash
   curl http://localhost:8080
   ```

## Development

The project follows Test-Driven Development (TDD) practices:

1. Write a failing test
2. Make the test pass
3. Refactor if needed
4. Repeat

### Running Tests

```bash
cargo test
```

## Project Structure

- `src/main.rs`: Main application code with the HTTP server
- `src/tests`: Unit tests for the server
- `Cargo.toml`: Project dependencies and configuration

## License

MIT License
