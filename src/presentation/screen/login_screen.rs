use std::any::Any;

use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, HorizontalAlignment, Layout},
    style::{Color, Modifier, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListState, Padding, Paragraph},
};

use crate::{
    common::{Context, Registry, State},
    data::datasources::{local::ConfigurationLocalDatasource, remote::SlackRemoteDatasource},
    presentation::screen::Screen,
};

struct LoginScreenState {
    pub title: String,
    pub selected_item: usize,
    pub total_item: usize,
}

impl State for LoginScreenState {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Clone for LoginScreenState {
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            selected_item: self.selected_item.clone(),
            total_item: self.total_item.clone(),
        }
    }
}

pub fn login_screen() -> Screen {
    return Screen { render, keymap };
}

fn render(context: &mut Context, frame: &mut Frame) {
    let items = ["Login to Slack", "Modify configuration", "Exit"];

    // GENERATING STATE
    let default_state = LoginScreenState {
        title: "Lorem Ipsum".to_string(),
        selected_item: 0,
        total_item: items.len(),
    };

    let state = match context.get_state::<LoginScreenState>() {
        Some(state) => state,
        None => {
            context.set_state(default_state.clone());
            &default_state
        }
    };

    // CREATING WRAPPER
    let area = frame.area();

    let wrapper = Block::default()
        .borders(Borders::ALL)
        .title(state.title.clone())
        .title_alignment(HorizontalAlignment::Center)
        .padding(Padding::uniform(1));
    frame.render_widget(wrapper.clone(), area);

    // CREATING CHUNKS FOR CONTENTS
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(7),
            Constraint::Length(1),
            Constraint::Length(20),
            Constraint::Min(0),
        ])
        .split(wrapper.inner(area));

    // RENDERING LOGO
    let logo_ascii = vec![
        r#"    _____ _            _       _______ _    _ _____ "#.into(),
        r#"   / ____| |          | |     |__   __| |  | |_   _|"#.into(),
        r#"  | (___ | | __ _  ___| | __     | |  | |  | | | |  "#.into(),
        r#"   \___ \| |/ _` |/ __| |/ /     | |  | |  | | | |  "#.into(),
        r#"   ____) | | (_| | (__|   <      | |  | |__| |_| |_ "#.into(),
        r#"  |_____/|_|\__,_|\___|_|\_\     |_|   \____/|_____|"#.into(),
        r#"                                                    "#.into(),
    ];
    let logo = Paragraph::new(logo_ascii).alignment(HorizontalAlignment::Center);
    frame.render_widget(logo, chunks[1]);

    // RENDERING TITLE
    let title = Line::from_iter([
        Span::from("Please select option below:").bold(),
        Span::from(" (Press 'q' to quit and arrow keys to navigate)"),
    ])
    .centered();
    frame.render_widget(title, chunks[2]);

    // RENDERING LUST ITEM
    let wrapper = Block::default().borders(Borders::ALL);

    let items = ["Login to Slack", "Modify configuration", "Exit"];

    let mut list_state = ListState::default().with_selected(Some(state.selected_item));

    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    frame.render_widget(wrapper.clone(), chunks[3]);
    frame.render_stateful_widget(list, wrapper.inner(chunks[3]), &mut list_state);
}

fn keymap(context: &mut Context) -> Option<bool> {
    let state = context.get_state::<LoginScreenState>()?;

    if let Some(key) = crossterm::event::read().unwrap().as_key_press_event() {
        let mut selected_item: Option<usize> = None;
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if state.selected_item < state.total_item - 1 {
                    selected_item = Some(state.selected_item + 1)
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if state.selected_item > 0 {
                    selected_item = Some(state.selected_item - 1)
                }
            }
            KeyCode::Char('q') | KeyCode::Esc => return Some(true),
            KeyCode::Enter => match state.selected_item {
                0 => {
                    let slack_remote_datasource = Registry::of(context)
                        .resolve::<SlackRemoteDatasource>()
                        .expect("SlackRemoteDatasource not registered in registry");

                    let code = slack_remote_datasource
                        .oauth_authorize(&context.config.client_id, &context.config.redirect_url)?;

                    slack_remote_datasource.exchange_code(
                        &context.config.client_id,
                        &context.config.client_secret,
                        &code,
                    );
                }
                1 => {
                    let configuration_local_datasource = Registry::of(context)
                        .resolve::<ConfigurationLocalDatasource>()
                        .expect("ConfigurationLocalDatasource not registered in registry");
                    configuration_local_datasource.edit();
                    context.refresh_config();
                    return Some(false);
                }
                2 => return Some(true),
                _ => {}
            },
            _ => {}
        }

        context.set_state(LoginScreenState {
            selected_item: selected_item?,
            ..state.clone()
        });
    }

    return None;
}
