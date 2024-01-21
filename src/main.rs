#![allow(dead_code)]
#![allow(unused_variables)]

use anyhow::{Ok, Result};
mod model;
mod tui;
mod view;

fn main() -> Result<()> {
    // Setup
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;

    // Running app
    let mut app = model::App::new();
    view::ran_app(&mut app, &mut terminal)?;

    // Teardown
    tui::restore_terminal()?;
    Ok(())
}
