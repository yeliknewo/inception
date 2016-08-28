extern crate gfx;
extern crate gfx_device_gl;
extern crate glutin;
extern crate specs;
extern crate nalgebra;
extern crate time;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate graphics;
extern crate utils;
extern crate components as comps;
extern crate math;
extern crate art;

pub mod control;
pub mod render;

pub use self::control::System as Control;
pub use self::render::System as Render;
