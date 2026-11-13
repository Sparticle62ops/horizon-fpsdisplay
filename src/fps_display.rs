//! 1:1 of FpsDisplayMod.java (ClientModInitializer entry).

use std::sync::atomic::Ordering;

use crate::config::{ConfigData, ConfigManager, KeyMode};
use crate::seams;

// java: public static final String MOD_ID = "fpsdisplay";
pub const MOD_ID: &str = "fpsdisplay";

// java: public static FpsHistory FpsHistory = new FpsHistory(); lives in
// seams::fps_history because java statics cross classes.

pub struct FpsDisplayMod;

impl FpsDisplayMod {
    // java: onInitializeClient().
    pub fn on_initialize_client() {
        // java: ConfigData config = ConfigManager.loadConfig(); the guard
        // field below stands in for the captured local across the three
        // lambdas.
        ConfigManager::load_config();

        // java: KeyMapping.Category keybinCategory =
        //   KeyMapping.Category.register(Identifier.fromNamespaceAndPath(MOD_ID, "category"));
        let keybin_category = seams::register_key_mapping_category("fpsdisplay:category");

        // java: KeyMapping toggleKeybinding = KeyMappingHelper.registerKeyMapping(new KeyMapping(
        //   "key.fpsdisplay.toggleOverlay", InputConstants.Type.KEYBOARD,
        //   InputConstants.UNKNOWN.getValue(), keybinCategory));
        let toggle_keybinding = seams::KeyMappingHandle::new(
            "key.fpsdisplay.toggleOverlay",
            seams::InputType::Keyboard,
            seams::INPUT_UNKNOWN_KEY,
            &keybin_category,
        );

        // java: ClientTickEvents.START_CLIENT_TICK.register(client -> {
        //   int currentFps = client.getFps(); FpsHistory.add(currentFps); });
        seams::register_start_client_tick(Box::new(|client| {
            let current_fps = client.get_fps();
            let mut history = seams::fps_history().lock().unwrap();
            history.add(current_fps);
        }));

        // java: ClientTickEvents.END_CLIENT_TICK.register(client -> {
        //   while (toggleKeybinding.consumeClick()
        //       && config.keybindMode == ConfigData.KeyMode.Toggle) {
        //     config.enabled = !config.enabled; }
        //   if (config.keybindMode == ConfigData.KeyMode.PushToShow) {
        //     ShowOverlay = toggleKeybinding.isDown(); }
        //   else { ShowOverlay = config.enabled; } });
        seams::register_end_client_tick(Box::new(move |_client| {
            let mut config = ConfigManager::get_config();
            while toggle_keybinding.consume_click() && config.keybind_mode == KeyMode::Toggle {
                config.enabled = !config.enabled;
            }
            if config.keybind_mode == KeyMode::PushToShow {
                seams::SHOW_OVERLAY.store(toggle_keybinding.is_down(), Ordering::SeqCst);
            } else {
                seams::SHOW_OVERLAY.store(config.enabled, Ordering::SeqCst);
            }
        }));

        // java: ClientLifecycleEvents.CLIENT_STOPPING.register(client -> {
        //   ConfigManager.saveConfig(); });
        seams::register_client_stopping(Box::new(|_client| {
            ConfigManager::save_config();
        }));

        // ConfigData is named in the java signature; keep the alias honest.
        let _: fn() -> std::sync::MutexGuard<'static, ConfigData> = ConfigManager::load_config;
    }
}
