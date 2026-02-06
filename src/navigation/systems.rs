use super::components::{InNavigationRegion, NavMeshRef, NavigationPath, NavigationRegion};
use crate::npcs::components::Npc;
use avian2d::prelude::PhysicsGizmos;
use bevy::{color::palettes, prelude::*};
use bevy_ecs_tiled::prelude::*;
use moonshine_kind::Instance;
use vleue_navigator::prelude::*;

/// Populates NavigationRegion vertices from Tiled polygon data,
/// then spawns the NavMesh for pathfinding.
pub fn populate_navigation_region_vertices(
    trigger: On<TiledEvent<ObjectCreated>>,
    mut commands: Commands,
    mut regions: Query<(Entity, &Transform, &mut NavigationRegion)>,
    map_assets: Res<Assets<TiledMapAsset>>,
) {
    let event = trigger.event();

    // Get the NavigationRegion component if this object has one
    let Ok((entity, transform, mut region)) = regions.get_mut(event.origin) else {
        return;
    };

    // Get the raw tiled object data
    let Some(object) = event.get_object(&map_assets) else {
        return;
    };

    // Extract polygon vertices from object shape
    let tiled::ObjectShape::Polygon { points } = &object.shape else {
        warn!("NavigationRegion object is not a polygon");
        return;
    };

    // Store world-space vertices for contains() and random_point()
    let origin = transform.translation.truncate();
    region.vertices = points
        .iter()
        .map(|(x, y)| origin + Vec2::new(*x, -*y))
        .collect();

    // Spawn navmesh with world-space coordinates (no Transform needed)
    let navmesh_entity = commands
        .spawn((
            Name::new("NavMesh"),
            NavMeshSettings {
                fixed: Triangulation::from_outer_edges(&region.vertices),
                agent_radius: 3.0,
                simplify: 1.0,
                ..default()
            },
            NavMeshUpdateMode::Direct,
        ))
        .id();

    // Store reference to navmesh entity on the region
    commands.entity(entity).insert(NavMeshRef(navmesh_entity));
}

/// Links NPCs to their navigation region based on spawn position.
pub fn link_npcs_to_navigation_regions(
    mut commands: Commands,
    new_npcs: Query<(Instance<Npc>, &Transform), Without<InNavigationRegion>>,
    regions: Query<(Instance<NavigationRegion>, &NavigationRegion)>,
) {
    for (npc, transform) in &new_npcs {
        let pos = transform.translation.truncate();

        // Find which region contains this position
        let mut found = false;
        for (region_instance, region) in &regions {
            if region.contains(pos) {
                commands
                    .entity(npc.entity())
                    .insert(InNavigationRegion(region_instance));
                found = true;
                break;
            }
        }

        if !found {
            warn!("No navigation region found for NPC at {:?}", pos);
        }
    }
}

/// Draws the navigation path as a gizmo line (only when PhysicsGizmos enabled).
pub fn draw_navigation_path_gizmo(
    mut gizmos: Gizmos,
    npcs: Query<(&Transform, &NavigationPath)>,
    config_store: Res<GizmoConfigStore>,
) {
    let (config, _) = config_store.config::<PhysicsGizmos>();
    if !config.enabled {
        return;
    }

    for (transform, path) in &npcs {
        if path.waypoints.is_empty() {
            continue;
        }

        // Build path from current position through all waypoints
        let current_pos = transform.translation.truncate();
        let mut points = vec![current_pos];
        points.extend(path.waypoints.iter().copied());

        gizmos.linestrip_2d(points, palettes::tailwind::YELLOW_400);

        // Draw destination marker
        gizmos.circle_2d(
            Isometry2d::from_translation(path.destination),
            3.0,
            palettes::tailwind::RED_400,
        );
    }
}
