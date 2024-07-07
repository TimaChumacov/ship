use bevy::prelude::*;

pub trait Spawn {
    fn spawn(
        &self,
        x: usize,
        y: usize,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>
    );

    fn spawn_ui(
       &self,
       parent: &mut ChildBuilder, 
       asset_server: &Res<AssetServer>
    );
}

pub trait Rotate {
    fn get_rotation(&self) -> f32; 

    fn rotate_90_right(&mut self);
}


pub trait Description {
    fn get_info(&self) -> String {
        format!("Description here...")
    }

    fn get_stats(&self) -> String {
        format!("hp: [UI NOT READY]\n")
    }

    fn get_title(&self) -> String {
        format!("Title")
    }

    // fn get_stats(target: &Destructible) -> String {
    //     format!("hp: {}/{} \n", target.hp, target.max_hp)
    // }
}

pub fn get_generic_info() -> String {
    format!("Click on ship block on the grid to unequip it. Click on a block in the inventory on the right to select a new block to place. 
You can rotate some blocks with [R] and deselect them with the deselect button.
You can't remove Core of the ship (red thingy).
Press [ESC] to exit this menu, the changes are saved automatically.
Bevy UI hover is broken, so blocks gonna stay selected, but it's only visual.")
}

pub fn get_generic_stats() -> String {
    format!("INSTRUCTIONS\n")
}

pub fn get_generic_title() -> String {
    format!("SHIP EDIT MENU")
}