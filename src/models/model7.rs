#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model7 {}

impl Model for Model7 {
    const ID: u16 = 7;
    const LENGTH: u16 = 11;
}

pub const RqSeq: Point<Model7, u16> = Point::new(0, 1, false);
pub const Sts: Point<Model7, u16> = Point::new(1, 1, false);
pub const Ts: Point<Model7, u32> = Point::new(2, 1, false);
pub const Ms: Point<Model7, u16> = Point::new(4, 1, false);
pub const Seq: Point<Model7, u16> = Point::new(5, 1, false);
pub const Alm: Point<Model7, u16> = Point::new(6, 1, false);
pub const Rsrvd: Point<Model7, u16> = Point::new(7, 1, false);
pub const Alg: Point<Model7, u16> = Point::new(8, 1, false);
pub const N: Point<Model7, u16> = Point::new(9, 1, false);
