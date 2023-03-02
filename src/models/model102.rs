#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model102 {}

impl Model for Model102 {
    const ID: u16 = 102;
    const LENGTH: u16 = 50;
}

pub const A: Point<Model102, u16> = Point::new(0, 1, false);
pub const AphA: Point<Model102, u16> = Point::new(1, 1, false);
pub const AphB: Point<Model102, u16> = Point::new(2, 1, false);
pub const AphC: Point<Model102, u16> = Point::new(3, 1, false);
pub const A_SF: Point<Model102, u16> = Point::new(4, 1, false);
pub const PPVphAB: Point<Model102, u16> = Point::new(5, 1, false);
pub const PPVphBC: Point<Model102, u16> = Point::new(6, 1, false);
pub const PPVphCA: Point<Model102, u16> = Point::new(7, 1, false);
pub const PhVphA: Point<Model102, u16> = Point::new(8, 1, false);
pub const PhVphB: Point<Model102, u16> = Point::new(9, 1, false);
pub const PhVphC: Point<Model102, u16> = Point::new(10, 1, false);
pub const V_SF: Point<Model102, u16> = Point::new(11, 1, false);
pub const W: Point<Model102, i16> = Point::new(12, 1, false);
pub const W_SF: Point<Model102, u16> = Point::new(13, 1, false);
pub const Hz: Point<Model102, u16> = Point::new(14, 1, false);
pub const Hz_SF: Point<Model102, u16> = Point::new(15, 1, false);
pub const VA: Point<Model102, i16> = Point::new(16, 1, false);
pub const VA_SF: Point<Model102, u16> = Point::new(17, 1, false);
pub const VAr: Point<Model102, i16> = Point::new(18, 1, false);
pub const VAr_SF: Point<Model102, u16> = Point::new(19, 1, false);
pub const PF: Point<Model102, i16> = Point::new(20, 1, false);
pub const PF_SF: Point<Model102, u16> = Point::new(21, 1, false);
pub const WH: Point<Model102, u32> = Point::new(22, 1, false);
pub const WH_SF: Point<Model102, u16> = Point::new(24, 1, false);
pub const DCA: Point<Model102, u16> = Point::new(25, 1, false);
pub const DCA_SF: Point<Model102, u16> = Point::new(26, 1, false);
pub const DCV: Point<Model102, u16> = Point::new(27, 1, false);
pub const DCV_SF: Point<Model102, u16> = Point::new(28, 1, false);
pub const DCW: Point<Model102, i16> = Point::new(29, 1, false);
pub const DCW_SF: Point<Model102, u16> = Point::new(30, 1, false);
pub const TmpCab: Point<Model102, i16> = Point::new(31, 1, false);
pub const TmpSnk: Point<Model102, i16> = Point::new(32, 1, false);
pub const TmpTrns: Point<Model102, i16> = Point::new(33, 1, false);
pub const TmpOt: Point<Model102, i16> = Point::new(34, 1, false);
pub const Tmp_SF: Point<Model102, u16> = Point::new(35, 1, false);
pub const St: Point<Model102, u16> = Point::new(36, 1, false);
pub const StVnd: Point<Model102, u16> = Point::new(37, 1, false);
pub const Evt1: Point<Model102, u32> = Point::new(38, 1, false);
pub const Evt2: Point<Model102, u32> = Point::new(40, 1, false);
pub const EvtVnd1: Point<Model102, u32> = Point::new(42, 1, false);
pub const EvtVnd2: Point<Model102, u32> = Point::new(44, 1, false);
pub const EvtVnd3: Point<Model102, u32> = Point::new(46, 1, false);
pub const EvtVnd4: Point<Model102, u32> = Point::new(48, 1, false);
