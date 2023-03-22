#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model12 {}

impl Model for Model12 {
    const ID: u16 = 12;
    const LENGTH: u16 = 98;
}

pub const Nam: Point<Model12, String> = Point::new(0, 4, true);
pub const CfgSt: Point<Model12, u16> = Point::new(4, 1, false);
pub const ChgSt: Point<Model12, u16> = Point::new(5, 1, false);
pub const Cap: Point<Model12, u16> = Point::new(6, 1, false);
pub const Cfg: Point<Model12, u16> = Point::new(7, 1, true);
pub const Ctl: Point<Model12, u16> = Point::new(8, 1, true);
pub const Addr: Point<Model12, String> = Point::new(9, 8, true);
pub const Msk: Point<Model12, String> = Point::new(17, 8, true);
pub const Gw: Point<Model12, String> = Point::new(25, 8, true);
pub const DNS1: Point<Model12, String> = Point::new(33, 8, true);
pub const DNS2: Point<Model12, String> = Point::new(41, 8, true);
pub const NTP1: Point<Model12, String> = Point::new(49, 12, true);
pub const NTP2: Point<Model12, String> = Point::new(61, 12, true);
pub const DomNam: Point<Model12, String> = Point::new(73, 12, true);
pub const HostNam: Point<Model12, String> = Point::new(85, 12, true);
pub const Pad: Point<Model12, u16> = Point::new(97, 1, false);
