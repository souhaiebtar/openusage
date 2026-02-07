//! Platform-specific panel implementation
//!
//! On macOS, uses tauri-nspanel for floating panel behavior.
//! On Windows, uses a regular window with show/hide.

#[cfg(target_os = "macos")]
mod platform {
    pub use crate::panel_macos::*;
}

#[cfg(target_os = "windows")]
mod platform {
    pub use crate::panel_windows::*;
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
mod platform {
    // Fallback for Linux and other platforms - use Windows-style implementation
    pub use crate::panel_windows::*;
}

pub use platform::*;
