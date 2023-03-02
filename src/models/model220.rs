#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model220 {}

impl Model for Model220 {
    const ID: u16 = 220;
    const LENGTH: u16 = 43;
}

pub const A: Point<Model220, i16> = Point::new(0, 1, false);
pub const A_SF: Point<Model220, u16> = Point::new(1, 1, false);
pub const PhV: Point<Model220, i16> = Point::new(2, 1, false);
pub const V_SF: Point<Model220, u16> = Point::new(3, 1, false);
pub const Hz: Point<Model220, i16> = Point::new(4, 1, false);
pub const Hz_SF: Point<Model220, u16> = Point::new(5, 1, false);
pub const W: Point<Model220, i16> = Point::new(6, 1, false);
pub const W_SF: Point<Model220, u16> = Point::new(7, 1, false);
pub const VA: Point<Model220, i16> = Point::new(8, 1, false);
pub const VA_SF: Point<Model220, u16> = Point::new(9, 1, false);
pub const VAR: Point<Model220, i16> = Point::new(10, 1, false);
pub const VAR_SF: Point<Model220, u16> = Point::new(11, 1, false);
pub const PF: Point<Model220, i16> = Point::new(12, 1, false);
pub const PF_SF: Point<Model220, u16> = Point::new(13, 1, false);
pub const TotWhExp: Point<Model220, u32> = Point::new(14, 1, false);
pub const TotWhImp: Point<Model220, u32> = Point::new(16, 1, false);
pub const TotWh_SF: Point<Model220, u16> = Point::new(18, 1, false);
pub const TotVAhExp: Point<Model220, u32> = Point::new(19, 1, false);
pub const TotVAhImp: Point<Model220, u32> = Point::new(21, 1, false);
pub const TotVAh_SF: Point<Model220, u16> = Point::new(23, 1, false);
pub const TotVArhImpQ1: Point<Model220, u32> = Point::new(24, 1, false);
pub const TotVArhImpQ2: Point<Model220, u32> = Point::new(26, 1, false);
pub const TotVArhExpQ3: Point<Model220, u32> = Point::new(28, 1, false);
pub const TotVArhExpQ4: Point<Model220, u32> = Point::new(30, 1, false);
pub const TotVArh_SF: Point<Model220, u16> = Point::new(32, 1, false);
pub const Evt: Point<Model220, u32> = Point::new(33, 1, false);
pub const Rsrvd: Point<Model220, u16> = Point::new(35, 1, false);
pub const Ts: Point<Model220, u32> = Point::new(36, 1, false);
pub const Ms: Point<Model220, u16> = Point::new(38, 1, false);
pub const Seq: Point<Model220, u16> = Point::new(39, 1, false);
pub const Alg: Point<Model220, u16> = Point::new(40, 1, false);
pub const N: Point<Model220, u16> = Point::new(41, 1, false);
