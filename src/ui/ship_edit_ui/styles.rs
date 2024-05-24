use bevy::prelude::*;

pub const WRAPP_BG_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.3);
pub const MAIN_COLOR: Color = Color::rgb(245.0, 220.0, 220.0);

pub fn wrapp() -> Style {
    Style {
    width: Val::Percent(100.0),
    height: Val::Percent(100.0),
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..Style::DEFAULT
    }
}

pub fn menu() -> Style {
    Style {
    width: Val::Px(600.0), 
    height: Val::Px(600.0),
    padding: UiRect::all(Val::Px(10.0)),
    flex_direction: FlexDirection::Row,
    flex_wrap: FlexWrap::Wrap,
    align_content: AlignContent::FlexStart,
    ..Style::DEFAULT
    }
}

pub fn block() -> Style {
    Style {
    width: Val::Px(100.0), 
    height: Val::Px(100.0),
    margin: UiRect::all(Val::Px(8.0)),
    border: UiRect::all(Val::Px(3.0)),
    ..Style::DEFAULT
    }
}