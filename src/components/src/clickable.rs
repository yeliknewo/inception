use specs::{self, VecStorage};

use math::Rect;

#[derive(Debug)]
pub struct Component {
    clicked: bool,
    hitbox: Rect,
}

impl Component {
    pub fn new(hitbox: Rect) -> Component {
        Component {
            clicked: false,
            hitbox: hitbox,
        }
    }

    pub fn get_mut_clicked(&mut self) -> &mut bool {
        &mut self.clicked
    }

    pub fn get_mut_hitbox(&mut self) -> &mut Rect {
        &mut self.hitbox
    }

    pub fn get_clicked(&self) -> bool {
        self.clicked
    }

    pub fn get_hitbox(&self) -> &Rect {
        &self.hitbox
    }
}

impl specs::Component for Component {
    type Storage = VecStorage<Component>;
}
