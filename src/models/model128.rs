#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model128 {}

impl Model for Model128 {
    const ID: u16 = 128;
    const LENGTH: u16 = 14;
}

pub const ArGraMod: Point<Model128, u16> = Point::new(0, 1, false);
pub const ArGraSag: Point<Model128, u16> = Point::new(1, 1, false);
pub const ArGraSwell: Point<Model128, u16> = Point::new(2, 1, false);
pub const ModEna: Point<Model128, u16> = Point::new(3, 1, false);
pub const FilTms: Point<Model128, u16> = Point::new(4, 1, false);
pub const DbVMin: Point<Model128, u16> = Point::new(5, 1, false);
pub const DbVMax: Point<Model128, u16> = Point::new(6, 1, false);
pub const BlkZnV: Point<Model128, u16> = Point::new(7, 1, false);
pub const HysBlkZnV: Point<Model128, u16> = Point::new(8, 1, false);
pub const BlkZnTmms: Point<Model128, u16> = Point::new(9, 1, false);
pub const HoldTmms: Point<Model128, u16> = Point::new(10, 1, false);
pub const ArGra_SF: Point<Model128, u16> = Point::new(11, 1, false);
pub const VRefPct_SF: Point<Model128, u16> = Point::new(12, 1, false);
pub const Pad: Point<Model128, u16> = Point::new(13, 1, false);
