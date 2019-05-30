use tweek::prelude::*;

#[allow(unused_imports)]
use quicksilver::graphics::{Color, Font, FontStyle};

static ROBOTO_REGULAR: &[u8] = include_bytes!("../../static/Roboto-Regular.ttf");
static ROBOTO_BOLD: &[u8] = include_bytes!("../../static/Roboto-Bold.ttf");

pub struct ThemeManager {}

impl ThemeManager {
    pub fn default_theme() -> Theme {
        let mut theme = Theme::new(ROBOTO_REGULAR);
        theme.font_size = 18.0;
        theme.font_bytes = ROBOTO_REGULAR.into();
        theme.bg_color = Color::from_hex("#FFFFEE");

        let font = Font::from_slice(ROBOTO_BOLD).unwrap();
        theme.title_font = Some(font);
        theme
    }
}
