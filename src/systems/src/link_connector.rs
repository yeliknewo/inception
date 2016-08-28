use specs::{self, RunArg};

use comps::{Wire, WireIn, WireOut};
use comps::non_components::{Map};

use utils::Delta;

pub struct System {

}

impl System {
    pub fn new() -> System {
        System {

        }
    }
}

impl specs::System<Delta> for System {
    fn run(&mut self, arg: RunArg, _: Delta) {
        use specs::Join;

        let (mut wires, mut wires_in, mut wires_out, map) = arg.fetch(|w|
            (
                w.write::<Wire>(),
                w.write::<WireIn>(),
                w.write::<WireOut>(),
                w.read_resource::<Map>()
            )
        );

        for mut wire in (&mut wires).iter() {
            if wire.get_output().get_fast().is_none() {
                let location = wire.get_output().get_slow().clone();
                *wire.get_mut_output().get_mut_fast() = map.get_map().get(&location).map(|e| *e);
            }
            if wire.get_input().get_fast().is_none() {
                let location = wire.get_input().get_slow().clone();
                *wire.get_mut_input().get_mut_fast() = map.get_map().get(&location).map(|e| *e);
            }
            if wire.get_me().get_fast().is_none() {
                let location = wire.get_me().get_slow().clone();
                *wire.get_mut_me().get_mut_fast() = map.get_map().get(&location).map(|e| *e);
            }
        }

        for mut wire_in in (&mut wires_in).iter() {
            if wire_in.get_output().get_fast().is_none() {
                let location = wire_in.get_output().get_slow().clone();
                *wire_in.get_mut_output().get_mut_fast() = map.get_map().get(&location).map(|e| *e);
            }
            if wire_in.get_me().get_fast().is_none() {
                let location = wire_in.get_me().get_slow().clone();
                *wire_in.get_mut_me().get_mut_fast() = map.get_map().get(&location).map(|e| *e);
            }
        }

        for mut wire_out in (&mut wires_out).iter() {
            if wire_out.get_input().get_fast().is_none() {
                let location = wire_out.get_input().get_slow().clone();
                *wire_out.get_mut_input().get_mut_fast() = map.get_map().get(&location).map(|e| *e);
            }
            if wire_out.get_me().get_fast().is_none() {
                let location = wire_out.get_me().get_slow().clone();
                *wire_out.get_mut_me().get_mut_fast() = map.get_map().get(&location).map(|e| *e);
            }
        }
    }
}
