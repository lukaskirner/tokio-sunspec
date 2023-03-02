#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model8 {}

impl Model for Model8 {
    const ID: u16 = 8;
    const LENGTH: u16 = 3;
}

pub const Fmt: Point<Model8, u16> = Point::new(0, 1, false);
pub const N: Point<Model8, u16> = Point::new(1, 1, false);
