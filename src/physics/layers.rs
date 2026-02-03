use avian2d::prelude::*;

/// Physics collision layers for the game.
#[derive(PhysicsLayer, Clone, Copy, Debug, Default)]
pub enum GameLayer {
    /// Default layer for world objects (walls, furniture, etc.)
    #[default]
    Default,
    /// Player layer - collides with Default
    Player,
    /// Interactable sensors - detect Player only
    Interactable,
}
