# AI Agency Design Philosophy

## Core Principles

### 1. Technology Stack
- **Primary Languages**: Rust (with functional programming influences)
- **Cloud Native**: Embrace CNCF projects and cloud-native principles
- **FOSS Only**: Exclusively use free and open-source software

### 2. Architecture
- **Service Architecture**: gRPC for synchronous communication
- **Event Streaming**: NATS JetStream for asynchronous communication
- **Data Storage**: PostgreSQL for relational data, Redis for non-relational state
- **Observability**: OpenTelemetry with LGTM stack (Loki, Grafana, Tempo, Mimir)

### 3. Development Practices
- **TDD First**: Test-Driven Development for all components
- **Functional Approach**: Write Rust with Haskell influences
- **Domain Modeling**: Inspired by Scott Wlaschin's Domain Modeling Made Functional
- **Clean Architecture**: Clear separation of concerns

### 4. Technology Preferences
- **Preferred Languages**: Rust, Haskell (influences)
- **Rejected Technologies**:
  - Java ecosystem
  - PHP
  - C/C++
  - MongoDB

### 5. Observability
- **Tracing**: OpenTelemetry for distributed tracing
- **Metrics**: Prometheus-compatible metrics
- **Logging**: Structured logging with Loki
- **Monitoring**: Grafana for visualization

### 6. Data Handling
- **Serialization**: Protocol Buffers for data transfer
- **RPC**: gRPC for command/query operations
- **Events**: NATS JetStream for event streaming
- **State Management**: PostgreSQL for relational data, Redis for ephemeral state

### 7. Development Style
- **Functional Programming**: Embrace immutability and pure functions
- **Domain-Driven Design**: Clear domain boundaries and bounded contexts
- **Clean Code**: Maintainability and readability over premature optimization
- **Incremental Development**: Evolve architecture as needed

### 8. Infrastructure
- **Containerization**: Kubernetes for orchestration
- **CI/CD**: GitHub Actions for automation
- **Version Control**: Git with conventional commits
- **Documentation**: Comprehensive logging and development history

## Implementation Guidelines

1. **Start Small**: Begin with minimal viable components
2. **Evolve Gradually**: Add complexity incrementally
3. **Maintain Clean Code**: Prioritize readability and maintainability
4. **Document Decisions**: Keep detailed development logs
5. **Test Everything**: Write tests before implementation
6. **Monitor Everything**: Use OpenTelemetry for observability

## Technical Stack Details

### Language & Runtime
- Rust for primary implementation
- Functional programming patterns
- Strong type system
- Zero-cost abstractions

### Data Communication
- gRPC for synchronous operations
- NATS JetStream for event streaming
- Protocol Buffers for data serialization
- OpenTelemetry for tracing

### Data Storage
- PostgreSQL for relational data
- Redis for ephemeral state
- Single source of truth principle

### Observability
- OpenTelemetry SDK
- Loki for structured logging
- Tempo for distributed tracing
- Mimir for metrics
- Grafana for visualization

### Development Tools
- GitHub Actions for CI/CD
- Rust toolchain for development
- Protocol Buffers for code generation
- OpenTelemetry instrumentation

This philosophy will guide our development process and ensure consistency across the AI Agency implementation.
