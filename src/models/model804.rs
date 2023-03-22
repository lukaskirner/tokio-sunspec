#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model804 {}

impl Model for Model804 {
    const ID: u16 = 804;
    const LENGTH: u16 = 62;
}

pub const Idx: Point<Model804, u16> = Point::new(0, 1, false);
pub const NMod: Point<Model804, u16> = Point::new(1, 1, false);
pub const St: Point<Model804, u32> = Point::new(2, 1, false);
pub const ConFail: Point<Model804, u16> = Point::new(4, 1, false);
pub const NCellBal: Point<Model804, u16> = Point::new(5, 1, false);
pub const SoC: Point<Model804, u16> = Point::new(6, 1, false);
pub const DoD: Point<Model804, u16> = Point::new(7, 1, false);
pub const NCyc: Point<Model804, u32> = Point::new(8, 1, false);
pub const SoH: Point<Model804, u16> = Point::new(10, 1, false);
pub const A: Point<Model804, i16> = Point::new(11, 1, false);
pub const V: Point<Model804, u16> = Point::new(12, 1, false);
pub const CellVMax: Point<Model804, u16> = Point::new(13, 1, false);
pub const CellVMaxMod: Point<Model804, u16> = Point::new(14, 1, false);
pub const CellVMin: Point<Model804, u16> = Point::new(15, 1, false);
pub const CellVMinMod: Point<Model804, u16> = Point::new(16, 1, false);
pub const CellVAvg: Point<Model804, u16> = Point::new(17, 1, false);
pub const ModTmpMax: Point<Model804, i16> = Point::new(18, 1, false);
pub const ModTmpMaxMod: Point<Model804, u16> = Point::new(19, 1, false);
pub const ModTmpMin: Point<Model804, i16> = Point::new(20, 1, false);
pub const ModTmpMinMod: Point<Model804, u16> = Point::new(21, 1, false);
pub const ModTmpAvg: Point<Model804, i16> = Point::new(22, 1, false);
pub const Pad1: Point<Model804, u16> = Point::new(23, 1, false);
pub const ConSt: Point<Model804, u32> = Point::new(24, 1, false);
pub const Evt1: Point<Model804, u32> = Point::new(26, 1, false);
pub const Evt2: Point<Model804, u32> = Point::new(28, 1, false);
pub const EvtVnd1: Point<Model804, u32> = Point::new(30, 1, false);
pub const EvtVnd2: Point<Model804, u32> = Point::new(32, 1, false);
pub const SetEna: Point<Model804, u16> = Point::new(34, 1, true);
pub const SetCon: Point<Model804, u16> = Point::new(35, 1, true);
pub const SoC_SF: Point<Model804, u16> = Point::new(36, 1, false);
pub const SoH_SF: Point<Model804, u16> = Point::new(37, 1, false);
pub const DoD_SF: Point<Model804, u16> = Point::new(38, 1, false);
pub const A_SF: Point<Model804, u16> = Point::new(39, 1, false);
pub const V_SF: Point<Model804, u16> = Point::new(40, 1, false);
pub const CellV_SF: Point<Model804, u16> = Point::new(41, 1, false);
pub const ModTmp_SF: Point<Model804, u16> = Point::new(42, 1, false);
pub const Pad2: Point<Model804, u16> = Point::new(43, 1, false);
pub const Pad3: Point<Model804, u16> = Point::new(44, 1, false);
pub const Pad4: Point<Model804, u16> = Point::new(45, 1, false);
