mod landing_screen;
mod login_screen;

pub use login_screen::login_screen;
use ratatui::Frame;

use crate::common::Context;

pub struct Screen {
    pub render: fn(context: &mut Context, frame: &mut Frame),
    pub keymap: fn(context: &mut Context) -> Option<bool>,
}
