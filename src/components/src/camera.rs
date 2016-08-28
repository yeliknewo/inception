use nalgebra::{self, Isometry3, ToHomogeneous, OrthographicMatrix3};

use specs::{self, VecStorage};

use utils::{GfxCoord, Coord};

use math::{OrthographicHelper, Point2};

#[derive(Debug)]
pub struct Component {
    eye: nalgebra::Point3<GfxCoord>,
    target: nalgebra::Point3<GfxCoord>,
    up: nalgebra::Vector3<GfxCoord>,
    proj: OrthographicMatrix3<GfxCoord>,
    aspect_ratio: GfxCoord,
    is_main: bool,
    dirty: bool,
    dirty_2: bool,
}

impl Component {
    pub fn new(
        eye: nalgebra::Point3<GfxCoord>,
        target: nalgebra::Point3<GfxCoord>,
        up: nalgebra::Vector3<GfxCoord>,
        proj: OrthographicMatrix3<GfxCoord>,
        aspect_ratio: GfxCoord,
        is_main: bool
    ) -> Component {
        Component {
            eye: eye,
            target: target,
            up: up,
            proj: proj,
            aspect_ratio: aspect_ratio,
            is_main: is_main,
            dirty: true,
            dirty_2: true,
        }
    }

    pub fn new_from_proj_args(
        eye: nalgebra::Point3<GfxCoord>,
        target: nalgebra::Point3<GfxCoord>,
        up: nalgebra::Vector3<GfxCoord>,
        aspect_ratio: GfxCoord,
        fov: GfxCoord,
        near: GfxCoord,
        far: GfxCoord,
        is_main: bool
    ) -> Component {
        Component::new(eye, target, up, OrthographicMatrix3::from_fov(aspect_ratio, fov, near, far), aspect_ratio, is_main)
    }

    pub fn new_from_ortho_helper(
        eye: nalgebra::Point3<GfxCoord>,
        target: nalgebra::Point3<GfxCoord>,
        up: nalgebra::Vector3<GfxCoord>,
        ortho_helper: &OrthographicHelper,
        is_main: bool
    ) -> Component {
        Component::new(eye, target, up, ortho_helper.build_matrix(), ortho_helper.get_aspect_ratio(), is_main)
    }

    pub fn set_offset(&mut self, offset: Point2) {
        self.set_eye(nalgebra::Point3::new(offset.get_x() as GfxCoord, offset.get_y() as GfxCoord, 2.0));
        self.set_target(nalgebra::Point3::new(offset.get_x() as GfxCoord, offset.get_y() as GfxCoord, 0.0));
        self.set_dirty();
    }

    fn set_eye(&mut self, eye: nalgebra::Point3<GfxCoord>) {
        self.eye = eye;
    }

    fn set_target(&mut self, target: nalgebra::Point3<GfxCoord>) {
        self.target = target;
    }

    pub fn set_proj(&mut self, ortho_helper: &OrthographicHelper) {
        self.proj = ortho_helper.build_matrix();
        self.aspect_ratio = ortho_helper.get_aspect_ratio();
        self.dirty = true;
    }

    pub fn get_offset(&self) -> Point2 {
        Point2::new(self.eye.x as Coord, self.eye.y as Coord)
    }

    pub fn get_view(&self) -> [[GfxCoord; 4]; 4] {
        *Isometry3::look_at_rh(&self.eye, &self.target, &self.up).to_homogeneous().as_ref()
    }

    pub fn get_proj(&self) -> [[GfxCoord; 4]; 4] {
        *self.proj.as_matrix().as_ref()
    }

    pub fn is_main(&self) -> bool {
        self.is_main
    }

    pub fn screen_to_world_point(&self, screen_point: Point2) -> Point2 {
        let view_depth = self.proj.zfar() - self.proj.znear();

        let world_point = Point2::new(
            (((screen_point.get_x() * 2.0) - 1.0) * view_depth as Coord) * 4.0 / 5.0 + self.get_offset().get_x(),
            (((1.0 - screen_point.get_y()) * 2.0 - 1.0) * view_depth as Coord / self.aspect_ratio as Coord) * 4.0 / 5.0 + self.get_offset().get_y()
        );

        world_point
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

impl specs::Component for Component {
    type Storage = VecStorage<Component>;
}
