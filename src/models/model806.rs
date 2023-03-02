#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model806 {}

impl Model for Model806 {
    const ID: u16 = 806;
    const LENGTH: u16 = 2;
}

pub const BatTBD: Point<Model806, u16> = Point::new(0, 1, false);
