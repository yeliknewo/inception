use std::sync::mpsc::{Sender, Receiver};
use std::sync::Arc;

use gfx::traits::{Factory, FactoryExt};
use gfx::{Encoder, Primitive};
use gfx::handle::{ShaderResourceView, RenderTargetView, DepthStencilView};
use gfx::state::{Rasterizer};
use gfx::tex::{SamplerInfo, FilterMethod, WrapMode};

use gfx_device_gl::{Resources, CommandBuffer};
use gfx_device_gl::Factory as GLFactory;

use specs::RunArg;
use specs;

use graphics::{ColorFormat, DepthFormat, ProjectionData};
use graphics::spritesheet::{Vertex, Index, make_shaders, Bundle, Packet, TextureData, pipe};
use graphics::Shaders;

use utils::Delta;

use comps::{RenderId, Transform, Camera, RenderData};

pub type Channel = (
    Sender<SendEvent>,
    Receiver<RecvEvent>
);

pub enum SendEvent {
    Encoder(Encoder<Resources, CommandBuffer>),
    Exited,
}

pub enum RecvEvent {
    Encoder(Encoder<Resources, CommandBuffer>),
    GraphicsData(RenderTargetView<Resources, ColorFormat>, DepthStencilView<Resources, DepthFormat>),
    Exit,
}

pub struct System {
    channel: Channel,
    out_color: RenderTargetView<Resources, ColorFormat>,
    out_depth: DepthStencilView<Resources, DepthFormat>,
    bundles: Arc<Vec<Bundle>>,
    shaders: Shaders,
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
            bundles: Arc::new(Vec::new()),
            shaders: make_shaders(),
            exited: false,
        }
    }

    pub fn add_render_spritesheet(&mut self,
        factory: &mut GLFactory,
        packet: &Packet,
        texture: ShaderResourceView<Resources, [f32; 4]>
    ) -> RenderId {
        self.add_render_spritesheet_raw(factory, packet.get_vertices(), packet.get_indices(), packet.get_rasterizer(), texture)
    }

    fn add_render_spritesheet_raw(&mut self,
        factory: &mut GLFactory,
        vertices: &[Vertex],
        indices: &[Index],
        rasterizer: Rasterizer,
        spritesheet: ShaderResourceView<Resources, [f32; 4]>
    ) -> RenderId {
        let shader_set = match factory.create_shader_set(self.shaders.get_vertex_shader(), self.shaders.get_fragment_shader()) {
            Ok(shaders) => shaders,
            Err(err) => panic!("add render type spritesheet raw create shader set error: {}", err),
        };

        let program  = match factory.create_program(&shader_set) {
            Ok(program) => program,
            Err(err) => panic!("add render type spritesheet raw create program error: {}", err),
        };

        let pso = match factory.create_pipeline_from_program(
            &program,
            Primitive::TriangleList,
            rasterizer,
            pipe::new()
        ) {
            Ok(pso) => pso,
            Err(err) => panic!("add render type spritesheet raw create pipeline error: {}", err),
        };

        let sampler_info = SamplerInfo::new(
            FilterMethod::Scale,
            WrapMode::Mirror
        );

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertices, indices);
        let data = pipe::Data {
            vbuf: vbuf,
            spritesheet: (spritesheet, factory.create_sampler(sampler_info)),
            texture_data: factory.create_constant_buffer(1),
            projection_cb: factory.create_constant_buffer(1),
            out_color: self.out_color.clone(),
            out_depth: self.out_depth.clone(),
        };

        let id = self.bundles.len();
        let mut bundles = match Arc::get_mut(&mut self.bundles) {
            Some(bundles) => bundles,
            None => panic!("add render type spritesheet raw get mut bundles was none"),
        };
        bundles.push(Bundle::new(slice, pso, data));
        RenderId {
            id: id,
        }
    }

    fn render(&mut self, arg: &RunArg, mut encoder: Encoder<Resources, CommandBuffer>) {
        use specs::Join;

        let (draw, transform, mut camera, mut render_data) = arg.fetch(|w|
            (
                w.read::<RenderId>(),
                w.read::<Transform>(),
                w.write::<Camera>(),
                w.write::<RenderData>()
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
                projection_data = Some(ProjectionData {
                    model: t.get_model(),
                    view: view,
                    proj: proj,
                });
            }

            let mut texture_data = None;

            if rd.take_dirty() {
                texture_data = Some(TextureData {
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
            let b = &self.bundles[data.0];

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

    fn set_graphics_data(&mut self, out_color: RenderTargetView<Resources, ColorFormat>, out_depth: DepthStencilView<Resources, DepthFormat>) {
        self.out_color = out_color;
        self.out_depth = out_depth;

        for bundle in match Arc::get_mut(&mut self.bundles) {
            Some(bundle) => bundle,
            None => panic!("set graphics data get mut texture bundles was none"),
        }  {
            bundle.data.out_color = self.out_color.clone();
            bundle.data.out_depth = self.out_depth.clone();
        }
    }

    fn exit(&mut self, arg: &RunArg) {
        //use to save

        arg.fetch(|_| ());
    }

    fn process_event(&mut self, arg: &RunArg, event: RecvEvent) -> bool {
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

impl specs::System<::utils::Delta> for System {
    fn run(&mut self, arg: RunArg, _: Delta) {
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
