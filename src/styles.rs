//! This handles most of the styling for the rustcast elements
use crate::config::Theme as ConfigTheme;
use iced::border::Radius;
use iced::widget::{button, container};
use iced::{Background, Border, Color, widget::text_input};

/// Helper: mix base color with white (simple “tint”)
pub fn tint(mut c: Color, amount: f32) -> Color {
    c.r = c.r + (1.0 - c.r) * amount;
    c.g = c.g + (1.0 - c.g) * amount;
    c.b = c.b + (1.0 - c.b) * amount;
    c
}
/// Helper: apply alpha
pub fn with_alpha(mut c: Color, a: f32) -> Color {
    c.a = a;
    c
}

/// Styling for the main text box
pub fn rustcast_text_input_style(theme: &ConfigTheme) -> text_input::Style {
    let base = theme.bg_color();
    let focused = false; // if you have state, pass it in and use it
    let surface = glass_surface(base, focused);
    text_input::Style {
        background: Background::Color(surface),
        border: Border {
            color: glass_border(theme.text_color(1.0), focused),
            width: 0.,
            radius: Radius::new(15.).bottom(0.),
        },
        icon: theme.text_color(0.75),
        placeholder: theme.text_color(0.50),
        value: theme.text_color(1.0),
        selection: with_alpha(theme.text_color(1.0), 0.20),
    }
}

/// Container styling for all the elements in the rustcast window
pub fn contents_style(theme: &ConfigTheme) -> container::Style {
    container::Style {
        background: None,
        text_color: None,
        border: iced::Border {
            color: theme.text_color(0.7),
            width: 0.4,
            radius: Radius::new(14.0),
        },
        ..Default::default()
    }
}

/// Styling for each of the buttons that are what the "results" of rustcast are
pub fn result_button_style(theme: &ConfigTheme) -> button::Style {
    button::Style {
        text_color: theme.text_color(1.),
        background: Some(Background::Color(theme.bg_color())),
        ..Default::default()
    }
}

/// Each rustcast results rows style
pub fn result_row_container_style(tile: &ConfigTheme, focused: bool) -> container::Style {
    container::Style {
        background: Some(Background::Color(glass_surface(tile.bg_color(), focused))),
        border: Border {
            color: glass_border(tile.text_color(1.), focused),
            width: 0.,
            radius: Radius::new(0.0),
        },
        text_color: Some(tile.text_color(1.0)),
        ..Default::default()
    }
}

/// The emoji results container style
///
/// Takes a focused boolean, to know if this specific button is focused or not
pub fn emoji_button_container_style(tile_theme: &ConfigTheme, focused: bool) -> container::Style {
    container::Style {
        background: Some(Background::Color(glass_surface(
            tile_theme.bg_color(),
            focused,
        ))),
        text_color: Some(tile_theme.text_color(1.0)),
        border: Border {
            color: glass_border(tile_theme.text_color(1.0), focused),
            width: 1.0,
            radius: Radius::new(10.0),
        },
        ..Default::default()
    }
}

/// Emoji buttons styling
pub fn emoji_button_style(tile_theme: &ConfigTheme) -> button::Style {
    let base = tile_theme.bg_color();
    let bg = with_alpha(tint(base, 0.10), 0.28);
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: tile_theme.text_color(1.0),
        border: Border {
            color: glass_border(tile_theme.text_color(1.0), false),
            width: 1.0,
            radius: Radius::new(10.0),
        },
        ..Default::default()
    }
}

/// Helper fn for making a color look like its glassy
pub fn glass_surface(base: Color, focused: bool) -> Color {
    let t = if focused { 0.3 } else { 0.06 };
    let a = if focused { 0.3 } else { 0.22 };
    with_alpha(tint(base, t), a)
}

/// Helper fn for making a borders color look like its glassy
pub fn glass_border(base_text: Color, focused: bool) -> Color {
    let a = if focused { 0.35 } else { 0.22 };
    with_alpha(base_text, a)
}
