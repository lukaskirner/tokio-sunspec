#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model136 {}

impl Model for Model136 {
    const ID: u16 = 136;
    const LENGTH: u16 = 60;
}

pub const ActCrv: Point<Model136, u16> = Point::new(0, 1, false);
pub const ModEna: Point<Model136, u16> = Point::new(1, 1, false);
pub const WinTms: Point<Model136, u16> = Point::new(2, 1, false);
pub const RvrtTms: Point<Model136, u16> = Point::new(3, 1, false);
pub const RmpTms: Point<Model136, u16> = Point::new(4, 1, false);
pub const NCrv: Point<Model136, u16> = Point::new(5, 1, false);
pub const NPt: Point<Model136, u16> = Point::new(6, 1, false);
pub const Tms_SF: Point<Model136, u16> = Point::new(7, 1, false);
pub const Hz_SF: Point<Model136, u16> = Point::new(8, 1, false);
pub const Pad: Point<Model136, u16> = Point::new(9, 1, false);
