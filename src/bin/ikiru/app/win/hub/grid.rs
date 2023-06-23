use std::sync::OnceLock;

use egui::ImageButton;
use image::io::Reader;
use image::ImageFormat;

use crate::app::cfg::Instance;
use crate::app::win::hub;

pub fn draw(layout: &mut hub::Layout, app: &mut Instance, ctx: &egui::Context) -> eyre::Result<()> {
    let _e = tracing::debug_span!("draw_hub_layout").entered();

    let hub::Layout::Grid { selected } = layout else {
        unreachable!();
    };

    egui::SidePanel::right("hub_panel").show_animated(ctx, selected.is_some(), |ui| {
        ui.heading("Game Info");
        ui.button("Play");
    });

    egui::CentralPanel::default()
        .show(ctx, |ui| -> eyre::Result<()> {
            egui::Grid::new("hub_grid")
                .show(ui, |ui| {
                    for (&title, game) in app.game_library.entries() {
                        if !title.is_game() {
                            continue;
                        }

                        let tex = match app.game_img_tex.get(&title) {
                            Some(tex) => tex.clone(),
                            None => {
                                // load the image file
                                let icon = game.get_asset("meta/iconTex.tga")?;
                                let icon = Reader::with_format(icon, ImageFormat::Tga).decode()?;
                                let img = egui::ColorImage::from_rgba_unmultiplied(
                                    [icon.width().try_into()?, icon.height().try_into()?],
                                    icon.to_rgba8().as_flat_samples().as_slice(),
                                );

                                // load the image into a texture
                                let name = format!("hub-grid-img-{title}");
                                let tex = ctx.load_texture(name, img, Default::default());

                                // insert the handle into the cache
                                app.game_img_tex.insert(title, tex.clone());

                                tex
                            }
                        };

                        ui.add(ImageButton::new(&tex, egui::Vec2::new(64.0, 64.0)));
                    }

                    Ok(())
                })
                .inner
        })
        .inner?;

    Ok(())
}
