use specs::{Entity};

use math::{Point3I};

#[derive(Debug)]
pub struct Link {
    slow: Point3I,
    fast: Option<Entity>,
}

impl Link {
    pub fn new(slow: Point3I) -> Link {
        Link {
            slow: slow,
            fast: None,
        }
    }

    pub fn get_mut_slow(&mut self) -> &mut Point3I {
        &mut self.slow
    }

    pub fn get_mut_fast(&mut self) -> &mut Option<Entity> {
        &mut self.fast
    }

    pub fn get_slow(&self) -> &Point3I {
        &self.slow
    }

    pub fn get_fast(&self) -> Option<&Entity> {
        self.fast.as_ref()
    }
}
