#[derive(Clone, Copy, Debug)]
/// A simple RGBA color structure for the raycaster.
pub struct Color {
    /// Red channel (0-255)
    pub r: u8,
    /// Green channel (0-255)
    pub g: u8,
    /// Blue channel (0-255)
    pub b: u8,
    /// Alpha channel (0-255)
    pub a: u8,
}

impl Color {
    /// Creates a new color from raw RGBA components.
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a color from a 24-bit hex integer (0xRRGGBB).
    /// Alpha is set to 255 (fully opaque) by default.
    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
            a: 255,
        }
    }

    /// Creates a color from a 32-bit hex integer with Alpha (0xRRGGBBAA).
    pub const fn from_hex_alpha(hex: u32) -> Self {
        Self {
            r: ((hex >> 24) & 0xFF) as u8,
            g: ((hex >> 16) & 0xFF) as u8, // Fixed shift
            b: ((hex >> 8) & 0xFF) as u8,
            a: (hex & 0xFF) as u8,
        }
    }

    /// Returns the color as a `[u8; 4]` array in RGBA order.
    /// This is compatible with the `pixels` crate buffer format.
    pub fn to_array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Returns a new color with the RGB channels multiplied by `factor`.
    ///
    /// Useful for simple shading. A factor of 0.5 makes the color half as bright.
    /// Note: The Alpha channel remains unchanged.
    pub fn darkened(&self, factor: f32) -> Self {
        Self {
            r: (self.r as f32 * factor).round() as u8,
            g: (self.g as f32 * factor).round() as u8,
            b: (self.b as f32 * factor).round() as u8,
            a: self.a, // Keep walls opaque!
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_parsing() {
        let red = Color::from_hex(0xFF0000);
        assert_eq!(red.r, 255);
        assert_eq!(red.g, 0);
        assert_eq!(red.b, 0);
        assert_eq!(red.a, 255);
    }

    #[test]
    fn test_hex_alpha_parsing() {
        let translucent_blue = Color::from_hex_alpha(0x0000FF80);
        assert_eq!(translucent_blue.r, 0);
        assert_eq!(translucent_blue.g, 0);
        assert_eq!(translucent_blue.b, 255);
        assert_eq!(translucent_blue.a, 128); // 0x80 is 128
    }

    #[test]
    fn test_darkened() {
        let white = Color::new(200, 200, 200, 255);
        let gray = white.darkened(0.5);
        assert_eq!(gray.r, 100);
        assert_eq!(gray.a, 255); // Alpha should stay the same
    }

    #[test]
    fn test_to_array() {
        let color = Color::new(1, 2, 3, 4);
        assert_eq!(color.to_array(), [1, 2, 3, 4]);
    }
}
