# Contributing to Zaroxi Studio

Thank you for your interest in contributing to Zaroxi Studio! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

Please be respectful and considerate of others when participating in this project. We aim to foster an inclusive and welcoming community.

## How Can I Contribute?

### Reporting Bugs

Before creating a bug report, please check the existing issues to avoid duplicates.

**How to Report a Bug:**

1. Use the bug report template when creating an issue
2. Describe the exact steps to reproduce the problem
3. Include system information (OS, Rust version, etc.)
4. Include any relevant logs or error messages
5. If possible, include a minimal reproducible example

### Suggesting Enhancements

We welcome suggestions for new features and improvements.

**How to Suggest an Enhancement:**

1. Use the feature request template when creating an issue
2. Clearly describe the feature and its benefits
3. Explain why this feature would be useful to most users
4. Consider potential implementation approaches

### Contributing Code

#### Setting Up Development Environment

1. Fork the repository
2. Clone your fork locally
3. Install Rust and Cargo (via rustup)
4. Build the project: `cargo build --workspace`
5. Run tests: `cargo test --workspace`

#### Development Workflow

1. **Create a branch** for your changes:

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the coding standards

3. **Test your changes**:

   ```bash
   cargo test --workspace
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets -- -D warnings
   ```

4. **Commit your changes** with descriptive commit messages:

   ```bash
   git commit -m "feat: add new feature"
   ```

5. **Push to your fork**:

   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request** from your fork to the main repository

#### Pull Request Guidelines

- Fill out the PR template completely
- Keep PRs focused on a single change
- Include tests for new functionality
- Update documentation as needed
- Ensure all CI checks pass

### Documentation

Good documentation is crucial. We welcome contributions to:

- API documentation (Rustdoc comments)
- User guides and tutorials
- Architecture documentation
- Code comments explaining complex logic

## Coding Standards

### Rust Style Guide

- Follow the official Rust style guide
- Run `cargo fmt` before committing
- Address all `cargo clippy` warnings
- Use meaningful variable and function names

### Code Organization

- Keep functions small and focused
- Use modules to organize related functionality
- Document public APIs with Rustdoc
- Write unit tests for non-trivial functions

### Commit Messages

Use conventional commit messages:

```
<type>: <description>

[optional body]

[optional footer]
```

**Types:**

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**

```
feat: add AI context collection
fix: resolve buffer overflow in editor
docs: update architecture documentation
```

## Testing

### Writing Tests

- Write unit tests for individual functions
- Write integration tests for component interactions
- Use property-based testing where appropriate
- Mock external dependencies

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p core-types

# Run tests with verbose output
cargo test --workspace -- --nocapture
```

## Review Process

1. **Initial Review**: A maintainer will review your PR within a few days
2. **Feedback**: You may receive feedback or requested changes
3. **Revision**: Make requested changes and push updates
4. **Approval**: Once approved, a maintainer will merge your PR

## Recognition

All contributors will be recognized in:

- The project's README (for significant contributions)
- Release notes
- The contributors graph on GitHub

## Questions?

If you have questions about contributing:

- Check the documentation in `docs/`
- Join discussions on GitHub
- Open an issue with the `question` label

Thank you for contributing to Zaroxi Studio! 🎉
