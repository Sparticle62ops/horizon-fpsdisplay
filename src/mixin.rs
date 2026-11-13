//! 1:1 of mixin/HudMixin.java. The @Mixin carrier (Hud + @Shadow isHidden +
//! the TAIL injection on extractRenderState) is engine-stage; the injected
//! body and the @Unique renderText helper are ported exactly.

use crate::config::ConfigManager;
use crate::seams;
use crate::MOD_ID;

pub struct HudMixin;

impl HudMixin {
    // java: @Inject(at = @At("TAIL"), method = "extractRenderState")
    //   public void render(GuiGraphicsExtractor graphics, DeltaTracker deltaTracker, CallbackInfo ci)
    pub fn render(graphics: &mut seams::GuiGraphicsExtractor, _delta_tracker: ()) {
        let client = seams::Minecraft::get_instance();
        let config = ConfigManager::get_config();

        // java: if (!this.isHidden && !client.debugEntries.isOverlayVisible()
        //   && config.enabled && config.textAlpha > 3 && FpsDisplayMod.ShowOverlay)
        let hud_state = seams::HudRenderState { is_hidden: false };
        if !hud_state.is_hidden
            && !client.debug_overlay_visible()
            && config.enabled
            && config.text_alpha > 3
            && seams::SHOW_OVERLAY.load(std::sync::atomic::Ordering::SeqCst)
        {
            let text: String;
            if !config.advanced_stats {
                // java: text = client.getFps() + " FPS";
                text = format!("{} FPS", client.get_fps());
            } else {
                // java: text = String.format("%d FPS (%d min | %d avg | %d max)", ...);
                let min = seams::fps_history().lock().unwrap().get_minimum();
                let avg = seams::fps_history().lock().unwrap().get_average();
                let max = seams::fps_history().lock().unwrap().get_maximum();
                text = format!(
                    "{} FPS ({} min | {} avg | {} max)",
                    client.get_fps(),
                    min,
                    avg,
                    max
                );
            }

            // java: int textPosX = config.offsetLeft; int textPosY = config.offsetTop;
            let mut text_pos_x = config.offset_left;
            let mut text_pos_y = config.offset_top;

            // java: double guiScale = client.getWindow().getGuiScale();
            let gui_scale = client.window_gui_scale();
            // java: if (guiScale > 0) { textPosX /= guiScale; textPosY /= guiScale; }
            // java compound assignment on an int truncates toward zero.
            if gui_scale > 0.0 {
                text_pos_x = (text_pos_x as f64 / gui_scale) as i32;
                text_pos_y = (text_pos_y as f64 / gui_scale) as i32;
            }

            // java: int maxTextPosX = client.getWindow().getGuiScaledWidth()
            //   - client.font.width(text);
            //   int maxTextPosY = client.getWindow().getGuiScaledHeight()
            //   - client.font.lineHeight;
            let max_text_pos_x = client.window_gui_scaled_width() - client.font.width(&text);
            let max_text_pos_y = client.window_gui_scaled_height() - client.font.line_height();
            // java: Math.min clamps so text stays in screenspace.
            text_pos_x = text_pos_x.min(max_text_pos_x);
            text_pos_y = text_pos_y.min(max_text_pos_y);

            // java: int textColor = ((config.textAlpha & 0xFF) << 24) | (config.textColor & 0xFFFFFF);
            let text_color =
                ((config.text_alpha & 0xFF) << 24) | (config.text_color & 0xFFFFFF);

            Self::render_text(
                graphics,
                &client.font,
                &text,
                text_pos_x,
                text_pos_y,
                text_color,
                config.text_size,
                config.text_shadows,
            );
        }

        let _ = MOD_ID;
    }

    // java: @Unique private void renderText(GuiGraphicsExtractor graphics,
    //   Font textRenderer, String text, int x, int y, int color, float scale,
    //   boolean shadowed)
    fn render_text(
        graphics: &mut seams::GuiGraphicsExtractor,
        text_renderer: &seams::Font,
        text: &str,
        x: i32,
        y: i32,
        color: i32,
        scale: f32,
        shadowed: bool,
    ) {
        if scale != 1.0 {
            // java: Matrix3x2fStack matrixStack = graphics.pose();
            //   matrixStack.pushMatrix(); matrixStack.translate(x, y);
            //   matrixStack.scale(scale, scale); matrixStack.translate(-x, -y);
            //   graphics.text(...); matrixStack.popMatrix();
            let mut matrix_stack = graphics.pose();
            matrix_stack.push_matrix();
            matrix_stack.translate(x as f32, y as f32);
            matrix_stack.scale(scale, scale);
            matrix_stack.translate(-(x as f32), -(y as f32));
            graphics.text(text_renderer, text, x, y, color, shadowed);
            matrix_stack.pop_matrix();
        } else {
            graphics.text(text_renderer, text, x, y, color, shadowed);
        }
    }
}
