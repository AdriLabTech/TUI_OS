# ADR-001: Language and Base Libraries Choice

**Date:** 2026-09-04

**Status:** Accepted

## Context

TUI-OS requires a systems-level programming language for developing the TUI runtime,
compositor, shell, and applications. The language must provide:

- Strong memory safety guarantees
- Low-level control for terminal escape sequence handling
- High-performance rendering capabilities
- Excellent tooling for testing and documentation
- Cross-platform compilation support

## Decision

We have chosen **Rust** as the primary implementation language with the following rationale:

### Why Rust?

1. **Memory Safety**: Rust's ownership system prevents data races, buffer overflows,
   and null pointer dereferences at compile time. This is critical for a system
   that handles terminal buffers and user input directly.

2. **Performance**: Rust provides C-like performance with zero-cost abstractions,
   essential for real-time TUI rendering without perceptible latency.

3. **Rich Ecosystem**: Mature crates for terminal handling (termion, crossterm),
   async networking (tokio), and CLI argument parsing (clap).

4. **Tooling**: First-class support for testing (built-in), benchmarking, formatting
   (rustfmt), and linting (clippy).

5. **Type Safety**: Strong typing reduces runtime errors in complex subsystems
   like the compositor and window management.

### Key Crates

| Crate | Purpose |
|-------|---------|
| `termion` | Terminal handling, escape sequences |
| `crossterm` | Cross-platform terminal operations |
| `tokio` | Async runtime for networking |
| `clap` | CLI argument parsing |
| `serde` | Serialization/deserialization |
| `tracing` | Structured logging |

## Consequences

### Positive

- Memory-safe implementation of core TUI components
- Single language across all userspace components
- Excellent FFI support for integrating C libraries (ncurses, etc.)
- Strong compiler error messages improve developer experience

### Negative

- Steeper learning curve compared to Python or Go
- Compilation times longer than scripting languages
- Smaller pool of contributors familiar with Rust

## Alternatives Considered

### Go

- Pros: Simpler syntax, faster compilation, garbage collected
- Cons: Runtime overhead, no true zero-cost abstractions, GC pauses unacceptable for real-time TUI

### Python

- Rejected: Performance insufficient for real-time rendering, GIL limitations

### C/C++

- Rejected: Memory safety concerns, undefined behavior risks, poor tooling

### JavaScript/TypeScript

- Rejected: Unsuitable for systems programming, performance limitations

## References

- [Rust Official](https://www.rust-lang.org/)
- [termion crate](https://docs.rs/termion/)
- [crossterm crate](https://docs.rs/crossterm/)
