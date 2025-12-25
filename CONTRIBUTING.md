# Contributing to UBL 2 ∞

Thank you for your interest in contributing to UBL (Universal Business Ledger)! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Code Style](#code-style)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Sprint Structure](#sprint-structure)

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and professional in all interactions.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/UBL-container.git
   cd UBL-container
   ```
3. **Set up the development environment**:
   ```bash
   # Rust toolchain will be automatically configured via rust-toolchain.toml
   cargo build
   cargo test
   ```

## Development Workflow

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the code style guidelines

3. **Run tests and checks**:
   ```bash
   # Format code
   cargo fmt --all

   # Run clippy
   cargo clippy --all-targets --all-features -- -D warnings

   # Run tests
   cargo test --all-features

   # Check documentation
   cargo doc --all-features --no-deps
   ```

4. **Commit your changes**:
   ```bash
   git add .
   git commit -m "feat: Add your feature description"
   ```

5. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Open a Pull Request** on GitHub

## Code Style

### Rust

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for code formatting (enforced by CI)
- Use `clippy` for linting (enforced by CI with `-D warnings`)
- Write documentation comments for all public APIs using `///`
- Include examples in documentation where appropriate

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Test additions or changes
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Maintenance tasks

Example:
```
feat(ubl-kernel): implement canonical JSON serialization

Add deterministic JSON serialization following Json✯Atomic spec.
Includes tests for edge cases and benchmarks.
```

## Testing

### Unit Tests

- Write unit tests for all new functionality
- Place tests in a `tests` module at the bottom of the file
- Use `#[cfg(test)]` to conditionally compile tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_function() {
        assert_eq!(your_function(), expected_result);
    }
}
```

### Integration Tests

- Place integration tests in `tests/` directory
- Each file in `tests/` is compiled as a separate crate

### Coverage

- Aim for ≥90% coverage in `ubl-kernel`
- Aim for ≥75% coverage in service crates
- Run coverage with:
  ```bash
  cargo tarpaulin --out Html --output-dir coverage
  ```

### Property-Based Testing

- Use `quickcheck` for property-based tests
- Especially important for cryptographic functions

```rust
#[cfg(test)]
mod quickcheck_tests {
    use quickcheck::quickcheck;

    quickcheck! {
        fn prop_your_property(input: SomeType) -> bool {
            // Your property test
            true
        }
    }
}
```

## Pull Request Process

1. **Ensure CI passes**:
   - All tests pass
   - Code is formatted (`cargo fmt`)
   - Clippy passes with no warnings
   - Documentation builds without errors

2. **Update documentation**:
   - Update README.md if adding new features
   - Update inline documentation for changed APIs
   - Add examples where appropriate

3. **Request review**:
   - PRs require at least one approval
   - Address all review comments
   - Keep PRs focused on a single feature/fix

4. **Merge**:
   - Squash and merge is preferred for feature branches
   - Maintain a clean, linear history

## Sprint Structure

UBL follows a sprint-based development model (see [TASKLIST.md](TASKLIST.md)):

- **Sprint 0**: Bootstrap & CI ✅
- **Sprint 1**: Deterministic Kernel (ubl-kernel)
- **Sprint 2**: Ledger Engine
- **Sprint 3**: Membrana
- **Sprint 4**: Wallet & CLI
- **Sprint 5**: Policy Engine (TDLN → WASM)
- **Sprint 6**: Runner & Receipt
- **Sprint 7**: Portal & Observability

See [AGENTS.md](AGENTS.md) for agent/role structure.

## Security

- **Never commit secrets** or credentials
- Use `trivy`, `cargo-audit`, and `semgrep` for security scanning
- Report security vulnerabilities privately to the maintainers
- Follow the principle of least privilege
- All cryptographic operations must be deterministic and reproducible

## Performance

- Membrana must maintain **p95 ≤ 1ms** for `/verify` endpoint
- Use benchmarks to validate performance:
  ```bash
  cargo bench
  ```
- Profile before optimizing:
  ```bash
  cargo flamegraph
  ```

## Questions?

- Open an issue for bug reports or feature requests
- Join `#ubl-war-room` for discussions (see AGENTS.md)
- Check existing issues and PRs before creating new ones

## License

By contributing to UBL, you agree that your contributions will be licensed under the MIT OR Apache-2.0 license.

---

Thank you for contributing to UBL 2 ∞! 🚀
