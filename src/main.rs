use bevy::{prelude::*, window::WindowResolution};

mod camera;
#[cfg(feature = "debug")]
mod debug;
mod physics;
mod tiled;

use camera::CameraPlugin;
use physics::PhysicsPlugin;
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
    ));

    #[cfg(feature = "debug")]
    app.add_plugins(debug::DebugPlugin);

    app.run()
}
