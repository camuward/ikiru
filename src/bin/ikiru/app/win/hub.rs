use std::sync::OnceLock;

use egui::{Align, Button, Id, ImageButton, RichText, Sense};
use serde::{Deserialize, Serialize};
use strum::EnumDiscriminants;

use crate::app::cfg::Instance;
use crate::app::win::title_bar::TitleBar;
use ikiru::game::TitleId;

mod grid;

#[derive(Debug, Default)]
pub struct Window {
    pub is_open: bool,

    pub layout: Layout,
    pub children: Children,
}

impl Window {
    pub fn show(
        &mut self,
        app: &mut Instance,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
    ) -> eyre::Result<()> {
        // if !self.is_open {
        //     return Ok(());
        // }

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
                    ui.label("Layout");
                    ui.radio_value(&mut self.layout, Layout::Grid { selected: None }, "Grid");
                    ui.radio_value(&mut self.layout, Layout::List, "List");
                    ui.radio_value(&mut self.layout, Layout::Pro, "Pro");
                });
            });

            ui.menu_button("Help", |ui| {
                if ui.button("Check for updates...").clicked() {}
                if ui.button("About").clicked() {}
            });
        });

        self.layout.draw(app, ctx)
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumDiscriminants)]
#[strum_discriminants(derive(Default, Serialize, Deserialize))]
#[strum_discriminants(name(LayoutType))]
pub enum Layout {
    #[strum_discriminants(default)]
    Grid {
        selected: Option<TitleId>,
    },
    List,
    Pro,
}

#[derive(Debug, Default)]
pub struct Children {
    // input_cfg: InputSettingsWindow,
    // global_cfg: GlobalSettings,
    // game_cfg: Vec<GameSettings>
}

pub struct GameSettings {
    title: TitleId,
    // input_cfg: InputSettings,
    // graphic_packs: Vec<GraphicPack>,
}

impl Layout {
    pub fn draw(&mut self, app: &mut Instance, ctx: &egui::Context) -> eyre::Result<()> {
        match self {
            Layout::Grid { .. } => grid::draw(self, app, ctx),
            Layout::List => Ok(draw_list(app, ctx)),
            Layout::Pro => Ok(draw_pro(app, ctx)),
        }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self::Grid { selected: None }
    }
}
