#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model145 {}

impl Model for Model145 {
    const ID: u16 = 145;
    const LENGTH: u16 = 8;
}

pub const NomRmpUpRte: Point<Model145, u16> = Point::new(0, 1, false);
pub const NomRmpDnRte: Point<Model145, u16> = Point::new(1, 1, false);
pub const EmgRmpUpRte: Point<Model145, u16> = Point::new(2, 1, false);
pub const EmgRmpDnRte: Point<Model145, u16> = Point::new(3, 1, false);
pub const ConnRmpUpRte: Point<Model145, u16> = Point::new(4, 1, false);
pub const ConnRmpDnRte: Point<Model145, u16> = Point::new(5, 1, false);
pub const AGra: Point<Model145, u16> = Point::new(6, 1, false);
pub const Rmp_SF: Point<Model145, u16> = Point::new(7, 1, false);
