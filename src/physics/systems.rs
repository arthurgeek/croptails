use crate::navigation::components::NavigationRegion;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

/// Observer that adds RigidBody::Static to all Tiled colliders.
pub fn make_colliders_static(
    collider_created: On<TiledEvent<ColliderCreated>>,
    mut commands: Commands,
) {
    commands
        .entity(collider_created.event().origin)
        .insert(RigidBody::Static);
}

/// Removes colliders from NavigationRegion entities and their children.
pub fn remove_navigation_region_colliders(
    mut commands: Commands,
    nav_regions: Query<(Entity, Option<&Children>), With<NavigationRegion>>,
    colliders: Query<Entity, With<Collider>>,
) {
    for (entity, children) in &nav_regions {
        // Remove from entity itself
        if colliders.contains(entity) {
            commands.entity(entity).remove::<(Collider, RigidBody)>();
        }

        // Remove from children
        if let Some(children) = children {
            for child in children.iter() {
                if colliders.contains(child) {
                    commands.entity(child).remove::<(Collider, RigidBody)>();
                }
            }
        }
    }
}
