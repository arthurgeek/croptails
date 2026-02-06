use avian2d::prelude::PhysicsGizmos;
use bevy::{color::palettes, prelude::*};
use vleue_navigator::prelude::NavMeshesDebug;

/// Syncs NavMeshesDebug resource with PhysicsGizmos enabled state.
pub fn sync_navmesh_debug_with_physics_gizmos(
    mut commands: Commands,
    config_store: Res<GizmoConfigStore>,
    navmesh_debug: Option<Res<NavMeshesDebug>>,
) {
    let (config, _) = config_store.config::<PhysicsGizmos>();

    match (config.enabled, navmesh_debug.is_some()) {
        (true, false) => {
            commands.insert_resource(NavMeshesDebug(palettes::tailwind::GREEN_400.into()));
        }
        (false, true) => {
            commands.remove_resource::<NavMeshesDebug>();
        }
        _ => {}
    }
}
