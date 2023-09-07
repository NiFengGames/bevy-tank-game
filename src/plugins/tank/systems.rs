use super::components::Tank;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const TANK_SPEED: f32 = 1.0;

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
    mut tank_query: Query<&mut Transform, With<Tank>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = tank_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        } else if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        } else if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 0.0, 1.0);
        } else if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, 0.0, -1.0);
        }

        transform.translation += direction * TANK_SPEED * time.delta_seconds();
    }
}
