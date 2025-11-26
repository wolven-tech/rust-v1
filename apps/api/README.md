# V1 API

Rust-based REST API service built with Axum, following Clean Architecture principles.

## Architecture

This service implements Clean Architecture with the following layers:

```
┌─────────────────────────────────────────┐
│            PRESENTATION                 │ <- HTTP Handlers, DTOs
├─────────────────────────────────────────┤
│            APPLICATION                  │ <- Services, Use Cases
├─────────────────────────────────────────┤
│              DOMAIN                     │ <- Business Logic (future)
├─────────────────────────────────────────┤
│          INFRASTRUCTURE                 │ <- External Services, State
└─────────────────────────────────────────┘
```

### Features

- ✅ **Clean Architecture** - Separation of concerns with clear boundaries
- ✅ **OpenAPI Documentation** - Auto-generated API docs via Scalar UI
- ✅ **Hot Reload** - Fast development with bacon
- ✅ **Structured Logging** - JSON logs in production, pretty logs in dev
- ✅ **Error Handling** - Type-safe error handling with thiserror
- ✅ **CORS Support** - Permissive CORS for development
- ✅ **Health Checks** - `/health` endpoint for monitoring
- 🚧 **gRPC Support** - Optional microservice communication (future)
- 🚧 **Metrics** - Prometheus metrics (future)

## Endpoints

### General
- `GET /` - Root endpoint with API info
- `GET /health` - Health check endpoint
- `GET /api/docs` - OpenAPI documentation (Scalar UI)

### Subscription
- `POST /api/subscribe` - Newsletter subscription via Loops API
  - Request: `{ "email": "user@example.com", "userGroup": "newsletter" }`
  - Response: `{ "success": true, "message": "...", "id": "..." }`

## Development

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Bacon (install: `cargo install bacon`)

### Setup

1. Copy environment variables:
```bash
cp .env.example .env
# Edit .env with your Loops API credentials
```

2. Install dependencies:
```bash
cargo build
```

### Running

**Option 1: Using bacon (recommended for development)**
```bash
bacon run-long
```

This will:
- Watch for file changes
- Auto-restart on changes
- Show compilation errors in real-time

**Option 2: Using cargo**
```bash
cargo run
```

**Option 3: Via meta orchestrator (recommended)**
```bash
meta dev -p api
```

### Testing

Run tests:
```bash
cargo test
```

Run tests with bacon:
```bash
bacon test
```

### Linting

```bash
cargo clippy
```

Or with bacon:
```bash
bacon clippy
```

## Project Structure

```
apps/api/
├── src/
│   ├── application/        # Application layer
│   │   └── services/       # Business logic services
│   ├── domain/             # Domain layer (future)
│   ├── infrastructure/     # Infrastructure layer
│   │   └── state.rs        # Application state
│   ├── presentation/       # Presentation layer
│   │   ├── dto/            # Data Transfer Objects
│   │   └── handlers/       # HTTP request handlers
│   ├── config.rs           # Configuration management
│   ├── error.rs            # Error types
│   ├── lib.rs              # Library entry point
│   └── main.rs             # Binary entry point
├── proto/                  # gRPC protocol definitions (future)
├── tests/                  # Integration tests
├── bacon.toml              # Bacon configuration
├── Cargo.toml              # Rust dependencies
└── README.md               # This file
```

## Configuration

Environment variables (see `.env.example`):

| Variable | Description | Default |
|----------|-------------|---------|
| `PORT` | Server port | `3002` |
| `HOST` | Server host | `0.0.0.0` |
| `ENVIRONMENT` | Environment (`development`/`production`) | `development` |
| `RUST_LOG` | Log level | `api=debug,tower_http=debug` |
| `NEXT_PUBLIC_LOOPS_FORM_ID` | Loops API form ID | - |
| `NEXT_PUBLIC_LOOPS_API_KEY` | Loops API key | - |

## Adding New Features

Follow the Clean Architecture workflow:

1. **Domain Layer** - Define entities and business rules
2. **Application Layer** - Create service with use case logic
3. **Infrastructure Layer** - Implement external integrations
4. **Presentation Layer** - Add HTTP handlers and DTOs

See [Development Guide](../../docs/rust/DEVELOPMENT_GUIDE.md) for detailed templates.

## Future Enhancements

- [ ] gRPC microservice support (protocol buffers)
- [ ] Database integration (PostgreSQL with sqlx)
- [ ] Metrics and observability (Prometheus, OpenTelemetry)
- [ ] Authentication/Authorization (JWT)
- [ ] Rate limiting
- [ ] Caching layer
- [ ] CI/CD pipeline
- [ ] Docker containerization

## License

MIT
