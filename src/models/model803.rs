#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model803 {}

impl Model for Model803 {
    const ID: u16 = 803;
    const LENGTH: u16 = 58;
}

pub const NStr: Point<Model803, u16> = Point::new(0, 1, false);
pub const NStrCon: Point<Model803, u16> = Point::new(1, 1, false);
pub const ModTmpMax: Point<Model803, i16> = Point::new(2, 1, false);
pub const ModTmpMaxStr: Point<Model803, u16> = Point::new(3, 1, false);
pub const ModTmpMaxMod: Point<Model803, u16> = Point::new(4, 1, false);
pub const ModTmpMin: Point<Model803, i16> = Point::new(5, 1, false);
pub const ModTmpMinStr: Point<Model803, u16> = Point::new(6, 1, false);
pub const ModTmpMinMod: Point<Model803, u16> = Point::new(7, 1, false);
pub const ModTmpAvg: Point<Model803, i16> = Point::new(8, 1, false);
pub const StrVMax: Point<Model803, u16> = Point::new(9, 1, false);
pub const StrVMaxStr: Point<Model803, u16> = Point::new(10, 1, false);
pub const StrVMin: Point<Model803, u16> = Point::new(11, 1, false);
pub const StrVMinStr: Point<Model803, u16> = Point::new(12, 1, false);
pub const StrVAvg: Point<Model803, u16> = Point::new(13, 1, false);
pub const StrAMax: Point<Model803, i16> = Point::new(14, 1, false);
pub const StrAMaxStr: Point<Model803, u16> = Point::new(15, 1, false);
pub const StrAMin: Point<Model803, i16> = Point::new(16, 1, false);
pub const StrAMinStr: Point<Model803, u16> = Point::new(17, 1, false);
pub const StrAAvg: Point<Model803, i16> = Point::new(18, 1, false);
pub const NCellBal: Point<Model803, u16> = Point::new(19, 1, false);
pub const CellV_SF: Point<Model803, u16> = Point::new(20, 1, false);
pub const ModTmp_SF: Point<Model803, u16> = Point::new(21, 1, false);
pub const A_SF: Point<Model803, u16> = Point::new(22, 1, false);
pub const SoH_SF: Point<Model803, u16> = Point::new(23, 1, false);
pub const SoC_SF: Point<Model803, u16> = Point::new(24, 1, false);
pub const V_SF: Point<Model803, u16> = Point::new(25, 1, false);
