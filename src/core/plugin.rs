use super::components::{
    Active, AnimationFinished, Damage, Health, SequenceAnimation, Speed, SpriteAnimation,
};
use super::messages::Hit;
use super::systems::{animate_sequences, animate_sprites};
use crate::tools::components::{Axe, Hoe, WateringCan};
use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SpriteAnimation>()
            .register_type::<SequenceAnimation>()
            .register_type::<AnimationFinished>()
            .register_type::<Speed>()
            .register_type::<Health>()
            .register_type::<Damage>()
            .register_type::<Active>()
            .add_message::<Hit<Axe>>()
            .add_message::<Hit<Hoe>>()
            .add_message::<Hit<WateringCan>>()
            .add_systems(Update, (animate_sprites, animate_sequences));
    }
}
