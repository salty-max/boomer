pub struct Map {
    pub width: u32,
    pub height: u32,
    pub grid: Vec<u8>
}

impl Map {
    pub fn get_tile(&self, x: u32, y: u32) -> Option<u8> {
        if x > self.width || y > self.height {
            return None;
        }
        let index = (x + (y * self.width)) as usize;
        Some(self.grid[index])
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_indexing() {
        // A 3x2 map
        // 1 1 1
        // 1 0 1
        let map = Map {
            width: 3,
            height: 2,
            grid: vec![
                1, 1, 1,
                1, 0, 1,
            ],
        };

        assert_eq!(map.get_tile(1, 1), Some(0)); // The empty center
        assert_eq!(map.get_tile(2, 0), Some(1)); // The top-right wall
    }

    #[test]
    fn test_map_safety() {
        let map = Map {
            width: 2,
            height: 2,
            grid: vec![0, 0, 0, 0], // All empty floor
        };

        // This coordinate is way outside the 2x2 grid
        let tile = map.get_tile(10, 10);

        // It should return None instead of crashing
        assert_eq!(tile, None);
    }
}
