pub fn apply_scale_factor(value: u16, scale_factor: u16) -> u16 {
    return value * u16::pow(10, scale_factor as u32);
}
