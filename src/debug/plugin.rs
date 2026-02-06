use super::systems::sync_navmesh_debug_with_physics_gizmos;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_plugins(WorldInspectorPlugin::new())
            .add_plugins(PhysicsDebugPlugin)
            .insert_gizmo_config(
                PhysicsGizmos::default(),
                GizmoConfig {
                    enabled: false,
                    ..default()
                },
            )
            .add_systems(Update, sync_navmesh_debug_with_physics_gizmos);
    }
}
