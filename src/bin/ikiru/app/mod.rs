//! GUI and application code.
use tokio::runtime::Runtime;

use crate::gfx;
use ikiru::emu::EmuParams;
use ikiru::misc::TitleId;

pub mod cfg;

pub mod win {
    pub mod emu;
    pub mod hub;
}
// pub mod misc;

pub struct App {
    rt: Runtime,
    app: cfg::Instance,

    // windows
    hub: win::hub::Window,
    emu: Vec<win::emu::Window>,
}

impl App {
    /// Create a new instance, opening the hub window without starting any games.
    pub fn new(cc: &eframe::CreationContext<'_>, app: cfg::Instance) -> eyre::Result<Self> {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Ok(Self {
            app,
            rt: Runtime::new()?,
            // cfg: cfg::Cfg::load(Path::new("cfg.toml")),
            hub: win::hub::Window::new_open(),
            emu: Vec::new(),
        })
    }

    /// Create a new instance, starting the provided title ID. The hub window
    /// be hidden unless the user manually closes the game via `Emulation >
    /// Stop`.
    pub fn new_load_title(
        cc: &eframe::CreationContext<'_>,
        app: cfg::Instance,
        title: TitleId,
    ) -> eyre::Result<Self> {
        let _cfg = app.game_cfgs.get_cfg(title);

        Ok(Self {
            app,
            rt: Runtime::new()?,
            // cfg: cfg::Cfg::load(Path::new("cfg.toml")),
            hub: win::hub::Window::new_closed(),
            emu: vec![gfx::spawn(EmuParams {
                paths: todo!(),
                dlc: todo!(),
                update: todo!(),
                title,
            })?],
        })
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.hub.show(ctx);

        for emu in &mut self.emu {
            emu.show(ctx);
        }
    }
}
