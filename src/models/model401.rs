#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model401 {}

impl Model for Model401 {
    const ID: u16 = 401;
    const LENGTH: u16 = 22;
}

pub const DCA_SF: Point<Model401, u16> = Point::new(0, 1, false);
pub const DCAhr_SF: Point<Model401, u16> = Point::new(1, 1, false);
pub const DCV_SF: Point<Model401, u16> = Point::new(2, 1, false);
pub const DCAMax: Point<Model401, u16> = Point::new(3, 1, false);
pub const N: Point<Model401, u16> = Point::new(4, 1, false);
pub const Evt: Point<Model401, u32> = Point::new(5, 1, false);
pub const EvtVnd: Point<Model401, u32> = Point::new(7, 1, false);
pub const DCA: Point<Model401, i16> = Point::new(9, 1, false);
pub const DCAhr: Point<Model401, u32> = Point::new(10, 1, false);
pub const DCV: Point<Model401, u16> = Point::new(12, 1, false);
pub const Tmp: Point<Model401, i16> = Point::new(13, 1, false);
