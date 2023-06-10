use std::sync::Arc;
use std::thread::JoinHandle;

// use crate::cfg::Cfg;
use crate::emu::Emulator;
// use crate::util::TitleId;
// use crate::util::TitleId;

pub struct Window {
    pub thread: JoinHandle<()>,
    pub emu: Arc<Emulator>,
}

impl Window {
    pub fn show(&mut self, ctx: &egui::Context) {
        let uptime = humantime::format_duration(self.emu.uptime());
        egui::Window::new(format!("ikiru - {uptime}")).show(ctx, |ui| {
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
