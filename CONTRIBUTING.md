# Contributing to TabStash NVMe

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing.

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Respect different viewpoints

## How to Contribute

### Reporting Issues

1. Check if the issue already exists
2. Use a clear, descriptive title
3. Provide steps to reproduce
4. Include system information (OS, Chrome version, etc.)
5. Add relevant logs or error messages

### Suggesting Features

1. Check if the feature was already suggested
2. Explain the use case
3. Describe the expected behavior
4. Consider implementation complexity

### Pull Requests

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes**
4. **Test thoroughly**
5. **Update documentation** if needed
6. **Commit with clear messages**: `git commit -m "Add amazing feature"`
7. **Push to your fork**: `git push origin feature/amazing-feature`
8. **Open a Pull Request**

## Development Setup

### Prerequisites

- Rust (latest stable)
- Chrome browser
- Git
- Cargo

### Building

```bash
# Build native host
cd tabstash-native
cargo build --release

# Test
cargo test
```

### Extension Development

1. Load extension as unpacked in Chrome
2. Make changes to extension code
3. Reload extension in `chrome://extensions`
4. Test changes

### Testing

```bash
# Run Rust tests
cd tabstash-native
cargo test

# Run specific test
cargo test test_name
```

## Code Style

### Rust

- Follow Rust standard formatting: `cargo fmt`
- Run clippy: `cargo clippy`
- Document public APIs
- Use meaningful variable names

### JavaScript

- Use modern ES6+ syntax
- Follow Chrome extension best practices
- Comment complex logic
- Keep functions focused

## Commit Messages

Use clear, descriptive commit messages:

```
Add health check UI to extension popup

- Implement connection status indicator
- Add installer download links
- Show helpful error messages
```

## Pull Request Guidelines

1. **Keep PRs focused** - One feature or fix per PR
2. **Update documentation** - If adding features
3. **Add tests** - For new functionality
4. **Test on multiple platforms** - If applicable
5. **Update CHANGELOG** - Document changes

## Areas for Contribution

### High Priority

- Icon design and assets
- Cross-platform testing
- Performance optimizations
- Documentation improvements
- Bug fixes

### Medium Priority

- Firefox support
- macOS support
- Snapshot encryption
- Automatic cleanup
- UI/UX improvements

### Low Priority

- Cloud backup (optional)
- Incremental snapshots
- Advanced features

## Questions?

- Open a GitHub issue
- Start a discussion
- Check existing documentation

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (to be determined).

