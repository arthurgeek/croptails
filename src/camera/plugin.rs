use bevy::prelude::*;

use crate::camera::systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_camera)
            .add_systems(Update, systems::resize_canvas);
    }
}
