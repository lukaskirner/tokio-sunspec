#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model302 {}

impl Model for Model302 {
    const ID: u16 = 302;
    const LENGTH: u16 = 5;
}

pub const GHI: Point<Model302, u16> = Point::new(0, 1, false);
pub const POAI: Point<Model302, u16> = Point::new(1, 1, false);
pub const DFI: Point<Model302, u16> = Point::new(2, 1, false);
pub const DNI: Point<Model302, u16> = Point::new(3, 1, false);
pub const OTI: Point<Model302, u16> = Point::new(4, 1, false);
