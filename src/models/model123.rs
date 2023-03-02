#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model123 {}

impl Model for Model123 {
    const ID: u16 = 123;
    const LENGTH: u16 = 24;
}

pub const Conn_WinTms: Point<Model123, u16> = Point::new(0, 1, false);
pub const Conn_RvrtTms: Point<Model123, u16> = Point::new(1, 1, false);
pub const Conn: Point<Model123, u16> = Point::new(2, 1, false);
pub const WMaxLimPct: Point<Model123, u16> = Point::new(3, 1, false);
pub const WMaxLimPct_WinTms: Point<Model123, u16> = Point::new(4, 1, false);
pub const WMaxLimPct_RvrtTms: Point<Model123, u16> = Point::new(5, 1, false);
pub const WMaxLimPct_RmpTms: Point<Model123, u16> = Point::new(6, 1, false);
pub const WMaxLim_Ena: Point<Model123, u16> = Point::new(7, 1, false);
pub const OutPFSet: Point<Model123, i16> = Point::new(8, 1, false);
pub const OutPFSet_WinTms: Point<Model123, u16> = Point::new(9, 1, false);
pub const OutPFSet_RvrtTms: Point<Model123, u16> = Point::new(10, 1, false);
pub const OutPFSet_RmpTms: Point<Model123, u16> = Point::new(11, 1, false);
pub const OutPFSet_Ena: Point<Model123, u16> = Point::new(12, 1, false);
pub const VArWMaxPct: Point<Model123, i16> = Point::new(13, 1, false);
pub const VArMaxPct: Point<Model123, i16> = Point::new(14, 1, false);
pub const VArAvalPct: Point<Model123, i16> = Point::new(15, 1, false);
pub const VArPct_WinTms: Point<Model123, u16> = Point::new(16, 1, false);
pub const VArPct_RvrtTms: Point<Model123, u16> = Point::new(17, 1, false);
pub const VArPct_RmpTms: Point<Model123, u16> = Point::new(18, 1, false);
pub const VArPct_Mod: Point<Model123, u16> = Point::new(19, 1, false);
pub const VArPct_Ena: Point<Model123, u16> = Point::new(20, 1, false);
pub const WMaxLimPct_SF: Point<Model123, u16> = Point::new(21, 1, false);
pub const OutPFSet_SF: Point<Model123, u16> = Point::new(22, 1, false);
pub const VArPct_SF: Point<Model123, u16> = Point::new(23, 1, false);
