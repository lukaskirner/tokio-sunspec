#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model113 {}

impl Model for Model113 {
    const ID: u16 = 113;
    const LENGTH: u16 = 60;
}

pub const A: Point<Model113, f32> = Point::new(0, 1, false);
pub const AphA: Point<Model113, f32> = Point::new(2, 1, false);
pub const AphB: Point<Model113, f32> = Point::new(4, 1, false);
pub const AphC: Point<Model113, f32> = Point::new(6, 1, false);
pub const PPVphAB: Point<Model113, f32> = Point::new(8, 1, false);
pub const PPVphBC: Point<Model113, f32> = Point::new(10, 1, false);
pub const PPVphCA: Point<Model113, f32> = Point::new(12, 1, false);
pub const PhVphA: Point<Model113, f32> = Point::new(14, 1, false);
pub const PhVphB: Point<Model113, f32> = Point::new(16, 1, false);
pub const PhVphC: Point<Model113, f32> = Point::new(18, 1, false);
pub const W: Point<Model113, f32> = Point::new(20, 1, false);
pub const Hz: Point<Model113, f32> = Point::new(22, 1, false);
pub const VA: Point<Model113, f32> = Point::new(24, 1, false);
pub const VAr: Point<Model113, f32> = Point::new(26, 1, false);
pub const PF: Point<Model113, f32> = Point::new(28, 1, false);
pub const WH: Point<Model113, f32> = Point::new(30, 1, false);
pub const DCA: Point<Model113, f32> = Point::new(32, 1, false);
pub const DCV: Point<Model113, f32> = Point::new(34, 1, false);
pub const DCW: Point<Model113, f32> = Point::new(36, 1, false);
pub const TmpCab: Point<Model113, f32> = Point::new(38, 1, false);
pub const TmpSnk: Point<Model113, f32> = Point::new(40, 1, false);
pub const TmpTrns: Point<Model113, f32> = Point::new(42, 1, false);
pub const TmpOt: Point<Model113, f32> = Point::new(44, 1, false);
pub const St: Point<Model113, u16> = Point::new(46, 1, false);
pub const StVnd: Point<Model113, u16> = Point::new(47, 1, false);
pub const Evt1: Point<Model113, u32> = Point::new(48, 1, false);
pub const Evt2: Point<Model113, u32> = Point::new(50, 1, false);
pub const EvtVnd1: Point<Model113, u32> = Point::new(52, 1, false);
pub const EvtVnd2: Point<Model113, u32> = Point::new(54, 1, false);
pub const EvtVnd3: Point<Model113, u32> = Point::new(56, 1, false);
pub const EvtVnd4: Point<Model113, u32> = Point::new(58, 1, false);
