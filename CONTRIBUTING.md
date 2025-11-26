# Contributing to Process Key Sender

Thank you for your interest in contributing to Process Key Sender! 🎉

## 🚨 **Important Ethics Notice**

Before contributing, please understand that this tool is designed for **educational purposes and legitimate automation only**. All contributions must align with our ethical guidelines:

- ✅ Support for offline/single-player games
- ✅ Accessibility and productivity tools
- ✅ Educational and research purposes
- ❌ Features that enable cheating in online games
- ❌ Anti-cheat system circumvention
- ❌ Violation of Terms of Service

## 🛠️ **Development Setup**

### Prerequisites
- Rust 1.85+ (2021 edition)
- Git
- Windows 10+ or Linux (for development)

### Setup Instructions
```bash
# Clone the repository
git clone https://github.com/KyleDerZweite/process-key-sender.git
cd process-key-sender

# Build the project
cargo build

# Run tests
cargo test

# Run with example config
cargo run -- --config example-configs/config.json
```

## 📝 **How to Contribute**

### 1. Fork and Clone
```bash
git clone https://github.com/YOUR-USERNAME/process-key-sender.git
cd process-key-sender
git remote add upstream https://github.com/KyleDerZweite/process-key-sender.git
```

### 2. Create a Feature Branch
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b bugfix/your-bugfix-name
```

### 3. Make Your Changes
- Follow Rust best practices and idiomatic code
- Add tests for new functionality
- Update documentation as needed
- Ensure all safety disclaimers remain intact

### 4. Test Your Changes
```bash
# Run all tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy for linting
cargo clippy -- -D warnings

# Test with example configs
cargo run -- --config example-configs/config.json
```

### 5. Commit and Push
```bash
git add .
git commit -m "feat: add your feature description"
git push origin feature/your-feature-name
```

### 6. Create Pull Request
- Use clear, descriptive titles
- Include detailed description of changes
- Reference any related issues
- Ensure all checks pass

## 🎯 **What We're Looking For**

### High Priority
- 🐧 **Linux/X11 implementation** (process finding and key sending)
- 🍎 **macOS support**
- 🔥 **Global hotkey implementation** for pause/resume
- 🎨 **GUI version** (optional)
- 📊 **Better error handling and user feedback**

### Medium Priority
- 🔧 **Additional key types** (multimedia keys, etc.)
- ⚡ **Performance optimizations**
- 📝 **Better documentation and examples**
- 🧪 **More comprehensive tests**
- 🎲 **Randomized intervals** within ranges

### Low Priority
- 🎮 **Game-specific configuration templates** (offline games only)
- 📱 **Configuration GUI**
- 🔍 **Window title filtering**
- 📈 **Usage statistics and metrics**

## 💡 **Feature Requests**

When suggesting new features, please:

1. **Check existing issues** first
2. **Describe the use case** - what problem does it solve?
3. **Ensure ethical alignment** - no cheating/unfair advantages
4. **Provide examples** of how it would be used
5. **Consider cross-platform compatibility**

## 🐛 **Bug Reports**

When reporting bugs, please include:

```markdown
**Environment:**
- OS: Windows 11 / Ubuntu 22.04 / etc.
- Rust version: `rustc --version`
- Tool version: `pks --version`

**Steps to Reproduce:**
1. Run command: `pks --config example.json`
2. Expected behavior: X should happen
3. Actual behavior: Y happened instead

**Configuration:**
```json
{
  // Include your config file (remove sensitive info)
}
```

**Error Messages:**
```
Include any error messages or logs
```
```

## 📋 **Code Style**

### Rust Guidelines
- Follow standard `rustfmt` formatting
- Use `clippy` suggestions
- Prefer explicit error handling over `unwrap()`
- Use meaningful variable and function names
- Add documentation comments for public APIs

### Git Commit Messages
```
type(scope): description

feat(keys): add multimedia key support
fix(windows): resolve process detection issue  
docs(readme): update installation instructions
test(config): add independent keys test cases
refactor(core): simplify key parsing logic
```

### Code Comments
```rust
// Good: Explain WHY, not WHAT
// Use global key sending as fallback when window targeting fails
self.send_key_global_windows(key)

// Bad: Explain WHAT (obvious from code)
// Send key using global windows method
self.send_key_global_windows(key)
```

## 🔐 **Security Considerations**

- Never commit API keys, passwords, or sensitive data
- Be mindful of process privilege escalation
- Validate all user inputs thoroughly
- Consider Windows UAC and Linux permissions
- Review security implications of new features

## 📄 **License**

By contributing, you agree that your contributions will be licensed under the MIT License.

## 🤝 **Code of Conduct**

### Our Pledge
We are committed to making participation in our project harassment-free for everyone.

### Our Standards
- ✅ Using welcoming and inclusive language
- ✅ Being respectful of differing viewpoints
- ✅ Gracefully accepting constructive criticism
- ✅ Focusing on what is best for the community
- ❌ Trolling, insulting/derogatory comments
- ❌ Public or private harassment
- ❌ Publishing others' private information

## 📞 **Questions?**

- 💬 **GitHub Discussions**: Ask questions and share ideas
- 🐛 **GitHub Issues**: Report bugs and request features
- 📧 **Email**: KyleDerZweite@example.com (replace with your actual email)

Thank you for contributing! 🚀