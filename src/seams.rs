//! Host seam surface: every Minecraft/fabric type the port touches, with
//! no-op defaults so the standalone check builds without the engine. The
//! engine stage binds the real implementations (same pattern as
//! ping-wheel/src/seams.rs).

use std::path::PathBuf;
use std::sync::OnceLock;

use crate::config::KeyMode;
use crate::fps_history::FpsHistory;

// ---- logging (java: LogManager.getLogger()) ----
pub fn log_info(message: &str) {
    // Default: drop on the floor like log4j with no appender wired.
    let _ = message;
}

// java: ex.printStackTrace().
pub fn print_stack_trace(_message: &str) {}

// java: FabricLoader.getInstance().getConfigDir().
pub fn config_dir() -> PathBuf {
    PathBuf::from("config")
}

// java: FabricLoader.getInstance().isModLoaded(id).
pub fn is_mod_loaded(_id: &str) -> bool {
    false
}

// ---- client lifecycle (fabric event seams) ----
pub struct ClientTickContext;

impl ClientTickContext {
    // java: client.getFps().
    pub fn get_fps(&self) -> i32 {
        0
    }
}

pub fn register_start_client_tick(_handler: Box<dyn Fn(&ClientTickContext) + Send>) {}
pub fn register_end_client_tick(_handler: Box<dyn Fn(&ClientTickContext) + Send>) {}
pub fn register_client_stopping(_handler: Box<dyn Fn(&ClientTickContext) + Send>) {}

// ---- keymapping (KeyMapping.Category.register + KeyMappingHelper) ----
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    Keyboard,
}

// java: InputConstants.UNKNOWN.getValue() - the "unknown" keycode.
pub const INPUT_UNKNOWN_KEY: i32 = -1;

pub struct KeyMappingCategory {
    pub id: String,
}

pub fn register_key_mapping_category(_path: &str) -> KeyMappingCategory {
    KeyMappingCategory { id: String::new() }
}

pub struct KeyMappingHandle;

impl KeyMappingHandle {
    pub fn new(
        _name: &str,
        _kind: InputType,
        _key: i32,
        _category: &KeyMappingCategory,
    ) -> KeyMappingHandle {
        KeyMappingHandle
    }
    // java: toggleKeybinding.consumeClick().
    pub fn consume_click(&self) -> bool {
        false
    }
    // java: toggleKeybinding.isDown().
    pub fn is_down(&self) -> bool {
        false
    }
}

// ---- Hud mixin surface (HudMixin.render inject carrier is engine-stage) ----
pub struct HudRenderState {
    // java: @Shadow private boolean isHidden.
    pub is_hidden: bool,
}

pub struct GuiGraphicsExtractor;

impl GuiGraphicsExtractor {
    // java: graphics.pose() -> Matrix3x2fStack. The default seam keeps no
    // pose state; the engine stage binds the real matrix stack.
    pub fn pose(&mut self) -> Matrix3x2fStack {
        Matrix3x2fStack
    }
    // java: graphics.text(textRenderer, text, x, y, color, shadowed).
    pub fn text(&mut self, _font: &Font, _text: &str, _x: i32, _y: i32, _color: i32, _shadowed: bool) {}
}

pub struct Matrix3x2fStack;

impl Matrix3x2fStack {
    pub fn push_matrix(&mut self) {}
    pub fn translate(&mut self, _x: f32, _y: f32) {}
    pub fn scale(&mut self, _x: f32, _y: f32) {}
    pub fn pop_matrix(&mut self) {}
}

pub struct Font;

impl Font {
    // java: client.font.width(text).
    pub fn width(&self, _text: &str) -> i32 {
        0
    }
    // java: client.font.lineHeight.
    pub fn line_height(&self) -> i32 {
        9
    }
}

pub struct Minecraft {
    pub font: Font,
}

impl Minecraft {
    // java: Minecraft.getInstance().
    pub fn get_instance() -> Minecraft {
        Minecraft { font: Font }
    }
    // java: client.getFps().
    pub fn get_fps(&self) -> i32 {
        0
    }
    // java: client.debugEntries.isOverlayVisible().
    pub fn debug_overlay_visible(&self) -> bool {
        false
    }
    // java: client.getWindow().getGuiScale() (double).
    pub fn window_gui_scale(&self) -> f64 {
        0.0
    }
    // java: client.getWindow().getGuiScaledWidth().
    pub fn window_gui_scaled_width(&self) -> i32 {
        0
    }
    // java: client.getWindow().getGuiScaledHeight().
    pub fn window_gui_scaled_height(&self) -> i32 {
        0
    }
}

