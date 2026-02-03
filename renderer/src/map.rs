pub struct Map {
    pub width: u32,
    pub height: u32,
    pub grid: Vec<u8>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            // Initialize the grid with `0` (empty floor) for every tile
            grid: vec![0; (width * height) as usize],
        }
    }

    pub fn set_tile(&mut self, x: u32, y: u32, value: u8) {
        // Bounds check
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.grid[idx] = value;
        }
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Option<u8> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let idx = (y * self.width + x) as usize;
        self.grid.get(idx).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map = Map::new(10, 10);
        assert_eq!(map.grid.len(), 100);
        assert_eq!(map.get_tile(9, 9), Some(0));
    }

    #[test]
    fn test_set_and_get() {
        let mut map = Map::new(3, 3);
        map.set_tile(1, 1, 255);
        assert_eq!(map.get_tile(1, 1), Some(255));
    }

    #[test]
    fn test_map_bounds_safety() {
        let map = Map::new(2, 2);
        // Checking the exact edge (x=2 on a width=2 map is OUT)
        assert_eq!(map.get_tile(2, 0), None);
        assert_eq!(map.get_tile(0, 2), None);
    }
}
