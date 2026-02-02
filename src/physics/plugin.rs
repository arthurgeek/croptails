use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TiledPhysicsAvianBackend, TiledPhysicsPlugin};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default().with_length_unit(16.0))
            .add_plugins(TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default())
            .insert_resource(Gravity(Vec2::ZERO));
    }
}
