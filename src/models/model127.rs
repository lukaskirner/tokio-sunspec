#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model127 {}

impl Model for Model127 {
    const ID: u16 = 127;
    const LENGTH: u16 = 10;
}

pub const WGra: Point<Model127, u16> = Point::new(0, 1, true);
pub const HzStr: Point<Model127, i16> = Point::new(1, 1, true);
pub const HzStop: Point<Model127, i16> = Point::new(2, 1, true);
pub const HysEna: Point<Model127, u16> = Point::new(3, 1, true);
pub const ModEna: Point<Model127, u16> = Point::new(4, 1, true);
pub const HzStopWGra: Point<Model127, u16> = Point::new(5, 1, true);
pub const WGra_SF: Point<Model127, u16> = Point::new(6, 1, false);
pub const HzStrStop_SF: Point<Model127, u16> = Point::new(7, 1, false);
pub const RmpIncDec_SF: Point<Model127, u16> = Point::new(8, 1, false);
pub const Pad: Point<Model127, u16> = Point::new(9, 1, false);
