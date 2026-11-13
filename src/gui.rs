//! 1:1 of config/ModMenuIntegration.java + gui/FallbackOptionScreen.java +
//! gui/ClothOptionScreen.java + gui/YaclOptionScreen.java.

use crate::config::{ConfigData, ConfigManager, KeyMode};
use crate::seams;

// ---- ModMenuIntegration.java ----
pub struct ModMenuIntegration;

impl ModMenuIntegration {
    // java: getModConfigScreenFactory() - the mod-loaded probe picks the
    // config-lib backend, with the vanilla fallback screen last.
    pub fn get_mod_config_screen_factory(parent: Option<seams::ScreenHandle>) -> seams::ScreenHandle {
        if seams::is_mod_loaded("yet_another_config_lib_v3") {
            yacl_option_screen::generate_screen(parent)
        } else if seams::is_mod_loaded("cloth-config") {
            cloth_option_screen::generate_screen(parent)
        } else {
            fallback_option_screen::generate_screen(parent)
        }
    }
}

// ---- FallbackOptionScreen.java ----
pub mod fallback_option_screen {
    use super::*;

    // java: FallbackOptionScreen(Screen parent) extends Screen with title
    // Component.translatable("text.fpsdisplay.options.fallback_title").
    pub fn generate_screen(parent: Option<seams::ScreenHandle>) -> seams::ScreenHandle {
        let parent = parent.map(Box::new);
        let mut screen = seams::ScreenHandle { parent, width: 0, height: 0 };

        // java: init() - Button doneButton = Button.builder(CommonComponents.GUI_DONE,
        //   (button) -> { this.onClose(); }).width(200).pos(this.width / 2 - 100,
        //   this.height - 27).build(); this.addRenderableWidget(doneButton);
        let done_button = seams::Button::builder_done(seams::GUI_DONE.to_string())
            .width(200)
            .pos(screen.width / 2 - 100, screen.height - 27)
            .build();
        screen.add_renderable_widget(done_button);

        // java: extractRenderState(...) draws the title at width/2, 20 and
        // the message at height/2, both CommonColors.WHITE, after super.
        screen.extract_render_state_base(0, 0, 0.0);
        let font = screen.font();
        let title = screen.title();
        seams::centered_text(&mut screen_g(), &font, &title, screen.width / 2, 20, seams::COMMON_COLORS_WHITE);
        seams::centered_text(
            &mut screen_g(),
            &font,
            &seams::translatable("text.fpsdisplay.options.fallback_message"),
            screen.width / 2,
            screen.height / 2,
            seams::COMMON_COLORS_WHITE,
        );

        // java: onClose() { this.minecraft.gui.setScreen(this.parent); }
        screen.set_screen_parent();
        screen
    }

    fn screen_g() -> seams::GuiGraphicsExtractor {
        seams::GuiGraphicsExtractor
    }
}

// ---- ClothOptionScreen.java (https://shedaniel.gitbook.io/cloth-config/) ----
pub mod cloth_option_screen {
    use super::*;

