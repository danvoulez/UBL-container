# UBL 2 ∞ — Universal Business Ledger

[![Build](https://github.com/danvoulez/UBL-container/actions/workflows/build.yml/badge.svg)](https://github.com/danvoulez/UBL-container/actions/workflows/build.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

> **Registrar, em prova criptográfica, qualquer fato de negócio, garantindo que só artefatos autorizados cruzem fronteiras (ALLOW/DENY determinístico) e que toda decisão seja reproduzível fora do cluster.**

## 🎯 Value Proposition

- ✅ **Sub-millisecond verdict** for any object (ZIP, API, LLM)
- 🛡️ **Immutable ledger** with Merkle-root anchored externally
- 🔒 **Passkey 2-eyes** for all power grants (Permit)
- 🪶 **Minimal scope** via Hop-Token (blast-radius ≪)
- 📜 **Complete audit** without online access (slice + root proof)

## 🏗️ Architecture

```
┌─────────────┐
│ ubl-kernel  │  ◄── Deterministic hash + signature
└──────┬──────┘
       │
┌──────▼──────────┐
│ Ledger Engine   │  ◄── Immutable append-only with Merkle-root
└──────┬──────────┘
       │
┌──────▼──────────┐
│   Membrana      │  ◄── ALLOW/DENY in < 1ms
└──────┬──────────┘
       │
    ┌──▼───┐  ┌──────────────┐  ┌──────────┐
    │Wallet│  │Policy Engine │  │  Runner  │
    └──────┘  └──────────────┘  └──────────┘
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.82+ (automatically configured via `rust-toolchain.toml`)
- PostgreSQL 15+ or SQLite 3.42+ (for Ledger Engine)
- Node.js 20+ (for TypeScript components)

### Build

```bash
# Clone the repository
git clone https://github.com/danvoulez/UBL-container.git
cd UBL-container

# Build all crates
cargo build --release

# Run tests
cargo test --all-features
```

### Development

```bash
# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run tests with coverage
cargo tarpaulin --out Html --output-dir coverage
```

## 📦 Components

### Core Crates

| Crate | Description | Sprint |
|-------|-------------|--------|
| **ubl-kernel** | Deterministic hashing and signing (BLAKE3 + Ed25519) | 1 |
| **ledger-engine** | Immutable append-only ledger with Merkle-root | 2 |
| **membrana** | Sub-millisecond ALLOW/DENY decision service | 3 |
| **wallet** | Passkey 2-eyes vault for Permit issuance | 4 |
| **policy-engine** | TDLN DSL to WASM compiler | 5 |
| **runner** | Isolated job execution with Receipt generation | 6 |

## 🗓️ Development Roadmap

See [TASKLIST.md](TASKLIST.md) for the complete sprint breakdown (0-7) and [AGENTS.md](AGENTS.md) for team structure.

### Milestones

- **Sprint 0**: Bootstrap & CI ✅
- **Sprint 1**: Deterministic Kernel (ubl-kernel)
- **Sprint 2**: Ledger Engine (append-only + Merkle)
- **Sprint 3**: Membrana (p95 ≤ 1ms)
- **Sprint 4**: Wallet + CLI end-to-end
- **Sprint 5**: Policy WASM + Runner
- **Sprint 6**: Portal premium GA
- **Sprint 7**: Observability & Release

## 🛡️ Security Principles

- **Zero trust**: All decisions cryptographically verified
- **Immutable audit trail**: No UPDATE/DELETE on ledger
- **Deterministic execution**: Same input → same output, always
- **Minimal blast radius**: Hop-Token scope limiting
- **External anchoring**: Daily Merkle-root to Git

## 📊 Performance Targets

| Metric | Target | Component |
|--------|--------|-----------|
| Verify latency (p95) | ≤ 1ms | Membrana |
| Verify latency (p95) | < 0.8ms | MacBook M2 |
| Test coverage | ≥ 90% | ubl-kernel |
| Test coverage | ≥ 75% | Services |
| Security findings | 0 critical | All |

## 🧪 Testing

```bash
# Run all tests
cargo test --all-features

# Run specific crate tests
cargo test -p ubl-kernel

# Run with nextest (faster)
cargo nextest run

# Fuzz testing (kernel)
cd crates/ubl-kernel
cargo +nightly fuzz run fuzz_target_1
```

## 📝 Documentation

Documentation is generated from inline Rustdoc comments:

```bash
# Generate and open documentation
cargo doc --all-features --no-deps --open
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Guidelines

- All PRs must pass CI (build, test, clippy, fmt)
- Maintain or improve test coverage
- Follow Rust API guidelines
- Update documentation for public APIs

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [BLAKE3](https://github.com/BLAKE3-team/BLAKE3) - Cryptographic hash function
- [Ed25519-Dalek](https://github.com/dalek-cryptography/ed25519-dalek) - Digital signatures
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [SQLx](https://github.com/launchbadge/sqlx) - SQL toolkit
- [Wasmtime](https://wasmtime.dev/) - WebAssembly runtime

---

> **Vamos entalhar confiança no Ledger — e deixar o resto do mundo com inveja.** 🚀
