use clap::Parser;

use ikiru::app::App;
use ikiru::cli::Cli;

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::vec2(480.0, 360.0)),
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };

    let instance = cli.read_cfg()?;

    Ok(match cli.title {
        Some(title) => {
            let title = title.parse()?;
            eframe::run_native(
                "ikiru",
                options,
                Box::new(move |cc| Box::new(App::new_load_title(cc, instance, title).unwrap())),
            )
            .unwrap()
        }
        None => eframe::run_native(
            "ikiru",
            options,
            Box::new(|cc| Box::new(App::new(cc, instance).unwrap())),
        )
        .unwrap(),
    })
}
