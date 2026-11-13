//! 1:1 of config/ConfigData.java + config/ConfigManager.java.

use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};

use serde::{Deserialize, Serialize};

use crate::seams;

// java: private static ConfigData config; - the shared static, backed by
// ConfigData::new() until load_config() populates it.
static CONFIG: OnceLock<Mutex<ConfigData>> = OnceLock::new();
// java: config == null is the "not loaded" state; the AtomicBool mirrors it.
static CONFIG_NULL: AtomicBool = AtomicBool::new(true);

fn config_cell() -> &'static Mutex<ConfigData> {
    CONFIG.get_or_init(|| Mutex::new(ConfigData::default()))
}

// java: public class ConfigData { ... } - field initializers are the
// constructor defaults, so every serde field is #[serde(default)] with the
// same default (Gson leaves absent fields at the Java defaults).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ConfigData {
    pub enabled: bool,
    pub advanced_stats: bool,
    pub text_size: f32,
    pub text_color: i32,
    pub text_alpha: i32,
    pub text_shadows: bool,
    pub offset_top: i32,
    pub offset_left: i32,
    pub keybind_mode: KeyMode,
}

// Field initializers from ConfigData.java, in declaration order.
impl Default for ConfigData {
    fn default() -> Self {
        ConfigData {
            enabled: true,
            advanced_stats: false,
            text_size: 1.0,
            text_color: 0xEEEEEE,
            text_alpha: 230,
            text_shadows: false,
            offset_top: 4,
            offset_left: 4,
            keybind_mode: KeyMode::Toggle,
        }
    }
}

// java: public enum KeyMode { Toggle, PushToShow } - Gson serializes the
// enum constant name, so the serde renames keep the exact JSON tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum KeyMode {
    #[default]
    #[serde(rename = "Toggle")]
    Toggle,
    #[serde(rename = "PushToShow")]
    PushToShow,
}

pub struct ConfigManager;

impl ConfigManager {
    // java: public static ConfigData getConfig() { return config; }
    pub fn get_config() -> std::sync::MutexGuard<'static, ConfigData> {
        config_cell().lock().unwrap()
    }

    fn get_file_path() -> std::path::PathBuf {
        // java: Paths.get(FabricLoader.getInstance().getConfigDir().toString(),
        //                   FpsDisplayMod.MOD_ID + ".json")
        seams::config_dir().join("fpsdisplay.json")
    }

    pub fn load_config() -> std::sync::MutexGuard<'static, ConfigData> {
        // java: BufferedReader + GSON.fromJson(br, ConfigData.class).
        let mut parsed: Option<ConfigData> = None;
        match std::fs::read_to_string(Self::get_file_path()) {
            Ok(text) => match serde_json::from_str::<ConfigData>(&text) {
                Ok(value) => parsed = Some(value),
                Err(err) => {
                    // java: catch (Exception ex) - log message + stack trace.
                    seams::log_info("[fpsdisplay] Couldn't load configuration file, reverting to defaults!");
                    seams::print_stack_trace(&err.to_string());
                }
            },
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                // java: catch (FileNotFoundException ex) - log only, no trace.
                seams::log_info("[fpsdisplay] Couldn't load configuration file, reverting to defaults!");
            }
            Err(err) => {
                // java: catch (Exception ex) branch.
                seams::log_info("[fpsdisplay] Couldn't load configuration file, reverting to defaults!");
                seams::print_stack_trace(&err.to_string());
            }
        }

        // java: if (parsed != null) { config = parsed; }
        if let Some(value) = parsed {
            *config_cell().lock().unwrap() = value;
            CONFIG_NULL.store(false, Ordering::SeqCst);
        }

        // java: if (config == null) { config = new ConfigData(); saveConfig(); }
        if CONFIG_NULL.load(Ordering::SeqCst) {
            *config_cell().lock().unwrap() = ConfigData::default();
            CONFIG_NULL.store(false, Ordering::SeqCst);
            Self::save_config();
        }

        config_cell().lock().unwrap()
    }

    pub fn save_config() {
        // java: GSON.toJson(config) with setPrettyPrinting().
        let json = {
            let guard = config_cell().lock().unwrap();
            serde_json::to_string_pretty(&*guard).unwrap_or_default()
        };
        if let Err(err) = std::fs::write(Self::get_file_path(), json) {
            // java: catch (IOException e) - log + stack trace.
            seams::log_info("[fpsdisplay] Couldn't save configuration file.");
            seams::print_stack_trace(&err.to_string());
        }
    }
}
