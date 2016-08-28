extern crate nalgebra;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate utils;

use std::ops::{Add, Sub, Mul, Div, SubAssign, MulAssign};
use std::convert::{Into};

use utils::{Coord, CoordI};

pub mod ortho_helper;

pub use self::ortho_helper::OrthographicHelper;

#[derive(Debug, Clone)]
pub struct Rect {
    corners: LineSeg,
}

impl Rect {
    pub fn new_from_coords(x0: Coord, y0: Coord, x1: Coord, y1: Coord) -> Rect {
        Rect::new_from_points(Point2::new(x0, y0), Point2::new(x1, y1))
    }

    pub fn new_from_points(a: Point2, b: Point2) -> Rect {
        Rect::new(LineSeg::new(a, b))
    }

    pub fn new(corners: LineSeg) -> Rect {
        assert!(corners.get_a().get_x() < corners.get_b().get_x());
        assert!(corners.get_a().get_y() < corners.get_b().get_y());
        Rect {
            corners: corners,
        }
    }

    pub fn get_corners(&self) -> LineSeg {
        self.corners.clone()
    }

    pub fn get_bot_left(&self) -> Point2 {
        self.corners.get_a()
    }

    pub fn get_top_right(&self) -> Point2 {
        self.corners.get_b()
    }

    pub fn check_collide_point(&self, point: Point2) -> bool {
        trace!("Rect Points: ({},{}) ({},{})", self.get_bot_left().get_x(), self.get_bot_left().get_y(), self.get_top_right().get_x(), self.get_top_right().get_y());
        self.get_bot_left().get_x() <= point.get_x() &&
        self.get_bot_left().get_y() <= point.get_y() &&
        self.get_top_right().get_x() >= point.get_x() &&
        self.get_top_right().get_y() >= point.get_y()
    }
}

#[derive(Debug, Clone)]
pub struct LineSeg {
    a: Point2,
    b: Point2,
}

impl LineSeg {
    pub fn new_from_coords(x0: Coord, y0: Coord, x1: Coord, y1: Coord) -> LineSeg {
        LineSeg::new(Point2::new(x0, y0), Point2::new(x1, y1))
    }

    pub fn new(a: Point2, b: Point2) -> LineSeg {
        LineSeg {
            a: a,
            b: b,
        }
    }

    pub fn get_a(&self) -> Point2 {
        self.a.clone()
    }

    pub fn get_b(&self) -> Point2 {
        self.b.clone()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Point2I {
    x: CoordI,
    y: CoordI,
}

impl Point2I {
    pub fn new(x: CoordI, y: CoordI) -> Point2I {
        Point2I {
            x: x,
            y: y,
        }
    }

    pub fn get_x(&self) -> CoordI {
        self.x
    }

    pub fn get_y(&self) -> CoordI {
        self.y
    }

    pub fn get_mut_x(&mut self) -> &mut CoordI {
        &mut self.x
    }

    pub fn get_mut_y(&mut self) -> &mut CoordI {
        &mut self.y
    }
}

impl Add<Point2I> for Point2I {
    type Output = Point2I;

    fn add(self, other: Point2I) -> Point2I {
        Point2I::new(self.get_x() + other.get_x(), self.get_y() + other.get_y())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Point2 {
    x: Coord,
    y: Coord,
}

impl Point2 {
    pub fn new(x: Coord, y: Coord) -> Point2 {
        Point2 {
            x: x,
            y: y,
        }
    }

    pub fn zero() -> Point2 {
        Point2::new(0.0, 0.0)
    }

    pub fn get_x(&self) -> Coord {
        self.x
    }

    pub fn get_y(&self) -> Coord {
        self.y
    }

    pub fn get_mut_x(&mut self) -> &mut Coord {
        &mut self.x
    }

    pub fn get_mut_y(&mut self) -> &mut Coord {
        &mut self.y
    }

    pub fn normalized(&self) -> Point2 {
        self.clone() / self.length()
    }

    pub fn length(&self) -> Coord {
        (self.get_x().powi(2) + self.get_y().powi(2)).sqrt()
    }

    pub fn is_zero(&self) -> bool {
        self.get_x() == 0.0 && self.get_y() == 0.0
    }

    pub fn is_normal(&self) -> bool {
        self.get_x().is_normal() && self.get_y().is_normal()
    }

    pub fn is_finite(&self) -> bool {
        self.get_x().is_finite() && self.get_y().is_finite()
    }

    pub fn abs(&self) -> Point2 {
        Point2::new(self.get_x().abs(), self.get_y().abs())
    }
}

impl Add<Point2> for Point2 {
    type Output = Point2;

    fn add(self, other: Point2) -> Point2 {
        Point2::new(self.get_x() + other.get_x(), self.get_y() + other.get_y())
    }
}

impl Sub<Point2> for Point2 {
    type Output = Point2;

    fn sub(self, other: Point2) -> Point2 {
        Point2::new(self.get_x() - other.get_x(), self.get_y() - other.get_y())
    }
}

impl Mul<Point2> for Point2 {
    type Output = Point2;

    fn mul(self, other: Point2) -> Point2 {
        Point2::new(self.get_x() * other.get_x(), self.get_y() * other.get_y())
    }
}

impl Mul<Coord> for Point2 {
    type Output = Point2;

    fn mul(self, other: Coord) -> Point2 {
        Point2::new(self.get_x() * other, self.get_y() * other)
    }
}

impl Div<Coord> for Point2 {
    type Output = Point2;

    fn div(self, other: Coord) -> Point2 {
        Point2::new(self.get_x() / other, self.get_y() / other)
    }
}

impl SubAssign<Point2> for Point2 {
    fn sub_assign(&mut self, other: Point2) {
        self.x -= other.get_x();
        self.y -= other.get_y();
    }
}

impl MulAssign<Point2> for Point2 {
    fn mul_assign(&mut self, other: Point2) {
        self.x *= other.get_x();
        self.y *= other.get_y();
    }
}

impl MulAssign<Coord> for Point2 {
    fn mul_assign(&mut self, other: Coord) {
        self.x *= other;
        self.y *= other;
    }
}

impl Into<Point2I> for Point2 {
    fn into(self) -> Point2I {
        Point2I::new(self.get_x().round() as CoordI, self.get_y().round() as CoordI)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Point3I {
    x: CoordI,
    y: CoordI,
    z: CoordI,
}

impl Point3I {
    pub fn new(x: CoordI, y: CoordI, z: CoordI) -> Point3I {
        Point3I {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn zero() -> Point3I {
        Point3I::new(0, 0, 0)
    }

    pub fn get_mut_x(&mut self) -> &mut CoordI {
        &mut self.x
    }

    pub fn get_mut_y(&mut self) -> &mut CoordI {
        &mut self.y
    }

    pub fn get_mut_z(&mut self) -> &mut CoordI {
        &mut self.z
    }

    pub fn get_x(&self) -> CoordI {
        self.x
    }

    pub fn get_y(&self) -> CoordI {
        self.y
    }

    pub fn get_z(&self) -> CoordI {
        self.z
    }
}
