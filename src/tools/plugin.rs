use super::{
    components::{Axe, Hoe, ToolMarker, WateringCan},
    systems::{detect_tool_hits, sync_tool_on_equip_change},
};
use bevy::prelude::*;

pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ToolMarker>()
            .register_type::<Axe>()
            .register_type::<Hoe>()
            .register_type::<WateringCan>()
            .add_systems(Update, sync_tool_on_equip_change)
            .add_systems(
                Update,
                (
                    detect_tool_hits::<Axe>,
                    detect_tool_hits::<Hoe>,
                    detect_tool_hits::<WateringCan>,
                ),
            );
    }
}
