use math::clamp;

use crate::palette::palette as colors;
use crate::{map::Map, player::Player, raycaster::Ray};

pub mod color;
pub mod map;
pub mod palette;
pub mod player;
pub mod raycaster;
pub mod texture;

pub fn render(player: &Player, map: &Map, buffer: &mut [u32], width: usize, height: usize) {
    // 1. Clear the screen (ceiling and floor)
    for i in 0..(width * height / 2) {
        buffer[i] = 0xFF333333;
    } // Ceiling
    for i in (width * height / 2)..(width * height) {
        buffer[i] = 0xFF111111;
    } // Floor

    // 2. Cast rays for every horizontal pixel
    for x in 0..width {
        // Calculate the ray direction based on the player's plane
        let camera_x = 2.0 * (x as f32) / (width as f32) - 1.0;
        let ray_dir = player.dir.add(&player.plane.scale(camera_x));

        let mut ray = Ray::new(player.pos, ray_dir);

        if let Some(res) = ray.cast(map) {
            // 3. Calculate wall height
            // It is the height of the screen divided by the distance
            let line_height = (height as f32 / res.distance) as i32;

            // Calculate where to start and end drawing the vertical line
            let half_h = height as i32 / 2;
            let start_y = clamp(-line_height / 2 + half_h, 0, height as i32 - 1);
            let end_y = clamp(line_height / 2 + half_h, 0, height as i32 - 1);

            // 4. Draw the pixels into the buffer
            let mut color = 0xFFCC0000;
            if res.hit_vertical {
                color = 0xFF880000;
            }

            for y in start_y..end_y {
                buffer[y as usize * width + x] = color;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_output() {
        let mut map = Map::new(10, 10);
        // Place a wall at X=5, Y=0 to 10 (a vertical strip)
        for y in 0..10 {
            map.set_tile(5, y, 1);
        }

        let player = Player::new(1.0, 5.0); // Standing at X=1, looking East
        let mut buffer = vec![0u32; 20 * 20];

        render(&player, &map, &mut buffer, 20, 20);

        let middle_pixel = buffer[10 * 20 + 10];

        // We use assert_eq! so Rust shows us the actual hex value found if it fails
        assert!(
            middle_pixel == 0xFFCC0000 || middle_pixel == 0xFF880000,
            "Expected wall color (Red), but found: {:#X}",
            middle_pixel
        );
    }
}
