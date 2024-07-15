use bevy::prelude::*;

pub const BLOCK_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.3);
pub const MAIN_COLOR: Color = Color::rgb(0.39, 0.36, 0.36);

pub fn absolute() -> Style {
    Style {
    position_type: PositionType::Absolute,
    width: Val::Percent(100.0),
    height: Val::Percent(100.0),
    ..Style::DEFAULT
    }
}

pub fn wrapp() -> Style {
    Style {
    width: Val::Percent(100.0),
    height: Val::Percent(100.0),
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..Style::DEFAULT
    }
}

pub fn grid_menu() -> Style {
    Style {
    width: Val::VMin(91.0), 
    height: Val::VMin(91.0),
    //padding: UiRect::all(Val::Px(10.0)),
    flex_direction: FlexDirection::Column,
    flex_wrap: FlexWrap::Wrap,
    align_content: AlignContent::Center,
    align_items: AlignItems::Center,
    justify_content: JustifyContent::Center,
    justify_items: JustifyItems::Center,
    ..Style::DEFAULT
    }
}

pub fn loot_menu() -> Style {
    Style {
    width: Val::VMin(40.0), 
    height: Val::VMin(91.0),
    //margin: UiRect { left: Val::VMin(2.0), right:  Val::VMin(2.0), top: Val::ZERO, bottom: Val::ZERO },
    flex_wrap: FlexWrap::Wrap,
    ..Style::DEFAULT
    }
}

pub fn loot_title() -> Style {
    Style {
    width: Val::Percent(100.0), 
    height: Val::Percent(6.5),
    align_items: AlignItems::End,
    justify_content: JustifyContent::Center,
    ..Style::DEFAULT
    }
}

pub fn loot_grid_wrapp() -> Style {
    Style {
    width: Val::Percent(100.0), 
    height: Val::Percent(93.5),
    padding: UiRect { left: Val::VMin(1.3), right: Val::VMin(1.3), top: Val::VMin(0.0), bottom: Val::VMin(0.0) },
    flex_direction: FlexDirection::Row,
    flex_wrap: FlexWrap::Wrap,
    align_content: AlignContent::FlexStart,
    justify_content: JustifyContent::Start,
    ..Style::DEFAULT
    }
}

pub fn info_menu() -> Style {
    let mut style = loot_menu();
    style.flex_direction = FlexDirection::Column;
    style.align_items = AlignItems::Center;
    style.align_content = AlignContent::Center;

    style
}

pub fn block() -> Style {
    Style {
    width: Val::VMin(18.0), 
    height: Val::VMin(18.0),
    ..Style::DEFAULT
    }
}
pub fn mini_block() -> Style {
    Style {
    width: Val::VMin(9.0), 
    height: Val::VMin(9.0),
    border: UiRect::all(Val::VMin(0.3)),
    margin: UiRect::all(Val::VMin(0.15)),
    ..Style::DEFAULT
    }
}

pub fn draggable() -> Style {
    Style {
    width: Val::VMin(14.0), 
    height: Val::VMin(14.0),
    position_type: PositionType::Absolute,
    top: Val::Percent(-100.0),
    ..Style::DEFAULT
    }
}

pub fn unselect_button() -> Style {
    Style {
        width: Val::Px(100.0),
        height: Val::Px(20.0),
        ..Style::DEFAULT
    }
}

pub fn selection_frame() -> Style {
    Style {
        width: Val::VMin(19.0), 
        height: Val::VMin(19.0),
        position_type: PositionType::Absolute,
        top: Val::VMin(-0.5),
        left: Val::VMin(-0.5),
        ..Style::DEFAULT
    }
}

pub fn loot_selection_frame() -> Style {
    Style {
        width: Val::VMin(10.0), 
        height: Val::VMin(10.0),
        position_type: PositionType::Absolute,
        top: Val::VMin(-1.0),
        left: Val::VMin(-1.0),
        ..Style::DEFAULT
    }
}

pub fn text() -> TextStyle{
    TextStyle {
        font_size: 20.0,
        color: Color::BLACK,
        ..default()
    }
}

pub fn stats_text() -> TextStyle{
    TextStyle {
        font_size: 25.0,
        color: Color::RED,
        ..default()
    }
}

pub fn title() -> TextStyle{
    TextStyle {
        font_size: 35.0,
        color: Color::BLACK,
        ..default()
    }
}