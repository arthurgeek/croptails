use super::components::{IdleTimer, Npc, WalkCycleProgress, WalkCycles, WanderConfig};
use crate::{
    core::components::{Moving, Speed},
    navigation::components::{InNavigationRegion, NavMeshRef, NavigationPath, NavigationRegion},
    player::components::Player,
};
use avian2d::prelude::*;
use bevy::prelude::*;
use moonshine_kind::Instance;
use rand::Rng;
use vleue_navigator::prelude::*;

/// Ticks idle timer for NPCs not moving.
pub fn tick_idle_timer(time: Res<Time>, mut npcs: Query<&mut IdleTimer, Without<Moving>>) {
    for mut timer in &mut npcs {
        timer.0.tick(time.delta());
    }
}

/// Transitions from Idle to Walking when timer expires.
/// Computes a path to a random point in the navigation region.
pub fn idle_to_walk_transition(
    mut commands: Commands,
    npcs: Query<
        (
            Instance<Npc>,
            &IdleTimer,
            &Transform,
            Option<&InNavigationRegion>,
            &WalkCycles,
            &WanderConfig,
        ),
        Without<Moving>,
    >,
    regions: Query<(&NavigationRegion, Option<&NavMeshRef>)>,
    navmesh_query: Query<(&NavMeshStatus, &ManagedNavMesh)>,
    navmeshes: Res<Assets<NavMesh>>,
) {
    let mut rng = rand::rng();

    for (npc, timer, transform, in_region, walk_cycles, config) in &npcs {
        if !timer.0.is_finished() {
            continue;
        }

        let Some(in_region) = in_region else {
            continue;
        };

        let Ok((region, navmesh_ref)) = regions.get(in_region.0.entity()) else {
            continue;
        };

        let Some(navmesh_ref) = navmesh_ref else {
            continue;
        };

        let Ok((status, managed)) = navmesh_query.get(navmesh_ref.0) else {
            continue;
        };

        if *status != NavMeshStatus::Built {
            continue;
        }

        let Some(navmesh) = navmeshes.get(&**managed) else {
            continue;
        };

        let current_pos = transform.translation.truncate();
        let target_pos = region.random_point();

        let Some(path) = navmesh.path(current_pos, target_pos) else {
            continue;
        };

        // Initialize walk cycle progress
        let progress = WalkCycleProgress {
            current: 0,
            target: rng.random_range(walk_cycles.min..=walk_cycles.max),
        };

        // Random speed for this walk cycle
        let walk_speed = rng.random_range(config.min_speed..=config.max_speed);

        commands.entity(npc.entity()).insert((
            Moving,
            NavigationPath::new(path.path, target_pos),
            progress,
            Speed(walk_speed),
        ));
    }
}

/// Handles arrival at waypoint - advance path or go idle.
pub fn on_waypoint_arrival(
    mut commands: Commands,
    mut npcs: Query<
        (
            Instance<Npc>,
            &Transform,
            &mut NavigationPath,
            &mut WalkCycleProgress,
            &mut IdleTimer,
            &WanderConfig,
            &InNavigationRegion,
        ),
        With<Moving>,
    >,
    regions: Query<(&NavigationRegion, &NavMeshRef)>,
    navmesh_query: Query<(&NavMeshStatus, &ManagedNavMesh)>,
    navmeshes: Res<Assets<NavMesh>>,
) {
    const ARRIVAL_THRESHOLD: f32 = 2.0;
    let mut rng = rand::rng();

    for (npc, transform, mut path, mut progress, mut timer, config, in_region) in &mut npcs {
        let current_pos = transform.translation.truncate();

        // Check if we reached current waypoint
        let Some(waypoint) = path.current() else {
            // Path complete - go idle or start new cycle
            progress.current += 1;

            if progress.current >= progress.target {
                // All cycles done - go idle
                let idle_time = rng.random_range(config.min_idle_time..config.max_idle_time);
                timer.0 = Timer::from_seconds(idle_time, TimerMode::Once);
                commands
                    .entity(npc.entity())
                    .remove::<(Moving, NavigationPath, WalkCycleProgress)>();
            } else {
                // More cycles - compute new path
                let Ok((region, navmesh_ref)) = regions.get(in_region.0.entity()) else {
                    continue;
                };
                let Ok((status, managed)) = navmesh_query.get(navmesh_ref.0) else {
                    continue;
                };
                if *status != NavMeshStatus::Built {
                    continue;
                }
                let Some(navmesh) = navmeshes.get(&**managed) else {
                    continue;
                };

                let target_pos = region.random_point();
                if let Some(new_path) = navmesh.path(current_pos, target_pos) {
                    *path = NavigationPath::new(new_path.path, target_pos);
                }
            }
            continue;
        };

        let distance = current_pos.distance(waypoint);
        if distance <= ARRIVAL_THRESHOLD {
            // Reached waypoint - advance to next
            path.advance();
        }
    }
}

