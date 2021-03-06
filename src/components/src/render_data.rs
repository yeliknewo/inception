use specs::{self, VecStorage};

#[derive(Debug)]
pub struct Component {
    tint: [f32; 4],
    layer: u8,
    spritesheet_rect: [f32; 4],
    spritesheet_size: [f32; 2],
    mirror_x: bool,
    mirror_y: bool,
    dirty: bool,
    dirty_2: bool, // required because double buffering
}

impl specs::Component for Component {
    type Storage = VecStorage<Component>;
}

impl Component {
    pub fn new(layer: u8, tint: [f32; 4], spritesheet_rect: [f32; 4], spritesheet_size: [f32; 2]) -> Component {
        Component {
            tint: tint,
            layer: layer,
            spritesheet_rect: spritesheet_rect,
            spritesheet_size: spritesheet_size,
            mirror_x: false,
            mirror_y: false,
            dirty: true,
            dirty_2: true,
        }
    }

    pub fn set_layer(&mut self, layer: u8) {
        self.layer = layer;
        self.set_dirty();
    }

    pub fn set_mirror_x(&mut self, mirror: bool) {
        self.mirror_x = mirror;
        self.set_dirty();
    }

    pub fn set_mirror_y(&mut self, mirror: bool) {
        self.mirror_y = mirror;
        self.set_dirty();
    }

    pub fn set_spritesheet_rect(&mut self, spritesheet_rect: [f32; 4]) {
        self.spritesheet_rect = spritesheet_rect;
        self.set_dirty();
    }

    pub fn set_tint(&mut self, tint: [f32; 4]) {
        self.tint = tint;
        self.set_dirty();
    }

    pub fn get_layer(&self) -> u8 {
        self.layer
    }

    pub fn get_mirror_x(&self) -> bool {
        self.mirror_x
    }

    pub fn get_mirror_y(&self) -> bool {
        self.mirror_y
    }

    pub fn get_tint(&self) -> [f32; 4] {
        self.tint.clone()
    }

    pub fn get_spritesheet_rect(&self) -> [f32; 4] {
        self.spritesheet_rect.clone()
    }

    pub fn get_spritesheet_size(&self) -> [f32; 2] {
        self.spritesheet_size.clone()
    }

    fn set_dirty(&mut self) {
        self.dirty = true;
        self.dirty_2 = true;
    }

    pub fn take_dirty(&mut self) -> bool {
        self.dirty = false;
        if self.dirty {
            self.dirty_2 = false;
            return true;
        }
        return self.dirty_2;
    }
}
