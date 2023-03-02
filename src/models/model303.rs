#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model303 {}

impl Model for Model303 {
    const ID: u16 = 303;
    const LENGTH: u16 = 1;
}

pub const TmpBOM: Point<Model303, i16> = Point::new(0, 1, false);
