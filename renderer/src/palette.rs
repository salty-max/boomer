use crate::color::Color;

/// A collection of predefined colors used throughout the engine.
pub mod palette {
    use super::Color;

    // Environment Colors
    pub const SKY: Color = Color::from_hex(0x2C3E50); // Deep Blue-Gray
    pub const FLOOR: Color = Color::from_hex(0x1A1A1A); // Near Black

    // Wall Colors (Using standard 90s FPS shades)
    pub const WALL_RED: Color = Color::from_hex(0xC0392B);
    pub const WALL_GREEN: Color = Color::from_hex(0x27AE60);
    pub const WALL_BLUE: Color = Color::from_hex(0x2980B9);
    pub const WALL_STONE: Color = Color::from_hex(0x7F8C8D);
}
