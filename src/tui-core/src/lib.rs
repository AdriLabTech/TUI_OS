//! TUI-CORE — Core rendering and input abstractions
//!
//! This crate provides the fundamental building blocks for TUI-OS:
//! - Cell: character with style
//! - Buffer: 2D grid of cells
//! - Surface: rectangular region with clipping
//! - Input: keyboard and mouse event handling

#![forbid(unsafe_code)]
#![deny(clippy::undocumented_unsafe_blocks)]

pub mod buffer;
pub mod cell;
pub mod input;
pub mod style;
pub mod surface;

pub use buffer::Buffer;
pub use cell::Cell;
pub use input::{InputEvent, InputHandler, KeyEvent, MouseEvent};
pub use style::Style;
pub use surface::Surface;
