#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model13 {}

impl Model for Model13 {
    const ID: u16 = 13;
    const LENGTH: u16 = 174;
}

pub const Nam: Point<Model13, String> = Point::new(0, 4, true);
pub const CfgSt: Point<Model13, u16> = Point::new(4, 1, false);
pub const ChgSt: Point<Model13, u16> = Point::new(5, 1, false);
pub const Cap: Point<Model13, u16> = Point::new(6, 1, false);
pub const Cfg: Point<Model13, u16> = Point::new(7, 1, true);
pub const Ctl: Point<Model13, u16> = Point::new(8, 1, true);
pub const Addr: Point<Model13, String> = Point::new(9, 20, true);
pub const CIDR: Point<Model13, String> = Point::new(29, 20, true);
pub const Gw: Point<Model13, String> = Point::new(49, 20, true);
pub const DNS1: Point<Model13, String> = Point::new(69, 20, true);
pub const DNS2: Point<Model13, String> = Point::new(89, 20, true);
pub const NTP1: Point<Model13, String> = Point::new(109, 20, true);
pub const NTP2: Point<Model13, String> = Point::new(129, 20, true);
pub const DomNam: Point<Model13, String> = Point::new(149, 12, true);
pub const HostNam: Point<Model13, String> = Point::new(161, 12, true);
pub const Pad: Point<Model13, u16> = Point::new(173, 1, false);
