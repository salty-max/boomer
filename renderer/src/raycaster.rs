use math::vector::Vector2;

use crate::map::Map;

pub struct CastResult {
    pub distance: f32,
    pub wall_value: u8,
    pub hit_vertical: bool,
}

/// Represents a single ray being cast from the player's position into the 2D map grid.
///
/// This struct uses the Digital Differential Analyzer (DDA) algorithm to efficiently
/// find the first intersection between the ray and a solid wall on a discrete grid.
pub struct Ray {
    /// The normalized direction vector of the ray.
    pub dir: Vector2,

    /// The total distance the ray must travel to cross exactly 1.0 units of the grid.
    ///
    /// `delta_dist.x` is the distance along the ray to move 1 unit horizontally.
    /// `delta_dist.y` is the distance along the ray to move 1 unit vertically.
    pub delta_dist: Vector2,

    /// The accumulated distance from the starting position to the next grid line.
    ///
    /// This is used during the DDA loop to decide whether to jump to the next
    /// horizontal (X) or vertical (Y) grid boundary.
    pub side_dist: Vector2,

    /// The direction to move on the map grid's X-axis (either 1 or -1).
    pub step_x: i32,

    /// The direction to move on the map grid's Y-axis (either 1 or -1).
    pub step_y: i32,

    /// The current integer X-coordinate of the map tile being checked.
    pub map_x: i32,

    /// The current integer Y-coordinate of the map tile being checked.
    pub map_y: i32,
}

impl Ray {
    pub fn new(player_pos: Vector2, ray_dir: Vector2) -> Self {
        let delta_dist_x = if ray_dir.x == 0.0 {
            1e30
        } else {
            (1.0 / ray_dir.x).abs()
        };
        let delta_dist_y = if ray_dir.y == 0.0 {
            1e30
        } else {
            (1.0 / ray_dir.y).abs()
        };

        let map_x = player_pos.x as i32;
        let map_y = player_pos.y as i32;

        let (step_x, side_dist_x) = if ray_dir.x < 0.0 {
            (-1, (player_pos.x - map_x as f32) * delta_dist_x)
        } else {
            (1, (map_x as f32 + 1.0 - player_pos.x) * delta_dist_x)
        };

        let (step_y, side_dist_y) = if ray_dir.y < 0.0 {
            (-1, (player_pos.y - map_y as f32) * delta_dist_y)
        } else {
            (1, (map_y as f32 + 1.0 - player_pos.y) * delta_dist_y)
        };

        Self {
            dir: ray_dir,
            delta_dist: Vector2::new(delta_dist_x, delta_dist_y),
            side_dist: Vector2::new(side_dist_x, side_dist_y),
            step_x,
            step_y,
            map_x,
            map_y,
        }
    }

    pub fn cast(&mut self, map: &Map) -> Option<CastResult> {
        let mut hit_vertical;

        loop {
            // 1. Jump to next grid square
            if self.side_dist.x < self.side_dist.y {
                self.side_dist.x += self.delta_dist.x;
                self.map_x += self.step_x;
                hit_vertical = true;
            } else {
                self.side_dist.y += self.delta_dist.y;
                self.map_y += self.step_y;
                hit_vertical = false;
            }

            // Boundary Check
            if self.map_x < 0 || self.map_y < 0 {
                return None;
            }

            // 2. Check Map
            match map.get_tile(self.map_x as u32, self.map_y as u32) {
                Some(tile) if tile > 0 => {
                    // 3. Calculate perpendicular distance
                    let dist = if hit_vertical {
                        self.side_dist.x - self.delta_dist.x
                    } else {
                        self.side_dist.y - self.delta_dist.y
                    };

                    return Some(CastResult {
                        distance: dist,
                        wall_value: tile,
                        hit_vertical,
                    });
                }
                Some(_) => continue, // It's an empty floor (0), keep jumping!
                None => return None, // We flew off the map
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_initialization_right() {
        let pos = Vector2::new(0.5, 0.5); // Center of tile (0,0)
        let dir = Vector2::new(1.0, 0.0); // Looking perfectly East
        let ray = Ray::new(pos, dir);

        assert_eq!(ray.map_x, 0);
        assert_eq!(ray.map_y, 0);
        assert_eq!(ray.step_x, 1);

        // Since we are at 0.5 and the tile ends at 1.0,
        // we have 0.5 units to go. delta_dist_x is 1.0.
        // So side_dist_x should be 0.5.
        assert!((ray.side_dist.x - 0.5).abs() < 1e-6);

        // Since we aren't moving vertically, delta_dist_y should be huge
        assert!(ray.delta_dist.y > 1e10);
    }

    #[test]
    fn test_ray_initialization_left() {
        let pos = Vector2::new(0.2, 0.5);
        let dir = Vector2::new(-1.0, 0.0); // Looking West
        let ray = Ray::new(pos, dir);

        assert_eq!(ray.step_x, -1);
        // Distance to the left edge (0.0) from 0.2 is 0.2
        assert!((ray.side_dist.x - 0.2).abs() < 1e-6);
    }

    #[test]
    fn test_ray_diagonal() {
        let pos = Vector2::new(0.5, 0.5);
        let dir = Vector2::new(1.0, 1.0);
        let ray = Ray::new(pos, dir);

        assert_eq!(ray.step_x, 1);
        assert_eq!(ray.step_y, 1);
        assert!((ray.side_dist.x - 0.5).abs() < 1e-6);
        assert!((ray.side_dist.y - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_cast() {
        let map = Map {
            width: 2,
            height: 2,
            grid: vec![0, 0, 255, 0, 0, 255],
        };
        let pos = Vector2::new(0.5, 0.5);
        let dir = Vector2::new(1.0, 0.0);
        let mut ray = Ray::new(pos, dir);

        let res = ray.cast(&map);
        assert!(res.is_some());
        assert_eq!(res.unwrap().distance, 1.5);
    }
}
