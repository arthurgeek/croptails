use bevy::prelude::*;
use moonshine_kind::Instance;
use rand::Rng;

/// Navigation region defined by a polygon. Spawned from Tiled.
/// Vertices are populated from Tiled polygon data after spawn.
/// A NavMesh is automatically created from this region.
#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct NavigationRegion {
    /// Polygon vertices in world space (populated from Tiled).
    #[reflect(ignore)]
    pub vertices: Vec<Vec2>,
}

impl NavigationRegion {
    /// Margin from polygon edges (should match navmesh agent_radius).
    const EDGE_MARGIN: f32 = 5.0;

    /// Get a random point inside the polygon using rejection sampling.
    /// Points are guaranteed to be at least EDGE_MARGIN away from edges.
    pub fn random_point(&self) -> Vec2 {
        if self.vertices.len() < 3 {
            return Vec2::ZERO;
        }

        let mut rng = rand::rng();

        // Bounding box rejection sampling
        let min_x = self
            .vertices
            .iter()
            .map(|v| v.x)
            .fold(f32::INFINITY, f32::min);
        let max_x = self
            .vertices
            .iter()
            .map(|v| v.x)
            .fold(f32::NEG_INFINITY, f32::max);
        let min_y = self
            .vertices
            .iter()
            .map(|v| v.y)
            .fold(f32::INFINITY, f32::min);
        let max_y = self
            .vertices
            .iter()
            .map(|v| v.y)
            .fold(f32::NEG_INFINITY, f32::max);

        for _ in 0..100 {
            let point = Vec2::new(
                rng.random_range(min_x..max_x),
                rng.random_range(min_y..max_y),
            );
            if self.contains(point) && self.distance_to_edge(point) >= Self::EDGE_MARGIN {
                return point;
            }
        }

        // Fallback to centroid
        self.vertices.iter().copied().sum::<Vec2>() / self.vertices.len() as f32
    }

    /// Calculate minimum distance from point to any polygon edge.
    fn distance_to_edge(&self, point: Vec2) -> f32 {
        let n = self.vertices.len();
        let mut min_dist = f32::INFINITY;

        for i in 0..n {
            let a = self.vertices[i];
            let b = self.vertices[(i + 1) % n];
            let dist = point_to_segment_distance(point, a, b);
            min_dist = min_dist.min(dist);
        }

        min_dist
    }

    /// Check if a point is inside the polygon (ray casting algorithm).
    pub fn contains(&self, point: Vec2) -> bool {
        let n = self.vertices.len();
        if n < 3 {
            return false;
        }

        let mut inside = false;
        let mut j = n - 1;

        for i in 0..n {
            let vi = self.vertices[i];
            let vj = self.vertices[j];

            if ((vi.y > point.y) != (vj.y > point.y))
                && (point.x < (vj.x - vi.x) * (point.y - vi.y) / (vj.y - vi.y) + vi.x)
            {
                inside = !inside;
            }
            j = i;
        }

        inside
    }
}

/// Reference to the navigation region this NPC belongs to.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct InNavigationRegion(pub Instance<NavigationRegion>);

/// Reference to the NavMesh entity for this region.
/// Uses Entity because the navmesh is spawned and immediately stored
/// (Instance requires the component to be queryable).
#[derive(Component)]
pub struct NavMeshRef(pub Entity);

/// Marker for obstacles that carve out the navmesh.
#[derive(Component)]
pub struct NavMeshObstacle;

/// Path that an NPC is currently following.
/// Waypoints from vleue_navigator pathfinding.
#[derive(Component, Default)]
pub struct NavigationPath {
    /// Waypoints to follow (first = next target).
    pub waypoints: Vec<Vec2>,
    /// Final destination (for gizmo drawing).
    pub destination: Vec2,
}

impl NavigationPath {
    /// Create a new path from waypoints.
    pub fn new(waypoints: Vec<Vec2>, destination: Vec2) -> Self {
        Self {
            waypoints,
            destination,
        }
    }

    /// Get the current waypoint (next destination).
    pub fn current(&self) -> Option<Vec2> {
        self.waypoints.first().copied()
    }

    /// Advance to the next waypoint. Returns true if path is complete.
    pub fn advance(&mut self) -> bool {
        if !self.waypoints.is_empty() {
            self.waypoints.remove(0);
        }
        self.waypoints.is_empty()
    }
}

/// Distance from point to line segment.
fn point_to_segment_distance(point: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ab = b - a;
    let ap = point - a;
    let len_sq = ab.length_squared();

    if len_sq == 0.0 {
        return ap.length();
    }

    // Project point onto line, clamped to segment
    let t = (ap.dot(ab) / len_sq).clamp(0.0, 1.0);
    let closest = a + ab * t;
    point.distance(closest)
}
