use bevy::prelude::*;

pub const WRAPP_BG_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.3);
pub const MAIN_COLOR: Color = Color::rgb(245.0, 220.0, 220.0);

pub const WRAPP: Style = Style {
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..Style::DEFAULT
};

pub const MENU: Style = Style {
    size: Size::new(Val::Px(600.0), Val::Px(600.0)),
    padding: UiRect::all(Val::Px(10.0)),
    flex_direction: FlexDirection::Row,
    flex_wrap: FlexWrap::Wrap,
    align_content: AlignContent::FlexStart,
    ..Style::DEFAULT
};

pub const BLOCK: Style = Style {
    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
    margin: UiRect::all(Val::Px(8.0)),
    ..Style::DEFAULT
};