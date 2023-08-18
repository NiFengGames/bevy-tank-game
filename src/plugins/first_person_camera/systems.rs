use super::{components::FirstPersionCamera, resources};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
pub fn add_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(13.0, 13.0, 13.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        FirstPersionCamera,
    ));
}

const SPEED: f32 = 1.0;

pub fn update_camera(
    mut camera_query: Query<&mut Transform, With<FirstPersionCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut resources: ResMut<resources::Resources>,
) {
    let mut camera = camera_query.get_single_mut().unwrap();

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        let forward = camera.left();
        camera.translation += forward * SPEED;
    } else if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        let forward = camera.right();
        camera.translation += forward * SPEED;
    } else if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        let forward = camera.forward();
        camera.translation += forward * SPEED;
    } else if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        let forward = camera.back();
        camera.translation += forward * SPEED;
    }
    if buttons.pressed(MouseButton::Right) {
        resources.mousedown = true;
    }
    if buttons.just_released(MouseButton::Right) {
        resources.mousedown = false;
    }
    // dbg!(resources.mousedown);
    for ev in motion_evr.iter() {
        if resources.mousedown {
            // println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
            let (mut yaw, mut pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);
            pitch -= (ev.delta.y).to_radians();
            yaw -= (ev.delta.x).to_radians();
            camera.rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);

            // let euler: (f32, f32, f32) = camera.rotation.to_euler(EulerRot::XYZ);
            // camera.rotation = Quat::from_euler(
            //     EulerRot::XYZ,
            //     euler.0 + ev.delta.x / 20.0,
            //     euler.1 + ev.delta.y / 20.0,
            //     euler.2,
            // );
        }
    }
    // println!("update_camera");
}