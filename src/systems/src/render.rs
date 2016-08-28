use gfx::traits::{Factory, FactoryExt};

pub type Channel = (
    ::std::sync::mpsc::Sender<SendEvent>,
    ::std::sync::mpsc::Receiver<RecvEvent>
);

pub enum SendEvent {
    Encoder(::gfx::Encoder<::gfx_device_gl::Resources, ::gfx_device_gl::CommandBuffer>),
    Exited,
}

pub enum RecvEvent {
    Encoder(::gfx::Encoder<::gfx_device_gl::Resources, ::gfx_device_gl::CommandBuffer>),
    GraphicsData(::gfx::handle::RenderTargetView<::gfx_device_gl::Resources, ::graphics::ColorFormat>, ::gfx::handle::DepthStencilView<::gfx_device_gl::Resources, ::graphics::DepthFormat>),
    Exit,
}

pub struct System {
    channel: Channel,
    out_color: ::gfx::handle::RenderTargetView<::gfx_device_gl::Resources, ::graphics::ColorFormat>,
    out_depth: ::gfx::handle::DepthStencilView<::gfx_device_gl::Resources, ::graphics::DepthFormat>,
    spritesheet_bundles: ::std::sync::Arc<Vec<::graphics::spritesheet::Bundle>>,
    spritesheet_shaders: ::graphics::Shaders,
    exited: bool,
}

impl System {
    pub fn new(
        channel: Channel
    ) -> System
    {
        let (out_color, out_depth) = match channel.1.recv() {
            Ok(event) => match event {
                RecvEvent::GraphicsData(out_color, out_depth) => (out_color, out_depth),
                _ => panic!("render system received non graphics data first from channel"),
            },
            Err(err) => panic!("new channel 1 rect error: {}", err),
        } ;

        System {
            channel: channel,
            out_color: out_color,
            out_depth: out_depth,
            spritesheet_bundles: ::std::sync::Arc::new(Vec::new()),
            spritesheet_shaders: ::graphics::spritesheet::make_shaders(),
            exited: false,
        }
    }

    pub fn add_render_type_spritesheet(&mut self,
        factory: &mut ::gfx_device_gl::Factory,
        packet: &::graphics::spritesheet::Packet,
        texture: ::gfx::handle::ShaderResourceView<::gfx_device_gl::Resources, [f32; 4]>
    ) -> ::comps::RenderType {

        self.add_render_type_spritesheet_raw(factory, packet.get_vertices(), packet.get_indices(), packet.get_rasterizer(), texture)
    }

    fn add_render_type_spritesheet_raw(&mut self,
        factory: &mut ::gfx_device_gl::Factory,
        vertices: &[::graphics::spritesheet::Vertex],
        indices: &[::graphics::spritesheet::Index],
        rasterizer: ::gfx::state::Rasterizer,
        spritesheet: ::gfx::handle::ShaderResourceView<::gfx_device_gl::Resources, [f32; 4]>
    ) -> ::comps::RenderType {
        let shader_set = match factory.create_shader_set(self.spritesheet_shaders.get_vertex_shader(), self.spritesheet_shaders.get_fragment_shader()) {
            Ok(shaders) => shaders,
            Err(err) => panic!("add render type spritesheet raw create shader set error: {}", err),
        };

        let program  = match factory.create_program(&shader_set) {
            Ok(program) => program,
            Err(err) => panic!("add render type spritesheet raw create program error: {}", err),
        };

        let pso = match factory.create_pipeline_from_program(
            &program,
            ::gfx::Primitive::TriangleList,
            rasterizer,
            ::graphics::spritesheet::pipe::new()
        ) {
            Ok(pso) => pso,
            Err(err) => panic!("add render type spritesheet raw create pipeline error: {}", err),
        };

        let sampler_info = ::gfx::tex::SamplerInfo::new(
            ::gfx::tex::FilterMethod::Scale,
            ::gfx::tex::WrapMode::Mirror
        );

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertices, indices);
        let data = ::graphics::spritesheet::pipe::Data {
            vbuf: vbuf,
            spritesheet: (spritesheet, factory.create_sampler(sampler_info)),
            texture_data: factory.create_constant_buffer(1),
            projection_cb: factory.create_constant_buffer(1),
            out_color: self.out_color.clone(),
            out_depth: self.out_depth.clone(),
        };

