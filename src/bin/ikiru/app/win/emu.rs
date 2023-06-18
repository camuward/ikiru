use std::sync::Arc;
use std::thread::JoinHandle;

// use crate::cfg::Cfg;
use ikiru::emu::Emulator;
// use ikiru::misc::TitleId;
// use ikiru::misc::TitleId;

/// A running emulator instance.
pub struct Window {
    /// Window ID. Used to persist window state between frames.
    pub id: uuid::Uuid,
    pub thread: JoinHandle<()>,
    pub emu: Arc<Emulator>,
    pub show_title_bar: bool,
}

impl Window {
    fn egui_window(&self) -> egui::Window {
        let time = humantime::format_duration(self.emu.uptime());
        let title = format!("ikiru - {time}");

        egui::Window::new(title)
            .id(egui::Id::new(self.id))
            .title_bar(self.show_title_bar)
            .resizable(true)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let uptime = humantime::format_duration(self.emu.uptime());
        let title = format!("ikiru - {}", uptime);

        egui::Window::new(title)
            .id(egui::Id::new(self.id))
            .show(ctx, |ui| {
                egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.menu_button("File", |ui| if ui.button("S").clicked() {})
                    })
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Hello World!");
                });
            });
    }
}
