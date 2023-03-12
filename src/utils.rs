pub fn apply_scale_factor(value: u16, scale_factor: u16) -> u16 {
    return value * u16::pow(10, scale_factor as u32);
}

pub(crate) fn to_be_bytes(data: Vec<u16>) -> Vec<u8> {
    return data
        .iter()
        .flat_map(|v| v.to_be_bytes())
        .collect::<Vec<u8>>();
}

pub(crate) fn to_u16_vector(data: &[u8]) -> Vec<u16> {
    return data
        .chunks_exact(2)
        .into_iter()
        .map(|a| u16::from_be_bytes([a[0], a[1]]))
        .collect();
}
