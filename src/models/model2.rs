#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model2 {}

impl Model for Model2 {
    const ID: u16 = 2;
    const LENGTH: u16 = 14;
}

pub const AID: Point<Model2, u16> = Point::new(0, 1, false);
pub const N: Point<Model2, u16> = Point::new(1, 1, false);
pub const UN: Point<Model2, u16> = Point::new(2, 1, false);
pub const St: Point<Model2, u16> = Point::new(3, 1, false);
pub const StVnd: Point<Model2, u16> = Point::new(4, 1, false);
pub const Evt: Point<Model2, u32> = Point::new(5, 1, false);
pub const EvtVnd: Point<Model2, u32> = Point::new(7, 1, false);
pub const Ctl: Point<Model2, u16> = Point::new(9, 1, false);
pub const CtlVnd: Point<Model2, u32> = Point::new(10, 1, false);
pub const CtlVl: Point<Model2, u32> = Point::new(12, 1, false);
