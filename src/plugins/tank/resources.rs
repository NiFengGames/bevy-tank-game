use bevy::prelude::*;

#[derive(Resource)]
pub struct Resources {
    pub angle: f32,
}
impl Default for Resources {
    fn default() -> Self {
        Self { angle: 0.0 }
    }
}
