#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model64111 {}

impl Model for Model64111 {
    const ID: u16 = 64111;
    const LENGTH: u16 = 23;
}

pub const Port: Point<Model64111, u16> = Point::new(0, 1, false);
pub const V_SF: Point<Model64111, u16> = Point::new(1, 1, false);
pub const A_SF: Point<Model64111, u16> = Point::new(2, 1, false);
pub const P_SF: Point<Model64111, u16> = Point::new(3, 1, false);
pub const AH_SF: Point<Model64111, u16> = Point::new(4, 1, false);
pub const KWH_SF: Point<Model64111, u16> = Point::new(5, 1, false);
pub const BattV: Point<Model64111, u16> = Point::new(6, 1, false);
pub const ArrayV: Point<Model64111, u16> = Point::new(7, 1, false);
pub const OutputA: Point<Model64111, u16> = Point::new(8, 1, false);
pub const InputA: Point<Model64111, u16> = Point::new(9, 1, false);
pub const ChargerSt: Point<Model64111, u16> = Point::new(10, 1, false);
pub const OutputW: Point<Model64111, u16> = Point::new(11, 1, false);
pub const TodayMinBatV: Point<Model64111, u16> = Point::new(12, 1, false);
pub const TodayMaxBatV: Point<Model64111, u16> = Point::new(13, 1, false);
pub const VOCV: Point<Model64111, u16> = Point::new(14, 1, false);
pub const TodayMaxVOC: Point<Model64111, u16> = Point::new(15, 1, false);
pub const TodaykWhOutput: Point<Model64111, u16> = Point::new(16, 1, false);
pub const TodayAHOutput: Point<Model64111, u16> = Point::new(17, 1, false);
pub const LifeTimeKWHOut: Point<Model64111, u16> = Point::new(18, 1, false);
pub const LifeTimeAHOut: Point<Model64111, u16> = Point::new(19, 1, false);
pub const LifeTimeMaxOut: Point<Model64111, u16> = Point::new(20, 1, false);
pub const LifeTimeMaxBatt: Point<Model64111, u16> = Point::new(21, 1, false);
pub const LifeTimeMaxVOC: Point<Model64111, u16> = Point::new(22, 1, false);