/// Separation behavior constants.
const SEPARATION_RADIUS: f32 = 25.0;
const SEPARATION_STRENGTH: f32 = 3.0;

/// Applies velocity towards current waypoint with separation from nearby entities.
pub fn apply_npc_movement(
    mut npcs: Query<
        (
            Instance<Npc>,
            &mut LinearVelocity,
            &Transform,
            &NavigationPath,
            &Speed,
        ),
        With<Moving>,
    >,
    all_npcs: Query<(Instance<Npc>, &Transform)>,
    player: Query<&Transform, With<Player>>,
) {
    // Collect positions to avoid borrow issues
    let npc_positions: Vec<(Instance<Npc>, Vec2)> = all_npcs
        .iter()
        .map(|(e, t)| (e, t.translation.truncate()))
        .collect();
    let player_pos = player.single().ok().map(|t| t.translation.truncate());

    for (npc, mut velocity, transform, path, speed) in &mut npcs {
        let Some(waypoint) = path.current() else {
            velocity.0 = Vec2::ZERO;
            continue;
        };

        let current_pos = transform.translation.truncate();

        // Path-following direction
        let path_dir = (waypoint - current_pos).normalize_or_zero();

        // Separation force from nearby entities
        let mut separation = Vec2::ZERO;

        // Avoid other NPCs
        for (other_npc, other_pos) in &npc_positions {
            if *other_npc == npc {
                continue;
            }
            let diff = current_pos - *other_pos;
            let dist = diff.length();
            if dist > 0.0 && dist < SEPARATION_RADIUS {
                separation += diff.normalize() * (1.0 - dist / SEPARATION_RADIUS);
            }
        }

        // Avoid player
        if let Some(p_pos) = player_pos {
            let diff: Vec2 = current_pos - p_pos;
            let dist = diff.length();
            if dist > 0.0 && dist < SEPARATION_RADIUS {
                separation += diff.normalize() * (1.0 - dist / SEPARATION_RADIUS);
            }
        }

        // Blend path direction with separation
        let steering = path_dir + separation * SEPARATION_STRENGTH;
        velocity.0 = steering.normalize_or_zero() * speed.0;
    }
}

/// Stops velocity when not walking.
pub fn stop_npc_movement(mut npcs: Query<&mut LinearVelocity, Without<Moving>>) {
    for mut velocity in &mut npcs {
        velocity.0 = Vec2::ZERO;
    }
}

/// Flips sprite based on movement direction.
pub fn flip_npc_sprite(mut npcs: Query<(&Transform, &NavigationPath, &mut Sprite), With<Moving>>) {
    for (transform, path, mut sprite) in &mut npcs {
        let Some(waypoint) = path.current() else {
            continue;
        };

        let current_pos = transform.translation.truncate();
        let direction = waypoint - current_pos;
        if direction.x != 0.0 {
            sprite.flip_x = direction.x < 0.0;
        }
    }
}
