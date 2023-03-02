#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model808 {}

impl Model for Model808 {
    const ID: u16 = 808;
    const LENGTH: u16 = 2;
}

pub const ModuleTBD: Point<Model808, u16> = Point::new(0, 1, false);
