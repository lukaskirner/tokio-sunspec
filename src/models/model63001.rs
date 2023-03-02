#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model63001 {}

impl Model for Model63001 {
    const ID: u16 = 63001;
    const LENGTH: u16 = 152;
}

pub const sunssf_1: Point<Model63001, u16> = Point::new(0, 1, false);
pub const sunssf_2: Point<Model63001, u16> = Point::new(1, 1, false);
pub const sunssf_3: Point<Model63001, u16> = Point::new(2, 1, false);
pub const sunssf_4: Point<Model63001, u16> = Point::new(3, 1, false);
pub const int16_1: Point<Model63001, i16> = Point::new(4, 1, false);
pub const int16_2: Point<Model63001, i16> = Point::new(5, 1, false);
pub const int16_3: Point<Model63001, i16> = Point::new(6, 1, false);
pub const int16_4: Point<Model63001, i16> = Point::new(7, 1, false);
pub const int16_5: Point<Model63001, i16> = Point::new(8, 1, false);
pub const int16_u: Point<Model63001, i16> = Point::new(9, 1, false);
pub const uint16_1: Point<Model63001, u16> = Point::new(10, 1, false);
pub const uint16_2: Point<Model63001, u16> = Point::new(11, 1, false);
pub const uint16_3: Point<Model63001, u16> = Point::new(12, 1, false);
pub const uint16_4: Point<Model63001, u16> = Point::new(13, 1, false);
pub const uint16_5: Point<Model63001, u16> = Point::new(14, 1, false);
pub const uint16_u: Point<Model63001, u16> = Point::new(15, 1, false);
pub const acc16: Point<Model63001, u16> = Point::new(16, 1, false);
pub const acc16_u: Point<Model63001, u16> = Point::new(17, 1, false);
pub const enum16: Point<Model63001, u16> = Point::new(18, 1, false);
pub const enum16_u: Point<Model63001, u16> = Point::new(19, 1, false);
pub const bitfield16: Point<Model63001, u16> = Point::new(20, 1, false);
pub const bitfield16_u: Point<Model63001, u16> = Point::new(21, 1, false);
pub const int32_1: Point<Model63001, i32> = Point::new(22, 1, false);
pub const int32_2: Point<Model63001, i32> = Point::new(24, 1, false);
pub const int32_3: Point<Model63001, i32> = Point::new(26, 1, false);
pub const int32_4: Point<Model63001, i32> = Point::new(28, 1, false);
pub const int32_5: Point<Model63001, i32> = Point::new(30, 1, false);
pub const int32_u: Point<Model63001, i32> = Point::new(32, 1, false);
pub const uint32_1: Point<Model63001, u32> = Point::new(34, 1, false);
pub const uint32_2: Point<Model63001, u32> = Point::new(36, 1, false);
pub const uint32_3: Point<Model63001, u32> = Point::new(38, 1, false);
pub const uint32_4: Point<Model63001, u32> = Point::new(40, 1, false);
pub const uint32_5: Point<Model63001, u32> = Point::new(42, 1, false);
pub const uint32_u: Point<Model63001, u32> = Point::new(44, 1, false);
pub const acc32: Point<Model63001, u32> = Point::new(46, 1, false);
pub const acc32_u: Point<Model63001, u32> = Point::new(48, 1, false);
pub const enum32: Point<Model63001, u32> = Point::new(50, 1, false);
pub const enum32_u: Point<Model63001, u32> = Point::new(52, 1, false);
pub const bitfield32: Point<Model63001, u32> = Point::new(54, 1, false);
pub const bitfield32_u: Point<Model63001, u32> = Point::new(56, 1, false);
pub const ipaddr: Point<Model63001, u32> = Point::new(58, 1, false);
pub const ipaddr_u: Point<Model63001, u32> = Point::new(60, 1, false);
pub const int64: Point<Model63001, i64> = Point::new(62, 1, false);
pub const int64_u: Point<Model63001, i64> = Point::new(66, 1, false);
pub const acc64: Point<Model63001, u64> = Point::new(70, 1, false);
pub const acc64_u: Point<Model63001, u64> = Point::new(74, 1, false);
pub const ipv6addr: Point<Model63001, u128> = Point::new(78, 1, false);
pub const ipv6addr_u: Point<Model63001, u128> = Point::new(86, 1, false);
pub const float32: Point<Model63001, f32> = Point::new(94, 1, false);
pub const float32_u: Point<Model63001, f32> = Point::new(96, 1, false);
pub const string: Point<Model63001, String> = Point::new(98, 16, false);
pub const string_u: Point<Model63001, String> = Point::new(114, 16, false);
pub const sunssf_5: Point<Model63001, u16> = Point::new(130, 1, false);
pub const sunssf_6: Point<Model63001, u16> = Point::new(131, 1, false);
pub const sunssf_7: Point<Model63001, u16> = Point::new(132, 1, false);
pub const pad_1: Point<Model63001, u16> = Point::new(133, 1, false);
