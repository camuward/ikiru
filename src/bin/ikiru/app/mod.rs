//! GUI and application code.
use tokio::runtime::Runtime;

use crate::cfg::Instance;
use crate::emu::EmuParams;
use crate::gfx;
use crate::util::TitleId;

pub mod cfg;

pub mod win {
    pub mod emu;
    pub mod hub;
}
// pub mod misc;

pub struct App {
    rt: Runtime,
    app: Instance,

    // windows
    hub: hub::Window,
    emu: Vec<emu::Window>,
}

impl App {
    /// Create a new instance, opening the hub window without starting any games.
    pub fn new(cc: &eframe::CreationContext<'_>, app: Instance) -> eyre::Result<Self> {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Ok(Self {
            app,
            rt: Runtime::new()?,
            // cfg: cfg::Cfg::load(Path::new("cfg.toml")),
            hub: hub::Window::new_open(),
            emu: Vec::new(),
        })
    }

    /// Create a new instance, starting the provided title ID. The hub window
    /// be hidden unless the user manually closes the game via `Emulation >
    /// Stop`.
    pub fn new_load_title(
        cc: &eframe::CreationContext<'_>,
        app: Instance,
        title: TitleId,
    ) -> eyre::Result<Self> {
        let game = app.game_cfgs.get_cfg(title)?;

        Ok(Self {
            app,
            rt: Runtime::new()?,
            // cfg: cfg::Cfg::load(Path::new("cfg.toml")),
            hub: hub::Window::new_closed(),
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
