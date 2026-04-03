mod common;
mod presentation;
mod utils;

use ratatui::DefaultTerminal;

use crate::{common::Context, presentation::screen::login_screen};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut context = Context::new();

    loop {
        let screen = login_screen();

        let _ = terminal.draw(|frame| (screen.render)(&mut context, frame));

        if let Some(finished) = (screen.keymap)(&mut context) {
            terminal.clear()?;
            if finished {
                break Ok(());
            }
        }
    }
}
