#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model64020 {}

impl Model for Model64020 {
    const ID: u16 = 64020;
    const LENGTH: u16 = 46;
}

pub const Aux0Tmp: Point<Model64020, i16> = Point::new(0, 1, false);
pub const Aux1Tmp: Point<Model64020, i16> = Point::new(1, 1, false);
pub const Aux2Tmp: Point<Model64020, i16> = Point::new(2, 1, false);
pub const Aux3Tmp: Point<Model64020, i16> = Point::new(3, 1, false);
pub const Aux4Tmp: Point<Model64020, i16> = Point::new(4, 1, false);
pub const ProbeTmp: Point<Model64020, i16> = Point::new(6, 1, false);
pub const MainTmp: Point<Model64020, i16> = Point::new(5, 1, false);
pub const SensorV_SF: Point<Model64020, u16> = Point::new(7, 1, false);
pub const SensorA_SF: Point<Model64020, u16> = Point::new(8, 1, false);
pub const SensorHz_SF: Point<Model64020, u16> = Point::new(9, 1, false);
pub const Sensor1Voltage: Point<Model64020, i16> = Point::new(10, 1, false);
pub const Sensor2Voltage: Point<Model64020, i16> = Point::new(11, 1, false);
pub const Sensor3Voltage: Point<Model64020, i16> = Point::new(12, 1, false);
pub const Sensor4Voltage: Point<Model64020, i16> = Point::new(13, 1, false);
pub const Sensor5Voltage: Point<Model64020, i16> = Point::new(14, 1, false);
pub const Sensor6Voltage: Point<Model64020, i16> = Point::new(15, 1, false);
pub const Sensor7Voltage: Point<Model64020, i16> = Point::new(16, 1, false);
pub const Sensor1Current: Point<Model64020, i16> = Point::new(17, 1, false);
pub const Sensor2Current: Point<Model64020, i16> = Point::new(18, 1, false);
pub const Sensor3Current: Point<Model64020, i16> = Point::new(19, 1, false);
pub const Sensor4Current: Point<Model64020, i16> = Point::new(20, 1, false);
pub const Sensor5Current: Point<Model64020, i16> = Point::new(21, 1, false);
pub const Sensor6Current: Point<Model64020, i16> = Point::new(22, 1, false);
pub const Sensor7Current: Point<Model64020, i16> = Point::new(23, 1, false);
pub const Sensor8: Point<Model64020, u16> = Point::new(24, 1, false);
pub const Relay1: Point<Model64020, u16> = Point::new(25, 1, false);
pub const Relay2: Point<Model64020, u16> = Point::new(26, 1, false);
pub const Relay3: Point<Model64020, u16> = Point::new(27, 1, false);
pub const ResetAccumulators: Point<Model64020, u16> = Point::new(28, 1, false);
pub const Reset: Point<Model64020, u16> = Point::new(29, 1, false);
