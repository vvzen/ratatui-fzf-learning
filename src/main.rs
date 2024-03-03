use std::io::Write;

use color_eyre::owo_colors::OwoColorize;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

mod app;
mod errors;
mod tui;

fn main() -> color_eyre::Result<()> {
    // Install the error handlers by 'eyre'
    errors::install_hooks()?;

    // Enable LOG info by default if the caller didn't provide any overrides
    // or if it set the RUST_LOG env var incorrectly
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    // Initialize the logging library
    env_logger::builder()
        // Customize the formatting to offer full color per line
        .format(|buf, record| {
            let style = match record.level() {
                log::Level::Info => color_eyre::owo_colors::Style::new().blue(),
                log::Level::Warn => color_eyre::owo_colors::Style::new().yellow(),
                log::Level::Error => color_eyre::owo_colors::Style::new().red().bold(),
                _ => color_eyre::owo_colors::Style::new().white(),
            };

            writeln!(
                buf,
                "| {} | {}",
                record.level().style(style),
                record.args().style(style)
            )
        })
        .init();

    log::info!("Entering RAW mode..");

    std::io::stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = tui::init()?;
    let app_result = app::App::default().run(&mut terminal)?;
    log::info!("App result: {app_result:?}");
    log::info!("Exiting cleanly...");

    std::io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
