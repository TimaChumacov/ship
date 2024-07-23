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

pub enum RotDirection {
    Left,
    Right
}
pub trait Rotate {
    fn get_rotation(&self) -> f32; 

    fn rotate_90(&mut self, direction: RotDirection);
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
    format!("Drag and Drop blocks of your ship to rearrange the ship. Right panel is your storage, you can drag blocks in and out of there. All looted blocks go into storage. Clicking blocks anywhere will give info about them. While blocks are selected or dragged around you can rotate some of them with [Q][E].
The Core of the ship (red thingy) has to always stay on the ship.
Press [ESC] to exit this menu, the changes are saved automatically.")
}

pub fn get_generic_stats() -> String {
    format!("INSTRUCTIONS\n")
}

pub fn get_generic_title() -> String {
    format!("Edit menu")
}