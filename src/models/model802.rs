#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model802 {}

impl Model for Model802 {
    const ID: u16 = 802;
    const LENGTH: u16 = 62;
}

pub const AHRtg: Point<Model802, u16> = Point::new(0, 1, false);
pub const WHRtg: Point<Model802, u16> = Point::new(1, 1, false);
pub const WChaRteMax: Point<Model802, u16> = Point::new(2, 1, false);
pub const WDisChaRteMax: Point<Model802, u16> = Point::new(3, 1, false);
pub const DisChaRte: Point<Model802, u16> = Point::new(4, 1, false);
pub const SoCMax: Point<Model802, u16> = Point::new(5, 1, false);
pub const SoCMin: Point<Model802, u16> = Point::new(6, 1, false);
pub const SocRsvMax: Point<Model802, u16> = Point::new(7, 1, true);
pub const SoCRsvMin: Point<Model802, u16> = Point::new(8, 1, true);
pub const SoC: Point<Model802, u16> = Point::new(9, 1, false);
pub const DoD: Point<Model802, u16> = Point::new(10, 1, false);
pub const SoH: Point<Model802, u16> = Point::new(11, 1, false);
pub const NCyc: Point<Model802, u32> = Point::new(12, 1, false);
pub const ChaSt: Point<Model802, u16> = Point::new(14, 1, false);
pub const LocRemCtl: Point<Model802, u16> = Point::new(15, 1, false);
pub const Hb: Point<Model802, u16> = Point::new(16, 1, false);
pub const CtrlHb: Point<Model802, u16> = Point::new(17, 1, true);
pub const AlmRst: Point<Model802, u16> = Point::new(18, 1, true);
pub const Typ: Point<Model802, u16> = Point::new(19, 1, false);
pub const State: Point<Model802, u16> = Point::new(20, 1, false);
pub const StateVnd: Point<Model802, u16> = Point::new(21, 1, false);
pub const WarrDt: Point<Model802, u32> = Point::new(22, 1, false);
pub const Evt1: Point<Model802, u32> = Point::new(24, 1, false);
pub const Evt2: Point<Model802, u32> = Point::new(26, 1, false);
pub const EvtVnd1: Point<Model802, u32> = Point::new(28, 1, false);
pub const EvtVnd2: Point<Model802, u32> = Point::new(30, 1, false);
pub const V: Point<Model802, u16> = Point::new(32, 1, false);
pub const VMax: Point<Model802, u16> = Point::new(33, 1, false);
pub const VMin: Point<Model802, u16> = Point::new(34, 1, false);
pub const CellVMax: Point<Model802, u16> = Point::new(35, 1, false);
pub const CellVMaxStr: Point<Model802, u16> = Point::new(36, 1, false);
pub const CellVMaxMod: Point<Model802, u16> = Point::new(37, 1, false);
pub const CellVMin: Point<Model802, u16> = Point::new(38, 1, false);
pub const CellVMinStr: Point<Model802, u16> = Point::new(39, 1, false);
pub const CellVMinMod: Point<Model802, u16> = Point::new(40, 1, false);
pub const CellVAvg: Point<Model802, u16> = Point::new(41, 1, false);
pub const A: Point<Model802, i16> = Point::new(42, 1, false);
pub const AChaMax: Point<Model802, u16> = Point::new(43, 1, false);
pub const ADisChaMax: Point<Model802, u16> = Point::new(44, 1, false);
pub const W: Point<Model802, i16> = Point::new(45, 1, false);
pub const ReqInvState: Point<Model802, u16> = Point::new(46, 1, false);
pub const ReqW: Point<Model802, i16> = Point::new(47, 1, false);
pub const SetOp: Point<Model802, u16> = Point::new(48, 1, true);
pub const SetInvState: Point<Model802, u16> = Point::new(49, 1, true);
pub const AHRtg_SF: Point<Model802, u16> = Point::new(50, 1, false);
pub const WHRtg_SF: Point<Model802, u16> = Point::new(51, 1, false);
pub const WChaDisChaMax_SF: Point<Model802, u16> = Point::new(52, 1, false);
pub const DisChaRte_SF: Point<Model802, u16> = Point::new(53, 1, false);
pub const SoC_SF: Point<Model802, u16> = Point::new(54, 1, false);
pub const DoD_SF: Point<Model802, u16> = Point::new(55, 1, false);
pub const SoH_SF: Point<Model802, u16> = Point::new(56, 1, false);
pub const V_SF: Point<Model802, u16> = Point::new(57, 1, false);
pub const CellV_SF: Point<Model802, u16> = Point::new(58, 1, false);
pub const A_SF: Point<Model802, u16> = Point::new(59, 1, false);
pub const AMax_SF: Point<Model802, u16> = Point::new(60, 1, false);
pub const W_SF: Point<Model802, u16> = Point::new(61, 1, false);
