#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model501 {}

impl Model for Model501 {
    const ID: u16 = 501;
    const LENGTH: u16 = 31;
}

pub const Stat: Point<Model501, u16> = Point::new(0, 1, false);
pub const StatVend: Point<Model501, u16> = Point::new(1, 1, false);
pub const Evt: Point<Model501, u32> = Point::new(2, 1, false);
pub const EvtVend: Point<Model501, u32> = Point::new(4, 1, false);
pub const Ctl: Point<Model501, u16> = Point::new(6, 1, false);
pub const CtlVend: Point<Model501, u32> = Point::new(7, 1, false);
pub const CtlVal: Point<Model501, i32> = Point::new(9, 1, false);
pub const Tms: Point<Model501, u32> = Point::new(11, 1, false);
pub const OutA: Point<Model501, f32> = Point::new(13, 1, false);
pub const OutV: Point<Model501, f32> = Point::new(15, 1, false);
pub const OutWh: Point<Model501, f32> = Point::new(17, 1, false);
pub const OutW: Point<Model501, f32> = Point::new(19, 1, false);
pub const Tmp: Point<Model501, f32> = Point::new(21, 1, false);
pub const InA: Point<Model501, f32> = Point::new(23, 1, false);
pub const InV: Point<Model501, f32> = Point::new(25, 1, false);
pub const InWh: Point<Model501, f32> = Point::new(27, 1, false);
pub const InW: Point<Model501, f32> = Point::new(29, 1, false);
