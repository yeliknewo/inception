use nalgebra::{Vector3, Isometry3, Translation, ToHomogeneous};

use specs::{self, VecStorage};

use utils::{GfxCoord, Coord};

use math::{Point2};

#[derive(Debug)]
pub struct Component {
    isometry: Isometry3<GfxCoord>,
    scale: Vector3<GfxCoord>,
    pos: Point2,
}

impl Component {
    pub fn new_identity() -> Component {
        Component::new(
            Isometry3::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0)),
            Vector3::new(1.0, 1.0, 1.0)
        )
    }

    pub fn new(isometry: Isometry3<GfxCoord>, scale: Vector3<GfxCoord>) -> Component {
        Component {
            isometry: isometry,
            scale: scale,
            pos: ::math::Point2::new(isometry.translation.x as Coord, isometry.translation.y as Coord),
        }
    }

    pub fn set_position(&mut self, pos: Point2) {
        self.isometry.translation.x = pos.get_x() as GfxCoord;
        self.isometry.translation.y = pos.get_y() as GfxCoord;
    }

    pub fn add_position(&mut self, pos_delta: Point2) {
        self.isometry.translation.x += pos_delta.get_x() as GfxCoord;
        self.isometry.translation.y += pos_delta.get_y() as GfxCoord;
    }

    pub fn get_model(&self) -> [[GfxCoord; 4]; 4] {
        let mut refer = *self.isometry.to_homogeneous().as_ref();
        refer[0][0] *= self.scale.x;
        refer[1][1] *= self.scale.y;
        refer[2][2] *= self.scale.z;
        refer
    }

    pub fn get_pos(&self) -> Point2 {
        Point2::new(self.isometry.translation.x as f64, self.isometry.translation.y as f64)
    }

    pub fn get_gui_offset(&self) -> Point2 {
        let translation = self.isometry.translation();
        Point2::new(-translation.x as f64, -translation.y as f64)
    }
}

impl specs::Component for Component {
    type Storage = VecStorage<Component>;
}
