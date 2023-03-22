#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model133 {}

impl Model for Model133 {
    const ID: u16 = 133;
    const LENGTH: u16 = 66;
}

pub const ActSchd: Point<Model133, u32> = Point::new(0, 2, true);
pub const ModEna: Point<Model133, u16> = Point::new(2, 1, true);
pub const NSchd: Point<Model133, u16> = Point::new(3, 1, false);
pub const NPts: Point<Model133, u16> = Point::new(4, 1, false);
pub const Pad: Point<Model133, u16> = Point::new(5, 1, false);
