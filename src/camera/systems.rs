use bevy::{
    camera::{RenderTarget, visibility::RenderLayers},
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    window::WindowResized,
};

/// The game's native resolution (viewport size)
pub const VIEWPORT_WIDTH: u32 = 640;
pub const VIEWPORT_HEIGHT: u32 = 360;

/// Marker for the main game camera that renders to the canvas
#[derive(Component)]
pub struct GameCamera;

/// Marker for the camera that displays the canvas to the screen
#[derive(Component)]
pub struct ScreenCamera;

/// The canvas sprite that displays the render texture
#[derive(Component)]
pub struct Canvas;

/// Layer for the canvas (separate from game world)
const CANVAS_LAYER: RenderLayers = RenderLayers::layer(1);

pub fn spawn_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // Create the render texture at native viewport resolution
    let size = Extent3d {
        width: VIEWPORT_WIDTH,
        height: VIEWPORT_HEIGHT,
        depth_or_array_layers: 1,
    };

    let mut canvas_image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("canvas"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    canvas_image.resize(size);

    let canvas_handle = images.add(canvas_image);

    // Game camera: renders the game world to the canvas texture
    commands.spawn((
        Name::new("GameCamera"),
        GameCamera,
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        RenderTarget::Image(canvas_handle.clone().into()),
    ));

    // Canvas sprite: displays the render texture
    commands.spawn((
        Name::new("Canvas"),
        Canvas,
        Sprite::from_image(canvas_handle),
        CANVAS_LAYER,
    ));

    // Screen camera: renders only the canvas layer to the window
    commands.spawn((
        Name::new("ScreenCamera"),
        ScreenCamera,
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        CANVAS_LAYER,
    ));
}

pub fn resize_canvas(
    mut resize_events: MessageReader<WindowResized>,
    mut canvas_query: Query<&mut Transform, With<Canvas>>,
) {
    for event in resize_events.read() {
        let Ok(mut transform) = canvas_query.single_mut() else {
            return;
        };

        let scale = calculate_integer_scale(event.width, event.height);
        transform.scale = Vec3::splat(scale);
    }
}

fn calculate_integer_scale(window_width: f32, window_height: f32) -> f32 {
    // prioritize fitting the height with integer scaling
    let height_scale = (window_height / VIEWPORT_HEIGHT as f32).floor().max(1.0);

    // Check if width fits at this scale
    let width_at_scale = VIEWPORT_WIDTH as f32 * height_scale;

    if width_at_scale <= window_width {
        height_scale
    } else {
        // Width doesn't fit, fall back to width-based integer scale
        (window_width / VIEWPORT_WIDTH as f32).floor().max(1.0)
    }
}
