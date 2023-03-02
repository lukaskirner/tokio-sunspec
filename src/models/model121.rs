#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model121 {}

impl Model for Model121 {
    const ID: u16 = 121;
    const LENGTH: u16 = 30;
}

pub const WMax: Point<Model121, u16> = Point::new(0, 1, false);
pub const VRef: Point<Model121, u16> = Point::new(1, 1, false);
pub const VRefOfs: Point<Model121, i16> = Point::new(2, 1, false);
pub const VMax: Point<Model121, u16> = Point::new(3, 1, false);
pub const VMin: Point<Model121, u16> = Point::new(4, 1, false);
pub const VAMax: Point<Model121, u16> = Point::new(5, 1, false);
pub const VArMaxQ1: Point<Model121, i16> = Point::new(6, 1, false);
pub const VArMaxQ2: Point<Model121, i16> = Point::new(7, 1, false);
pub const VArMaxQ3: Point<Model121, i16> = Point::new(8, 1, false);
pub const VArMaxQ4: Point<Model121, i16> = Point::new(9, 1, false);
pub const WGra: Point<Model121, u16> = Point::new(10, 1, false);
pub const PFMinQ1: Point<Model121, i16> = Point::new(11, 1, false);
pub const PFMinQ2: Point<Model121, i16> = Point::new(12, 1, false);
pub const PFMinQ3: Point<Model121, i16> = Point::new(13, 1, false);
pub const PFMinQ4: Point<Model121, i16> = Point::new(14, 1, false);
pub const VArAct: Point<Model121, u16> = Point::new(15, 1, false);
pub const ClcTotVA: Point<Model121, u16> = Point::new(16, 1, false);
pub const MaxRmpRte: Point<Model121, u16> = Point::new(17, 1, false);
pub const ECPNomHz: Point<Model121, u16> = Point::new(18, 1, false);
pub const ConnPh: Point<Model121, u16> = Point::new(19, 1, false);
pub const WMax_SF: Point<Model121, u16> = Point::new(20, 1, false);
pub const VRef_SF: Point<Model121, u16> = Point::new(21, 1, false);
pub const VRefOfs_SF: Point<Model121, u16> = Point::new(22, 1, false);
pub const VMinMax_SF: Point<Model121, u16> = Point::new(23, 1, false);
pub const VAMax_SF: Point<Model121, u16> = Point::new(24, 1, false);
pub const VArMax_SF: Point<Model121, u16> = Point::new(25, 1, false);
pub const WGra_SF: Point<Model121, u16> = Point::new(26, 1, false);
pub const PFMin_SF: Point<Model121, u16> = Point::new(27, 1, false);
pub const MaxRmpRte_SF: Point<Model121, u16> = Point::new(28, 1, false);
pub const ECPNomHz_SF: Point<Model121, u16> = Point::new(29, 1, false);
