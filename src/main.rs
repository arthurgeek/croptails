use bevy::{prelude::*, window::WindowResolution};

mod animals;
mod camera;
mod core;
#[cfg(feature = "debug")]
mod debug;
mod door;
mod interactable;
mod navigation;
mod npcs;
mod objects;
mod physics;
mod player;
mod shaders;
mod tiled;
mod tools;

use animals::AnimalsPlugin;
use camera::CameraPlugin;
use core::CorePlugin;
use door::DoorPlugin;
use interactable::InteractablePlugin;
use navigation::NavigationPlugin;
use npcs::NpcsPlugin;
use objects::ObjectsPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use shaders::ShadersPlugin;
use tiled::TiledPlugin;
use tools::ToolsPlugin;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Croptails".to_string(),
                    resolution: WindowResolution::new(1280, 720),
                    resizable: true,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        PhysicsPlugin,
        CameraPlugin,
        TiledPlugin,
        CorePlugin,
        PlayerPlugin,
        DoorPlugin,
        InteractablePlugin,
        ObjectsPlugin,
        ToolsPlugin,
        ShadersPlugin,
        NavigationPlugin,
        NpcsPlugin,
        AnimalsPlugin,
    ));

    #[cfg(feature = "debug")]
    app.add_plugins(debug::DebugPlugin);

    app.run()
}
