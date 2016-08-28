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
}

pub mod wires {
    pub const NAME: &'static str = "wires.png";
    pub const SIZE: [f32; 2] = [32.0, 32.0];
    pub const RECT: [f32; 4] = [0.0, 0.0, 32.0, 32.0];
    pub const DEFAULT_TINT: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
}

// pub const ERROR: &'static [f32; 4] = &[0.0, 0.0, 1.0, 1.0];
//
// pub mod layers {
//     pub const DEFAULT_LAYER: u8 = 5;
//     pub const PLAYER: u8 = 1;
//     pub const TILES: u8 = 0;
// }
//
// pub mod p1 {
//     pub const SIZE:   [f32; 2] = [508.0, 288.0];
//
//     pub const DEFAULT_TINT: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
//
//     pub const DUCK:   [f32; 4] = [365.0, 98.0,  69.0, 71.0];
//     pub const FRONT:  [f32; 4] = [0.0,   196.0, 66.0, 92.0];
//     pub const HURT:   [f32; 4] = [438.0, 0.0,   69.0, 92.0];
//     pub const JUMP:   [f32; 4] = [438.0, 93.0,  67.0, 94.0];
//     pub const STAND:  [f32; 4] = [67.0,  196.0, 66.0, 92.0];
//     pub const WALK01: [f32; 4] = [0.0,   0.0,   72.0, 97.0];
//     pub const WALK02: [f32; 4] = [73.0,  0.0,   72.0, 97.0];
//     pub const WALK03: [f32; 4] = [146.0, 0.0,   72.0, 97.0];
//     pub const WALK04: [f32; 4] = [0.0,   98.0,  72.0, 97.0];
//     pub const WALK05: [f32; 4] = [73.0,  98.0,  72.0, 97.0];
//     pub const WALK06: [f32; 4] = [146.0, 98.0,  72.0, 97.0];
//     pub const WALK07: [f32; 4] = [219.0, 0.0,   72.0, 97.0];
//     pub const WALK08: [f32; 4] = [292.0, 0.0,   72.0, 97.0];
//     pub const WALK09: [f32; 4] = [219.0, 98.0,  72.0, 97.0];
//     pub const WALK10: [f32; 4] = [365.0, 0.0,   72.0, 97.0];
//     pub const WALK11: [f32; 4] = [292.0, 98.0,  72.0, 97.0];
//
//     pub const WALK:   [[f32; 4]; 11] = [
//         WALK01,
//         WALK02,
//         WALK03,
//         WALK04,
//         WALK05,
//         WALK06,
//         WALK07,
//         WALK08,
//         WALK09,
//         WALK10,
//         WALK11
//     ];
// }
//
// pub mod tiles {
//     pub const SIZE:      [f32; 2] = [914.0, 936.0];
//
//     pub const SELECTED_TINT: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
//
//     pub const FOREGROUND_TINT: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
//
//     pub const BACKGROUND_TINT: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
//
//     pub const GRASS:     [f32; 4] = [648.0, 0.0,   70.0, 70.0];
//     // pub const GRASS_MID: [f32; 4] = [504.0, 576.0, 70.0, 70.0];
//     pub const GRASS_MID: [f32; 4] = [504.0 + 1.0,  576.0 + 1.0, 70.0 - 2.0, 70.0 - 2.0];
//     // pub const GRASS_CENTER: [f32; 4] = [576.0, 864.0, 70.0, 70.0];
//     pub const GRASS_CENTER: [f32; 4] = [576.0 + 1.0, 864.0 + 1.0, 70.0 - 2.0, 70.0 - 2.0];
// }
