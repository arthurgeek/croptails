use super::{
    components::{Door, DoorState},
    resources::DoorAtlas,
};
use crate::{
    core::components::{AnimationFinished, SequenceAnimation},
    interactable::{InteractableActive, InteractableSensor},
};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use moonshine_kind::Instance;

/// Loads the door spritesheet.
pub fn load_door_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("game/tilesets/doors.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        1,
        4,
        None,
        None,
    ));
    commands.insert_resource(DoorAtlas { texture, layout });
}

/// Starts the open animation when DoorState changes to Opening.
pub fn on_start_opening(
    mut commands: Commands,
    door_atlas: Res<DoorAtlas>,
    doors: Query<(&DoorState, &TiledObjectVisuals), Changed<DoorState>>,
    mut sprites: Query<&mut Sprite>,
) {
    for (state, visuals) in &doors {
        if *state != DoorState::Opening {
            continue;
        }

        let Some(&visual_entity) = visuals.first() else {
            continue;
        };

        // Set up sprite with our atlas
        if let Ok(mut sprite) = sprites.get_mut(visual_entity) {
            sprite.image = door_atlas.texture.clone();
            sprite.texture_atlas = Some(TextureAtlas {
                layout: door_atlas.layout.clone(),
                index: Door::OPEN_FRAMES[0],
            });
        }

        // Add animation to visual child
        commands
            .entity(visual_entity)
            .insert(SequenceAnimation::once(
                Door::OPEN_FRAMES.to_vec(),
                Door::FPS,
            ));
    }
}

/// Starts the close animation when DoorState changes to Closing.
pub fn on_start_closing(
    mut commands: Commands,
    door_atlas: Res<DoorAtlas>,
    doors: Query<(&DoorState, &TiledObjectVisuals), Changed<DoorState>>,
    mut sprites: Query<&mut Sprite>,
) {
    for (state, visuals) in &doors {
        if *state != DoorState::Closing {
            continue;
        }

        let Some(&visual_entity) = visuals.first() else {
            continue;
        };

        // Set up sprite with our atlas
        if let Ok(mut sprite) = sprites.get_mut(visual_entity) {
            sprite.image = door_atlas.texture.clone();
            sprite.texture_atlas = Some(TextureAtlas {
                layout: door_atlas.layout.clone(),
                index: Door::CLOSE_FRAMES[0],
            });
        }

        // Add animation to visual child
        commands
            .entity(visual_entity)
            .insert(SequenceAnimation::once(
                Door::CLOSE_FRAMES.to_vec(),
                Door::FPS,
            ));
    }
}

/// Updates DoorState when animation finishes.
pub fn on_door_animation_finished(
    mut commands: Commands,
    visuals_finished: Query<
        (Instance<TiledObjectVisualOf>, &ChildOf),
        (With<TiledObjectVisualOf>, Added<AnimationFinished>),
    >,
    mut doors: Query<(&mut DoorState, Has<InteractableActive>)>,
) {
    for (visual, child_of) in &visuals_finished {
        // The parent of visual is the door entity
        let Ok((mut state, has_active)) = doors.get_mut(child_of.parent()) else {
            continue;
        };

        // Update state based on what animation just finished
        // If player left during opening or entered during closing, immediately reverse
        match *state {
            DoorState::Opening => {
                if has_active {
                    *state = DoorState::Open;
                } else {
                    *state = DoorState::Closing;
                }
            }
            DoorState::Closing => {
                if has_active {
                    *state = DoorState::Opening;
                } else {
                    *state = DoorState::Closed;
                }
            }
            _ => {}
        }

        // Clean up animation components
        commands
            .entity(visual.entity())
            .remove::<SequenceAnimation>()
            .remove::<AnimationFinished>();
    }
}

/// Disables door collider when door starts opening.
pub fn on_door_opening(
    doors: Query<(Instance<Door>, &DoorState, &Children), Changed<DoorState>>,
    mut colliders: Query<&mut CollisionLayers, (With<Collider>, Without<InteractableSensor>)>,
) {
    for (_door, state, children) in &doors {
        if *state != DoorState::Opening {
            continue;
        }

        // Find the physical collider child (not the sensor) and disable it
        for child in children.iter() {
            if let Ok(mut layers) = colliders.get_mut(child) {
                *layers = CollisionLayers::NONE;
            }
        }
    }
}

/// Re-enables door collider when door starts closing.
pub fn on_door_closing(
    doors: Query<(Instance<Door>, &DoorState, &Children), Changed<DoorState>>,
    mut colliders: Query<&mut CollisionLayers, (With<Collider>, Without<InteractableSensor>)>,
) {
    for (_door, state, children) in &doors {
        if *state != DoorState::Closing {
            continue;
        }

        // Find the physical collider child and re-enable it
        for child in children.iter() {
            if let Ok(mut layers) = colliders.get_mut(child) {
                *layers = CollisionLayers::new(LayerMask::ALL, LayerMask::ALL);
            }
        }
    }
}

/// Opens door when player enters interactable area.
pub fn on_interactable_activated(mut doors: Query<&mut DoorState, Added<InteractableActive>>) {
    for mut state in &mut doors {
        if *state == DoorState::Closed {
            *state = DoorState::Opening;
        }
    }
}

/// Closes door when player exits interactable area.
pub fn on_interactable_deactivated(
    mut doors: Query<&mut DoorState, With<Door>>,
    mut removed: RemovedComponents<InteractableActive>,
) {
    for entity in removed.read() {
        if let Ok(mut state) = doors.get_mut(entity)
            && *state == DoorState::Open
        {
            *state = DoorState::Closing;
        }
    }
}
