use ratatui::{
    Frame,
    layout::{Constraint, Direction, HorizontalAlignment, Layout},
    widgets::{Block, Borders},
};

use crate::{common::Context, presentation::screen::Screen};

pub fn landing_screen() -> Screen {
    return Screen { render, keymap };
}

fn render(context: &mut Context, frame: &mut Frame) {
    let area = frame.area();

    let wrapper = Block::default()
        .borders(Borders::ALL)
        .title("Slack TUI")
        .title_alignment(HorizontalAlignment::Center);

    let inner_area = wrapper.inner(area);
    let content = Block::default().borders(Borders::RIGHT);

    let sidebar_width = (inner_area.width / 5).min(100);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(sidebar_width), // Top section (3 rows high)
            Constraint::Min(0),                // Bottom section (takes the rest)
        ])
        .split(inner_area);

    frame.render_widget(wrapper, area);
    frame.render_widget(content, chunks[0]);
}

fn keymap(context: &mut Context) -> Option<bool> {
    None
}
