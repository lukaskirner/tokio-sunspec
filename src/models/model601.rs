#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model601 {}

impl Model for Model601 {
    const ID: u16 = 601;
    const LENGTH: u16 = 48;
}

pub const Nam: Point<Model601, String> = Point::new(0, 8, false);
pub const Typ: Point<Model601, u16> = Point::new(8, 1, false);
pub const DtLoc: Point<Model601, String> = Point::new(9, 5, false);
pub const TmLoc: Point<Model601, String> = Point::new(14, 3, false);
pub const Day: Point<Model601, u16> = Point::new(17, 1, false);
pub const GlblElCtl: Point<Model601, i32> = Point::new(18, 1, true);
pub const GlblAzCtl: Point<Model601, i32> = Point::new(20, 1, true);
pub const GlblCtl: Point<Model601, u16> = Point::new(22, 1, true);
pub const GlblAlm: Point<Model601, u16> = Point::new(23, 1, false);
pub const Dgr_SF: Point<Model601, u16> = Point::new(24, 1, false);
pub const N: Point<Model601, u16> = Point::new(25, 1, false);
