extern crate gfx;
extern crate gfx_device_gl;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate specs;
extern crate nalgebra;
extern crate time;
extern crate find_folder;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate art;
extern crate systems as sys;
extern crate utils;
extern crate components as comps;
extern crate graphics;
extern crate math;

pub mod event;
pub mod game;

use std::thread;

use math::{Point2, OrthographicHelper};
use utils::{GfxCoord};
use graphics::{build_graphics};
use event::{DevEventHub};
use sys::{render, control};
use game::{Game};

pub fn start() {
    let (width, height): (u32, u32) = (640, 480);

    let fov = 90.0;

    let znear = 0.0;

    let zfar = 10.0;

    let aspect_ratio = width as GfxCoord / height as GfxCoord;

    let ortho_helper = OrthographicHelper::new(aspect_ratio, fov, znear, zfar);

    let ((mut out_color, mut out_depth), mut factory, encoder, window, mut device) = build_graphics(640, 480);

    let (mut event_dev, game_event) = DevEventHub::new();

    event_dev.send_to_render(render::RecvEvent::GraphicsData(out_color.clone(), out_depth.clone()));

    event_dev.send_to_render(render::RecvEvent::Encoder(encoder.clone_empty()));
    event_dev.send_to_render(render::RecvEvent::Encoder(encoder));

    let game = Game::new(
        &mut factory,
        game_event,
        Point2::new(0.0, 0.0),
        Point2::new(
            out_color.get_dimensions().0 as ::utils::Coord,
            out_color.get_dimensions().1 as ::utils::Coord
        ),
        ortho_helper
    );

    thread::spawn(|| {
        let mut game = game;
        while game.frame() {}
    });

    'main: loop {
        match event_dev.recv_from_render() {
            render::SendEvent::Encoder(mut encoder) => {
                use gfx::Device;

                for event in window.poll_events() {
                    match event {
                        glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                        glutin::Event::Closed => break 'main,
                        _ => event_dev.process_glutin(event),
                    }
                }

                encoder.flush(&mut device);
                event_dev.send_to_render(render::RecvEvent::Encoder(encoder));
                match window.swap_buffers() {
                    Ok(()) => (),
                    Err(err) => panic!("window swap buffers error: {}", err),
                };
                device.cleanup();
            },
            render::SendEvent::Exited => panic!("render system has exited while in main loop"),
        }

        match event_dev.try_recv_from_control() {
            Some(event) => match event {
                control::SendEvent::Resize => {
                    gfx_window_glutin::update_views(&window, &mut out_color, &mut out_depth);
                    event_dev.send_to_render(render::RecvEvent::GraphicsData(out_color.clone(), out_depth.clone()));
                },
                control::SendEvent::Exited => panic!("control system has exited while in main loop"),
            },
            None => (),
        }

        while match event_dev.try_recv_from_game() {
            Some(event) => match event {
                ::game::SendEvent::Exited => panic!("game exited while in main loop"),
            },
            None => false,
        } {

        }
    }

    event_dev.send_to_render(render::RecvEvent::Exit);
    event_dev.send_to_control(control::RecvEvent::Exit);
    event_dev.send_to_game(game::RecvEvent::Exit);

    // while match try!(event_dev.recv_from_render()) {
    //     ::sys::render::SendEvent::Exited => false,
    //     _ => true,
    // } {
    //
    // }
    //
    // while match try!(event_dev.recv_from_control()) {
    //     ::sys::control::SendEvent::Exited => false,
    //     _ => true,
    // } {
    //
    // }
    //
    // while match try!(event_dev.recv_from_game()) {
    //     ::game::SendEvent::Exited => false,
    //     // _ => true,
    // } {
    //
    // }
}
