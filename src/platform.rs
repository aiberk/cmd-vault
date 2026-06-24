/// Platform abstraction layer - isolates all OS-specific code
///
/// This module provides a clean interface for platform-specific operations.
/// Each platform implements the same trait, making it easy for different
/// developers to work on different platforms without affecting core logic.
use std::path::PathBuf;

/// Platform-specific operations that need different implementations
pub trait PlatformOps {
    /// Get the appropriate config directory for this platform
    fn get_config_dir() -> Result<PathBuf, PlatformError>;

    /// Copy text to system clipboard
    fn copy_to_clipboard(text: &str) -> Result<(), PlatformError>;

    /// Get platform-specific keyboard shortcuts for display
    fn get_shortcuts() -> PlatformShortcuts;

    /// Check if required system tools are available
    fn check_system_requirements() -> SystemCheck;

    /// Get platform identification info
    fn get_platform_info() -> PlatformInfo;
}

#[derive(Debug, Clone)]
pub struct PlatformShortcuts {
    pub copy: &'static str,
    pub paste: &'static str,
    pub quit: &'static str,
}

#[derive(Debug, Clone)]
pub struct SystemCheck {
    pub clipboard_message: String,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub name: &'static str,
    pub arch: &'static str,
    pub clipboard_method: &'static str,
}

#[derive(Debug, thiserror::Error)]
pub enum PlatformError {
    #[error("Config directory not accessible: {0}")]
    ConfigDir(String),
    #[error("Clipboard operation failed: {0}")]
    Clipboard(String),
}

// Platform selection - this chooses which implementation to use
#[cfg(target_os = "windows")]
pub type Platform = WindowsPlatform;

#[cfg(target_os = "macos")]
pub type Platform = MacOSPlatform;

#[cfg(target_os = "linux")]
pub type Platform = LinuxPlatform;

// Convenience functions that delegate to the platform implementation
pub fn get_config_dir() -> Result<PathBuf, PlatformError> {
    Platform::get_config_dir()
}

pub fn copy_to_clipboard(text: &str) -> Result<(), PlatformError> {
    Platform::copy_to_clipboard(text)
}

pub fn get_shortcuts() -> PlatformShortcuts {
    Platform::get_shortcuts()
}

pub fn check_system_requirements() -> SystemCheck {
    Platform::check_system_requirements()
}

pub fn get_platform_info() -> PlatformInfo {
    Platform::get_platform_info()
}

// =============================================================================
// LINUX IMPLEMENTATION (you maintain this)
// =============================================================================

#[cfg(target_os = "linux")]
pub struct LinuxPlatform;

#[cfg(target_os = "linux")]
impl PlatformOps for LinuxPlatform {
    fn get_config_dir() -> Result<PathBuf, PlatformError> {
        let config_dir = dirs::config_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
            .ok_or_else(|| {
                PlatformError::ConfigDir("Cannot determine config directory".to_string())
            })?;

        let app_dir = config_dir.join("cmd-vault");

        if let Err(e) = std::fs::create_dir_all(&app_dir) {
            return Err(PlatformError::ConfigDir(format!(
                "Cannot create config directory: {}",
                e
            )));
        }

        Ok(app_dir.join("vault.json"))
    }

    fn copy_to_clipboard(text: &str) -> Result<(), PlatformError> {
        use std::io::Write;
        use std::process::{Command, Stdio};

        // Try xclip first
        if Command::new("xclip").arg("-version").output().is_ok() {
            let mut child = Command::new("xclip")
                .args(["-selection", "clipboard"])
                .stdin(Stdio::piped())
                .spawn()
                .map_err(|e| PlatformError::Clipboard(format!("Failed to spawn xclip: {}", e)))?;

            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(text.as_bytes()).map_err(|e| {
                    PlatformError::Clipboard(format!("Failed to write to xclip: {}", e))
                })?;
            }

            child
                .wait()
                .map_err(|e| PlatformError::Clipboard(format!("xclip process failed: {}", e)))?;

            return Ok(());
        }

        // Try xsel as fallback
        if Command::new("xsel").arg("--version").output().is_ok() {
            let mut child = Command::new("xsel")
                .args(["--clipboard", "--input"])
                .stdin(Stdio::piped())
                .spawn()
                .map_err(|e| PlatformError::Clipboard(format!("Failed to spawn xsel: {}", e)))?;

            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(text.as_bytes()).map_err(|e| {
                    PlatformError::Clipboard(format!("Failed to write to xsel: {}", e))
                })?;
            }

            child
                .wait()
                .map_err(|e| PlatformError::Clipboard(format!("xsel process failed: {}", e)))?;

            return Ok(());
        }

        Err(PlatformError::Clipboard(
            "No clipboard tool available. Install xclip or xsel".to_string(),
        ))
    }

    fn get_shortcuts() -> PlatformShortcuts {
        PlatformShortcuts {
            copy: "Ctrl+C",
            paste: "Ctrl+V",
            quit: "Ctrl+Q",
        }
    }

    fn check_system_requirements() -> SystemCheck {
        use std::process::Command;

        let xclip_available = Command::new("xclip").arg("-version").output().is_ok();
        let xsel_available = Command::new("xsel").arg("--version").output().is_ok();

        let clipboard_available = xclip_available || xsel_available;

        let clipboard_message = if clipboard_available {
            if xclip_available {
                "✅ xclip available".to_string()
            } else {
                "✅ xsel available".to_string()
            }
        } else {
            "❌ No clipboard tool found".to_string()
        };

        let mut recommendations = Vec::new();
        if !clipboard_available {
            recommendations.push("Install xclip: sudo apt-get install xclip".to_string());
            recommendations.push("Or install xsel: sudo apt-get install xsel".to_string());
        }

        SystemCheck {
            clipboard_message,
            recommendations,
        }
    }

    fn get_platform_info() -> PlatformInfo {
        PlatformInfo {
            name: "Linux",
            arch: std::env::consts::ARCH,
            clipboard_method: "xclip/xsel",
        }
    }
}

