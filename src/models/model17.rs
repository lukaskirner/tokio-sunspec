#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model17 {}

impl Model for Model17 {
    const ID: u16 = 17;
    const LENGTH: u16 = 12;
}

pub const Nam: Point<Model17, String> = Point::new(0, 4, false);
pub const Rte: Point<Model17, u32> = Point::new(4, 1, false);
pub const Bits: Point<Model17, u16> = Point::new(6, 1, false);
pub const Pty: Point<Model17, u16> = Point::new(7, 1, false);
pub const Dup: Point<Model17, u16> = Point::new(8, 1, false);
pub const Flw: Point<Model17, u16> = Point::new(9, 1, false);
pub const Typ: Point<Model17, u16> = Point::new(10, 1, false);
pub const Pcol: Point<Model17, u16> = Point::new(11, 1, false);
