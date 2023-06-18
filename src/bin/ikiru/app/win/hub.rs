use std::sync::OnceLock;

use egui::{Align, Button, Id, Layout, RichText, Sense};

use crate::app::cfg::Instance;
use crate::cfg::HubViewMode;
use ikiru::misc::TitleId;

use super::title_bar::TitleBar;

pub struct Window {
    state: HubWindowState,
    children: HubChildren,
}

impl Window {
    pub fn new_open() -> Self {
        Self {
            state: HubWindowState::Open(),
            children: HubChildren {},
        }
    }

    pub fn new_closed() -> Self {
        Self {
            state: HubWindowState::Closed,
            children: HubChildren {},
        }
    }

    pub fn show(&mut self, app: &mut Instance, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match &mut self.state {
            HubWindowState::Open() => {
                TitleBar("hub_title_bar").ui(app, ctx, frame, |app, ctx, frame, ui| {
                    ui.menu_button("File", |ui| {
                        ui.menu_button("Load recent", |ui| {
                            ui.label("No recent files");
                        });
                        if ui.button("Load folder...").clicked() {}
                        if ui.button("Load archive...").clicked() {}

                        ui.separator();

                        if ui.button("Open explorer").clicked() {
                            // frame.open_url(app.cfg_dir);
                        }
                        if ui.button("Preferences...").clicked() {}
                        if ui.button("Exit").clicked() {
                            frame.close();
                        }
                    });

                    ui.menu_button("View", |ui| {
                        if ui.button("Refresh").clicked() {}

                        ui.separator();

                        if ui.button("Show console").clicked() {}
                        if ui.button("Show log").clicked() {}

                        ui.separator();

                        ui.group(|ui| {
                            // ui.label("Theme");
                            ui.radio_value(&mut app.cfg.view, HubViewMode::Grid, "Grid");
                            ui.radio_value(&mut app.cfg.view, HubViewMode::List, "List");
                            ui.radio_value(&mut app.cfg.view, HubViewMode::Pro, "Pro");
                        });
                    });

                    ui.menu_button("Help", |ui| {
                        if ui.button("Check for updates...").clicked() {}
                        if ui.button("About").clicked() {}
                    });
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Hello World!");
                });
            }
            HubWindowState::Closed => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Goodbye World!");
                });
            }
        }
    }
}

pub enum HubWindowState {
    Open(),
    Closed,
}

pub struct HubChildren {
    // input_cfg: InputSettingsWindow,
    // global_cfg: GlobalSettings,
    // game_cfg: Vec<GameSettings>
}

pub struct GameSettings {
    title: TitleId,
    // input_cfg: InputSettings,
    // graphic_packs: Vec<GraphicPack>,
}
