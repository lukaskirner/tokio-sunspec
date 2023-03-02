#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model16 {}

impl Model for Model16 {
    const ID: u16 = 16;
    const LENGTH: u16 = 52;
}

pub const Nam: Point<Model16, String> = Point::new(0, 4, false);
pub const Cfg: Point<Model16, u16> = Point::new(4, 1, false);
pub const Ctl: Point<Model16, u16> = Point::new(5, 1, false);
pub const Addr: Point<Model16, String> = Point::new(6, 8, false);
pub const Msk: Point<Model16, String> = Point::new(14, 8, false);
pub const Gw: Point<Model16, String> = Point::new(22, 8, false);
pub const DNS1: Point<Model16, String> = Point::new(30, 8, false);
pub const DNS2: Point<Model16, String> = Point::new(38, 8, false);
pub const MAC: Point<Model16, String> = Point::new(46, 1, false);
pub const LnkCtl: Point<Model16, u16> = Point::new(50, 1, false);
pub const Pad: Point<Model16, u16> = Point::new(51, 1, false);