    pub fn generate_screen(parent: Option<seams::ScreenHandle>) -> seams::ScreenHandle {
        let mut builder = seams::ClothBuilder::create();
        builder.set_parent_screen(parent);
        // java: builder.setTitle(Component.translatable("text.fpsdisplay.options.title"));
        builder.set_title(seams::translatable("text.fpsdisplay.options.title"));
        // java: builder.setSavingRunnable(() -> { ConfigManager.saveConfig(); });
        builder.set_saving_runnable(Box::new(|| {
            ConfigManager::save_config();
        }));

        // java: final ConfigData configDefaults = new ConfigData();
        let config_defaults = ConfigData::default();
        // java: ConfigData config = ConfigManager.getConfig();
        let config = ConfigManager::get_config();
        let enabled = config.enabled;
        let advanced_stats = config.advanced_stats;
        let text_size = config.text_size;
        let text_color = config.text_color;
        let text_alpha = config.text_alpha;
        let text_shadows = config.text_shadows;
        let offset_top = config.offset_top;
        let offset_left = config.offset_left;
        let keybind_mode = config.keybind_mode;
        drop(config);

        // java: ConfigEntryBuilder entryBuilder = builder.entryBuilder();
        let entry_builder = builder.entry_builder();

        // java: ConfigCategory general = builder.getOrCreateCategory(Component.nullToEmpty("General"));
        let mut general = builder.get_or_create_category(String::new());

        // Entries in upstream declaration order; each setSaveConsumer writes
        // the same config field.
        general.add_entry(entry_builder.start_boolean_toggle(
            seams::translatable("text.fpsdisplay.options.enabled"),
            enabled,
            config_defaults.enabled,
            Box::new(|new_value| ConfigManager::get_config().enabled = new_value),
        ));
        general.add_entry(entry_builder.start_boolean_toggle(
            seams::translatable("text.fpsdisplay.options.advancedStats"),
            advanced_stats,
            config_defaults.advanced_stats,
            Box::new(|new_value| ConfigManager::get_config().advanced_stats = new_value),
        ));
        general.add_entry(entry_builder.start_float_field(
            seams::translatable("text.fpsdisplay.options.textSize"),
            text_size,
            config_defaults.text_size,
            Box::new(|new_value| ConfigManager::get_config().text_size = new_value),
        ));
        general.add_entry(entry_builder.start_color_field(
            seams::translatable("text.fpsdisplay.options.textColor"),
            text_color,
            config_defaults.text_color,
            Box::new(|new_value| ConfigManager::get_config().text_color = new_value),
        ));
        general.add_entry(entry_builder.start_int_slider(
            seams::translatable("text.fpsdisplay.options.textAlpha"),
            text_alpha,
            0,
            255,
            config_defaults.text_alpha,
            Box::new(|new_value| ConfigManager::get_config().text_alpha = new_value),
        ));
        general.add_entry(entry_builder.start_boolean_toggle(
            seams::translatable("text.fpsdisplay.options.textShadows"),
            text_shadows,
            config_defaults.text_shadows,
            Box::new(|new_value| ConfigManager::get_config().text_shadows = new_value),
        ));
        general.add_entry(entry_builder.start_int_field(
            seams::translatable("text.fpsdisplay.options.offsetTop"),
            offset_top,
            config_defaults.offset_top,
            Box::new(|new_value| ConfigManager::get_config().offset_top = new_value),
        ));
        general.add_entry(entry_builder.start_int_field(
            seams::translatable("text.fpsdisplay.options.offsetLeft"),
            offset_left,
            config_defaults.offset_left,
            Box::new(|new_value| ConfigManager::get_config().offset_left = new_value),
        ));
        general.add_entry(entry_builder.start_enum_selector(
            seams::translatable("text.fpsdisplay.options.keybindMode"),
            keybind_mode,
            config_defaults.keybind_mode,
            Box::new(|new_value| ConfigManager::get_config().keybind_mode = new_value),
        ));

        // java: return builder.build();
        builder.build()
    }
}

// ---- YaclOptionScreen.java (https://docs.isxander.dev/yet-another-config-lib/) ----
pub mod yacl_option_screen {
    use super::*;

