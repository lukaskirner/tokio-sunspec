#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model212 {}

impl Model for Model212 {
    const ID: u16 = 212;
    const LENGTH: u16 = 124;
}

pub const A: Point<Model212, f32> = Point::new(0, 1, false);
pub const AphA: Point<Model212, f32> = Point::new(2, 1, false);
pub const AphB: Point<Model212, f32> = Point::new(4, 1, false);
pub const AphC: Point<Model212, f32> = Point::new(6, 1, false);
pub const PhV: Point<Model212, f32> = Point::new(8, 1, false);
pub const PhVphA: Point<Model212, f32> = Point::new(10, 1, false);
pub const PhVphB: Point<Model212, f32> = Point::new(12, 1, false);
pub const PhVphC: Point<Model212, f32> = Point::new(14, 1, false);
pub const PPV: Point<Model212, f32> = Point::new(16, 1, false);
pub const PPVphAB: Point<Model212, f32> = Point::new(18, 1, false);
pub const PPVphBC: Point<Model212, f32> = Point::new(20, 1, false);
pub const PPVphCA: Point<Model212, f32> = Point::new(22, 1, false);
pub const Hz: Point<Model212, f32> = Point::new(24, 1, false);
pub const W: Point<Model212, f32> = Point::new(26, 1, false);
pub const WphA: Point<Model212, f32> = Point::new(28, 1, false);
pub const WphB: Point<Model212, f32> = Point::new(30, 1, false);
pub const WphC: Point<Model212, f32> = Point::new(32, 1, false);
pub const VA: Point<Model212, f32> = Point::new(34, 1, false);
pub const VAphA: Point<Model212, f32> = Point::new(36, 1, false);
pub const VAphB: Point<Model212, f32> = Point::new(38, 1, false);
pub const VAphC: Point<Model212, f32> = Point::new(40, 1, false);
pub const VAR: Point<Model212, f32> = Point::new(42, 1, false);
pub const VARphA: Point<Model212, f32> = Point::new(44, 1, false);
pub const VARphB: Point<Model212, f32> = Point::new(46, 1, false);
pub const VARphC: Point<Model212, f32> = Point::new(48, 1, false);
pub const PF: Point<Model212, f32> = Point::new(50, 1, false);
pub const PFphA: Point<Model212, f32> = Point::new(52, 1, false);
pub const PFphB: Point<Model212, f32> = Point::new(54, 1, false);
pub const PFphC: Point<Model212, f32> = Point::new(56, 1, false);
pub const TotWhExp: Point<Model212, f32> = Point::new(58, 1, false);
pub const TotWhExpPhA: Point<Model212, f32> = Point::new(60, 1, false);
pub const TotWhExpPhB: Point<Model212, f32> = Point::new(62, 1, false);
pub const TotWhExpPhC: Point<Model212, f32> = Point::new(64, 1, false);
pub const TotWhImp: Point<Model212, f32> = Point::new(66, 1, false);
pub const TotWhImpPhA: Point<Model212, f32> = Point::new(68, 1, false);
pub const TotWhImpPhB: Point<Model212, f32> = Point::new(70, 1, false);
pub const TotWhImpPhC: Point<Model212, f32> = Point::new(72, 1, false);
pub const TotVAhExp: Point<Model212, f32> = Point::new(74, 1, false);
pub const TotVAhExpPhA: Point<Model212, f32> = Point::new(76, 1, false);
pub const TotVAhExpPhB: Point<Model212, f32> = Point::new(78, 1, false);
pub const TotVAhExpPhC: Point<Model212, f32> = Point::new(80, 1, false);
pub const TotVAhImp: Point<Model212, f32> = Point::new(82, 1, false);
pub const TotVAhImpPhA: Point<Model212, f32> = Point::new(84, 1, false);
pub const TotVAhImpPhB: Point<Model212, f32> = Point::new(86, 1, false);
pub const TotVAhImpPhC: Point<Model212, f32> = Point::new(88, 1, false);
pub const TotVArhImpQ1: Point<Model212, f32> = Point::new(90, 1, false);
pub const TotVArhImpQ1phA: Point<Model212, f32> = Point::new(92, 1, false);
pub const TotVArhImpQ1phB: Point<Model212, f32> = Point::new(94, 1, false);
pub const TotVArhImpQ1phC: Point<Model212, f32> = Point::new(96, 1, false);
pub const TotVArhImpQ2: Point<Model212, f32> = Point::new(98, 1, false);
pub const TotVArhImpQ2phA: Point<Model212, f32> = Point::new(100, 1, false);
pub const TotVArhImpQ2phB: Point<Model212, f32> = Point::new(102, 1, false);
pub const TotVArhImpQ2phC: Point<Model212, f32> = Point::new(104, 1, false);
pub const TotVArhExpQ3: Point<Model212, f32> = Point::new(106, 1, false);
pub const TotVArhExpQ3phA: Point<Model212, f32> = Point::new(108, 1, false);
pub const TotVArhExpQ3phB: Point<Model212, f32> = Point::new(110, 1, false);
pub const TotVArhExpQ3phC: Point<Model212, f32> = Point::new(112, 1, false);
pub const TotVArhExpQ4: Point<Model212, f32> = Point::new(114, 1, false);
pub const TotVArhExpQ4phA: Point<Model212, f32> = Point::new(116, 1, false);
pub const TotVArhExpQ4phB: Point<Model212, f32> = Point::new(118, 1, false);
pub const TotVArhExpQ4phC: Point<Model212, f32> = Point::new(120, 1, false);
pub const Evt: Point<Model212, u32> = Point::new(122, 1, false);
