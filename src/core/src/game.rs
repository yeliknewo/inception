pub type Channel = (
    ::std::sync::mpsc::Sender<SendEvent>,
    ::std::sync::mpsc::Receiver<RecvEvent>,
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
    planner: ::specs::Planner<::utils::Delta>,
    last_time: u64,
    channel: Channel,
    fps_counter: ::utils::fps_counter::FpsCounter,
}

impl Game {
    pub fn new(
        factory: &mut ::gfx_device_gl::Factory,
        mut game_event_hub: ::event::GameEventHub,
        mouse_location: ::math::Point2,
        screen_resolution: ::math::Point2,
        ortho_helper: ::math::OrthographicHelper
    ) -> Game {
        let mut planner = {
            let mut w = ::specs::World::new();

            w.register::<::comps::RenderType>();
            w.register::<::comps::Transform>();
            w.register::<::comps::Camera>();
            w.register::<::comps::RenderData>();
            w.register::<::comps::Clickable>();
            // w.register::<::comps::Dwarf>();
            // w.register::<::comps::Living>();
            // w.register::<::comps::Physical>();
            // w.register::<::comps::Tile>();
            // w.register::<::comps::PathFindingData>();

            // w.add_resource(::comps::TileMap::new());
            // w.add_resource(::comps::PathsStorage::new());

            ::specs::Planner::<::utils::Delta>::new(w, 8)
        };

        let mut renderer = ::sys::render::System::new(match game_event_hub.render_channel.take() {
            Some(channel) => channel,
            None => panic!("game event hub render channel was none"),
        });

        planner.mut_world().create_now()
            .with(::comps::Camera::new_from_ortho_helper(
                ::nalgebra::Point3::new(0.0, 0.0, 2.0),
                ::nalgebra::Point3::new(0.0, 0.0, 0.0),
                ::nalgebra::Vector3::new(0.0, 1.0, 0.0),
                &ortho_helper,
                true
            ))
            .build();

        let packet = ::art::square::make_square_render();

        let assets_folder = match ::find_folder::Search::ParentsThenKids(3, 3).for_folder("assets") {
            Ok(path) => path,
            Err(err) => panic!("error finding assets folder: {}", err),
        };

        // let tiles_render = {
        //     let texture = ::graphics::load_texture(
        //         factory,
        //         assets_folder.join(
        //             "Tiles/tiles_spritesheet.png"
        //         )
        //     );
        //     renderer.add_render_type_spritesheet(
        //         factory,
        //         &packet,
        //         texture
        //     )
        // };

        let p1_render = {
            let texture = ::graphics::load_texture(
                factory,
                assets_folder.join(
                    "Player/p1_spritesheet.png"
                )
            );
            renderer.add_render_type_spritesheet(
                factory,
                &packet,
                texture
            )
        };

        // let p1_idle = vec!(::art::square::p1::STAND);

        let mut p1_walk = vec!();
        p1_walk.extend_from_slice(&::art::square::p1::WALK);

        // let p1_fall = vec!(::art::square::p1::HURT);

        for _ in 0..1 {
            planner.mut_world().create_now()
                .with(p1_render)
                .with(::comps::Transform::new(
                    ::nalgebra::Isometry3::new(
                        ::nalgebra::Vector3::new(0.0, 0.0, 1.0),
                        ::nalgebra::Vector3::new(0.0, 0.0, 0.0)
                    ),
                    ::nalgebra::Vector3::new(1.0, 1.0, 1.0)
                ))
                .with(::comps::RenderData::new(::art::square::layers::PLAYER, ::art::square::p1::DEFAULT_TINT, ::art::square::p1::STAND, ::art::square::p1::SIZE))
                // .with(::comps::Physical::new(::math::Point2::new(0.0, 0.0), ::math::Point2::new(1.0, 1.0), ::math::Point2::new(0.001, 0.001)))
                // .with(::comps::Living::new(
                //     p1_idle.clone(),
                //     p1_walk.clone(),
                //     p1_fall.clone()
                // ))
                // .with(::comps::Dwarf::new(5.0))
                // .with(::comps::PathFindingData::new())
                .build();
        }

        planner.add_system(
            ::sys::control::System::new(
                match game_event_hub.control_channel.take() {
                    Some(channel) => channel,
                    None => panic!("game event hub control channel was none"),
                },
                ::math::Point2::new(10.0, 10.0),
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
            last_time: ::time::precise_time_ns(),
            channel: match game_event_hub.game_channel.take() {
                Some(channel) => channel,
                None => panic!("game event hub game channel was none"),
            },
            fps_counter: ::utils::fps_counter::FpsCounter::new(),
        }
    }


    pub fn frame(&mut self) -> bool {
        let new_time = ::time::precise_time_ns();
        let delta = (new_time - self.last_time) as ::utils::Delta / 1e9;
        self.last_time = new_time;

        match self.channel.1.try_recv() {
            Err(::std::sync::mpsc::TryRecvError::Empty) => {
                self.planner.dispatch(delta);
                self.fps_counter.frame(delta);
                true
            },
            Ok(RecvEvent::Exit) |
            Err(::std::sync::mpsc::TryRecvError::Disconnected) => {
                self.planner.wait();
                false
            },
        }
    }
}
