use super::components::{Axe, Hoe, Tool, ToolMarker, ToolTarget, WateringCan};
use crate::core::components::Active;
use crate::core::messages::Hit;
use crate::objects::components::Object;
use crate::player::Player;
use crate::player::components::EquippedTool;
use avian2d::prelude::*;
use bevy::prelude::*;
use moonshine_kind::Instance;

/// Syncs tool child entity when EquippedTool changes.
pub fn sync_tool_on_equip_change(
    mut commands: Commands,
    players: Query<(Instance<Player>, &EquippedTool), Changed<EquippedTool>>,
    tools: Query<(Instance<ToolMarker>, &ChildOf)>,
) {
    for (player, equipped) in &players {
        // Despawn old tool child (find tool whose parent is this player)
        for (tool, child_of) in &tools {
            if child_of.parent() == player.entity() {
                commands.entity(tool.entity()).despawn();
            }
        }

        // Spawn new tool child based on equipped
        match equipped {
            EquippedTool::None => {}
            EquippedTool::Axe => {
                commands.entity(player.entity()).with_child(Axe);
            }
            EquippedTool::Hoe => {
                commands.entity(player.entity()).with_child(Hoe);
            }
            EquippedTool::WateringCan => {
                commands.entity(player.entity()).with_child(WateringCan);
            }
        }
    }
}

/// Detects collisions between tools of type T and ToolTarget<T>, firing Hit<T> messages.
/// Only fires when tool has Active component (during swing animation).
pub fn detect_tool_hits<T: Tool>(
    mut collisions: MessageReader<CollisionStart>,
    tools: Query<Instance<T>, With<Active>>,
    targets: Query<&ChildOf, With<ToolTarget<T>>>,
    objects: Query<Instance<Object>>,
    mut writer: MessageWriter<Hit<T>>,
) {
    for evt in collisions.read() {
        // Try both orderings (collision pairs can come in either order)
        let (tool_entity, target_entity) =
            if tools.contains(evt.collider1) && targets.contains(evt.collider2) {
                (evt.collider1, evt.collider2)
            } else if tools.contains(evt.collider2) && targets.contains(evt.collider1) {
                (evt.collider2, evt.collider1)
            } else {
                continue;
            };

        let Ok(tool) = tools.get(tool_entity) else {
            continue;
        };

        let Ok(child_of) = targets.get(target_entity) else {
            continue;
        };

        let Ok(target) = objects.get(child_of.parent()) else {
            continue;
        };

        writer.write(Hit::new(target, tool));
    }
}
