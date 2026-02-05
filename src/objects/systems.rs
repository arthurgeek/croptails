use super::components::{Collectable, Log, Tree};
use super::resources::ObjectsAtlas;
use crate::core::components::{Damage, Health};
use crate::core::messages::Hit;
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

/// Sets tree health based on variant when spawned.
pub fn configure_tree_health(mut trees: Query<(&Tree, &mut Health), Added<Tree>>) {
    for (tree, mut health) in &mut trees {
        let max = tree.variant.health();
        health.max = max;
        health.current = max;
    }
}

/// Applies damage to trees when hit by an axe.
/// Despawns immediately and spawns logs when health reaches 0.
pub fn apply_axe_damage(
    mut commands: Commands,
    mut hits: MessageReader<Hit<Axe>>,
    mut targets: Query<(&mut Health, &Tree, &GlobalTransform)>,
    tools: Query<&Damage>,
) {
    for hit in hits.read() {
        let Ok(damage) = tools.get(hit.tool.entity()) else {
            continue;
        };

        let Ok((mut health, tree, transform)) = targets.get_mut(hit.target.entity()) else {
            continue;
        };

        health.current -= damage.0;
        info!("Tree hit! Health: {}/{}", health.current, health.max);

        if health.current <= 0.0 {
            let pos = transform.translation();
            for offset in tree.variant.log_offsets() {
                commands.spawn((Log, Transform::from_translation(pos + *offset)));
            }
            commands.entity(hit.target.entity()).despawn();
        }
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
