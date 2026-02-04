use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub fn load_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load a map asset and retrieve its handle
    let map_handle = asset_server.load("tiled/maps/test_map_objects_trees.tmx");

    // Spawn the map centered in the view
    commands.spawn((TiledMap(map_handle), TilemapAnchor::Center));
}

/// Observer that spawns tilemaps from objects with a `map_file` property.
/// This enables Tiled objects to act as prefab spawn points.
pub fn spawn_map_from_object(
    trigger: On<TiledEvent<ObjectCreated>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_assets: Res<Assets<TiledMapAsset>>,
    transforms: Query<&Transform>,
) {
    let event = trigger.event();

    // Get the raw tiled::ObjectData to access custom properties
    let Some(object) = event.get_object(&map_assets) else {
        return;
    };

    // Check for map_file property
    let Some(map_file) = object.properties.get("map_file") else {
        return;
    };

    let tiled::PropertyValue::StringValue(path) = map_file else {
        return;
    };

    // Get the object's transform
    let Ok(object_transform) = transforms.get(event.origin) else {
        return;
    };

    // Spawn the referenced tilemap at the object's position
    // Use BottomLeft anchor to match Tiled's tile object anchor point
    let map_handle = asset_server.load(format!("tiled/maps/{}", path));
    commands.spawn((
        TiledMap(map_handle),
        TilemapAnchor::BottomLeft,
        *object_transform,
    ));
}
