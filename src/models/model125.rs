#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model125 {}

impl Model for Model125 {
    const ID: u16 = 125;
    const LENGTH: u16 = 8;
}

pub const ModEna: Point<Model125, u16> = Point::new(0, 1, true);
pub const SigType: Point<Model125, u16> = Point::new(1, 1, true);
pub const Sig: Point<Model125, i16> = Point::new(2, 1, true);
pub const WinTms: Point<Model125, u16> = Point::new(3, 1, true);
pub const RvtTms: Point<Model125, u16> = Point::new(4, 1, true);
pub const RmpTms: Point<Model125, u16> = Point::new(5, 1, true);
pub const Sig_SF: Point<Model125, u16> = Point::new(6, 1, false);
pub const Pad: Point<Model125, u16> = Point::new(7, 1, false);
