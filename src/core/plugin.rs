use super::components::{AnimationFinished, Speed, SpriteAnimation};
use super::systems::animate_sprites;
use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SpriteAnimation>()
            .register_type::<AnimationFinished>()
            .register_type::<Speed>()
            .add_systems(Update, animate_sprites);
    }
}
