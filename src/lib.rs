//! Exact 1:1 Rust translation of FPS - Display by Grayray75
//! (github.com/Grayray75/FPS-Display, tag v5.2.0, LGPL-3.0-only).
//! Upstream Java package io.grayray75.mods.fpsdisplay maps to the modules
//! below; every Minecraft client type sits behind the host seams in `seams`.
//! Log lines, field names, branch order and quirks are kept as-is.
//!
//! - `fps_history` <- FpsHistory.java
//! - `config`      <- config/ConfigData.java + config/ConfigManager.java
//! - `fps_display` <- FpsDisplayMod.java
//! - `mixin`       <- mixin/HudMixin.java (body ported as the TAIL callback)
//! - `gui`         <- config/ModMenuIntegration.java + gui/* (ModMenu dispatch,
//!                    ClothOptionScreen, YaclOptionScreen, FallbackOptionScreen)

pub use fps_display::MOD_ID;
pub mod fps_history;
pub mod config;
pub mod fps_display;
pub mod mixin;
pub mod gui;
pub mod seams;
