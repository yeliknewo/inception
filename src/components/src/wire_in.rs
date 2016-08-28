use specs::{self, VecStorage};

use ::non_components::link::Link;

use math::Point3I;

#[derive(Debug)]
pub struct Component {
    output: Link,
    me: Link,
    value: u8,
    dirty: bool,
}

impl Component {
    pub fn new(output: Link, me: Link) -> Component {
        Component {
            output: output,
            me: me,
            value: 0,
            dirty: true,
        }
    }

    pub fn new_from_points(output: Point3I, me: Point3I) -> Component {
        Component::new(Link::new(output), Link::new(me))
    }

    pub fn get_mut_me(&mut self) -> &mut Link {
        &mut self.me
    }

    pub fn get_mut_output(&mut self) -> &mut Link {
        &mut self.output
    }

    pub fn get_mut_value(&mut self) -> &mut u8 {
        &mut self.value
    }

    pub fn get_mut_dirty(&mut self) -> &mut bool {
        &mut self.dirty
    }

    pub fn get_me(&self) -> &Link {
        &self.me
    }

    pub fn get_output(&self) -> &Link {
        &self.output
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn get_dirty(&self) -> bool {
        self.dirty
    }
}

impl specs::Component for Component {
    type Storage = VecStorage<Component>;
}
