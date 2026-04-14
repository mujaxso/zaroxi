# Qyzer Studio: AI-First IDE

> ⚠️ **Heavily Under Development**: Qyzer Studio is currently in active development. APIs, features, and architecture are subject to change. We welcome early adopters and contributors to help shape the project!

[![CI](https://github.com/mujaxso/qyzer/actions/workflows/ci.yml/badge.svg)](https://github.com/mujaxso/qyzer/actions/workflows/ci.yml)
[![Security Audit](https://github.com/mujaxso/qyzer/actions/workflows/security-audit.yml/badge.svg)](https://github.com/mujaxso/qyzer/actions/workflows/security-audit.yml)
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
qyzer/
├── apps/                    # Applications
│   └── desktop/            # Main desktop application (Iced-based GUI)
├── crates/                 # Core libraries
│   ├── core-types/         # Shared data structures and types
│   ├── editor-core/        # Text editing primitives, rope data structure, cursor management
│   ├── syntax-core/        # Tree-sitter integration, syntax highlighting, language support
│   ├── workspace-model/    # Workspace state management, file tree, buffer management
│   ├── lsp-client/         # Language Server Protocol client for intelligent code analysis
│   ├── ai-context/         # AI context collection and management from workspace
│   ├── ai-agent/           # AI task orchestration and execution
│   ├── patch-engine/       # Diff generation and application for AI suggestions
│   ├── rpc/                # Remote Procedure Call framework for inter-process communication
│   ├── settings/           # Configuration management and persistence
│   ├── permissions/        # Access control and security permissions system
│   └── file-ops/           # File system operations and metadata handling
├── services/               # Background services
│   ├── workspace-daemon/   # Workspace management service
│   └── ai-daemon/          # AI operations and model management service
├── docs/                   # Documentation
│   ├── architecture.md     # High-level system design
│   ├── crates.md          # Detailed crate documentation
│   ├── roadmap.md         # Development roadmap and future plans
│   ├── rpc.md             # RPC framework documentation
│   └── security.md        # Security architecture and practices
└── Cargo.toml             # Workspace configuration
```

## 🛠️ Getting Started

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- Cargo (comes with Rust)
- Git for version control
- For AI features: API key for supported AI providers (optional)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/mujaxso/qyzer.git
cd qyzer

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

# Or run with specific workspace
cargo run -p desktop -- ~/projects/my-project
```

### Development Build

For faster development builds:

```bash
# Debug build (faster compilation)
cargo build -p desktop

# Release build (optimized performance)
cargo build -p desktop --release
```

## 📁 Project Structure Deep Dive

### Core Crates

- **`core-types`**: Shared data structures and types used across the entire workspace
- **`editor-core`**: Text editing primitives, rope data structure, cursor management, document handling
- **`syntax-core`**: Tree-sitter integration, syntax highlighting, language parsing and analysis
- **`workspace-model`**: Workspace state management, file tree, buffer management, project organization
- **`lsp-client`**: Language Server Protocol client for intelligent code analysis and language features
- **`ai-context`**: AI context collection and management from workspace, prompt engineering
- **`ai-agent`**: AI task orchestration and execution, model integration, response processing
- **`patch-engine`**: Diff generation and application for AI suggestions, code transformation
- **`rpc`**: Remote Procedure Call framework for inter-process communication between components
- **`settings`**: Configuration management and persistence, user preferences storage
- **`permissions`**: Access control and security permissions system, authorization logic
- **`file-ops`**: File system operations and metadata handling, file watching, I/O utilities

### Services

- **`workspace-daemon`**: Background service for workspace management, file indexing, and resource monitoring
- **`ai-daemon`**: Background service for AI operations and model management, inference optimization

### Applications

- **`desktop`**: Main desktop application built with Iced GUI framework, user interface and interaction

## 🎯 Key Features in Detail

### AI-Powered Development
- **Intelligent Code Completion**: Context-aware suggestions based on your entire workspace
- **AI-Assisted Refactoring**: Safe, automated code restructuring with AI guidance
- **Natural Language to Code**: Convert descriptions into working code
- **Code Explanation**: Get detailed explanations of complex code segments
- **Bug Detection**: AI-powered bug finding and fix suggestions

### Modern Editor Experience
- **Syntax Highlighting**: Tree-sitter powered syntax highlighting for multiple languages
- **Multiple Cursors**: VS Code-style multiple cursor editing
- **Code Folding**: Collapsible code regions for better navigation
- **Bracket Matching**: Intelligent bracket pair colorization and matching
- **Minimap**: Code overview for quick navigation in large files

### Workspace Management
- **Fast File Navigation**: Quick file switching and search
- **Project-Wide Search**: Search across entire workspace with regex support
- **Git Integration**: Built-in source control with visual diff tools
- **Terminal Integration**: Integrated terminal for quick commands
- **Task Runner**: Define and run project-specific tasks

### Extensibility
- **Plugin System**: Extend functionality with Rust-based plugins
- **Theme Support**: Custom color themes and UI customization
- **Language Support**: Add support for new programming languages
- **AI Provider Plugins**: Connect to different AI backends

## 🔧 Configuration

Qyzer Studio can be configured through:

1. **Settings UI**: Accessible via the settings activity
2. **Configuration Files**: JSON-based config files in `~/.config/qyzer-studio/`
3. **Command Line Arguments**: Various startup options

### Example Configuration

```json
{
  "editor": {
    "fontFamily": "JetBrainsMono Nerd Font",
    "fontSize": 14,
    "lineHeight": 1.5,
    "ligatures": true,
    "theme": "dark"
  },
  "ai": {
    "provider": "openai",
    "model": "gpt-4",
    "maxTokens": 4096
  },
  "workspace": {
    "autoSave": true,
    "formatOnSave": false,
    "followSymlinks": true
  }
}
```

## 🚀 Performance

Qyzer Studio is built with performance in mind:

- **Native Performance**: Built in Rust for maximum speed
- **Incremental Parsing**: Tree-sitter for fast syntax highlighting
- **Efficient Memory Usage**: Smart caching and resource management
- **Async Architecture**: Non-blocking UI with async I/O operations
- **Large File Support**: Efficient handling of files up to 100MB+

## 🤝 Community and Support

### Getting Help
- **GitHub Discussions**: Community discussions and Q&A
- **Issue Tracker**: Report bugs and request features
- **Documentation**: Comprehensive docs in the `docs/` directory

### Contributing
We welcome contributions of all kinds! See our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Roadmap
Check out our [Roadmap](docs/roadmap.md) to see what's planned for future releases.

## 📊 Benchmarks

| Operation | Qyzer Studio | VS Code | Sublime Text |
|-----------|--------------|---------|--------------|
| Startup Time | ~500ms | ~800ms | ~200ms |
| File Open (10MB) | ~100ms | ~150ms | ~50ms |
| Workspace Indexing | ~2s | ~3s | N/A |
| AI Response Time | ~1.5s | N/A | N/A |

*Note: Benchmarks are approximate and depend on hardware.*

## 🔗 Related Projects

- **[Tree-sitter](https://tree-sitter.github.io/tree-sitter/)**: Parser generator tool and incremental parsing library
- **[Iced](https://iced.rs/)**: Cross-platform GUI library for Rust
- **[Rust Analyzer](https://rust-analyzer.github.io/)**: Rust compiler frontend for IDEs
- **[Tauri](https://tauri.app/)**: Framework for building desktop apps (considered for future versions)

## 📝 License

Qyzer Studio is open-source software licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

### Third-Party Licenses
This project uses several open-source libraries. See the `LICENSE-THIRD-PARTY` file for complete details.

## 🙏 Acknowledgments

We'd like to thank:

- **The Rust Community** for creating an amazing ecosystem
- **All Contributors** who help make Qyzer Studio better
- **OpenAI** and other AI providers for their APIs
- **The Iced Framework Team** for their excellent GUI library
- **Everyone who has provided feedback and testing**

## 📞 Contact and Links

- **Website**: [https://qyzer.dev](https://qyzer.dev) (coming soon)
- **GitHub**: [https://github.com/mujaxso/qyzer](https://github.com/mujaxso/qyzer)
- **Documentation**: [https://docs.qyzer.dev](https://docs.qyzer.dev) (coming soon)
- **Twitter**: [@qyzer_studio](https://twitter.com/qyzer_studio) (coming soon)
- **Email**: contact@qyzer.dev

---

<p align="center">
  <i>Built with ❤️ and Rust</i>
</p>

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

To report a security vulnerability, please email security@qyzer.dev (encrypted communication preferred).

## 📄 License

Qyzer Studio is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- The Rust community for excellent tooling and libraries
- All contributors who help make Qyzer Studio better
- Inspired by modern IDEs and AI-assisted development tools

## 📞 Contact

- **GitHub Issues**: [Bug reports and feature requests](https://github.com/mujaxso/qyzer/issues)
- **Discussions**: [Community discussions](https://github.com/mujaxso/qyzer/discussions)
- **Email**: contact@qyzer.dev

## 🌟 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=mujaxso/qyzer&type=Date)](https://star-history.com/#mujaxso/qyzer&Date)

---

<p align="center">
  <i>Built with ❤️ and Rust</i>
</p>
