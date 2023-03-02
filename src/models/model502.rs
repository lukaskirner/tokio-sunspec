#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model502 {}

impl Model for Model502 {
    const ID: u16 = 502;
    const LENGTH: u16 = 28;
}

pub const A_SF: Point<Model502, u16> = Point::new(0, 1, false);
pub const V_SF: Point<Model502, u16> = Point::new(1, 1, false);
pub const W_SF: Point<Model502, u16> = Point::new(2, 1, false);
pub const Wh_SF: Point<Model502, u16> = Point::new(3, 1, false);
pub const Stat: Point<Model502, u16> = Point::new(4, 1, false);
pub const StatVend: Point<Model502, u16> = Point::new(5, 1, false);
pub const Evt: Point<Model502, u32> = Point::new(6, 1, false);
pub const EvtVend: Point<Model502, u32> = Point::new(8, 1, false);
pub const Ctl: Point<Model502, u16> = Point::new(10, 1, false);
pub const CtlVend: Point<Model502, u32> = Point::new(11, 1, false);
pub const CtlVal: Point<Model502, i32> = Point::new(13, 1, false);
pub const Tms: Point<Model502, u32> = Point::new(15, 1, false);
pub const OutA: Point<Model502, i16> = Point::new(17, 1, false);
pub const OutV: Point<Model502, i16> = Point::new(18, 1, false);
pub const OutWh: Point<Model502, u32> = Point::new(19, 1, false);
pub const OutPw: Point<Model502, i16> = Point::new(21, 1, false);
pub const Tmp: Point<Model502, i16> = Point::new(22, 1, false);
pub const InA: Point<Model502, i16> = Point::new(23, 1, false);
pub const InV: Point<Model502, i16> = Point::new(24, 1, false);
pub const InWh: Point<Model502, u32> = Point::new(25, 1, false);
pub const InW: Point<Model502, i16> = Point::new(27, 1, false);
