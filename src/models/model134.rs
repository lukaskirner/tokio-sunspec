#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model134 {}

impl Model for Model134 {
    const ID: u16 = 134;
    const LENGTH: u16 = 68;
}

pub const ActCrv: Point<Model134, u16> = Point::new(0, 1, true);
pub const ModEna: Point<Model134, u16> = Point::new(1, 1, true);
pub const WinTms: Point<Model134, u16> = Point::new(2, 1, true);
pub const RvrtTms: Point<Model134, u16> = Point::new(3, 1, true);
pub const RmpTms: Point<Model134, u16> = Point::new(4, 1, true);
pub const NCrv: Point<Model134, u16> = Point::new(5, 1, false);
pub const NPt: Point<Model134, u16> = Point::new(6, 1, false);
pub const Hz_SF: Point<Model134, u16> = Point::new(7, 1, false);
pub const W_SF: Point<Model134, u16> = Point::new(8, 1, false);
pub const RmpIncDec_SF: Point<Model134, u16> = Point::new(9, 1, false);
