# Contributing to TUI-OS

Thank you for your interest in contributing to TUI-OS!

## Development Environment

### Requirements

- Rust 1.81+
- Git
- QEMU (for testing the target system)

### Setting Up

1. Clone the repository:
```bash
git clone https://github.com/your-org/tui-os.git
cd tui-os
```

2. Build the project:
```bash
cargo build --release
```

3. Run tests:
```bash
cargo test
```

4. Run linter:
```bash
cargo clippy --all
```

5. Format code:
```bash
cargo fmt --all
```

## Workflow

We follow a GitHub-flow inspired workflow:

1. Create a branch for your feature or fix
2. Make your changes
3. Ensure all tests pass
4. Submit a pull request

### Branch Naming

- `feat/tui-core/...` for new features
- `fix/...` for bug fixes
- `docs/...` for documentation
- `refactor/...` for refactoring

## Code Conventions

- 4 spaces for indentation
- Run `cargo fmt` before committing
- Run `cargo clippy` and address all warnings
- Maximum 100 characters per line
- Document public APIs with doc comments

## Testing

Every feature should be tested. We have multiple test levels:

- Unit tests within the crate
- Integration tests
- System tests (booting TUI-OS in QEMU)

## Project Structure

```
src/
  tui-core/    # Core TUI abstractions (Cell, Buffer, Surface)
  tui-compositor/ # Window compositor
  tui-shell/   # Interactive shell
```

## Getting Help

Open an issue on GitHub for questions.
