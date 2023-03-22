#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model111 {}

impl Model for Model111 {
    const ID: u16 = 111;
    const LENGTH: u16 = 60;
}

pub const A: Point<Model111, f32> = Point::new(0, 2, false);
pub const AphA: Point<Model111, f32> = Point::new(2, 2, false);
pub const AphB: Point<Model111, f32> = Point::new(4, 2, false);
pub const AphC: Point<Model111, f32> = Point::new(6, 2, false);
pub const PPVphAB: Point<Model111, f32> = Point::new(8, 2, false);
pub const PPVphBC: Point<Model111, f32> = Point::new(10, 2, false);
pub const PPVphCA: Point<Model111, f32> = Point::new(12, 2, false);
pub const PhVphA: Point<Model111, f32> = Point::new(14, 2, false);
pub const PhVphB: Point<Model111, f32> = Point::new(16, 2, false);
pub const PhVphC: Point<Model111, f32> = Point::new(18, 2, false);
pub const W: Point<Model111, f32> = Point::new(20, 2, false);
pub const Hz: Point<Model111, f32> = Point::new(22, 2, false);
pub const VA: Point<Model111, f32> = Point::new(24, 2, false);
pub const VAr: Point<Model111, f32> = Point::new(26, 2, false);
pub const PF: Point<Model111, f32> = Point::new(28, 2, false);
pub const WH: Point<Model111, f32> = Point::new(30, 2, false);
pub const DCA: Point<Model111, f32> = Point::new(32, 2, false);
pub const DCV: Point<Model111, f32> = Point::new(34, 2, false);
pub const DCW: Point<Model111, f32> = Point::new(36, 2, false);
pub const TmpCab: Point<Model111, f32> = Point::new(38, 2, false);
pub const TmpSnk: Point<Model111, f32> = Point::new(40, 2, false);
pub const TmpTrns: Point<Model111, f32> = Point::new(42, 2, false);
pub const TmpOt: Point<Model111, f32> = Point::new(44, 2, false);
pub const St: Point<Model111, u16> = Point::new(46, 1, false);
pub const StVnd: Point<Model111, u16> = Point::new(47, 1, false);
pub const Evt1: Point<Model111, u32> = Point::new(48, 1, false);
pub const Evt2: Point<Model111, u32> = Point::new(50, 1, false);
pub const EvtVnd1: Point<Model111, u32> = Point::new(52, 1, false);
pub const EvtVnd2: Point<Model111, u32> = Point::new(54, 1, false);
pub const EvtVnd3: Point<Model111, u32> = Point::new(56, 1, false);
pub const EvtVnd4: Point<Model111, u32> = Point::new(58, 1, false);
