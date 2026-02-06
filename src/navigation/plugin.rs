use super::{
    components::{InNavigationRegion, NavMeshObstacle, NavigationRegion},
    systems::{
        draw_navigation_path_gizmo, link_npcs_to_navigation_regions,
        populate_navigation_region_vertices,
    },
};
use avian2d::prelude::Collider;
use bevy::prelude::*;
use vleue_navigator::prelude::*;

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app
            // vleue_navigator plugins
            .add_plugins((
                VleueNavigatorPlugin,
                // Auto-update navmesh when obstacles change (uses avian2d Collider)
                NavmeshUpdaterPlugin::<Collider, NavMeshObstacle>::default(),
            ))
            // Register types
            .register_type::<NavigationRegion>()
            .register_type::<InNavigationRegion>()
            // Populate vertices and create NavMesh when NavigationRegion objects are created
            .add_observer(populate_navigation_region_vertices)
            // Link NPCs to their navigation regions
            .add_systems(Update, link_npcs_to_navigation_regions)
            // Debug gizmos for paths
            .add_systems(Update, draw_navigation_path_gizmo);
    }
}
