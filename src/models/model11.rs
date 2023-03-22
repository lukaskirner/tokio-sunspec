#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model11 {}

impl Model for Model11 {
    const ID: u16 = 11;
    const LENGTH: u16 = 13;
}

pub const Spd: Point<Model11, u16> = Point::new(0, 1, false);
pub const CfgSt: Point<Model11, u16> = Point::new(1, 1, false);
pub const St: Point<Model11, u16> = Point::new(2, 1, false);
pub const MAC: Point<Model11, String> = Point::new(3, 1, false);
pub const Nam: Point<Model11, String> = Point::new(7, 4, true);
pub const Ctl: Point<Model11, u16> = Point::new(11, 1, true);
pub const FrcSpd: Point<Model11, u16> = Point::new(12, 1, true);
