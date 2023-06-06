use std::cell::OnceCell;

use clap::Parser;

use ikiru::cli::Cli;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    println!("{cli:#?}\n\n\x1b[31;1mH\x1b[33;1me\x1b[32;1ml\x1b[34;1ml\x1b[35;1mo\x1b[36;1m,\x1b[31;1m \x1b[33;1mW\x1b[32;1mo\x1b[34;1mr\x1b[35;1ml\x1b[36;1md\x1b[0m!");

    let _hub: OnceCell<ikiru::app::Hub> = OnceCell::new();

    let gfx_thread = ikiru::gfx::spawn()?;

    gfx_thread.join().unwrap();

    Ok(())
}
