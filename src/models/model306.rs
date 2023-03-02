#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model306 {}

impl Model for Model306 {
    const ID: u16 = 306;
    const LENGTH: u16 = 4;
}

pub const GHI: Point<Model306, u16> = Point::new(0, 1, false);
pub const A: Point<Model306, u16> = Point::new(1, 1, false);
pub const V: Point<Model306, u16> = Point::new(2, 1, false);
pub const Tmp: Point<Model306, u16> = Point::new(3, 1, false);
