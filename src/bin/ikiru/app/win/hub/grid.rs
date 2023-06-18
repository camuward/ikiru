use std::sync::OnceLock;

use egui::ImageButton;

use crate::app::cfg::Instance;

pub fn draw(app: &mut Instance, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Grid");
        egui::Grid::new("hub_grid").show(ui, |ui| {
            for i in 0..100 {
                static TEX: OnceLock<egui::TextureHandle> = OnceLock::new();
                let t = TEX.get_or_init(|| {
                    ctx.load_texture(
                        "hub-grid-img",
                        egui::ColorImage::example(),
                        Default::default(),
                    )
                });

                ui.add(ImageButton::new(t, egui::Vec2::new(64.0, 128.0)));
            }
        });
    });
}
