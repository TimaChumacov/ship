use bevy::prelude::*;

use crate::game::player::components::ShipLayout;
use crate::game::ship_blocks::components::Blocks;
use super::components::UiBlock;

pub fn interact_with_ui_blocks(
    mut ship_layout: ResMut<ShipLayout>,
    mut button_query: Query<
    (&Interaction, &mut BorderColor, &UiBlock),
    Changed<Interaction>
    >,
) {
    if let Ok((interaction, mut border_color, ui_block)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *border_color = Color::PURPLE.into();
                ship_layout.blocks[ui_block.x][ui_block.y] = Some(Blocks::Core);
            },
            Interaction::Hovered => {
                *border_color = Color::RED.into();
            },
            Interaction::None => {
                *border_color = Color::NONE.into();
            }
        }
    }
}