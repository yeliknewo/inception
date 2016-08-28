use specs::{self, VecStorage};

//*************************************************************************************************

use math::{Point3I};

use ::non_components::link::Link;

//*************************************************************************************************

#[derive(Debug)]
pub struct Component {
    input: Link,
    me: Link,
    value: u8,
    dirty: bool,
}

impl Component {
    pub fn new(input: Link, me: Link) -> Component {
        Component {
            input: input,
            me: me,
            value: 0,
            dirty: true,
        }
    }

    pub fn new_from_points(input: Point3I, me: Point3I) -> Component {
        Component::new(Link::new(input), Link::new(me))
    }

    pub fn get_mut_input(&mut self) -> &mut Link {
        &mut self.input
    }

    pub fn get_mut_me(&mut self) -> &mut Link {
        &mut self.me
    }

    pub fn get_mut_value(&mut self) -> &mut u8 {
        &mut self.value
    }

    pub fn get_mut_dirty(&mut self) -> &mut bool {
        &mut self.dirty
    }

    pub fn get_input(&self) -> &Link {
        &self.input
    }

    pub fn get_me(&self) -> &Link {
        &self.me
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
