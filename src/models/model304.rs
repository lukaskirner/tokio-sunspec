#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model304 {}

impl Model for Model304 {
    const ID: u16 = 304;
    const LENGTH: u16 = 6;
}

pub const Inclx: Point<Model304, i32> = Point::new(0, 1, false);
pub const Incly: Point<Model304, i32> = Point::new(2, 1, false);
pub const Inclz: Point<Model304, i32> = Point::new(4, 1, false);
