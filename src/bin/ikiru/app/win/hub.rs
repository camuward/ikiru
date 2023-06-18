use std::sync::OnceLock;

use egui::{Align, Button, Id, ImageButton, Layout, RichText, Sense};

use crate::app::cfg::Instance;
use crate::app::win::title_bar::TitleBar;
use crate::cfg::HubLayout;
use ikiru::misc::TitleId;

mod grid;

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
        let HubWindowState::Open() = self.state else {
            return;
        };

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

                ui.label("Layout");
                ui.group(|ui| {
                    ui.radio_value(&mut app.layout, HubLayout::Grid, "Grid");
                    ui.radio_value(&mut app.layout, HubLayout::List, "List");
                    ui.radio_value(&mut app.layout, HubLayout::Pro, "Pro");
                });
            });

            ui.menu_button("Help", |ui| {
                if ui.button("Check for updates...").clicked() {}
                if ui.button("About").clicked() {}
            });
        });

        match app.layout {
            HubLayout::Grid => draw_grid(app, ctx),
            HubLayout::List => draw_list(app, ctx),
            HubLayout::Pro => draw_pro(app, ctx),
        }
    }
}

fn draw_grid(app: &mut Instance, ctx: &egui::Context) {
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

fn draw_list(app: &mut Instance, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("List");
    });
}

fn draw_pro(app: &mut Instance, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Pro");
    });
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
