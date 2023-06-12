use crate::util::TitleId;

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

    pub fn show(&mut self, ctx: &egui::Context) {
        match &mut self.state {
            HubWindowState::Open() => {
                egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.menu_button("File", |ui| if ui.button("S").clicked() {})
                    })
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
