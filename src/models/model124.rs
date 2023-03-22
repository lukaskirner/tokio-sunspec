#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model124 {}

impl Model for Model124 {
    const ID: u16 = 124;
    const LENGTH: u16 = 24;
}

pub const WChaMax: Point<Model124, u16> = Point::new(0, 1, true);
pub const WChaGra: Point<Model124, u16> = Point::new(1, 1, true);
pub const WDisChaGra: Point<Model124, u16> = Point::new(2, 1, true);
pub const StorCtl_Mod: Point<Model124, u16> = Point::new(3, 1, true);
pub const VAChaMax: Point<Model124, u16> = Point::new(4, 1, true);
pub const MinRsvPct: Point<Model124, u16> = Point::new(5, 1, true);
pub const ChaState: Point<Model124, u16> = Point::new(6, 1, false);
pub const StorAval: Point<Model124, u16> = Point::new(7, 1, false);
pub const InBatV: Point<Model124, u16> = Point::new(8, 1, false);
pub const ChaSt: Point<Model124, u16> = Point::new(9, 1, false);
pub const OutWRte: Point<Model124, i16> = Point::new(10, 1, true);
pub const InWRte: Point<Model124, i16> = Point::new(11, 1, true);
pub const InOutWRte_WinTms: Point<Model124, u16> = Point::new(12, 1, true);
pub const InOutWRte_RvrtTms: Point<Model124, u16> = Point::new(13, 1, true);
pub const InOutWRte_RmpTms: Point<Model124, u16> = Point::new(14, 1, true);
pub const ChaGriSet: Point<Model124, u16> = Point::new(15, 1, true);
pub const WChaMax_SF: Point<Model124, u16> = Point::new(16, 1, false);
pub const WChaDisChaGra_SF: Point<Model124, u16> = Point::new(17, 1, false);
pub const VAChaMax_SF: Point<Model124, u16> = Point::new(18, 1, false);
pub const MinRsvPct_SF: Point<Model124, u16> = Point::new(19, 1, false);
pub const ChaState_SF: Point<Model124, u16> = Point::new(20, 1, false);
pub const StorAval_SF: Point<Model124, u16> = Point::new(21, 1, false);
pub const InBatV_SF: Point<Model124, u16> = Point::new(22, 1, false);
pub const InOutWRte_SF: Point<Model124, u16> = Point::new(23, 1, false);
