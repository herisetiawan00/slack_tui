use std::{any::Any, collections::HashMap, env, process::Command};

use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, HorizontalAlignment, Layout},
    style::{Color, Modifier, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListState, Padding, Paragraph},
};
use reqwest::Url;
use tiny_http::{Response, Server};

use crate::{
    common::{Config, Context, State},
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
                    let base_url = "https://slack.com/oauth/v2/authorize";
                    let redirect_uri = "https://localhost:7777";
                    let scope: Vec<&str> = vec![];
                    let user_scope: Vec<&str> = vec![
                        "users:read",
                        "usergroups:read",
                        "channels:read",
                        "channels:history",
                        "groups:read",
                        "groups:history",
                        "mpim:read",
                        "mpim:history",
                        "im:read",
                        "im:history",
                        "chat:write",
                    ];

                    let mut auth_url = Url::parse(base_url).ok()?;
                    let mut params: HashMap<String, String> = HashMap::new();

                    params.insert("scope".to_string(), scope.join(","));
                    params.insert("user_scope".to_string(), user_scope.join(","));
                    params.insert("redirect_uri".to_string(), redirect_uri.to_string());
                    params.insert("client_id".to_string(), context.config.client_id.clone());

                    for (key, value) in params {
                        auth_url
                            .query_pairs_mut()
                            .append_pair(key.as_str(), value.as_str());
                    }

                    opener::open(auth_url.to_string()).ok()?;

                    let server = Server::http("127.0.0.1:7777").unwrap();

                    for request in server.incoming_requests() {
                        println!("{:?}", request.url());
                        request
                            .respond(Response::from_string("Success").with_status_code(200))
                            .ok()?;
                        break;
                    }
                }
                1 => {
                    let config_path = Config::get_path();
                    let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

                    let mut child = Command::new(editor).arg(config_path).spawn().ok()?;

                    child.wait().ok()?;
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
