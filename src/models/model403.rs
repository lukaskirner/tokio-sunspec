#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model403 {}

impl Model for Model403 {
    const ID: u16 = 403;
    const LENGTH: u16 = 24;
}

pub const DCA_SF: Point<Model403, u16> = Point::new(0, 1, false);
pub const DCAhr_SF: Point<Model403, u16> = Point::new(1, 1, false);
pub const DCV_SF: Point<Model403, u16> = Point::new(2, 1, false);
pub const DCAMax: Point<Model403, u16> = Point::new(3, 1, false);
pub const N: Point<Model403, u16> = Point::new(4, 1, false);
pub const Evt: Point<Model403, u32> = Point::new(5, 1, false);
pub const EvtVnd: Point<Model403, u32> = Point::new(7, 1, false);
pub const DCA: Point<Model403, i16> = Point::new(9, 1, false);
pub const DCAhr: Point<Model403, u32> = Point::new(10, 1, false);
pub const DCV: Point<Model403, i16> = Point::new(12, 1, false);
pub const Tmp: Point<Model403, i16> = Point::new(13, 1, false);
pub const InDCA_SF: Point<Model403, u16> = Point::new(14, 1, false);
pub const InDCAhr_SF: Point<Model403, u16> = Point::new(15, 1, false);
