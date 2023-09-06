use super::{components::FirstPersonCamera, resources};
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
pub fn add_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(13.0, 13.0, 13.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        FirstPersonCamera,
    ));
}

const SPEED: f32 = 1.0;
const ZOOM_MIN: f32 = 4.0;
const ZOOM_MAX: f32 = 24.0;

pub fn update_camera(
    mut camera_query: Query<&mut Transform, With<FirstPersonCamera>>,
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
    } else if keyboard_input.pressed(KeyCode::E) {
        let forward = camera.up();
        camera.translation += forward * SPEED;
    } else if keyboard_input.pressed(KeyCode::Q) {
        let forward = camera.down();
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

pub fn update_camera_move(
    mut camera_query: Query<&mut Transform, With<FirstPersonCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.get_single_mut().unwrap();

    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::A) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    } else if keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    } else if keyboard_input.pressed(KeyCode::W) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    } else if keyboard_input.pressed(KeyCode::S) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }

    camera.translation += direction * SPEED * time.delta_seconds();
}

pub fn update_camera_zoom(
    mut camera_query: Query<&mut Transform, With<FirstPersonCamera>>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
) {
    let mut camera = camera_query.get_single_mut().unwrap();

    for event in mouse_wheel_reader.iter() {
        match event.unit {
            MouseScrollUnit::Line => {
                let translation = camera.forward() * event.y;
                camera.translation += translation;
                if camera.translation.z < ZOOM_MIN {
                    camera.translation.z = ZOOM_MIN;
                } else if camera.translation.z > ZOOM_MAX {
                    camera.translation.z = ZOOM_MAX;
                }
            }
            MouseScrollUnit::Pixel => {
                let translation = camera.forward() * event.y;
                camera.translation += translation;
                if camera.translation.z < ZOOM_MIN {
                    camera.translation.z = ZOOM_MIN;
                } else if camera.translation.z > ZOOM_MAX {
                    camera.translation.z = ZOOM_MAX;
                }
            }
        }
    }
}
