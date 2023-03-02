#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model64001 {}

impl Model for Model64001 {
    const ID: u16 = 64001;
    const LENGTH: u16 = 71;
}

pub const Cmd: Point<Model64001, u16> = Point::new(0, 1, false);
pub const HWRev: Point<Model64001, u16> = Point::new(1, 1, false);
pub const RSFWRev: Point<Model64001, u16> = Point::new(2, 1, false);
pub const OSFWRev: Point<Model64001, u16> = Point::new(3, 1, false);
pub const ProdRev: Point<Model64001, String> = Point::new(4, 2, false);
pub const Boots: Point<Model64001, u16> = Point::new(6, 1, false);
pub const Switch: Point<Model64001, u16> = Point::new(7, 1, false);
pub const Sensors: Point<Model64001, u16> = Point::new(8, 1, false);
pub const Talking: Point<Model64001, u16> = Point::new(9, 1, false);
pub const Status: Point<Model64001, u16> = Point::new(10, 1, false);
pub const Config: Point<Model64001, u16> = Point::new(11, 1, false);
pub const LEDblink: Point<Model64001, u16> = Point::new(12, 1, false);
pub const LEDon: Point<Model64001, u16> = Point::new(13, 1, false);
pub const Reserved: Point<Model64001, u16> = Point::new(14, 1, false);
pub const Loc: Point<Model64001, String> = Point::new(15, 16, false);
pub const S1ID: Point<Model64001, u16> = Point::new(31, 1, false);
pub const S1Addr: Point<Model64001, u16> = Point::new(32, 1, false);
pub const S1OSVer: Point<Model64001, u16> = Point::new(33, 1, false);
pub const S1Ver: Point<Model64001, String> = Point::new(34, 2, false);
pub const S1Serial: Point<Model64001, String> = Point::new(36, 5, false);
pub const S2ID: Point<Model64001, u16> = Point::new(41, 1, false);
pub const S2Addr: Point<Model64001, u16> = Point::new(42, 1, false);
pub const S2OSVer: Point<Model64001, u16> = Point::new(43, 1, false);
pub const S2Ver: Point<Model64001, String> = Point::new(44, 2, false);
pub const S2Serial: Point<Model64001, String> = Point::new(46, 5, false);
pub const S3ID: Point<Model64001, u16> = Point::new(51, 1, false);
pub const S3Addr: Point<Model64001, u16> = Point::new(52, 1, false);
pub const S3OSVer: Point<Model64001, u16> = Point::new(53, 1, false);
pub const S3Ver: Point<Model64001, String> = Point::new(54, 2, false);
pub const S3Serial: Point<Model64001, String> = Point::new(56, 5, false);
pub const S4ID: Point<Model64001, u16> = Point::new(61, 1, false);
pub const S4Addr: Point<Model64001, u16> = Point::new(62, 1, false);
pub const S4OSVer: Point<Model64001, u16> = Point::new(63, 1, false);
pub const S4Ver: Point<Model64001, String> = Point::new(64, 2, false);
pub const S4Serial: Point<Model64001, String> = Point::new(66, 5, false);
