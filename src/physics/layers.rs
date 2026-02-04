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
    /// Player tool sensors - active during tool actions (axe swing, etc.)
    Tool,
    /// Damageable object sensors - things that can be hit (trees, rocks, etc.)
    Object,
    /// Collectable items - can be picked up by Player
    Collectable,
}
