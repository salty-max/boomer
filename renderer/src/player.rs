use math::vector::Vector2;

pub struct Player {
    pub pos: Vector2,
    pub dir: Vector2,
    pub plane: Vector2,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vector2::new(x, y),
            dir: Vector2::new(0.0, -1.0),
            plane: Vector2::new(0.66, 0.0),
        }
    }

    pub fn rotate(&mut self, angle: f32) {
        self.dir.rotate_mut(angle);
        self.plane.rotate_mut(angle);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_rotation_integrity() {
        let mut p = Player::new(0.0, 0.0);
        // Rotate the player 45 degrees
        p.rotate(math::deg_to_rad(45.0));

        // The dot product should still be roughly 0
        let dot = p.dir.dot(&p.plane);
        assert!(dot.abs() < 1e-6);
    }
}
