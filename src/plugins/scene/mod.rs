use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
pub mod components;
mod resources;
mod systems;
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::Resources>()
            .add_systems(Startup, systems::add)
            .add_systems(Update, systems::update);
    }
}
