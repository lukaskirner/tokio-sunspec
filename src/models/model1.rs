#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model1 {}

impl Model for Model1 {
    const ID: u16 = 1;
    const LENGTH: u16 = 66;
}

pub const Mn: Point<Model1, String> = Point::new(0, 16, false);
pub const Md: Point<Model1, String> = Point::new(16, 16, false);
pub const Opt: Point<Model1, String> = Point::new(32, 8, false);
pub const Vr: Point<Model1, String> = Point::new(40, 8, false);
pub const SN: Point<Model1, String> = Point::new(48, 16, false);
pub const DA: Point<Model1, u16> = Point::new(64, 1, true);
pub const Pad: Point<Model1, u16> = Point::new(65, 1, false);
