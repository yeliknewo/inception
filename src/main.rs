extern crate nalgebra;

extern crate systems as sys;
extern crate graphics;
extern crate core;
extern crate art;
extern crate components as comps;
extern crate utils;

fn main() {
    core::start(|planner, renderer, factory| {
        let square_render = ::art::make_square_render(renderer, factory);

        for y in -10..10i32 {
            for x in -10..10i32 {
                planner.mut_world().create_now()
                    .with(square_render)
                    .with(comps::Transform::new(
                        nalgebra::Isometry3::new(
                            nalgebra::Vector3::new(x as f32, y as f32, 0.0),
                            nalgebra::Vector3::new(0.0, 0.0, 0.0),
                        ),
                        nalgebra::Vector3::new(1.0, 1.0, 1.0)
                    ))
                    .with(comps::RenderData::new_texture([
                        (x + 10) as f32 / 20.0,
                        (y + 10) as f32 / 20.0,
                        ((x + 10) as f32 / 20.0 + (y + 10) as f32 / 20.0) / 2.0,
                        1.0
                    ]))
                    .build();
            }
        }
    });
}
