#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model14 {}

impl Model for Model14 {
    const ID: u16 = 14;
    const LENGTH: u16 = 52;
}

pub const Nam: Point<Model14, String> = Point::new(0, 4, true);
pub const Cap: Point<Model14, u16> = Point::new(4, 1, true);
pub const Cfg: Point<Model14, u16> = Point::new(5, 1, true);
pub const Typ: Point<Model14, u16> = Point::new(6, 1, true);
pub const Addr: Point<Model14, String> = Point::new(7, 20, true);
pub const Port: Point<Model14, u16> = Point::new(27, 1, true);
pub const User: Point<Model14, String> = Point::new(28, 12, true);
pub const Pw: Point<Model14, String> = Point::new(40, 12, true);
