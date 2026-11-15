# SigmaOS-rdbms-postgresql

> **PostgreSQL — RDBMS** — Relational database / data platform integration for SigmaOS.

Part of the [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS) ecosystem.  
Production-grade relational database with ACID compliance — PostgreSQL Global Dev Group

## Quickstart

```bash
cargo test    # run deterministic AAA unit tests
cargo check   # verify compilation
```

## Component Role

This crate implements the `postgresql` subsystem for SigmaOS:
- Fully real Rust implementation (no stubs)
- Fail-fast validation with typed errors
- Deterministic unit tests (Arrange-Act-Assert pattern)
- Single Responsibility, Modular, Zero-Copy where applicable

## API

```rust
use sigmaos_rdbms_postgresql::*;
// See src/lib.rs for full public API
```

## Documentation

Full documentation is in the [SigmaOS GitHub Wiki → Data-Science-Tools](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Data-Science-Tools).

## License

MIT
