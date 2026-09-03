# TUI-OS v0.1 Specification

**Version:** 0.1.0-draft
**Date:** 2026-09-04
**Status:** Draft

## Overview

TUI-OS is an experimental operating system that treats the terminal/TUI as the primary
graphical interface instead of a traditional desktop environment.

## Scope

TUI-OS v0.1 focuses on:

1. **M0 - Project Foundation**: Establish project structure, tooling, and CI
2. **M1 - Linux Base**: Arch Linux base system with systemd, network, and audio
3. **M2 - TUI Runtime**: Core rendering abstractions (Cell, Buffer, Surface)
4. **M3 - TUI Compositor**: Window management and composition

## Core Principles

1. **Linux is infrastructure**: Reuse mature Linux functionality
2. **TUI is primary**: Everything through terminal interface
3. **No GUI desktops**: No GNOME, KDE, XFCE, X11, or Wayland
4. **Rust for userspace**: Type safety and performance

## Architecture

### Layers

```
┌─────────────────────────────────┐
│       TUI Applications          │
├─────────────────────────────────┤
│         TUI Shell               │
├─────────────────────────────────┤
│      TUI Compositor             │
├─────────────────────────────────┤
│        TUI Runtime              │
├─────────────────────────────────┤
│         Linux Kernel            │
└─────────────────────────────────┘
```

### TUI Runtime Components

- **Cell**: Character with foreground, background, attributes
- **Buffer**: 2D grid of cells (rows × columns)
- **Surface**: Rectangular region with clipping
- **Input**: Keyboard and mouse event handling
- **Renderer**: ANSI escape sequence emission

### Compositor Components

- **Window**: ID, title, surface, position, z-index
- **Z-order**: Stacking order of windows
- **Focus**: Active window receives input
- **Dirty regions**: Optimization for selective redraw

## Non-Goals (v0.1)

- Web browser implementation
- Package manager frontend
- Media player
- Full system integration (login, shutdown)

## Dependencies

- Arch Linux (target)
- systemd
- Linux kernel
- Rust 1.81+
- QEMU/KVM (development)

## Milestone Timeline

| Milestone | Description | Status |
|-----------|-------------|--------|
| M0 | Project Foundation | In Progress |
| M1 | Linux Base | Planned |
| M2 | TUI Runtime | Planned |
| M3 | TUI Compositor | Planned |

## References

- [AGENT.md](../.context/AGENT.md) - Development agent instructions
- [Definicion_Milestones_Proyecto.txt](../.context/Definicion_Milestones_Proyecto.txt) - Detailed milestones
- [ADR-001](adr/ADR-001-language-and-base-libraries.md) - Language choice
