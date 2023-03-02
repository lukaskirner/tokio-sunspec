#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model18 {}

impl Model for Model18 {
    const ID: u16 = 18;
    const LENGTH: u16 = 22;
}

pub const Nam: Point<Model18, String> = Point::new(0, 4, false);
pub const IMEI: Point<Model18, u32> = Point::new(4, 1, false);
pub const APN: Point<Model18, String> = Point::new(6, 4, false);
pub const Num: Point<Model18, String> = Point::new(10, 6, false);
pub const Pin: Point<Model18, String> = Point::new(16, 6, false);
