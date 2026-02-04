use super::components::{Collectable, Log, PendingDespawn, Tree};
use super::resources::ObjectsAtlas;
use crate::core::components::{AnimationFinished, Damage, Health};
use crate::core::messages::Hit;
use crate::player::components::Chopping;
use crate::player::Player;
use crate::tools::components::Axe;
use avian2d::prelude::*;
use bevy::prelude::*;
use moonshine_kind::Instance;

pub fn load_objects_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("game/objects/basic_grass_biome_things.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        9,
        5,
        None,
        None,
    ));
    commands.insert_resource(ObjectsAtlas { texture, layout });
}

/// Applies damage to objects when hit by an axe.
pub fn apply_axe_damage(
    mut commands: Commands,
    mut hits: MessageReader<Hit<Axe>>,
    mut targets: Query<&mut Health>,
    tools: Query<&Damage>,
) {
    for hit in hits.read() {
        let Ok(damage) = tools.get(hit.tool.entity()) else {
            continue;
        };

        let Ok(mut health) = targets.get_mut(hit.target.entity()) else {
            continue;
        };

        health.current -= damage.0;

        if health.current <= 0.0 {
            commands.entity(hit.target.entity()).insert(PendingDespawn);
        }
    }
}

/// Despawns objects marked PendingDespawn when chopping animation finishes.
/// Spawns drops (e.g., Log from Tree) at the object's position.
pub fn despawn_pending(
    mut commands: Commands,
    players: Query<(), (With<Chopping>, Added<AnimationFinished>)>,
    pending_trees: Query<(Instance<Tree>, &GlobalTransform), With<PendingDespawn>>,
) {
    if players.is_empty() {
        return;
    }

    for (tree, transform) in &pending_trees {
        let pos = transform.translation();
        // Offset northeast to align with tree trunk
        commands.spawn((Log, Transform::from_xyz(pos.x + 8.0, pos.y + 8.0, pos.z)));

        commands.entity(tree.entity()).despawn();
    }
}

/// Collects items when player collides with them.
pub fn collect_items(
    mut commands: Commands,
    mut collisions: MessageReader<CollisionStart>,
    player_colliders: Query<&ChildOf, With<Collider>>,
    players: Query<Instance<Player>>,
    collectables: Query<(Instance<Collectable>, &Name)>,
) {
    for evt in collisions.read() {
        // Check both orderings - one is player collider, other is collectable
        let collectable_entity = if collectables.contains(evt.collider1) {
            // Check if collider2 belongs to a player
            let Ok(child_of) = player_colliders.get(evt.collider2) else {
                continue;
            };
            if !players.contains(child_of.parent()) {
                continue;
            }
            evt.collider1
        } else if collectables.contains(evt.collider2) {
            let Ok(child_of) = player_colliders.get(evt.collider1) else {
                continue;
            };
            if !players.contains(child_of.parent()) {
                continue;
            }
            evt.collider2
        } else {
            continue;
        };

        let Ok((collectable, name)) = collectables.get(collectable_entity) else {
            continue;
        };

        info!("Collected: {}", name);
        commands.entity(collectable.entity()).despawn();
    }
}
