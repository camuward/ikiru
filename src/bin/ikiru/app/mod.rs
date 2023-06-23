//! GUI and application code.
use tokio::runtime::Runtime;

use crate::gfx;
use ikiru::emu::EmuParams;
use ikiru::game::TitleId;

pub mod cfg;

pub mod win {
    pub(self) mod title_bar;

    pub mod emu;
    pub mod hub;
}
// pub mod misc;

pub struct App {
    rt: Runtime,
    app: Box<cfg::Instance>,
    drop_tx: Option<oneshot::Sender<Box<cfg::Instance>>>,

    // windows
    hub: win::hub::Window,
    emu: Vec<win::emu::Window>,
}

impl App {
    /// Create a new instance, opening the hub window without starting any games.
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        app: Box<cfg::Instance>,
        drop_tx: oneshot::Sender<Box<cfg::Instance>>,
    ) -> eyre::Result<Self> {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::set_fonts(&cc.egui_ctx);

        Ok(Self {
            app,
            drop_tx: Some(drop_tx),

            rt: Runtime::new()?,
            // cfg: cfg::Cfg::load(Path::new("cfg.toml")),
            hub: win::hub::Window {
                is_open: true,
                ..Default::default()
            },
            emu: Vec::new(),
        })
    }

    /// Create a new instance, starting the provided title ID. The hub window
    /// be hidden unless the user manually closes the game via `Emulation >
    /// Stop`.
    pub fn new_load_title(
        cc: &eframe::CreationContext<'_>,
        mut app: Box<cfg::Instance>,
        title: TitleId,
        drop_tx: oneshot::Sender<Box<cfg::Instance>>,
    ) -> eyre::Result<Self> {
        let _cfg = app.game_cfgs.get_cfg(title);

        Self::set_fonts(&cc.egui_ctx);

        Ok(Self {
            app,
            drop_tx: Some(drop_tx),

            rt: Runtime::new()?,
            // cfg: cfg::Cfg::load(Path::new("cfg.toml")),
            hub: Default::default(),
            emu: vec![gfx::spawn(EmuParams {
                paths: todo!(),
                dlc: todo!(),
                update: todo!(),
                title,
            })?],
        })
    }

    fn set_fonts(egui_ctx: &egui::Context) {
        use egui::{FontData, FontDefinitions, FontFamily};

        let mut fonts = FontDefinitions::default();

        #[cfg(windows)]
        if let Ok(data) = std::fs::read("C:\\WINDOWS\\FONTS\\SEGOEUI.TTF") {
            fonts
                .font_data
                .insert("Segoe UI".to_owned(), FontData::from_owned(data));
            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "Segoe UI".to_owned());
            fonts
                .families
                .get_mut(&FontFamily::Monospace)
                .unwrap()
                .push("Segoe UI".to_owned());
        }

        #[cfg(target_os = "macos")]
        {
            let data = std::fs::read("/System/Library/Fonts/SFNSText.ttf")
                .context("hey! please open an issue, i don't have a mac to test this on!")?;

            fonts
                .font_data
                .insert("SFNSText".to_owned(), FontData::from_owned(data));
            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "SFNSText".to_owned());
            fonts
                .families
                .get_mut(&FontFamily::Monospace)
                .unwrap()
                .push("SFNSText".to_owned());
        }

        egui_ctx.set_fonts(fonts);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.hub.show(&mut self.app, ctx, frame);

        for emu in &mut self.emu {
            emu.show(ctx);
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.drop_tx.take().unwrap().send(self.app.clone()).unwrap();
    }
}
