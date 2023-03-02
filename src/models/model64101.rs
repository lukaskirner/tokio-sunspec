#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model64101 {}

impl Model for Model64101 {
    const ID: u16 = 64101;
    const LENGTH: u16 = 7;
}

pub const Eltek_Country_Code: Point<Model64101, u16> = Point::new(0, 1, false);
pub const Eltek_Feeding_Phase: Point<Model64101, u16> = Point::new(1, 1, false);
pub const Eltek_APD_Method: Point<Model64101, u16> = Point::new(2, 1, false);
pub const Eltek_APD_Power_Ref: Point<Model64101, u16> = Point::new(3, 1, false);
pub const Eltek_RPS_Method: Point<Model64101, u16> = Point::new(4, 1, false);
pub const Eltek_RPS_Q_Ref: Point<Model64101, u16> = Point::new(5, 1, false);
pub const Eltek_RPS_CosPhi_Ref: Point<Model64101, i16> = Point::new(6, 1, false);
