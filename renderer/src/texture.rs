pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

impl Texture {
    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let starting_index: usize = ((x + (y * self.width)) * 4).try_into().unwrap();

        [
            self.pixels[starting_index],
            self.pixels[starting_index + 1],
            self.pixels[starting_index + 2],
            self.pixels[starting_index + 3],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pixel() {
        // Create a tiny 2x2 texture (4 pixels total)
        // Pixel colors: [Red, Green, Blue, White]
        let pixels = vec![
            255, 0, 0, 255,   // (0,0) - Red
            0, 255, 0, 255,   // (1,0) - Green
            0, 0, 255, 255,   // (0,1) - Blue
            255, 255, 255, 255, // (1,1) - White
        ];

        let texture = Texture {
            width: 2,
            height: 2,
            pixels,
        };

        // Check the Red pixel
        assert_eq!(texture.get_pixel(0, 0), [255, 0, 0, 255]);
        // Check the Green pixel
        assert_eq!(texture.get_pixel(1, 0), [0, 255, 0, 255]);
        // Check the Blue pixel
        assert_eq!(texture.get_pixel(0, 1), [0, 0, 255, 255]);
        // Check the White pixel
        assert_eq!(texture.get_pixel(1, 1), [255, 255, 255, 255]);
    }
}
