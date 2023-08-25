use super::components::Tank;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn add(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Tank,
            // SceneBundle {
            //     scene: asset_server.load("models/tank/CompleteTank.gltf#Scene0"),
            //     transform: Transform::from_xyz(0.0, 10.0, 0.0),
            //     ..default()
            // },
            RigidBody::Dynamic,
            Collider::ball(0.5),
            Restitution::coefficient(0.7),
            // TransformBundle::from(Transform::from_xyz(0.0, 10.0, 0.0)),
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 10.0, 0.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: asset_server.load("models/tank/CompleteTank.gltf#Scene0"),
                transform: Transform::from_xyz(0.0, -0.5, 0.0),
                ..default()
            });
        });
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
