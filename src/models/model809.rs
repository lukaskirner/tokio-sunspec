#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model809 {}

impl Model for Model809 {
    const ID: u16 = 809;
    const LENGTH: u16 = 2;
}

pub const StackTBD: Point<Model809, u16> = Point::new(0, 1, false);
