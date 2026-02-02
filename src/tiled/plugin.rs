use super::systems::load_map;
use bevy::prelude::*;

pub struct TiledPlugin;

impl Plugin for TiledPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_ecs_tiled::prelude::TiledPlugin::default())
            .add_systems(Startup, load_map);
    }
}