        let id = self.spritesheet_bundles.len();
        let mut bundles = match ::std::sync::Arc::get_mut(&mut self.spritesheet_bundles) {
            Some(bundles) => bundles,
            None => panic!("add render type spritesheet raw get mut bundles was none"),
        };
        bundles.push(::graphics::spritesheet::Bundle::new(slice, pso, data));
        ::comps::RenderType {
            id: id,
            // renderer_type: ::graphics::RendererType::Spritesheet,
        }
    }

    fn render(&mut self, arg: &::specs::RunArg, mut encoder: ::gfx::Encoder<::gfx_device_gl::Resources, ::gfx_device_gl::CommandBuffer>) {
        use specs::Join;

        let (draw, transform, mut camera, mut render_data) = arg.fetch(|w|
            (
                w.read::<::comps::RenderType>(),
                w.read::<::comps::Transform>(),
                w.write::<::comps::Camera>(),
                w.write::<::comps::RenderData>()
            )
        );

        encoder.clear(&self.out_color, [1.0, 1.0, 1.0, 1.0]);
        encoder.clear_depth(&self.out_depth, 1.0);

        let (view, proj, dirty_cam) = {
            let mut camera = {
                let mut camera_opt = None;

                for c in (&mut camera).iter() {
                    camera_opt = Some(c);
                }

                match camera_opt {
                    Some(camera) => camera,
                    None => panic!("render camera opt was none"),
                }
            };

            (camera.get_view(), camera.get_proj(), camera.take_dirty())
        };

        let mut data = vec!();

        for (d, t, mut rd) in (&draw, &transform, &mut render_data).iter() {
            let mut projection_data = None;

            if dirty_cam {
                projection_data = Some(::graphics::ProjectionData {
                    model: t.get_model(),
                    view: view,
                    proj: proj,
                });
            }

            let mut texture_data = None;

            if rd.take_dirty() {
                texture_data = Some(::graphics::spritesheet::TextureData {
                    tint: rd.get_tint(),
                    spritesheet_rect: rd.get_spritesheet_rect(),
                    spritesheet_size: rd.get_spritesheet_size(),
                    mirror: rd.get_mirror(),
                });
            }

            data.push((d.id, rd.get_layer(), texture_data, projection_data));
        }

        data.sort_by_key(|k| k.1);

        for data in data {
            let b = &self.spritesheet_bundles[data.0];

            if let Some(texture_data) = data.2 {
                encoder.update_constant_buffer(&b.data.texture_data, &texture_data);
            }

            if let Some(projection_data) = data.3 {
                encoder.update_constant_buffer(&b.data.projection_cb, &projection_data);
            }

            b.encode(&mut encoder);
        }


        match self.channel.0.send(SendEvent::Encoder(encoder)) {
            Ok(()) => (),
            Err(err) => panic!("render channel 0 send error: {}", err),
        }
    }

    fn set_graphics_data(&mut self, out_color: ::gfx::handle::RenderTargetView<::gfx_device_gl::Resources, ::graphics::ColorFormat>, out_depth: ::gfx::handle::DepthStencilView<::gfx_device_gl::Resources, ::graphics::DepthFormat>) {
        self.out_color = out_color;
        self.out_depth = out_depth;

        for bundle in match ::std::sync::Arc::get_mut(&mut self.spritesheet_bundles) {
            Some(bundle) => bundle,
            None => panic!("set graphics data get mut texture bundles was none"),
        }  {
            bundle.data.out_color = self.out_color.clone();
            bundle.data.out_depth = self.out_depth.clone();
        }
    }

    fn exit(&mut self, arg: &::specs::RunArg) {
        //use to save

        arg.fetch(|_| ());
    }

    fn process_event(&mut self, arg: &::specs::RunArg, event: RecvEvent) -> bool {
        match event {
            RecvEvent::Encoder(encoder) => {
                self.render(arg, encoder);
                false
            },
            RecvEvent::GraphicsData(out_color, out_depth) => {
                self.set_graphics_data(out_color, out_depth);
                true
            },
            RecvEvent::Exit => {
                self.exit(arg);
                match self.channel.0.send(SendEvent::Exited) {
                    Ok(()) => (),
                    Err(err) => panic!("process event exit send error: {}", err),
                }
                self.exited = true;
                false
            },
        }
    }
}

impl ::specs::System<::utils::Delta> for System {
    fn run(&mut self, arg: ::specs::RunArg, _: ::utils::Delta) {
        if self.exited {
            arg.fetch(|_| ());
            return;
        }

        let mut event = match self.channel.1.recv() {
            Ok(event) => event,
            Err(err) => panic!("run channel 1 recv error: {}", err),
        };
        while self.process_event(&arg, event) {
            event = match self.channel.1.recv() {
                Ok(event) => event,
                Err(err) => panic!("run channel 1 recv error: {}", err),
            };
        }
    }
}
