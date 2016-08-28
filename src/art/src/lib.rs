extern crate gfx;
extern crate gfx_device_gl;
extern crate find_folder;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate graphics;
extern crate utils;

use gfx::state::Rasterizer;

use graphics::spritesheet::{Packet, Vertex};

pub fn make_square_render() -> Packet {
    let vertices = vec!(
        Vertex::new([0.0, 0.0, 0.0], [1.0, 1.0]),
        Vertex::new([0.0, 1.0, 0.0], [1.0, 0.0]),
        Vertex::new([1.0, 1.0, 0.0], [0.0, 0.0]),
        Vertex::new([1.0, 0.0, 0.0], [0.0, 1.0]),
    );

    let indices = vec!(
        0, 3, 2, 2, 1, 0,
    );

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

pub mod layers {
    pub const WIRES: u8 = 0;
    pub const EMPTY: u8 = 5;
}

pub mod wires {
    pub const NAME: &'static str = "wires.png";
    pub const SIZE: [f32; 2] = [32.0, 32.0];
    pub const RECT: [f32; 4] = [0.0, 0.0, 32.0, 32.0];
    pub const DEFAULT_TINT: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
}

pub mod empty {
    pub const NAME: &'static str = "empty.png";
    pub const SIZE: [f32; 2] = [32.0, 32.0];
    pub const RECT: [f32; 4] = [0.0, 0.0, 32.0, 32.0];
    pub const DEFAULT_TINT: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
}
