use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;
pub struct FirstPersonCameraPlugin;

impl Plugin for FirstPersonCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::Resources>()
            .add_systems(Startup, systems::add_camera)
            .add_systems(Update, systems::update_camera)
            .add_systems(Update, systems::update_camera_zoom);
    }
}
