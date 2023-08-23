use super::{components::Tank, resources};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn add(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn((
    //     SceneBundle {
    //         scene: asset_server.load("models/tank/CompleteTank.gltf#Scene0"),
    //         transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //         ..default()
    //     },
    //     Tank,
    // ));
    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(SceneBundle {
            scene: asset_server.load("models/tank/CompleteTank.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 10.0, 0.0)));
}

pub fn update(
    // mut curr_query: Query<&mut Transform, With<Tank>>,
    // mut resources: ResMut<resources::Resources>,
    positions: Query<&Transform, With<RigidBody>>,
) {
    // for transform in positions.iter() {
    //     println!("Ball altitude: {}", transform.translation.y);
    // }
}
