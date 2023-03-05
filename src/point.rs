use crate::model::Model;
use std::marker::PhantomData;

pub struct Point<T: Model, K: PointType<K>> {
    pub(crate) offset: u16,
    pub(crate) length: u16,
    pub write_access: bool,
    model_type: PhantomData<T>,
    value_type: PhantomData<K>,
}

impl<T: Model, K: PointType<K>> Point<T, K> {
    pub const fn new(offset: u16, length: u16, write_access: bool) -> Self {
        Self {
            offset,
            length,
            write_access,
            model_type: PhantomData,
            value_type: PhantomData,
        }
    }
}

fn to_be_bytes(data: Vec<u16>) -> Vec<u8> {
    return data
        .iter()
        .flat_map(|v| v.to_be_bytes())
        .collect::<Vec<u8>>();
}

pub trait PointType<T> {
    fn convert_to(data: Vec<u16>) -> T; // TODO: use Result<T, Error>
}

impl PointType<String> for String {
    fn convert_to(data: Vec<u16>) -> String {
        let bytes: Vec<u8> = to_be_bytes(data).try_into().unwrap();
        let fbytes: Vec<u8> = bytes.iter().filter(|b| **b != 0).map(|b| *b).collect();
        return String::from_utf8(fbytes).unwrap();
    }
}

impl PointType<i16> for i16 {
    fn convert_to(data: Vec<u16>) -> i16 {
        return data[0] as i16;
    }
}

impl PointType<i32> for i32 {
    fn convert_to(data: Vec<u16>) -> i32 {
        let bytes = to_be_bytes(data).try_into().unwrap();
        return i32::from_be_bytes(bytes);
    }
}

impl PointType<i64> for i64 {
    fn convert_to(data: Vec<u16>) -> i64 {
        let bytes = to_be_bytes(data).try_into().unwrap();
        return i64::from_be_bytes(bytes);
    }
}

impl PointType<u16> for u16 {
    fn convert_to(data: Vec<u16>) -> u16 {
        return data[0];
    }
}

impl PointType<u32> for u32 {
    fn convert_to(data: Vec<u16>) -> u32 {
        let bytes = to_be_bytes(data).try_into().unwrap();
        return u32::from_be_bytes(bytes);
    }
}

impl PointType<u64> for u64 {
    fn convert_to(data: Vec<u16>) -> u64 {
        let bytes = to_be_bytes(data).try_into().unwrap();
        return u64::from_be_bytes(bytes);
    }
}

impl PointType<u128> for u128 {
    fn convert_to(data: Vec<u16>) -> u128 {
        let bytes = to_be_bytes(data).try_into().unwrap();
        return u128::from_be_bytes(bytes);
    }
}

impl PointType<f32> for f32 {
    fn convert_to(data: Vec<u16>) -> f32 {
        let bytes = to_be_bytes(data).try_into().unwrap();
        return f32::from_be_bytes(bytes);
    }
}
