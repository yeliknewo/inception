use specs::{Entity};

use std::collections::{HashMap};

//*************************************************************************************************

use math::{Point3I};

//*************************************************************************************************

#[derive(Debug)]
pub struct Map {
    map: HashMap<Point3I, Entity>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            map: HashMap::new(),
        }
    }

    pub fn get_mut_map(&mut self) -> &mut HashMap<Point3I, Entity> {
        &mut self.map
    }

    pub fn get_map(&self) -> &HashMap<Point3I, Entity> {
        &self.map
    }
}
