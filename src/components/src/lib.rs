extern crate gfx;
extern crate gfx_device_gl;
extern crate specs;
extern crate nalgebra;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate graphics;
extern crate utils;
extern crate math;
extern crate art;

pub mod camera;
pub mod clickable;
pub mod render_data;
pub mod render_id;
pub mod transform;
pub mod wire_in;
pub mod wire_out;
pub mod wire;

pub use self::camera::Component as Camera;
pub use self::clickable::Component as Clickable;
pub use self::render_data::Component as RenderData;
pub use self::render_id::Component as RenderId;
pub use self::transform::Component as Transform;
pub use self::wire_in::Component as WireIn;
pub use self::wire_out::Component as WireOut;
pub use self::wire::Component as Wire;

pub mod non_components {
    pub mod link;
    pub mod map;

    pub use self::link::Link;
    pub use self::map::Map;
}
