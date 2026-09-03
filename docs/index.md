# TUI-OS

**TUI-OS** is an experimental operating system that treats the terminal/TUI as the primary
graphical interface instead of a traditional desktop environment.

## Overview

The project explores the question: **What would a modern operating system look like if a
terminal/TUI were treated as the primary graphical interface instead of a traditional
desktop environment?**

## Key Principles

- **Linux is infrastructure**: Reuse mature Linux functionality
- **TUI is primary**: Everything the user interacts with is presented through a TUI-first interface
- **No GUI desktops**: No GNOME, KDE, XFCE, X11, or Wayland
- **Rust for userspace**: Type safety and performance

## Current Status

**Milestone M0 - Project Foundation** is in progress.

### Project Structure

```
.
├── src/                   # Rust source code
│   ├── tui-core/          # Core TUI abstractions
│   ├── tui-compositor/   # Window compositor
│   └── tui-shell/        # Interactive shell
├── docs/                  # Documentation
│   ├── adr/               # Architecture Decision Records
│   └── spec/              # Specifications
├── tests/                 # Integration tests
├── scripts/               # Build and deployment scripts
├── config/                # Configuration files
└── .github/workflows/     # CI pipelines
```

## Getting Started

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions.

## Documentation

- [TUI-OS v0.1 Specification](spec/tui-os-v0.1.md)
- [Architecture Decision Records](adr/ADR-001-language-and-base-libraries.md)
