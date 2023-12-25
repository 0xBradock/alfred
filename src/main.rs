use anyhow::{Ok, Result};
mod model;
mod tui;
mod view;

fn main() -> Result<()> {
    tui::install_panic_hook();
    let terminal = tui::init_terminal()?;
    let app = model::App::new();

    view::ran_app(app, terminal)?;

    tui::restore_terminal()?;
    Ok(())
}
