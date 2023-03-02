#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model160 {}

impl Model for Model160 {
    const ID: u16 = 160;
    const LENGTH: u16 = 28;
}

pub const DCA_SF: Point<Model160, u16> = Point::new(0, 1, false);
pub const DCV_SF: Point<Model160, u16> = Point::new(1, 1, false);
pub const DCW_SF: Point<Model160, u16> = Point::new(2, 1, false);
pub const DCWH_SF: Point<Model160, u16> = Point::new(3, 1, false);
pub const Evt: Point<Model160, u32> = Point::new(4, 1, false);
pub const N: Point<Model160, u16> = Point::new(6, 1, false);
pub const TmsPer: Point<Model160, u16> = Point::new(7, 1, false);
