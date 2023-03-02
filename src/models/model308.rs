#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model308 {}

impl Model for Model308 {
    const ID: u16 = 308;
    const LENGTH: u16 = 4;
}

pub const GHI: Point<Model308, u16> = Point::new(0, 1, false);
pub const TmpBOM: Point<Model308, i16> = Point::new(1, 1, false);
pub const TmpAmb: Point<Model308, i16> = Point::new(2, 1, false);
pub const WndSpd: Point<Model308, u16> = Point::new(3, 1, false);
