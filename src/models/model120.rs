#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model120 {}

impl Model for Model120 {
    const ID: u16 = 120;
    const LENGTH: u16 = 26;
}

pub const DERTyp: Point<Model120, u16> = Point::new(0, 1, false);
pub const WRtg: Point<Model120, u16> = Point::new(1, 1, false);
pub const WRtg_SF: Point<Model120, u16> = Point::new(2, 1, false);
pub const VARtg: Point<Model120, u16> = Point::new(3, 1, false);
pub const VARtg_SF: Point<Model120, u16> = Point::new(4, 1, false);
pub const VArRtgQ1: Point<Model120, i16> = Point::new(5, 1, false);
pub const VArRtgQ2: Point<Model120, i16> = Point::new(6, 1, false);
pub const VArRtgQ3: Point<Model120, i16> = Point::new(7, 1, false);
pub const VArRtgQ4: Point<Model120, i16> = Point::new(8, 1, false);
pub const VArRtg_SF: Point<Model120, u16> = Point::new(9, 1, false);
pub const ARtg: Point<Model120, u16> = Point::new(10, 1, false);
pub const ARtg_SF: Point<Model120, u16> = Point::new(11, 1, false);
pub const PFRtgQ1: Point<Model120, i16> = Point::new(12, 1, false);
pub const PFRtgQ2: Point<Model120, i16> = Point::new(13, 1, false);
pub const PFRtgQ3: Point<Model120, i16> = Point::new(14, 1, false);
pub const PFRtgQ4: Point<Model120, i16> = Point::new(15, 1, false);
pub const PFRtg_SF: Point<Model120, u16> = Point::new(16, 1, false);
pub const WHRtg: Point<Model120, u16> = Point::new(17, 1, false);
pub const WHRtg_SF: Point<Model120, u16> = Point::new(18, 1, false);
pub const AhrRtg: Point<Model120, u16> = Point::new(19, 1, false);
pub const AhrRtg_SF: Point<Model120, u16> = Point::new(20, 1, false);
pub const MaxChaRte: Point<Model120, u16> = Point::new(21, 1, false);
pub const MaxChaRte_SF: Point<Model120, u16> = Point::new(22, 1, false);
pub const MaxDisChaRte: Point<Model120, u16> = Point::new(23, 1, false);
pub const MaxDisChaRte_SF: Point<Model120, u16> = Point::new(24, 1, false);
pub const Pad: Point<Model120, u16> = Point::new(25, 1, false);
