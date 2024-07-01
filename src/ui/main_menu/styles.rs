use bevy::prelude::*;

pub const BG_COLOR: Color = Color::rgba(0.1, 0.0, 0.1, 0.5);
pub const MENU_COLOR: Color = Color::rgba(0.2, 0.1, 0.2, 0.5);
pub const BLOCK_COLOR: Color = Color::rgb(0.6, 0.5, 0.6);
pub const HOVERED_COLOR: Color = Color::rgb(0.8, 0.7, 0.8);

pub fn wrapp() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Style::DEFAULT
    }
}

pub fn menu() -> Style {
    Style {
        width: Val::Px(400.0),
        height: Val::Px(600.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Style::DEFAULT
    }
}

pub fn button() -> Style {
    Style {
        width: Val::Px(200.0),
        height: Val::Px(60.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Style::DEFAULT
    }
}

pub fn button_text() -> TextStyle {
    TextStyle {
       font_size: 22.0,
        ..default()
    }
}