use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Selection {
    pub hovered_block: Option<Entity>,
    pub selected_block: Option<Entity>,
    pub hovered_loot: Option<Entity>,
    pub selected_loot: Option<Entity>,
    
    pub block_hover_frame: Option<Entity>,
    pub block_selected_frame: Option<Entity>,
    pub loot_hover_frame: Option<Entity>,
    pub loot_selected_frame: Option<Entity>,
}