// ---- FpsDisplayMod statics live here because java statics cross classes ----
pub static SHOW_OVERLAY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);
pub static FPS_HISTORY: OnceLock<std::sync::Mutex<FpsHistory>> = OnceLock::new();

pub fn fps_history() -> &'static std::sync::Mutex<FpsHistory> {
    FPS_HISTORY.get_or_init(|| std::sync::Mutex::new(FpsHistory::default()))
}

// ---- gui seams (Screen / Button / Component / config-lib builders) ----
pub fn translatable(_key: &str) -> String {
    String::new()
}

pub const GUI_DONE: &str = "gui.done";
pub const COMMON_COLORS_WHITE: i32 = 0xFFFFFF;

pub struct ScreenHandle {
    pub parent: Option<Box<ScreenHandle>>,
    pub width: i32,
    pub height: i32,
}

impl ScreenHandle {
    pub fn title(&self) -> String {
        String::new()
    }
    pub fn font(&self) -> Font {
        Font
    }
    pub fn add_renderable_widget(&mut self, _button: Button) {}
    pub fn on_close(&mut self) {}
    // java: this.minecraft.gui.setScreen(this.parent).
    pub fn set_screen_parent(&mut self) {}
    pub fn extract_render_state_base(&mut self, _mouse_x: i32, _mouse_y: i32, _delta: f32) {}
}

pub struct Button;

impl Button {
    pub fn builder_done(_label: String) -> ButtonBuilder {
        ButtonBuilder { width: 0, x: 0, y: 0 }
    }
}

pub struct ButtonBuilder {
    width: i32,
    x: i32,
    y: i32,
}

impl ButtonBuilder {
    pub fn width(self, _w: i32) -> ButtonBuilder {
        self
    }
    pub fn pos(self, _x: i32, _y: i32) -> ButtonBuilder {
        self
    }
    pub fn build(self) -> Button {
        Button
    }
}

pub fn centered_text(_g: &mut GuiGraphicsExtractor, _font: &Font, _text: &str, _x: i32, _y: i32, _color: i32) {}

// ---- Cloth Config builder surface ----
pub struct ClothBuilder;
pub struct ClothCategory;
pub struct ClothEntryBuilder;

impl ClothBuilder {
    pub fn create() -> ClothBuilder {
        ClothBuilder
    }
    pub fn set_parent_screen(&mut self, _parent: Option<ScreenHandle>) {}
    pub fn set_title(&mut self, _title: String) {}
    pub fn set_saving_runnable(&mut self, _f: Box<dyn FnMut() + Send>) {}
    pub fn entry_builder(&mut self) -> ClothEntryBuilder {
        ClothEntryBuilder
    }
    pub fn get_or_create_category(&mut self, _title: String) -> ClothCategory {
        ClothCategory
    }
    pub fn build(self) -> ScreenHandle {
        ScreenHandle { parent: None, width: 0, height: 0 }
    }
}

impl ClothEntryBuilder {
    pub fn start_boolean_toggle(
        &self,
        _name: String,
        _value: bool,
        _default: bool,
        _save: Box<dyn FnMut(bool) + Send>,
    ) -> ClothEntry {
        ClothEntry
    }
    pub fn start_float_field(
        &self,
        _name: String,
        _value: f32,
        _default: f32,
        _save: Box<dyn FnMut(f32) + Send>,
    ) -> ClothEntry {
        ClothEntry
    }
    pub fn start_color_field(
        &self,
        _name: String,
        _value: i32,
        _default: i32,
        _save: Box<dyn FnMut(i32) + Send>,
    ) -> ClothEntry {
        ClothEntry
    }
    pub fn start_int_slider(
        &self,
        _name: String,
        _value: i32,
        _min: i32,
        _max: i32,
        _default: i32,
        _save: Box<dyn FnMut(i32) + Send>,
    ) -> ClothEntry {
        ClothEntry
    }
    pub fn start_int_field(
        &self,
        _name: String,
        _value: i32,
        _default: i32,
        _save: Box<dyn FnMut(i32) + Send>,
    ) -> ClothEntry {
        ClothEntry
    }
    pub fn start_enum_selector(
        &self,
        _name: String,
        _value: KeyMode,
        _default: KeyMode,
        _save: Box<dyn FnMut(KeyMode) + Send>,
    ) -> ClothEntry {
        ClothEntry
    }
}

pub struct ClothEntry;

impl ClothEntry {
    pub fn build(self) {}
}

impl ClothCategory {
    pub fn add_entry(&mut self, _entry: ClothEntry) {}
}

// ---- YACL builder surface ----
pub struct YaclCategoryBuilder;
pub struct YaclBuilder;

