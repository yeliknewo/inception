use std::sync::mpsc::{Sender, Receiver, TryRecvError};

use nalgebra;

use specs::{Planner, World};

use gfx_device_gl::Factory as GLFactory;

use find_folder::Search;

use time::{precise_time_ns};

//*************************************************************************************************

use comps::{RenderId, Transform, Camera, RenderData, Clickable, Wire, WireIn, WireOut};
use comps::non_components::{Map};

use sys::{Render, Control};

use graphics::{load_texture};

use event::{GameEventHub};

use utils::{Delta};
use utils::fps_counter::{FpsCounter};

use math::{OrthographicHelper, Point2, Point3I};

use art::{layers, wires, make_square_render};

//*************************************************************************************************

pub type Channel = (
    Sender<SendEvent>,
    Receiver<RecvEvent>,
);

#[derive(Debug)]
pub enum RecvEvent {
    Exit,
}

#[derive(Debug)]
pub enum SendEvent {
    Exited,
}

pub struct Game {
    planner: Planner<Delta>,
    last_time: u64,
    channel: Channel,
    fps_counter: FpsCounter,
}

impl Game {
    pub fn new(
        factory: &mut GLFactory,
        mut game_event_hub: GameEventHub,
        mouse_location: Point2,
        screen_resolution: Point2,
        ortho_helper: OrthographicHelper
    ) -> Game {
        let mut planner = {
            let mut w = World::new();

            w.register::<RenderId>();
            w.register::<Transform>();
            w.register::<Camera>();
            w.register::<RenderData>();
            w.register::<Clickable>();
            w.register::<Wire>();
            w.register::<WireIn>();
            w.register::<WireOut>();

            w.add_resource(Map::new());

            Planner::<Delta>::new(w, 8)
        };

        let mut renderer = Render::new(match game_event_hub.render_channel.take() {
            Some(channel) => channel,
            None => panic!("game event hub render channel was none"),
        });

        //make the camera
        planner.mut_world().create_now()
            .with(Camera::new_from_ortho_helper(
                nalgebra::Point3::new(0.0, 0.0, 2.0),
                nalgebra::Point3::new(0.0, 0.0, 0.0),
                nalgebra::Vector3::new(0.0, 1.0, 0.0),
                &ortho_helper,
                true
            ))
            .build();

        //make the basic square render packet
        let packet = make_square_render();

        //find the assets folder
        let assets_folder = match Search::ParentsThenKids(3, 3).for_folder("assets") {
            Ok(path) => path,
            Err(err) => panic!("error finding assets folder: {}", err),
        };

        //wires render with spritesheet id
        let wires_render = {
            let texture = load_texture(
                factory,
                assets_folder.join(
                    wires::NAME
                )
            );
            renderer.add_render_spritesheet(
                factory,
                &packet,
                texture
            )
        };

        planner.mut_world().create_now()
            .with(wires_render)
            .with(Transform::new(
                nalgebra::Isometry3::new(
                    nalgebra::Vector3::new(-1.0, 0.0, 1.0),
                    nalgebra::Vector3::new(0.0, 0.0, 0.0)
                ),
                nalgebra::Vector3::new(1.0, 1.0, 1.0)
            ))
            .with(WireIn::new_from_points(Point3I::new(0, 0, 0), Point3I::new(-1, 0, 0)))
            .with(RenderData::new(layers::WIRES, wires::DEFAULT_TINT, wires::RECT, wires::SIZE))
            .build();

        planner.mut_world().create_now()
            .with(wires_render)
            .with(Transform::new(
                nalgebra::Isometry3::new(
                    nalgebra::Vector3::new(0.0, 0.0, 1.0),
                    nalgebra::Vector3::new(0.0, 0.0, 0.0)
                ),
                nalgebra::Vector3::new(1.0, 1.0, 1.0)
            ))
            .with(Wire::new_from_points(Point3I::new(-1, 0, 0), Point3I::new(1, 0, 0), Point3I::new(0, 0, 0)))
            .with(RenderData::new(layers::WIRES, wires::DEFAULT_TINT, wires::RECT, wires::SIZE))
            .build();

        planner.mut_world().create_now()
            .with(wires_render)
            .with(Transform::new(
                nalgebra::Isometry3::new(
                    nalgebra::Vector3::new(1.0, 0.0, 1.0),
                    nalgebra::Vector3::new(0.0, 0.0, 0.0)
                ),
                nalgebra::Vector3::new(1.0, 1.0, 1.0)
            ))
            .with(WireOut::new_from_points(Point3I::new(0, 0, 0), Point3I::new(1, 0, 0)))
            .with(RenderData::new(layers::WIRES, wires::DEFAULT_TINT, wires::RECT, wires::SIZE))
            .build();

        planner.add_system(
            Control::new(
                match game_event_hub.control_channel.take() {
                    Some(channel) => channel,
                    None => panic!("game event hub control channel was none"),
                },
                Point2::new(10.0, 10.0),
                mouse_location,
                screen_resolution,
                ortho_helper,
            ),
            "control",
            30
        );

        planner.add_system(renderer, "renderer", 10);

        Game {
            planner: planner,
            last_time: precise_time_ns(),
            channel: match game_event_hub.game_channel.take() {
                Some(channel) => channel,
                None => panic!("game event hub game channel was none"),
            },
            fps_counter: FpsCounter::new(),
        }
    }


    pub fn frame(&mut self) -> bool {
        let new_time = precise_time_ns();
        let delta = (new_time - self.last_time) as Delta / 1e9;
        self.last_time = new_time;

        match self.channel.1.try_recv() {
            Err(TryRecvError::Empty) => {
                self.planner.dispatch(delta);
                self.fps_counter.frame(delta);
                true
            },
            Ok(RecvEvent::Exit) |
            Err(TryRecvError::Disconnected) => {
                self.planner.wait();
                false
            },
        }
    }
}
