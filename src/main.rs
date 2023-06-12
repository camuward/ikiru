use clap::Parser;

mod app;
mod cli;
mod cfg;
mod gfx;

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let instance = cli.read_cfg()?;

    match cli.title {
        Some(title) => {
            let title = title.parse()?;
            start_eframe(move |cc| Box::new(App::new_load_title(cc, instance, title).unwrap()))
                .unwrap()
        }
        None => start_eframe(|cc| Box::new(App::new(cc, instance).unwrap())).unwrap(),
    }

    Ok(())
}

/// Run the [`App`].
fn start_eframe<F>(app_creator: F) -> Result<(), eframe::Error>
where
    F: FnOnce(&eframe::CreationContext) -> Box<dyn eframe::App> + 'static,
{
    let options = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::vec2(480.0, 360.0)),
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native("ikiru", options, Box::new(app_creator))
}