// =============================================================================
// MACOS IMPLEMENTATION (you maintain this)
// =============================================================================

#[cfg(target_os = "macos")]
pub struct MacOSPlatform;

#[cfg(target_os = "macos")]
impl PlatformOps for MacOSPlatform {
    fn get_config_dir() -> Result<PathBuf, PlatformError> {
        let config_dir = dirs::config_dir().ok_or_else(|| {
            PlatformError::ConfigDir("Cannot determine config directory".to_string())
        })?;

        let app_dir = config_dir.join("cmd-vault");

        if let Err(e) = std::fs::create_dir_all(&app_dir) {
            return Err(PlatformError::ConfigDir(format!(
                "Cannot create config directory: {}",
                e
            )));
        }

        Ok(app_dir.join("vault.json"))
    }

    fn copy_to_clipboard(text: &str) -> Result<(), PlatformError> {
        use std::io::Write;
        use std::process::{Command, Stdio};

        let mut child = Command::new("pbcopy")
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| PlatformError::Clipboard(format!("Failed to spawn pbcopy: {}", e)))?;

        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(text.as_bytes()).map_err(|e| {
                PlatformError::Clipboard(format!("Failed to write to pbcopy: {}", e))
            })?;
        }

        child
            .wait()
            .map_err(|e| PlatformError::Clipboard(format!("pbcopy process failed: {}", e)))?;

        Ok(())
    }

    fn get_shortcuts() -> PlatformShortcuts {
        PlatformShortcuts {
            copy: "⌘+C",
            paste: "⌘+V",
            quit: "⌘+Q",
        }
    }

    fn check_system_requirements() -> SystemCheck {
        use std::process::Command;

        let pbcopy_available = Command::new("pbcopy")
            .arg("-help")
            .output()
            .map(|output| output.status.code() != Some(127))
            .unwrap_or(false);

        SystemCheck {
            clipboard_message: if pbcopy_available {
                "✅ pbcopy/pbpaste available".to_string()
            } else {
                "❌ pbcopy not found (should be built-in)".to_string()
            },
            recommendations: if pbcopy_available {
                Vec::new()
            } else {
                vec!["pbcopy should be available by default on macOS".to_string()]
            },
        }
    }

    fn get_platform_info() -> PlatformInfo {
        PlatformInfo {
            name: "macOS",
            arch: std::env::consts::ARCH,
            clipboard_method: "pbcopy/pbpaste",
        }
    }
}

// =============================================================================
// WINDOWS IMPLEMENTATION (Windows developer maintains this)
// =============================================================================

#[cfg(target_os = "windows")]
pub struct WindowsPlatform;

#[cfg(target_os = "windows")]
impl PlatformOps for WindowsPlatform {
    fn get_config_dir() -> Result<PathBuf, PlatformError> {
        // TODO: Windows developer implements this
        // Use %APPDATA%\cmd-vault\vault.json
        // Or fall back to %USERPROFILE%\.cmd-vault.json

        let config_dir = dirs::config_dir().ok_or_else(|| {
            PlatformError::ConfigDir("Cannot determine config directory".to_string())
        })?;

        let app_dir = config_dir.join("cmd-vault");

        if let Err(e) = std::fs::create_dir_all(&app_dir) {
            return Err(PlatformError::ConfigDir(format!(
                "Cannot create config directory: {}",
                e
            )));
        }

        Ok(app_dir.join("vault.json"))
    }

    fn copy_to_clipboard(text: &str) -> Result<(), PlatformError> {
        // TODO: Windows developer implements this
        // Option 1: Use clip.exe command
        // Option 2: Use Windows API directly
        // Option 3: Try copypasta crate as fallback

        use std::io::Write;
        use std::process::{Command, Stdio};

        let mut child = Command::new("clip")
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| PlatformError::Clipboard(format!("Failed to spawn clip: {}", e)))?;

        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(text.as_bytes())
                .map_err(|e| PlatformError::Clipboard(format!("Failed to write to clip: {}", e)))?;
        }

        child
            .wait()
            .map_err(|e| PlatformError::Clipboard(format!("clip process failed: {}", e)))?;

        Ok(())
    }

    fn get_shortcuts() -> PlatformShortcuts {
        PlatformShortcuts {
            copy: "Ctrl+C",
            paste: "Ctrl+V",
            quit: "Alt+F4",
        }
    }

    fn check_system_requirements() -> SystemCheck {
        // TODO: Windows developer implements this
        // Check for clip.exe availability
        // Maybe check for PowerShell clipboard commands

        use std::process::Command;

        let clip_available = Command::new("clip")
            .arg("/?")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);

        SystemCheck {
            clipboard_message: if clip_available {
                "✅ Windows clip.exe available".to_string()
            } else {
                "❌ clip.exe not found".to_string()
            },
            recommendations: if clip_available {
                Vec::new()
            } else {
                vec!["clip.exe should be available by default on Windows".to_string()]
            },
        }
    }

    fn get_platform_info() -> PlatformInfo {
        PlatformInfo {
            name: "Windows",
            arch: std::env::consts::ARCH,
            clipboard_method: "clip.exe",
        }
    }
}