    pub fn generate_screen(parent: Option<seams::ScreenHandle>) -> seams::ScreenHandle {
        // java: ConfigCategory.Builder categoryBuilder = ConfigCategory.createBuilder();
        let mut category_builder = seams::YaclCategoryBuilder::create_builder();
        // java: categoryBuilder.name(Component.translatable("text.fpsdisplay.options.title"));
        category_builder.name(seams::translatable("text.fpsdisplay.options.title"));

        let config_defaults = ConfigData::default();
        let config = ConfigManager::get_config();
        let _enabled = config.enabled;
        let _advanced_stats = config.advanced_stats;
        let _text_size = config.text_size;
        let _text_color = config.text_color;
        let _text_alpha = config.text_alpha;
        let _text_shadows = config.text_shadows;
        let _offset_top = config.offset_top;
        let _offset_left = config.offset_left;
        let _keybind_mode = config.keybind_mode;
        drop(config);

        // Option rows in upstream order; binding(default, getter, setter)
        // captured from the same config fields the cloth screen uses.
        category_builder.option(seams::YaclOption::builder(seams::BindingValue::Bool(
            config_defaults.enabled,
            Box::new(move || ConfigManager::get_config().enabled),
            Box::new(|new_value| ConfigManager::get_config().enabled = new_value),
        ))
        .name(seams::translatable("text.fpsdisplay.options.enabled"))
        .controller(Box::new(move || create_boolean_controller()))
        .build());
        category_builder.option(seams::YaclOption::builder(seams::BindingValue::Bool(
            config_defaults.advanced_stats,
            Box::new(move || ConfigManager::get_config().advanced_stats),
            Box::new(|new_value| ConfigManager::get_config().advanced_stats = new_value),
        ))
        .name(seams::translatable("text.fpsdisplay.options.advancedStats"))
        .controller(Box::new(move || create_boolean_controller()))
        .build());

        category_builder.option(seams::YaclOption::builder(seams::BindingValue::Float(
            config_defaults.text_size,
            Box::new(move || ConfigManager::get_config().text_size),
            Box::new(|new_value| ConfigManager::get_config().text_size = new_value),
        ))
        .name(seams::translatable("text.fpsdisplay.options.textSize"))
        // java: option -> FloatSliderControllerBuilder.create(option).range(0.1f, 3.0f).step(0.1f)
        .controller(Box::new(move || {
            seams::float_slider_controller(0.1, 3.0, 0.1)
        }))
        .build());

        category_builder.option(seams::YaclOption::builder(seams::BindingValue::Color(
            config_defaults.text_color,
            Box::new(move || ConfigManager::get_config().text_color),
            // java: newValue -> config.textColor = (newValue.getRGB() & 0xFFFFFF)
            Box::new(|new_value| ConfigManager::get_config().text_color = new_value & 0xFFFFFF),
        ))
        .name(seams::translatable("text.fpsdisplay.options.textColor"))
        // java: option -> ColorControllerBuilder.create(option).allowAlpha(false)
        .controller(Box::new(move || seams::color_controller(false)))
        .build());

        category_builder.option(seams::YaclOption::builder(seams::BindingValue::Int(
            config_defaults.text_alpha,
            Box::new(move || ConfigManager::get_config().text_alpha),
            Box::new(|new_value| ConfigManager::get_config().text_alpha = new_value),
        ))
        .name(seams::translatable("text.fpsdisplay.options.textAlpha"))
        // java: IntegerSliderControllerBuilder.create(option).range(0, 255).step(1)
        .controller(Box::new(move || seams::integer_slider_controller(0, 255, 1)))
        .build());

        category_builder.option(seams::YaclOption::builder(seams::BindingValue::Bool(
            config_defaults.text_shadows,
            Box::new(move || ConfigManager::get_config().text_shadows),
            Box::new(|new_value| ConfigManager::get_config().text_shadows = new_value),
        ))
        .name(seams::translatable("text.fpsdisplay.options.textShadows"))
        .controller(Box::new(move || create_boolean_controller()))
        .build());

        category_builder.option(seams::YaclOption::builder(seams::BindingValue::Int(
            config_defaults.offset_top,
            Box::new(move || ConfigManager::get_config().offset_top),
            Box::new(|new_value| ConfigManager::get_config().offset_top = new_value),
        ))
        .name(seams::translatable("text.fpsdisplay.options.offsetTop"))
        // java: IntegerFieldControllerBuilder::create
        .controller(Box::new(move || seams::integer_field_controller()))
        .build());

        category_builder.option(seams::YaclOption::builder(seams::BindingValue::Int(
            config_defaults.offset_left,
            Box::new(move || ConfigManager::get_config().offset_left),
            Box::new(|new_value| ConfigManager::get_config().offset_left = new_value),
        ))
        .name(seams::translatable("text.fpsdisplay.options.offsetLeft"))
        .controller(Box::new(move || seams::integer_field_controller()))
        .build());

        category_builder.option(seams::YaclOption::builder(seams::BindingValue::Enum(
            config_defaults.keybind_mode,
            Box::new(move || ConfigManager::get_config().keybind_mode),
            Box::new(|new_value: KeyMode| ConfigManager::get_config().keybind_mode = new_value),
        ))
        .name(seams::translatable("text.fpsdisplay.options.keybindMode"))
        // java: option -> EnumControllerBuilder.create(option).enumClass(ConfigData.KeyMode.class)
        .controller(Box::new(move || seams::enum_controller()))
        .build());

        // java: YetAnotherConfigLib.Builder builder = YetAnotherConfigLib.createBuilder();
        let mut builder = seams::YaclBuilder::create_builder();
        builder.title(seams::translatable("text.fpsdisplay.options.title"));
        builder.category(category_builder.build());
        // java: builder.save(() -> ConfigManager.saveConfig());
        builder.save(Box::new(|| {
            ConfigManager::save_config();
        }));
        // java: return builder.build().generateScreen(parent);
        builder.build().generate_screen(parent)
    }

    // java: private static BooleanControllerBuilder createBooleanController(Option<Boolean> option) {
    //   return BooleanControllerBuilder.create(option).yesNoFormatter().coloured(true); }
    fn create_boolean_controller() -> seams::ControllerSpec {
        seams::boolean_controller()
    }
}
