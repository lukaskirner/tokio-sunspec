#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model213 {}

impl Model for Model213 {
    const ID: u16 = 213;
    const LENGTH: u16 = 124;
}

pub const A: Point<Model213, f32> = Point::new(0, 2, false);
pub const AphA: Point<Model213, f32> = Point::new(2, 2, false);
pub const AphB: Point<Model213, f32> = Point::new(4, 2, false);
pub const AphC: Point<Model213, f32> = Point::new(6, 2, false);
pub const PhV: Point<Model213, f32> = Point::new(8, 2, false);
pub const PhVphA: Point<Model213, f32> = Point::new(10, 2, false);
pub const PhVphB: Point<Model213, f32> = Point::new(12, 2, false);
pub const PhVphC: Point<Model213, f32> = Point::new(14, 2, false);
pub const PPV: Point<Model213, f32> = Point::new(16, 2, false);
pub const PPVphAB: Point<Model213, f32> = Point::new(18, 2, false);
pub const PPVphBC: Point<Model213, f32> = Point::new(20, 2, false);
pub const PPVphCA: Point<Model213, f32> = Point::new(22, 2, false);
pub const Hz: Point<Model213, f32> = Point::new(24, 2, false);
pub const W: Point<Model213, f32> = Point::new(26, 2, false);
pub const WphA: Point<Model213, f32> = Point::new(28, 2, false);
pub const WphB: Point<Model213, f32> = Point::new(30, 2, false);
pub const WphC: Point<Model213, f32> = Point::new(32, 2, false);
pub const VA: Point<Model213, f32> = Point::new(34, 2, false);
pub const VAphA: Point<Model213, f32> = Point::new(36, 2, false);
pub const VAphB: Point<Model213, f32> = Point::new(38, 2, false);
pub const VAphC: Point<Model213, f32> = Point::new(40, 2, false);
pub const VAR: Point<Model213, f32> = Point::new(42, 2, false);
pub const VARphA: Point<Model213, f32> = Point::new(44, 2, false);
pub const VARphB: Point<Model213, f32> = Point::new(46, 2, false);
pub const VARphC: Point<Model213, f32> = Point::new(48, 2, false);
pub const PF: Point<Model213, f32> = Point::new(50, 2, false);
pub const PFphA: Point<Model213, f32> = Point::new(52, 2, false);
pub const PFphB: Point<Model213, f32> = Point::new(54, 2, false);
pub const PFphC: Point<Model213, f32> = Point::new(56, 2, false);
pub const TotWhExp: Point<Model213, f32> = Point::new(58, 2, false);
pub const TotWhExpPhA: Point<Model213, f32> = Point::new(60, 2, false);
pub const TotWhExpPhB: Point<Model213, f32> = Point::new(62, 2, false);
pub const TotWhExpPhC: Point<Model213, f32> = Point::new(64, 2, false);
pub const TotWhImp: Point<Model213, f32> = Point::new(66, 2, false);
pub const TotWhImpPhA: Point<Model213, f32> = Point::new(68, 2, false);
pub const TotWhImpPhB: Point<Model213, f32> = Point::new(70, 2, false);
pub const TotWhImpPhC: Point<Model213, f32> = Point::new(72, 2, false);
pub const TotVAhExp: Point<Model213, f32> = Point::new(74, 2, false);
pub const TotVAhExpPhA: Point<Model213, f32> = Point::new(76, 2, false);
pub const TotVAhExpPhB: Point<Model213, f32> = Point::new(78, 2, false);
pub const TotVAhExpPhC: Point<Model213, f32> = Point::new(80, 2, false);
pub const TotVAhImp: Point<Model213, f32> = Point::new(82, 2, false);
pub const TotVAhImpPhA: Point<Model213, f32> = Point::new(84, 2, false);
pub const TotVAhImpPhB: Point<Model213, f32> = Point::new(86, 2, false);
pub const TotVAhImpPhC: Point<Model213, f32> = Point::new(88, 2, false);
pub const TotVArhImpQ1: Point<Model213, f32> = Point::new(90, 2, false);
pub const TotVArhImpQ1phA: Point<Model213, f32> = Point::new(92, 2, false);
pub const TotVArhImpQ1phB: Point<Model213, f32> = Point::new(94, 2, false);
pub const TotVArhImpQ1phC: Point<Model213, f32> = Point::new(96, 2, false);
pub const TotVArhImpQ2: Point<Model213, f32> = Point::new(98, 2, false);
pub const TotVArhImpQ2phA: Point<Model213, f32> = Point::new(100, 2, false);
pub const TotVArhImpQ2phB: Point<Model213, f32> = Point::new(102, 2, false);
pub const TotVArhImpQ2phC: Point<Model213, f32> = Point::new(104, 2, false);
pub const TotVArhExpQ3: Point<Model213, f32> = Point::new(106, 2, false);
pub const TotVArhExpQ3phA: Point<Model213, f32> = Point::new(108, 2, false);
pub const TotVArhExpQ3phB: Point<Model213, f32> = Point::new(110, 2, false);
pub const TotVArhExpQ3phC: Point<Model213, f32> = Point::new(112, 2, false);
pub const TotVArhExpQ4: Point<Model213, f32> = Point::new(114, 2, false);
pub const TotVArhExpQ4phA: Point<Model213, f32> = Point::new(116, 2, false);
pub const TotVArhExpQ4phB: Point<Model213, f32> = Point::new(118, 2, false);
pub const TotVArhExpQ4phC: Point<Model213, f32> = Point::new(120, 2, false);
pub const Evt: Point<Model213, u32> = Point::new(122, 1, false);
