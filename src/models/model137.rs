#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model137 {}

impl Model for Model137 {
    const ID: u16 = 137;
    const LENGTH: u16 = 60;
}

pub const ActCrv: Point<Model137, u16> = Point::new(0, 1, false);
pub const ModEna: Point<Model137, u16> = Point::new(1, 1, false);
pub const WinTms: Point<Model137, u16> = Point::new(2, 1, false);
pub const RvrtTms: Point<Model137, u16> = Point::new(3, 1, false);
pub const RmpTms: Point<Model137, u16> = Point::new(4, 1, false);
pub const NCrv: Point<Model137, u16> = Point::new(5, 1, false);
pub const NPt: Point<Model137, u16> = Point::new(6, 1, false);
pub const Tms_SF: Point<Model137, u16> = Point::new(7, 1, false);
pub const V_SF: Point<Model137, u16> = Point::new(8, 1, false);
pub const Pad: Point<Model137, u16> = Point::new(9, 1, false);
