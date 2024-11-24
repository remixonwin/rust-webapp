# Development Guide

## Development Environment Setup

### Prerequisites

- Rust (latest stable version)
- Git
- Visual Studio Code (recommended) or your preferred IDE
- Docker (optional, for containerized development)

### Getting Started

1. **Clone the Repository**
```bash
git clone [your-repo-url]
cd rust-webapp
```

2. **Install Rust Dependencies**
```bash
cargo build
```

3. **Set Up Configuration**
```bash
cp config/app.config.example.toml config/app.config.toml
# Edit app.config.toml with your local settings
```

4. **Run the Development Server**
```bash
cargo run
```

### Project Structure

```
rust-webapp/
├── assets/              # Development assets
├── config/             # Configuration files
├── docker/             # Docker-related files
├── docs/              # Documentation
├── scripts/           # PowerShell scripts
├── src/               # Source code
├── static/           # Static assets
├── tests/           # Test files
└── tools/          # Development tools
```

### Development Workflow

1. **Creating New Features**
   - Create a new branch for your feature
   - Write tests first (TDD approach)
   - Implement the feature
   - Run tests and ensure they pass
   - Submit a pull request

2. **Running Tests**
```bash
# Run all tests
cargo test

# Run specific tests
cargo test test_name

# Run tests with logging
RUST_LOG=debug cargo test
```

3. **Code Style and Formatting**
- Use `rustfmt` for consistent code formatting:
```bash
cargo fmt
```
- Run clippy for linting:
```bash
cargo clippy
```

### API Development

1. **Adding New Endpoints**
   - Add route in `src/routes/mod.rs`
   - Create handler in `src/handlers/`
   - Add models in `src/models/` if needed
   - Add tests in `tests/endpoints/`

2. **Testing Endpoints**
   - Use Postman or curl for manual testing
   - Write integration tests in the `tests/` directory

### Docker Development

1. **Building the Container**
```bash
docker-compose build
```

2. **Running the Container**
```bash
docker-compose up
```

3. **Running Tests in Container**
```bash
docker-compose run app cargo test
```

### Common Development Tasks

1. **Adding Dependencies**
   - Add to `Cargo.toml`
   - Run `cargo build` to update `Cargo.lock`

2. **Database Migrations**
   - Add migration scripts in `migrations/`
   - Use diesel CLI for running migrations

3. **Static Files**
   - Add to appropriate directory in `static/`
   - Update asset references in templates

### Troubleshooting

1. **Common Issues**
   - Port already in use: Check for running instances
   - Build errors: Run `cargo clean` and rebuild
   - Test failures: Check logs with `RUST_LOG=debug`

2. **Logging**
   - Set `RUST_LOG` environment variable
   - Check application logs
   - Use debug logging in development

### Best Practices

1. **Code Organization**
   - Follow Rust naming conventions
   - Keep functions small and focused
   - Document public APIs
   - Use proper error handling

2. **Testing**
   - Write unit tests for all new functions
   - Add integration tests for endpoints
   - Test error cases
   - Use test fixtures when appropriate

3. **Security**
   - Never commit secrets or credentials
   - Use environment variables for sensitive data
   - Follow OWASP security guidelines
   - Regularly update dependencies

### Deployment

1. **Staging Deployment**
   - Use staging configuration
   - Run full test suite
   - Verify all features

2. **Production Deployment**
   - Follow deployment checklist
   - Use production configuration
   - Monitor logs and metrics

### Resources

- [Rust Documentation](https://doc.rust-lang.org/book/)
- [Actix-web Documentation](https://actix.rs/docs/)
- [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
