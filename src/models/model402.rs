#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model402 {}

impl Model for Model402 {
    const ID: u16 = 402;
    const LENGTH: u16 = 34;
}

pub const DCA_SF: Point<Model402, u16> = Point::new(0, 1, false);
pub const DCAhr_SF: Point<Model402, u16> = Point::new(1, 1, false);
pub const DCV_SF: Point<Model402, u16> = Point::new(2, 1, false);
pub const DCW_SF: Point<Model402, u16> = Point::new(3, 1, false);
pub const DCWh_SF: Point<Model402, u16> = Point::new(4, 1, false);
pub const DCAMax: Point<Model402, u16> = Point::new(5, 1, false);
pub const N: Point<Model402, u16> = Point::new(6, 1, false);
pub const Evt: Point<Model402, u32> = Point::new(7, 1, false);
pub const EvtVnd: Point<Model402, u32> = Point::new(9, 1, false);
pub const DCA: Point<Model402, i16> = Point::new(11, 1, false);
pub const DCAhr: Point<Model402, u32> = Point::new(12, 1, false);
pub const DCV: Point<Model402, u16> = Point::new(14, 1, false);
pub const Tmp: Point<Model402, i16> = Point::new(15, 1, false);
pub const DCW: Point<Model402, i16> = Point::new(16, 1, false);
pub const DCPR: Point<Model402, u16> = Point::new(17, 1, false);
pub const DCWh: Point<Model402, u32> = Point::new(18, 1, false);
