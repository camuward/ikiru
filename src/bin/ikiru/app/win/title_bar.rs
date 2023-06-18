use std::sync::OnceLock;

use crate::app::cfg::Instance;
use crate::cfg::HubLayout;

use eframe::Frame;
use egui::{Align, Button, Context, Id, Layout, RichText, Sense};

pub struct TitleBar<'a>(pub &'a str);

impl TitleBar<'_> {
    pub fn ui<F>(&self, app: &mut Instance, ctx: &Context, frame: &mut Frame, add_contents: F)
    where
        F: FnOnce(&mut Instance, &Context, &mut Frame, &mut egui::Ui),
    {
        // grab the handle for the icon texture
        static ICON_TEX: OnceLock<egui::TextureHandle> = OnceLock::new();
        let tex = ICON_TEX.get_or_init(|| {
            // let icon = include_bytes!("../../../assets/icon.png");
            // ctx.texture_from_memory_rgba32("icon", icon, 128, 128);

            ctx.load_texture(
                "title-bar-icon",
                egui::ColorImage::example(),
                Default::default(),
            )
        });

        // draw the title bar
        egui::TopBottomPanel::top(Id::new(self.0)).show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // paint the icon
                ui.image(tex, egui::Vec2::new(16.0, 16.0));

                let title_bar = ui.max_rect();
                let response = ui.interact(title_bar, Id::new("title_bar"), Sense::click());

                if response.double_clicked() {
                    frame.set_maximized(!frame.info().window_info.maximized);
                } else if response.is_pointer_button_down_on() {
                    frame.drag_window();
                }

                ui.allocate_ui_at_rect(title_bar, |ui| {
                    ui.add_space(24.0);

                    add_contents(app, ctx, frame, ui);

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.spacing_mut().item_spacing.x = 2.0;
                        ui.visuals_mut().button_frame = false;
                        // ui.add_space(8.0);

                        let close = ui.add(Button::new(RichText::new("✖").size(16.0)));
                        if close.clicked() {
                            frame.close();
                        }

                        let max = ui.add(Button::new(RichText::new("🗖").size(12.0)));
                        if max.clicked() {
                            let maximized = frame.info().window_info.maximized;
                            frame.set_maximized(!maximized);
                        }

                        let min = ui.add(Button::new(RichText::new("🗕").size(12.0)));
                        if min.clicked() {
                            frame.set_minimized(true);
                        }
                    });

                    ui.separator();
                });
            })
        });
    }
}
