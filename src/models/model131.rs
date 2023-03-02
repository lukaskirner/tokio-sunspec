#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model131 {}

impl Model for Model131 {
    const ID: u16 = 131;
    const LENGTH: u16 = 64;
}

pub const ActCrv: Point<Model131, u16> = Point::new(0, 1, false);
pub const ModEna: Point<Model131, u16> = Point::new(1, 1, false);
pub const WinTms: Point<Model131, u16> = Point::new(2, 1, false);
pub const RvrtTms: Point<Model131, u16> = Point::new(3, 1, false);
pub const RmpTms: Point<Model131, u16> = Point::new(4, 1, false);
pub const NCrv: Point<Model131, u16> = Point::new(5, 1, false);
pub const NPt: Point<Model131, u16> = Point::new(6, 1, false);
pub const W_SF: Point<Model131, u16> = Point::new(7, 1, false);
pub const PF_SF: Point<Model131, u16> = Point::new(8, 1, false);
pub const RmpIncDec_SF: Point<Model131, u16> = Point::new(9, 1, false);
