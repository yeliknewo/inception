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
    fn run(&mut self, arg: RunArg, delta_time: Delta) {
        use specs::Join;

        let mut map = arg.fetch(|w|
            w.write_resource::<Map>()
        );
    }
}
