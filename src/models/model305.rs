#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model305 {}

impl Model for Model305 {
    const ID: u16 = 305;
    const LENGTH: u16 = 36;
}

pub const Tm: Point<Model305, String> = Point::new(0, 6, false);
pub const Date: Point<Model305, String> = Point::new(6, 4, false);
pub const Loc: Point<Model305, String> = Point::new(10, 20, false);
pub const Lat: Point<Model305, i32> = Point::new(30, 1, false);
pub const Long: Point<Model305, i32> = Point::new(32, 1, false);
pub const Alt: Point<Model305, i32> = Point::new(34, 1, false);
