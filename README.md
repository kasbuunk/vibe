# Vibe - A Journey into Code Flow

This project is a living exploration of code flow and development rhythm. It's not just about building an HTTP server - it's about learning to "vibe" with code, working in harmony with tools and processes to create maintainable, well-documented software.

## Project Purpose

This repository serves as a living example of:

- The development process itself
- How we work with code
- The principles that guide our decisions
- The journey of learning and adaptation

The HTTP server is simply a vehicle for this exploration. The real value lies in the development log, principles, and documentation that capture our journey together.

## Features

- Basic HTTP server with a single endpoint
- Test-driven development approach
- Proper error handling and testing
- Modern async/await Rust patterns

## Setup

1. Install prerequisites:
   ```bash
   # Install GitHub CLI (using nix)
   nix-env -iA nixpkgs.gh

   # Authenticate GitHub CLI
   gh auth login

   # Install Rust and Cargo
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
