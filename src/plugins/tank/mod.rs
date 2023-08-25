use bevy::prelude::*;
pub mod components;
mod resources;
mod systems;
pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::Resources>()
            .add_systems(Startup, systems::add)
            .add_systems(Update, systems::update);
    }
}
