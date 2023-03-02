#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model404 {}

impl Model for Model404 {
    const ID: u16 = 404;
    const LENGTH: u16 = 39;
}

pub const DCA_SF: Point<Model404, u16> = Point::new(0, 1, false);
pub const DCAhr_SF: Point<Model404, u16> = Point::new(1, 1, false);
pub const DCV_SF: Point<Model404, u16> = Point::new(2, 1, false);
pub const DCW_SF: Point<Model404, u16> = Point::new(3, 1, false);
pub const DCWh_SF: Point<Model404, u16> = Point::new(4, 1, false);
pub const DCAMax: Point<Model404, u16> = Point::new(5, 1, false);
pub const N: Point<Model404, u16> = Point::new(6, 1, false);
pub const Evt: Point<Model404, u32> = Point::new(7, 1, false);
pub const EvtVnd: Point<Model404, u32> = Point::new(9, 1, false);
pub const DCA: Point<Model404, i16> = Point::new(11, 1, false);
pub const DCAhr: Point<Model404, u32> = Point::new(12, 1, false);
pub const DCV: Point<Model404, i16> = Point::new(14, 1, false);
pub const Tmp: Point<Model404, i16> = Point::new(15, 1, false);
pub const DCW: Point<Model404, i16> = Point::new(16, 1, false);
pub const DCPR: Point<Model404, i16> = Point::new(17, 1, false);
pub const DCWh: Point<Model404, u32> = Point::new(18, 1, false);
pub const InDCA_SF: Point<Model404, u16> = Point::new(20, 1, false);
pub const InDCAhr_SF: Point<Model404, u16> = Point::new(21, 1, false);
pub const InDCV_SF: Point<Model404, u16> = Point::new(22, 1, false);
pub const InDCW_SF: Point<Model404, u16> = Point::new(23, 1, false);
pub const InDCWh_SF: Point<Model404, u16> = Point::new(24, 1, false);
