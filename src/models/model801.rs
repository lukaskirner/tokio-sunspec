#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model801 {}

impl Model for Model801 {
    const ID: u16 = 801;
    const LENGTH: u16 = 1;
}

pub const DEPRECATED: Point<Model801, u16> = Point::new(0, 1, false);
