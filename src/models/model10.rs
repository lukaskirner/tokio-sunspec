#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model10 {}

impl Model for Model10 {
    const ID: u16 = 10;
    const LENGTH: u16 = 4;
}

pub const St: Point<Model10, u16> = Point::new(0, 1, false);
pub const Ctl: Point<Model10, u16> = Point::new(1, 1, true);
pub const Typ: Point<Model10, u16> = Point::new(2, 1, false);
pub const Pad: Point<Model10, u16> = Point::new(3, 1, false);