impl YaclCategoryBuilder {
    pub fn create_builder() -> YaclCategoryBuilder {
        YaclCategoryBuilder
    }
    pub fn name(&mut self, _name: String) {}
    pub fn option(&mut self, _o: YaclOption) {}
    pub fn build(self) -> YaclCategory {
        YaclCategory
    }
}

pub struct YaclCategory;

impl YaclBuilder {
    pub fn create_builder() -> YaclBuilder {
        YaclBuilder
    }
    pub fn title(&mut self, _title: String) {}
    pub fn category(&mut self, _c: YaclCategory) {}
    pub fn save(&mut self, _f: Box<dyn FnMut() + Send>) {}
    pub fn build(self) -> YaclLib {
        YaclLib
    }
}

pub struct YaclLib;

impl YaclLib {
    pub fn generate_screen(self, _parent: Option<ScreenHandle>) -> ScreenHandle {
        ScreenHandle { parent: None, width: 0, height: 0 }
    }
}

pub struct YaclOption;

pub struct YaclOptionBuilder {
    pub option: YaclOption,
}

pub enum BindingValue {
    Bool(bool, Box<dyn Fn() -> bool + Send>, Box<dyn Fn(bool) + Send>),
    Float(f32, Box<dyn Fn() -> f32 + Send>, Box<dyn Fn(f32) + Send>),
    Color(i32, Box<dyn Fn() -> i32 + Send>, Box<dyn Fn(i32) + Send>),
    Int(i32, Box<dyn Fn() -> i32 + Send>, Box<dyn Fn(i32) + Send>),
    Enum(KeyMode, Box<dyn Fn() -> KeyMode + Send>, Box<dyn Fn(KeyMode) + Send>),
}

impl YaclOption {
    pub fn builder(binding: BindingValue) -> YaclOptionBuilder {
        let _ = binding;
        YaclOptionBuilder { option: YaclOption }
    }
}

impl YaclOptionBuilder {
    pub fn name(self, _name: String) -> YaclOptionBuilder {
        self
    }
    pub fn controller(self, _c: Box<dyn FnMut() -> ControllerSpec + Send>) -> YaclOptionBuilder {
        self
    }
    pub fn build(self) -> YaclOption {
        YaclOption
    }
}

// java: BooleanControllerBuilder / FloatSliderControllerBuilder /
// ColorControllerBuilder / IntegerSliderControllerBuilder /
// IntegerFieldControllerBuilder / EnumControllerBuilder collapse into one
// spec surface; the field set is the union of their builder methods.
pub struct ControllerSpec {
    pub kind: ControllerKind,
    pub range_min: f64,
    pub range_max: f64,
    pub step: f64,
    pub allow_alpha: bool,
    pub yes_no_formatter: bool,
    pub coloured: bool,
}

#[derive(PartialEq)]
pub enum ControllerKind {
    BooleanYesNo,
    FloatSlider,
    Color,
    IntegerSlider,
    IntegerField,
    Enum,
}

pub fn boolean_controller() -> ControllerSpec {
    ControllerSpec {
        kind: ControllerKind::BooleanYesNo,
        range_min: 0.0,
        range_max: 0.0,
        step: 0.0,
        allow_alpha: false,
        yes_no_formatter: true,
        coloured: true,
    }
}

pub fn float_slider_controller(min: f32, max: f32, step: f32) -> ControllerSpec {
    ControllerSpec {
        kind: ControllerKind::FloatSlider,
        range_min: min as f64,
        range_max: max as f64,
        step: step as f64,
        allow_alpha: false,
        yes_no_formatter: false,
        coloured: false,
    }
}

pub fn color_controller(allow_alpha: bool) -> ControllerSpec {
    ControllerSpec {
        kind: ControllerKind::Color,
        range_min: 0.0,
        range_max: 0.0,
        step: 0.0,
        allow_alpha,
        yes_no_formatter: false,
        coloured: false,
    }
}

pub fn integer_slider_controller(min: i32, max: i32, step: i32) -> ControllerSpec {
    ControllerSpec {
        kind: ControllerKind::IntegerSlider,
        range_min: min as f64,
        range_max: max as f64,
        step: step as f64,
        allow_alpha: false,
        yes_no_formatter: false,
        coloured: false,
    }
}

pub fn integer_field_controller() -> ControllerSpec {
    ControllerSpec {
        kind: ControllerKind::IntegerField,
        range_min: 0.0,
        range_max: 0.0,
        step: 0.0,
        allow_alpha: false,
        yes_no_formatter: false,
        coloured: false,
    }
}

pub fn enum_controller() -> ControllerSpec {
    ControllerSpec {
        kind: ControllerKind::Enum,
        range_min: 0.0,
        range_max: 0.0,
        step: 0.0,
        allow_alpha: false,
        yes_no_formatter: false,
        coloured: false,
    }
}
