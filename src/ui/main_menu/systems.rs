use bevy::prelude::*;

use crate::general::states::{AppState, PauseState};

use super::{components::{MainMenuScreen, PlayButton}, styles::{BLOCK_COLOR, HOVERED_COLOR}};

pub fn play_button(
    mut commands: Commands,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
    menu_query: Query<Entity, With<MainMenuScreen>>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>)
    >
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                let menu_entity = menu_query.single();
                commands.entity(menu_entity).despawn_recursive();
                next_app_state.set(AppState::Game);
                next_pause_state.set(PauseState::Running);
            },
            Interaction::Hovered => {
                *background_color = HOVERED_COLOR.into();
            },
            Interaction::None => {
                *background_color = BLOCK_COLOR.into();
            }
        }
    }
}