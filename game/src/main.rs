use std::{collections::HashSet, time::Instant};

use pixels::Pixels;
use renderer::{map::Map, player::Player};
use winit::keyboard::KeyCode;

struct Framework {
    device_state: Pixels,
    player: Player,
    map: Map,
    width: u32,
    height: u32,
    key_state: HashSet<KeyCode>,
    last_frame_time: Instant
}

impl Framework {
    pub fn log_fps(&self, dt: f32) {
        if dt > 0.0 {
            let fps = 1.0 / dt;
            // Only log once every few seconds so we don't spam the console
            log::debug!("Performance: {:.2} FPS ({:.2}ms)", fps, dt * 1000.0);
        }
    }
}

fn main() {
    env_logger::init();
    log::info!("Starting Raycasting Engine...");
}
