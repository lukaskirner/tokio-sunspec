#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model19 {}

impl Model for Model19 {
    const ID: u16 = 19;
    const LENGTH: u16 = 30;
}

pub const Nam: Point<Model19, String> = Point::new(0, 4, true);
pub const Rte: Point<Model19, u32> = Point::new(4, 1, true);
pub const Bits: Point<Model19, u16> = Point::new(6, 1, true);
pub const Pty: Point<Model19, u16> = Point::new(7, 1, true);
pub const Dup: Point<Model19, u16> = Point::new(8, 1, true);
pub const Flw: Point<Model19, u16> = Point::new(9, 1, true);
pub const Auth: Point<Model19, u16> = Point::new(10, 1, false);
pub const UsrNam: Point<Model19, String> = Point::new(11, 12, false);
pub const Pw: Point<Model19, String> = Point::new(23, 6, false);
pub const Pad: Point<Model19, u16> = Point::new(29, 1, false);
