#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model307 {}

impl Model for Model307 {
    const ID: u16 = 307;
    const LENGTH: u16 = 11;
}

pub const TmpAmb: Point<Model307, i16> = Point::new(0, 1, false);
pub const RH: Point<Model307, i16> = Point::new(1, 1, false);
pub const Pres: Point<Model307, i16> = Point::new(2, 1, false);
pub const WndSpd: Point<Model307, i16> = Point::new(3, 1, false);
pub const WndDir: Point<Model307, i16> = Point::new(4, 1, false);
pub const Rain: Point<Model307, i16> = Point::new(5, 1, false);
pub const Snw: Point<Model307, i16> = Point::new(6, 1, false);
pub const PPT: Point<Model307, i16> = Point::new(7, 1, false);
pub const ElecFld: Point<Model307, i16> = Point::new(8, 1, false);
pub const SurWet: Point<Model307, i16> = Point::new(9, 1, false);
pub const SoilWet: Point<Model307, i16> = Point::new(10, 1, false);
