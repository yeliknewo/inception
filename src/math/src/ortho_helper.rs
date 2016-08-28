use nalgebra::OrthographicMatrix3;

use utils::GfxCoord;

#[derive(Clone, Debug)]
pub struct OrthographicHelper {
    aspect_ratio: GfxCoord,
    fov: GfxCoord,
    znear: GfxCoord,
    zfar: GfxCoord,
}

impl OrthographicHelper {
    pub fn new(
        aspect_ratio: GfxCoord,
        fov: GfxCoord,
        znear: GfxCoord,
        zfar: GfxCoord
    ) -> OrthographicHelper {
        OrthographicHelper {
            aspect_ratio: aspect_ratio,
            fov: fov,
            znear: znear,
            zfar: zfar,
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: GfxCoord) {
        self.aspect_ratio = aspect_ratio;
    }

    pub fn get_aspect_ratio(&self) -> GfxCoord {
        self.aspect_ratio
    }

    pub fn get_fov(&self) -> GfxCoord {
        self.fov
    }

    pub fn get_znear(&self) -> GfxCoord {
        self.znear
    }

    pub fn get_zfar(&self) -> GfxCoord {
        self.zfar
    }

    pub fn get_view_depth(&self) -> GfxCoord {
        self.get_zfar() - self.get_znear()
    }

    pub fn build_matrix(&self) -> OrthographicMatrix3<GfxCoord> {
        OrthographicMatrix3::from_fov(self.get_aspect_ratio(), self.get_fov(), self.get_znear(), self.get_zfar())
    }
}
