#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model138 {}

impl Model for Model138 {
    const ID: u16 = 138;
    const LENGTH: u16 = 60;
}

pub const ActCrv: Point<Model138, u16> = Point::new(0, 1, false);
pub const ModEna: Point<Model138, u16> = Point::new(1, 1, false);
pub const WinTms: Point<Model138, u16> = Point::new(2, 1, false);
pub const RvrtTms: Point<Model138, u16> = Point::new(3, 1, false);
pub const RmpTms: Point<Model138, u16> = Point::new(4, 1, false);
pub const NCrv: Point<Model138, u16> = Point::new(5, 1, false);
pub const NPt: Point<Model138, u16> = Point::new(6, 1, false);
pub const Tms_SF: Point<Model138, u16> = Point::new(7, 1, false);
pub const V_SF: Point<Model138, u16> = Point::new(8, 1, false);
pub const Pad: Point<Model138, u16> = Point::new(9, 1, false);
