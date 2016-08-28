use specs::VecStorage;

use math::Rect;

#[derive(Debug)]
pub struct Component {
    pub clicked: bool,
    pub hitbox: Rect,
}

impl ::specs::Component for Component {
    type Storage = VecStorage<Component>;
}

impl Component {
    pub fn new(hitbox: Rect) -> Component {
        Component {
            clicked: false,
            hitbox: hitbox,
        }
    }
}
