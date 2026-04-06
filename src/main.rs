mod common;
mod data;
mod presentation;
mod utils;

use ratatui::DefaultTerminal;

use crate::{
    common::{Config, Context, Registry},
    data::datasources::{local::ConfigurationLocalDatasource, remote::SlackRemoteDatasource},
    presentation::screen::login_screen,
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut registry = Registry::new();

    setup_registry(&mut registry);
    let config = setup_config(&registry);

    let mut context = Context::new(registry, config);

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

fn setup_registry(registry: &mut Registry) {
    registry.register::<ConfigurationLocalDatasource>(ConfigurationLocalDatasource {});
    registry.register::<SlackRemoteDatasource>(SlackRemoteDatasource {});
}

fn setup_config(registry: &Registry) -> Config {
    let datasource = registry.resolve::<ConfigurationLocalDatasource>().expect("ConfigurationLocalDatasource not registered in registry");

    datasource.get()
}
