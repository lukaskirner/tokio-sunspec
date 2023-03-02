#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model15 {}

impl Model for Model15 {
    const ID: u16 = 15;
    const LENGTH: u16 = 24;
}

pub const Clr: Point<Model15, u16> = Point::new(0, 1, false);
pub const InCnt: Point<Model15, u32> = Point::new(1, 1, false);
pub const InUcCnt: Point<Model15, u32> = Point::new(3, 1, false);
pub const InNUcCnt: Point<Model15, u32> = Point::new(5, 1, false);
pub const InDscCnt: Point<Model15, u32> = Point::new(7, 1, false);
pub const InErrCnt: Point<Model15, u32> = Point::new(9, 1, false);
pub const InUnkCnt: Point<Model15, u32> = Point::new(11, 1, false);
pub const OutCnt: Point<Model15, u32> = Point::new(13, 1, false);
pub const OutUcCnt: Point<Model15, u32> = Point::new(15, 1, false);
pub const OutNUcCnt: Point<Model15, u32> = Point::new(17, 1, false);
pub const OutDscCnt: Point<Model15, u32> = Point::new(19, 1, false);
pub const OutErrCnt: Point<Model15, u32> = Point::new(21, 1, false);
pub const Pad: Point<Model15, u16> = Point::new(23, 1, false);
