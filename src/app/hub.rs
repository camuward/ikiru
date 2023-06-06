pub struct Window {
    state: HubWindowState,
    children: HubChildren,
}

pub enum HubWindowState {
    Open(),
    Closed,
}

pub struct HubChildren {
    input_cfg: InputSettingsWindow,
    global_cfg: GlobalSettings,
    game_cfg: Vec<GameSettings>
}


pub struct GameSettings {
    title_id: TitleId,
    input_cfg: InputSettings,
    graphic_packs: Vec<GraphicPack>,
}

