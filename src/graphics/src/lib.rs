#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate image;
extern crate find_folder;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate utils;

use std::io::{BufReader, Read};
use std::fs::{File};
use std::path::{Path};

use gfx::{Factory, Encoder};
use gfx::handle::{RenderTargetView, DepthStencilView, ShaderResourceView};
use gfx::tex::{Size, AaMode, Kind};
use gfx::format::{Rgba8, DepthStencil};

use gfx_device_gl::{Device, Resources, CommandBuffer};
use gfx_device_gl::Factory as GLFactory;

use glutin::{WindowBuilder, Window};

use find_folder::Search;

pub mod spritesheet;

pub type ColorFormat = Rgba8;
pub type DepthFormat = DepthStencil;

#[derive(Debug)]
pub struct Shaders {
    vertex: Vec<u8>,
    fragment: Vec<u8>,
}

impl Shaders {
    pub fn new(vertex_name: &'static str, fragment_name: &'static str) -> Shaders {
        let shaders_path = match Search::ParentsThenKids(3, 3).for_folder("shader") {
            Ok(shaders_path) => shaders_path,
            Err(err) => panic!("find folder shader error: {}", err),
        };

        let mut vertex_path = shaders_path.clone();
        let mut fragment_path = shaders_path.clone();

        vertex_path.push(vertex_name);
        fragment_path.push(fragment_name);

        let vertex_file = match File::open(vertex_path) {
            Ok(file) => file,
            Err(err) => panic!("vertex file open error: {}", err),
        };
        let fragment_file = match File::open(fragment_path) {
            Ok(file) => file,
            Err(err) => panic!("fragment file open error: {}", err),
        };

        let mut vertex_reader = BufReader::new(vertex_file);
        let mut fragment_reader = BufReader::new(fragment_file);

        let mut vertex_buffer = vec!();
        let mut fragment_buffer = vec!();

        match vertex_reader.read_to_end(&mut vertex_buffer) {
            Ok(_) => (),
            Err(err) => panic!("vertex reader read to end error: {}", err),
        };
        match fragment_reader.read_to_end(&mut fragment_buffer) {
            Ok(_) => (),
            Err(err) => panic!("fragment reader read to end error: {}", err),
        }

        Shaders {
            vertex: vertex_buffer,
            fragment: fragment_buffer,
        }
    }

    pub fn get_vertex_shader(&self) -> &[u8] {
        self.vertex.as_slice()
    }

    pub fn get_fragment_shader(&self) -> &[u8] {
        self.fragment.as_slice()
    }
}

pub fn load_texture<P>(factory: &mut GLFactory, path: P) -> ShaderResourceView<Resources, [f32; 4]>
where P: AsRef<Path>
{
    let image = match image::open(path) {
        Ok(image) => image,
        Err(err) => panic!("image load error: {}", err),
    }.to_rgba();
    let (width, height) = image.dimensions();
    let kind = Kind::D2(width as Size, height as Size, AaMode::Single);
    let (_, view) = match factory.create_texture_const_u8::<ColorFormat>(kind, &[&image]) {
        Ok(data) => data,
        Err(err) => panic!("factory create texture const error: {}", err),
    };
    view
}

pub fn build_graphics(width: u32, height: u32) -> (
    (RenderTargetView<Resources, ColorFormat>, DepthStencilView<Resources, DepthFormat>),
    GLFactory,
    Encoder<Resources, CommandBuffer>,
    Window,
    Device
) {
    let builder = WindowBuilder::new()
        .with_title("Explore")
        .with_dimensions(width, height)
        .with_vsync()
    ;

    let (window, device, mut factory, out_color, out_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    (
        (
            out_color,
            out_depth
        ),
        factory,
        encoder,
        window,
        device
    )
}

gfx_constant_struct!(
    ProjectionData {
        model: [[f32; 4]; 4] = "u_Model",
        view: [[f32; 4]; 4] = "u_View",
        proj: [[f32; 4]; 4] = "u_Proj",
    }
);
