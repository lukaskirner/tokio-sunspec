#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model201 {}

impl Model for Model201 {
    const ID: u16 = 201;
    const LENGTH: u16 = 105;
}

pub const A: Point<Model201, i16> = Point::new(0, 1, false);
pub const AphA: Point<Model201, i16> = Point::new(1, 1, false);
pub const AphB: Point<Model201, i16> = Point::new(2, 1, false);
pub const AphC: Point<Model201, i16> = Point::new(3, 1, false);
pub const A_SF: Point<Model201, u16> = Point::new(4, 1, false);
pub const PhV: Point<Model201, i16> = Point::new(5, 1, false);
pub const PhVphA: Point<Model201, i16> = Point::new(6, 1, false);
pub const PhVphB: Point<Model201, i16> = Point::new(7, 1, false);
pub const PhVphC: Point<Model201, i16> = Point::new(8, 1, false);
pub const PPV: Point<Model201, i16> = Point::new(9, 1, false);
pub const PPVphAB: Point<Model201, i16> = Point::new(10, 1, false);
pub const PPVphBC: Point<Model201, i16> = Point::new(11, 1, false);
pub const PPVphCA: Point<Model201, i16> = Point::new(12, 1, false);
pub const V_SF: Point<Model201, u16> = Point::new(13, 1, false);
pub const Hz: Point<Model201, i16> = Point::new(14, 1, false);
pub const Hz_SF: Point<Model201, u16> = Point::new(15, 1, false);
pub const W: Point<Model201, i16> = Point::new(16, 1, false);
pub const WphA: Point<Model201, i16> = Point::new(17, 1, false);
pub const WphB: Point<Model201, i16> = Point::new(18, 1, false);
pub const WphC: Point<Model201, i16> = Point::new(19, 1, false);
pub const W_SF: Point<Model201, u16> = Point::new(20, 1, false);
pub const VA: Point<Model201, i16> = Point::new(21, 1, false);
pub const VAphA: Point<Model201, i16> = Point::new(22, 1, false);
pub const VAphB: Point<Model201, i16> = Point::new(23, 1, false);
pub const VAphC: Point<Model201, i16> = Point::new(24, 1, false);
pub const VA_SF: Point<Model201, u16> = Point::new(25, 1, false);
pub const VAR: Point<Model201, i16> = Point::new(26, 1, false);
pub const VARphA: Point<Model201, i16> = Point::new(27, 1, false);
pub const VARphB: Point<Model201, i16> = Point::new(28, 1, false);
pub const VARphC: Point<Model201, i16> = Point::new(29, 1, false);
pub const VAR_SF: Point<Model201, u16> = Point::new(30, 1, false);
pub const PF: Point<Model201, i16> = Point::new(31, 1, false);
pub const PFphA: Point<Model201, i16> = Point::new(32, 1, false);
pub const PFphB: Point<Model201, i16> = Point::new(33, 1, false);
pub const PFphC: Point<Model201, i16> = Point::new(34, 1, false);
pub const PF_SF: Point<Model201, u16> = Point::new(35, 1, false);
pub const TotWhExp: Point<Model201, u32> = Point::new(36, 1, false);
pub const TotWhExpPhA: Point<Model201, u32> = Point::new(38, 1, false);
pub const TotWhExpPhB: Point<Model201, u32> = Point::new(40, 1, false);
pub const TotWhExpPhC: Point<Model201, u32> = Point::new(42, 1, false);
pub const TotWhImp: Point<Model201, u32> = Point::new(44, 1, false);
pub const TotWhImpPhA: Point<Model201, u32> = Point::new(46, 1, false);
pub const TotWhImpPhB: Point<Model201, u32> = Point::new(48, 1, false);
pub const TotWhImpPhC: Point<Model201, u32> = Point::new(50, 1, false);
pub const TotWh_SF: Point<Model201, u16> = Point::new(52, 1, false);
pub const TotVAhExp: Point<Model201, u32> = Point::new(53, 1, false);
pub const TotVAhExpPhA: Point<Model201, u32> = Point::new(55, 1, false);
pub const TotVAhExpPhB: Point<Model201, u32> = Point::new(57, 1, false);
pub const TotVAhExpPhC: Point<Model201, u32> = Point::new(59, 1, false);
pub const TotVAhImp: Point<Model201, u32> = Point::new(61, 1, false);
pub const TotVAhImpPhA: Point<Model201, u32> = Point::new(63, 1, false);
pub const TotVAhImpPhB: Point<Model201, u32> = Point::new(65, 1, false);
pub const TotVAhImpPhC: Point<Model201, u32> = Point::new(67, 1, false);
pub const TotVAh_SF: Point<Model201, u16> = Point::new(69, 1, false);
pub const TotVArhImpQ1: Point<Model201, u32> = Point::new(70, 1, false);
pub const TotVArhImpQ1PhA: Point<Model201, u32> = Point::new(72, 1, false);
pub const TotVArhImpQ1PhB: Point<Model201, u32> = Point::new(74, 1, false);
pub const TotVArhImpQ1PhC: Point<Model201, u32> = Point::new(76, 1, false);
pub const TotVArhImpQ2: Point<Model201, u32> = Point::new(78, 1, false);
pub const TotVArhImpQ2PhA: Point<Model201, u32> = Point::new(80, 1, false);
pub const TotVArhImpQ2PhB: Point<Model201, u32> = Point::new(82, 1, false);
pub const TotVArhImpQ2PhC: Point<Model201, u32> = Point::new(84, 1, false);
pub const TotVArhExpQ3: Point<Model201, u32> = Point::new(86, 1, false);
pub const TotVArhExpQ3PhA: Point<Model201, u32> = Point::new(88, 1, false);
pub const TotVArhExpQ3PhB: Point<Model201, u32> = Point::new(90, 1, false);
pub const TotVArhExpQ3PhC: Point<Model201, u32> = Point::new(92, 1, false);
pub const TotVArhExpQ4: Point<Model201, u32> = Point::new(94, 1, false);
pub const TotVArhExpQ4PhA: Point<Model201, u32> = Point::new(96, 1, false);
pub const TotVArhExpQ4PhB: Point<Model201, u32> = Point::new(98, 1, false);
pub const TotVArhExpQ4PhC: Point<Model201, u32> = Point::new(100, 1, false);
pub const TotVArh_SF: Point<Model201, u16> = Point::new(102, 1, false);
pub const Evt: Point<Model201, u32> = Point::new(103, 1, false);
