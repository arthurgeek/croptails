use bevy::{prelude::*, window::WindowResolution};

mod camera;
mod core;
#[cfg(feature = "debug")]
mod debug;
mod door;
mod interactable;
mod physics;
mod player;
mod tiled;

use camera::CameraPlugin;
use core::CorePlugin;
use door::DoorPlugin;
use interactable::InteractablePlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use tiled::TiledPlugin;

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
    ));

    #[cfg(feature = "debug")]
    app.add_plugins(debug::DebugPlugin);

    app.run()
}
