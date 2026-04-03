mod common;
mod presentation;

use ratatui::{DefaultTerminal, Frame};

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

        if let Some(true) = (screen.keymap)(&mut context) {
            break Ok(());
        }
    }
}
