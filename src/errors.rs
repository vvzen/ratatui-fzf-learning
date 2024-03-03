use std::panic;

use color_eyre::{config::HookBuilder, eyre};

use crate::tui;

pub fn install_hooks() -> color_eyre::Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();

    // Convert from a color_eyre PanicHook to a standard one
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        tui::restore().expect("Failed to restore TUI");
        panic_hook(panic_info);
    }));

    // Convert from a color_eyre EyreHook to a eyre ErrorHook
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(
        move |error: &(dyn std::error::Error + 'static)| {
            tui::restore().expect("Failed to restore TUI");
            eyre_hook(error)
        },
    ))?;

    Ok(())
}
