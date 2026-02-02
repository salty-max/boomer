pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn add(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn scale(&self, factor: f32) -> Vector2 {
        Vector2 {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    pub fn normalize(&self) -> Vector2 {
        let len = self.length();
        if len < 1e-6 {
            Vector2::zero()
        } else {
            Vector2 {
                x: self.x / len,
                y: self.y / len,
            }
        }
    }

    pub fn dot(&self, other: &Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn rotate(&self, _angle: f32) -> Vector2 {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vector2::new(1.0, 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
    }

    #[test]
    fn test_add() {
        let v1 = Vector2::new(3.0, 1.0);
        let v2 = Vector2::new(1.0, 2.0);
        let res = v1.add(&v2);
        assert_eq!(res.x, 4.0);
        assert_eq!(res.y, 3.0);
    }

    #[test]
    fn test_length() {
        let v = Vector2::new(3.0, 4.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_scale() {
        let v = Vector2::new(2.0, 3.0);
        let res = v.scale(2.0);
        assert_eq!(res.x, 4.0);
        assert_eq!(res.y, 6.0);
    }

    #[test]
    fn test_normalization() {
        let v = Vector2::new(3.0, 4.0);
        let unit = v.normalize();
        // Check if length is now 1.0
        assert!((unit.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_dot_product() {
        let v_forward = Vector2::new(1.0, 0.0);
        let v_up = Vector2::new(0.0, 1.0);
        let v_backward = Vector2::new(-1.0, 0.0);

        // 1. Perpendicular vectors (90 degrees) must be 0.0
        assert_eq!(v_forward.dot(&v_up), 0.0);

        // 2. Parallel vectors (facing same way) should be 1.0 * 1.0 = 1.0
        assert_eq!(v_forward.dot(&v_forward), 1.0);

        // 3. Opposite vectors should be -1.0
        assert_eq!(v_forward.dot(&v_backward), -1.0);
    }

    #[test]
    fn test_rotate() {
        let v = Vector2::new(1.0, 0.0);
        let quarter_turn = std::f32::consts::PI / 2.0;
        let result = v.rotate(quarter_turn);

        // We check absolute difference because floats are rarely "perfectly" 0.0
        assert!(result.x.abs() < 1e-6);
        assert!((result.y - 1.0).abs() < 1e-6);
    }
}
