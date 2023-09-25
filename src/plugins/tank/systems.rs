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
            Collider::cuboid(1.0, 1.0, 1.0),
            // Restitution::coefficient(0.7),
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 1.2, 0.0),
                ..default()
            },
        ))
        .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        })
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 5.0, 10.0)
                    .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
                ..default()
            });
            let mut t = Transform::from_xyz(0.0, -0.5, 0.0);
            t.rotate_axis(Vec3::Y, 180.0f32.to_radians());
            parent.spawn(SceneBundle {
                scene: asset_server.load("models/tank/CompleteTank.gltf#Scene0"),
                transform: t,
                ..default()
            });
        });
}

const SPEED: f32 = 5.0;

pub fn update(
    mut tank_query: Query<&mut Transform, With<Tank>>,
    keyboard_input: Res<Input<KeyCode>>,
    // mut resources: ResMut<resources::Resources>,
    // positions: Query<&V, With<RigidBody>>,
    mut velocities: Query<&mut Velocity>,
) {
    let camera = tank_query.get_single_mut().unwrap();

    let mut linvel = Vec3::new(0.0, 0.0, 0.0);
    let mut angvel = Vec3::new(0.0, 0.0, 0.0);

    if keyboard_input.pressed(KeyCode::A) {
        angvel = Vec3::new(0.0, 1.0, 0.0);
    } else if keyboard_input.pressed(KeyCode::D) {
        angvel = Vec3::new(0.0, -1.0, 0.0);
    } else if keyboard_input.pressed(KeyCode::W) {
        let forward = camera.forward();
        linvel = forward * SPEED;
    } else if keyboard_input.pressed(KeyCode::S) {
        let forward = camera.back();
        linvel = forward * SPEED;
    }

    for mut vel in velocities.iter_mut() {
        vel.linvel = linvel;
        vel.angvel = angvel;
    }
}
