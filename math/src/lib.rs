use std::f32::consts::PI;

pub mod vector;

/// Clamps a value between a minimum and maximum bound.
/// Works for any type that implements PartialOrd (f32, i32, etc.)
pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

/// Returns the smaller of two values.
pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

/// Returns the larger of two values.
pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

pub fn deg_to_rad(deg: f32) -> f32 {
    deg * (PI / 180.0)
}

pub fn rad_to_deg(rad: f32) -> f32 {
    rad * (180.0 / PI)
}

pub fn wrap_angle(rad: f32) -> f32 {
    let two_pi = 2.0 * PI;
    ((rad % two_pi) + two_pi) % two_pi
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_math() {
        // Testing with floats
        assert_eq!(clamp(10.5, 0.0, 5.0), 5.0);

        // Testing with integers
        assert_eq!(clamp(10, 0, 5), 5);

        // Testing min/max with integers
        assert_eq!(min(1, 10), 1);
        assert_eq!(max(1, 10), 10);
    }

    #[test]
    fn test_conversions() {
        assert!((deg_to_rad(180.0) - PI).abs() < 1e-6);
        assert!((rad_to_deg(PI) - 180.0).abs() < 1e-6);
    }

    #[test]
    fn test_angle_wrap() {
        // 450 degrees is 360 + 90. It should wrap to 90 degrees (PI/2)
        let large_angle = deg_to_rad(450.0);
        let wrapped = wrap_angle(large_angle);
        assert!((wrapped - (PI / 2.0)).abs() < 1e-6);
    }
}
