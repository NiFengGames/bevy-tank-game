use super::{components::Scene, resources};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn add(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/tank/_Complete-Game.gltf#Scene0"),
        ..default()
    });
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}

pub fn update(
    mut scene_query: Query<&mut Transform, With<Scene>>,
    mut resources: ResMut<resources::Resources>,
) {
}
