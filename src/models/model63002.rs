#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model63002 {}

impl Model for Model63002 {
    const ID: u16 = 63002;
    const LENGTH: u16 = 4;
}

pub const sunssf_1: Point<Model63002, u16> = Point::new(0, 1, false);
pub const int16_1: Point<Model63002, i16> = Point::new(1, 1, false);
pub const int16_2: Point<Model63002, i16> = Point::new(2, 1, false);
pub const sunssf_2: Point<Model63002, u16> = Point::new(3, 1, false);
