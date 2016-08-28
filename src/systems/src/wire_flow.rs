use specs::{self, RunArg};

use comps::{Wire, WireIn, WireOut};

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

        let (mut wires, mut wires_in, mut wires_out) = arg.fetch(|w|
            (
                w.write::<Wire>(),
                w.write::<WireIn>(),
                w.write::<WireOut>()
            )
        );

        for mut wire_in in (&mut wires_in).iter() {
            // if let Some(fast) = wire_in.get_output().get_fast() {
            //     if let Some(mut wire) = wires.get_mut(fast) {
            //
            //     }
            // }
            //
            // if let Some(mut wire) = wires.get_mut(wire_in_output) {
            //     if wire.get_input() == wire_in.get_my_entity() {
            //
            //     }
            // }
        }
    }
}
