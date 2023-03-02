#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model122 {}

impl Model for Model122 {
    const ID: u16 = 122;
    const LENGTH: u16 = 44;
}

pub const PVConn: Point<Model122, u16> = Point::new(0, 1, false);
pub const StorConn: Point<Model122, u16> = Point::new(1, 1, false);
pub const ECPConn: Point<Model122, u16> = Point::new(2, 1, false);
pub const ActWh: Point<Model122, u64> = Point::new(3, 4, false);
pub const ActVAh: Point<Model122, u64> = Point::new(7, 4, false);
pub const ActVArhQ1: Point<Model122, u64> = Point::new(11, 4, false);
pub const ActVArhQ2: Point<Model122, u64> = Point::new(15, 4, false);
pub const ActVArhQ3: Point<Model122, u64> = Point::new(19, 4, false);
pub const ActVArhQ4: Point<Model122, u64> = Point::new(23, 4, false);
pub const VArAval: Point<Model122, i16> = Point::new(27, 1, false);
pub const VArAval_SF: Point<Model122, u16> = Point::new(28, 1, false);
pub const WAval: Point<Model122, u16> = Point::new(29, 1, false);
pub const WAval_SF: Point<Model122, u16> = Point::new(30, 1, false);
pub const StSetLimMsk: Point<Model122, u32> = Point::new(31, 2, false);
pub const StActCtl: Point<Model122, u32> = Point::new(33, 2, false);
pub const TmSrc: Point<Model122, String> = Point::new(35, 4, false);
pub const Tms: Point<Model122, u32> = Point::new(39, 2, false);
pub const RtSt: Point<Model122, u16> = Point::new(41, 1, false);
pub const Ris: Point<Model122, u16> = Point::new(42, 1, false);
pub const Ris_SF: Point<Model122, u16> = Point::new(43, 1, false);
