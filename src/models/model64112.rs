#![allow(non_upper_case_globals)]
use crate::model::Model;
use crate::point::Point;

pub struct Model64112 {}

impl Model for Model64112 {
    const ID: u16 = 64112;
    const LENGTH: u16 = 64;
}

pub const Port: Point<Model64112, u16> = Point::new(0, 1, false);
pub const V_SF: Point<Model64112, u16> = Point::new(1, 1, false);
pub const C_SF: Point<Model64112, u16> = Point::new(2, 1, false);
pub const H_SF: Point<Model64112, u16> = Point::new(3, 1, false);
pub const P_SF: Point<Model64112, u16> = Point::new(4, 1, false);
pub const AH_SF: Point<Model64112, u16> = Point::new(5, 1, false);
pub const KWH_SF: Point<Model64112, u16> = Point::new(6, 1, false);
pub const CC_Config_fault: Point<Model64112, u16> = Point::new(7, 1, false);
pub const CC_Config_absorb_V: Point<Model64112, u16> = Point::new(8, 1, false);
pub const CC_Config_absorb_Hr: Point<Model64112, u16> = Point::new(9, 1, false);
pub const CC_Config_absorb_End_A: Point<Model64112, u16> = Point::new(10, 1, false);
pub const CC_Config_rebulk_V: Point<Model64112, u16> = Point::new(11, 1, false);
pub const CC_Config_float_V: Point<Model64112, u16> = Point::new(12, 1, false);
pub const CC_Config_max_Chg_A: Point<Model64112, u16> = Point::new(13, 1, false);
pub const CC_Config_equalize_V: Point<Model64112, u16> = Point::new(14, 1, false);
pub const CC_Config_equalize_Hr: Point<Model64112, u16> = Point::new(15, 1, false);
pub const CC_Config_auto_equalize: Point<Model64112, u16> = Point::new(16, 1, false);
pub const CC_Config_MPPT_mode: Point<Model64112, u16> = Point::new(17, 1, false);
pub const CC_Config_sweep_width: Point<Model64112, u16> = Point::new(18, 1, false);
pub const CC_Config_sweep_max: Point<Model64112, u16> = Point::new(19, 1, false);
pub const CC_Config_U_Pick_Duty_cyc: Point<Model64112, u16> = Point::new(20, 1, false);
pub const CC_Config_grid_tie: Point<Model64112, u16> = Point::new(21, 1, false);
pub const CC_Config_temp_comp: Point<Model64112, u16> = Point::new(22, 1, false);
pub const CC_Config_temp_comp_llimt: Point<Model64112, u16> = Point::new(23, 1, false);
pub const CC_Config_temp_comp_hlimt: Point<Model64112, u16> = Point::new(24, 1, false);
pub const CC_Config_auto_restart: Point<Model64112, u16> = Point::new(25, 1, false);
pub const CC_Config_wakeup_VOC: Point<Model64112, u16> = Point::new(26, 1, false);
pub const CC_Config_snooze_mode_A: Point<Model64112, u16> = Point::new(27, 1, false);
pub const CC_Config_wakeup_interval: Point<Model64112, u16> = Point::new(28, 1, false);
pub const CC_Config_AUX_mode: Point<Model64112, u16> = Point::new(29, 1, false);
pub const CC_Config_AUX_control: Point<Model64112, u16> = Point::new(30, 1, false);
pub const CC_Config_AUX_state: Point<Model64112, u16> = Point::new(31, 1, false);
pub const CC_Config_AUX_polarity: Point<Model64112, u16> = Point::new(32, 1, false);
pub const CC_Config_AUX_L_Batt_disc: Point<Model64112, u16> = Point::new(33, 1, false);
pub const CC_Config_AUX_L_Batt_rcon: Point<Model64112, u16> = Point::new(34, 1, false);
pub const CC_Config_AUX_L_Batt_dly: Point<Model64112, u16> = Point::new(35, 1, false);
pub const CC_Config_AUX_Vent_fan_V: Point<Model64112, u16> = Point::new(36, 1, false);
pub const CC_Config_AUX_PV_triggerV: Point<Model64112, u16> = Point::new(37, 1, false);
pub const CC_Config_AUX_PV_trg_h_tm: Point<Model64112, u16> = Point::new(38, 1, false);
pub const CC_Config_AUX_Nlite_ThrsV: Point<Model64112, u16> = Point::new(39, 1, false);
pub const CC_Config_AUX_Nlite_On_tm: Point<Model64112, u16> = Point::new(40, 1, false);
pub const CC_Config_AUX_Nlite_On_hist: Point<Model64112, u16> = Point::new(41, 1, false);
pub const CC_Config_AUX_Nlite_Off_hist: Point<Model64112, u16> = Point::new(42, 1, false);
pub const CC_Config_AUX_Error_batt_V: Point<Model64112, u16> = Point::new(43, 1, false);
pub const CC_Config_AUX_Divert_h_time: Point<Model64112, u16> = Point::new(44, 1, false);
pub const CC_Config_AUX_Divert_dly_time: Point<Model64112, u16> = Point::new(45, 1, false);
pub const CC_Config_AUX_Divert_Rel_V: Point<Model64112, u16> = Point::new(46, 1, false);
pub const CC_Config_AUX_Divert_Hyst_V: Point<Model64112, u16> = Point::new(47, 1, false);
pub const CC_Config_MajorFWRev: Point<Model64112, u16> = Point::new(48, 1, false);
pub const CC_Config_MidFWRev: Point<Model64112, u16> = Point::new(49, 1, false);
pub const CC_Config_MinorFWRev: Point<Model64112, u16> = Point::new(50, 1, false);
pub const CC_Config_DataLog_Day_offset: Point<Model64112, u16> = Point::new(51, 1, false);
pub const CC_Config_DataLog_Cur_Day_off: Point<Model64112, u16> = Point::new(52, 1, false);
pub const CC_Config_DataLog_Daily_AH: Point<Model64112, u16> = Point::new(53, 1, false);
pub const CC_Config_DataLog_Daily_KWH: Point<Model64112, u16> = Point::new(54, 1, false);
pub const CC_Config_DataLog_Max_Out_A: Point<Model64112, u16> = Point::new(55, 1, false);
pub const CC_Config_DataLog_Max_Out_W: Point<Model64112, u16> = Point::new(56, 1, false);
pub const CC_Config_DataLog_Absorb_T: Point<Model64112, u16> = Point::new(57, 1, false);
pub const CC_Config_DataLog_Float_T: Point<Model64112, u16> = Point::new(58, 1, false);
pub const CC_Config_DataLog_Min_Batt_V: Point<Model64112, u16> = Point::new(59, 1, false);
pub const CC_Config_DataLog_Max_Batt_V: Point<Model64112, u16> = Point::new(60, 1, false);
pub const CC_Config_DataLog_Max_Input_V: Point<Model64112, u16> = Point::new(61, 1, false);
pub const CC_Config_DataLog_Clear: Point<Model64112, u16> = Point::new(62, 1, false);
pub const CC_Config_DataLog_Clr_Comp: Point<Model64112, u16> = Point::new(63, 1, false);
