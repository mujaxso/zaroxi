# Qyzer Studio: AI-First IDE

> ⚠️ **Heavily Under Development**: Qyzer Studio is currently in active development. APIs, features, and architecture are subject to change. We welcome early adopters and contributors to help shape the project!

[![CI](https://github.com/mujaxso/qyzer-studio/actions/workflows/ci.yml/badge.svg)](https://github.com/mujaxso/qyzer-studio/actions/workflows/ci.yml)
[![Security Audit](https://github.com/mujaxso/qyzer-studio/actions/workflows/security-audit.yml/badge.svg)](https://github.com/mujaxso/qyzer-studio/actions/workflows/security-audit.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

Qyzer Studio is an open-source, AI-first integrated development environment built in Rust. It combines modern IDE features with AI-powered assistance to create a next-generation development experience.

## 🚀 Features

- **AI-Powered Development**: Built-in AI assistance for code completion, refactoring, debugging, and documentation
- **Modular Architecture**: Clean separation of concerns with a workspace-based crate structure
- **Cross-Platform**: Native support for Linux, macOS, and Windows
- **Extensible**: Plugin system for languages, themes, and AI providers
- **Performance-Focused**: Built in Rust for speed and reliability
- **Security-First**: Built-in permission system and security best practices

## 🏗️ Architecture

Qyzer Studio follows a modular architecture with clear separation between UI, business logic, and AI capabilities:

```
┌─────────────────┐    ┌──────────────────┐
│   Desktop App   │◄──►│  Workspace Model │
└─────────────────┘    └──────────────────┘
         │                        │
         ▼                        ▼
┌─────────────────┐    ┌──────────────────┐
│   UI Modules    │    │   LSP Client     │
└─────────────────┘    └──────────────────┘
         │                        │
         ▼                        ▼
┌─────────────────┐    ┌──────────────────┐
│   AI Context    │◄──►│     AI Agent     │
└─────────────────┘    └──────────────────┘
         │                        │
         ▼                        ▼
┌─────────────────┐    ┌──────────────────┐
│   AI Daemon     │    │  RPC Framework   │
└─────────────────┘    └──────────────────┘
```

## 📦 Project Structure

```
qyzer-studio/
├── apps/                    # Applications
│   └── desktop/            # Main desktop application
├── crates/                 # Core libraries
│   ├── core-types/         # Shared data structures
│   ├── editor-core/        # Text editing primitives
│   ├── workspace-model/    # Workspace state management
│   ├── lsp-client/         # Language Server Protocol client
│   ├── ai-context/         # AI context management
│   ├── ai-agent/           # AI task orchestration
│   ├── patch-engine/       # Diff generation and application
│   ├── rpc/                # Remote Procedure Call framework
│   ├── settings/           # Configuration management
│   └── permissions/        # Access control and security
├── services/               # Background services
│   ├── workspace-daemon/   # Workspace management service
│   └── ai-daemon/          # AI operations service
├── extensions/             # Language extensions
├── docs/                   # Documentation
└── tests/                  # Test suites
```

## 🛠️ Getting Started

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/mujaxso/neote.git
cd neote

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Check formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --workspace --all-targets -- -D warnings
```

### Running the Desktop Application

```bash
# Build and run the desktop app
cargo run -p desktop -- /path/to/workspace
```

## 📚 Documentation

Comprehensive documentation is available in the `docs/` directory:

- [Architecture](docs/architecture.md) - High-level system design
- [Crates](docs/crates.md) - Detailed crate documentation
- [RPC Framework](docs/rpc.md) - Communication protocol documentation
- [Security](docs/security.md) - Security architecture and practices
- [Roadmap](docs/roadmap.md) - Development roadmap and future plans

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

- Follow Rust formatting with `cargo fmt`
- Use `cargo clippy` for linting
- Write tests for new functionality
- Document public APIs with Rustdoc comments

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Run integration tests
cargo test --test integration

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --workspace --ignore-tests
```

## 🔒 Security

Security is a top priority for Qyzer Studio. Please review our [Security Documentation](docs/security.md) for details on:

- Threat model and security principles
- Authentication and authorization
- Data protection and encryption
- AI safety measures
- Vulnerability reporting process

To report a security vulnerability, please email security@qyzer-studio.dev (encrypted communication preferred).

## 📄 License

Qyzer Studio is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- The Rust community for excellent tooling and libraries
- All contributors who help make Qyzer Studio better
- Inspired by modern IDEs and AI-assisted development tools

## 📞 Contact

- **GitHub Issues**: [Bug reports and feature requests](https://github.com/mujaxso/qyzer-studio/issues)
- **Discussions**: [Community discussions](https://github.com/mujaxso/qyzer-studio/discussions)
- **Email**: contact@qyzer-studio.dev

## 🌟 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=mujaxso/qyzer-studio&type=Date)](https://star-history.com/#mujaxso/qyzer-studio&Date)

---

<p align="center">
  <i>Built with ❤️ and Rust</i>
</p>
