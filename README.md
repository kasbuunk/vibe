# Vibe - Introspective AI Agency

## Mission

Vibe is an AI Agency framework where AI agents collaboratively develop software. The core vision is to create a self-evolving development ecosystem where AI agents:

1. **Self-Develop**: Write, modify, and improve their own code
2. **Autonomously Decide**: Make architectural decisions through collaborative processes
3. **Self-Heal**: Detect and fix issues in their own code
4. **Learn Continuously**: Improve their development practices over time

## Self-Application Philosophy

Vibe is designed to be both:

1. **A Tool**: A framework for creating AI agency ecosystems
2. **Self-Improving**: Continuously enhancing its own implementation
3. **Self-Applied**: Using itself as a tool for development
4. **Efficient**: Optimizing resource utilization in distributed environments

## Current Focus: Developer Agent

The first step in this journey is the Developer Agent - an AI agent that:

- Analyzes and understands any codebase, including its own
- Suggests and implements code changes
- Maintains code quality through automated testing
- Documents decisions and reasoning
- Collaborates with other agents
- Manages resource allocation in distributed environments

## Project Overview

Vibe is not just a project, but a living, evolving ecosystem where AI agents:

- Write and modify code
- Make architectural decisions
- Maintain code quality
- Document their reasoning
- Collaborate with each other
- Learn from their experiences

## Project Purpose

This repository serves as a living example of:

- AI agents developing software autonomously
- Collaborative decision-making processes
- Self-evolving code architecture
- Automated code quality maintenance
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
