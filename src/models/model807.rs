#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model807 {}

impl Model for Model807 {
    const ID: u16 = 807;
    const LENGTH: u16 = 58;
}

pub const Idx: Point<Model807, u16> = Point::new(0, 1, false);
pub const NMod: Point<Model807, u16> = Point::new(1, 1, false);
pub const NModCon: Point<Model807, u16> = Point::new(2, 1, false);
pub const ModVMax: Point<Model807, u16> = Point::new(3, 1, false);
pub const ModVMaxMod: Point<Model807, u16> = Point::new(4, 1, false);
pub const ModVMin: Point<Model807, u16> = Point::new(5, 1, false);
pub const ModVMinMod: Point<Model807, u16> = Point::new(6, 1, false);
pub const ModVAvg: Point<Model807, u16> = Point::new(7, 1, false);
pub const CellVMax: Point<Model807, u16> = Point::new(8, 1, false);
pub const CellVMaxMod: Point<Model807, u16> = Point::new(9, 1, false);
pub const CellVMaxStk: Point<Model807, u16> = Point::new(10, 1, false);
pub const CellVMin: Point<Model807, u16> = Point::new(11, 1, false);
pub const CellVMinMod: Point<Model807, u16> = Point::new(12, 1, false);
pub const CellVMinStk: Point<Model807, u16> = Point::new(13, 1, false);
pub const CellVAvg: Point<Model807, u16> = Point::new(14, 1, false);
pub const TmpMax: Point<Model807, i16> = Point::new(15, 1, false);
pub const TmpMaxMod: Point<Model807, u16> = Point::new(16, 1, false);
pub const TmpMin: Point<Model807, i16> = Point::new(17, 1, false);
pub const TmpMinMod: Point<Model807, u16> = Point::new(18, 1, false);
pub const TmpAvg: Point<Model807, i16> = Point::new(19, 1, false);
pub const Evt1: Point<Model807, u32> = Point::new(20, 1, false);
pub const Evt2: Point<Model807, u32> = Point::new(22, 1, false);
pub const EvtVnd1: Point<Model807, u32> = Point::new(24, 1, false);
pub const EvtVnd2: Point<Model807, u32> = Point::new(26, 1, false);
pub const ModV_SF: Point<Model807, u16> = Point::new(28, 1, false);
pub const CellV_SF: Point<Model807, u16> = Point::new(29, 1, false);
pub const Tmp_SF: Point<Model807, u16> = Point::new(30, 1, false);
pub const SoC_SF: Point<Model807, u16> = Point::new(31, 1, false);
pub const OCV_SF: Point<Model807, u16> = Point::new(32, 1, false);
pub const Pad1: Point<Model807, u16> = Point::new(33, 1, false);
