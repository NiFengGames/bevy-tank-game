use bevy::prelude::*;

#[derive(Resource)]
pub struct Resources {
    pub mousedown: bool,
}
impl Default for Resources {
    fn default() -> Self {
        Self { mousedown: false }
    }
}
