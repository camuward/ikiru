//! GUI and application code.
use std::path::Path;

mod cfg;
mod emu;
mod hub;

pub struct App {
    hub: hub::Window,
    emu: Vec<emu::Window>,
}

impl App {
    /// Create a new instance, opening the hub window without starting any games.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        todo!()
    }

    /// Create a new instance, starting the provided title ID. The hub window
    /// be hidden unless the user manually closes the game via `Emulation >
    /// Stop`.
    pub fn new_load_title(
        title: ikiru::TitleId,
        include_dirs: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> Self {
        todo!()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.hub.show(ctx);

        for emu in &mut self.emu {
            emu.show(ctx);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}
