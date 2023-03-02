#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model805 {}

impl Model for Model805 {
    const ID: u16 = 805;
    const LENGTH: u16 = 46;
}

pub const StrIdx: Point<Model805, u16> = Point::new(0, 1, false);
pub const ModIdx: Point<Model805, u16> = Point::new(1, 1, false);
pub const NCell: Point<Model805, u16> = Point::new(2, 1, false);
pub const SoC: Point<Model805, u16> = Point::new(3, 1, false);
pub const DoD: Point<Model805, u16> = Point::new(4, 1, false);
pub const SoH: Point<Model805, u16> = Point::new(5, 1, false);
pub const NCyc: Point<Model805, u32> = Point::new(6, 1, false);
pub const V: Point<Model805, u16> = Point::new(8, 1, false);
pub const CellVMax: Point<Model805, u16> = Point::new(9, 1, false);
pub const CellVMaxCell: Point<Model805, u16> = Point::new(10, 1, false);
pub const CellVMin: Point<Model805, u16> = Point::new(11, 1, false);
pub const CellVMinCell: Point<Model805, u16> = Point::new(12, 1, false);
pub const CellVAvg: Point<Model805, u16> = Point::new(13, 1, false);
pub const CellTmpMax: Point<Model805, i16> = Point::new(14, 1, false);
pub const CellTmpMaxCell: Point<Model805, u16> = Point::new(15, 1, false);
pub const CellTmpMin: Point<Model805, i16> = Point::new(16, 1, false);
pub const CellTmpMinCell: Point<Model805, u16> = Point::new(17, 1, false);
pub const CellTmpAvg: Point<Model805, i16> = Point::new(18, 1, false);
pub const NCellBal: Point<Model805, u16> = Point::new(19, 1, false);
pub const SN: Point<Model805, String> = Point::new(20, 16, false);
pub const SoC_SF: Point<Model805, u16> = Point::new(36, 1, false);
pub const SoH_SF: Point<Model805, u16> = Point::new(37, 1, false);
pub const DoD_SF: Point<Model805, u16> = Point::new(38, 1, false);
pub const V_SF: Point<Model805, u16> = Point::new(39, 1, false);
pub const CellV_SF: Point<Model805, u16> = Point::new(40, 1, false);
pub const Tmp_SF: Point<Model805, u16> = Point::new(41, 1, false);
