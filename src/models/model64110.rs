#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model64110 {}

impl Model for Model64110 {
    const ID: u16 = 64110;
    const LENGTH: u16 = 282;
}

pub const MajorFWRev: Point<Model64110, u16> = Point::new(0, 1, false);
pub const MidFWRev: Point<Model64110, u16> = Point::new(1, 1, false);
pub const MinorFWRev: Point<Model64110, u16> = Point::new(2, 1, false);
pub const EncrypKey: Point<Model64110, u16> = Point::new(3, 1, false);
pub const MAC_Address: Point<Model64110, String> = Point::new(4, 7, false);
pub const WritePassword: Point<Model64110, String> = Point::new(11, 8, false);
pub const EnableDHCP: Point<Model64110, u16> = Point::new(19, 1, false);
pub const TCPIP_address: Point<Model64110, u32> = Point::new(20, 1, false);
pub const Gateway_address: Point<Model64110, u32> = Point::new(22, 1, false);
pub const TCPIP_Netmask: Point<Model64110, u32> = Point::new(24, 1, false);
pub const DNS1_address: Point<Model64110, u32> = Point::new(26, 1, false);
pub const DNS2_address: Point<Model64110, u32> = Point::new(28, 1, false);
pub const Modbus_port: Point<Model64110, u16> = Point::new(30, 1, false);
pub const SMTP_server_nm: Point<Model64110, String> = Point::new(31, 20, false);
pub const SMTP_account_nm: Point<Model64110, String> = Point::new(51, 16, false);
pub const SMTP_enable_SSL: Point<Model64110, u16> = Point::new(67, 1, false);
pub const SMTP_password: Point<Model64110, String> = Point::new(68, 8, false);
pub const SMTP_user_nm: Point<Model64110, String> = Point::new(76, 20, false);
pub const Stat_email_int: Point<Model64110, u16> = Point::new(96, 1, false);
pub const Stat_start_HR: Point<Model64110, u16> = Point::new(97, 1, false);
pub const Stat_email_sub: Point<Model64110, String> = Point::new(98, 25, false);
pub const Stat_email_addr1: Point<Model64110, String> = Point::new(123, 20, false);
pub const Stat_email_addr2: Point<Model64110, String> = Point::new(143, 20, false);
pub const Alarm_email_en: Point<Model64110, u16> = Point::new(163, 1, false);
pub const Alarm_email_sub: Point<Model64110, String> = Point::new(164, 25, false);
pub const Alarm_email_addr1: Point<Model64110, String> = Point::new(189, 20, false);
pub const Alarm_email_addr2: Point<Model64110, String> = Point::new(209, 20, false);
pub const FTP_password: Point<Model64110, String> = Point::new(229, 8, false);
pub const TELNET_password: Point<Model64110, String> = Point::new(237, 8, false);
pub const Log_write_int: Point<Model64110, u16> = Point::new(245, 1, false);
pub const Log_retain: Point<Model64110, u16> = Point::new(246, 1, false);
pub const Log_mode: Point<Model64110, u16> = Point::new(247, 1, false);
pub const NTP_server_nm: Point<Model64110, String> = Point::new(248, 20, false);
pub const NTP_enable: Point<Model64110, u16> = Point::new(268, 1, false);
pub const TimeZone: Point<Model64110, i16> = Point::new(269, 1, false);
pub const Date_year: Point<Model64110, u16> = Point::new(270, 1, false);
pub const Date_month: Point<Model64110, u16> = Point::new(271, 1, false);
pub const Date_Day: Point<Model64110, u16> = Point::new(272, 1, false);
pub const Time_hour: Point<Model64110, u16> = Point::new(273, 1, false);
pub const Time_minute: Point<Model64110, u16> = Point::new(274, 1, false);
pub const Time_second: Point<Model64110, u16> = Point::new(275, 1, false);
pub const Battery_temp: Point<Model64110, i16> = Point::new(276, 1, false);
pub const Ambient_temp: Point<Model64110, i16> = Point::new(277, 1, false);
pub const Temp_SF: Point<Model64110, u16> = Point::new(278, 1, false);
pub const AXS_Error: Point<Model64110, u16> = Point::new(279, 1, false);
pub const AXS_Status: Point<Model64110, u16> = Point::new(280, 1, false);
pub const AXS_Spare: Point<Model64110, u16> = Point::new(281, 1, false);
