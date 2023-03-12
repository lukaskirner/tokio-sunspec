use crate::{
    model::Model,
    utils::{to_be_bytes, to_u16_vector},
};
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

pub trait PointType<T> {
    fn decode(data: Vec<u16>) -> T;
    fn encode(data: T) -> Vec<u16>;
}

impl PointType<String> for String {
    fn decode(data: Vec<u16>) -> String {
        let bytes: Vec<u8> = to_be_bytes(data).try_into().unwrap();
        let fbytes: Vec<u8> = bytes.iter().filter(|b| **b != 0).map(|b| *b).collect();
        return String::from_utf8(fbytes).unwrap();
    }

    fn encode(data: String) -> Vec<u16> {
        return to_u16_vector(data.as_bytes());
    }
}

impl PointType<i16> for i16 {
    fn decode(data: Vec<u16>) -> i16 {
        return data[0] as i16;
    }

    fn encode(data: i16) -> Vec<u16> {
        return vec![data as u16];
    }
}

impl PointType<i32> for i32 {
    fn decode(data: Vec<u16>) -> i32 {
        let bytes = to_be_bytes(data).try_into().unwrap();
        return i32::from_be_bytes(bytes);
    }

    fn encode(data: i32) -> Vec<u16> {
        return to_u16_vector(&data.to_be_bytes());
    }
}

impl PointType<i64> for i64 {
    fn decode(data: Vec<u16>) -> i64 {
        let bytes = to_be_bytes(data).try_into().unwrap();
        return i64::from_be_bytes(bytes);
    }

    fn encode(data: i64) -> Vec<u16> {
        return to_u16_vector(&data.to_be_bytes());
    }
}

impl PointType<u16> for u16 {
    fn decode(data: Vec<u16>) -> u16 {
        return data[0];
    }

    fn encode(data: u16) -> Vec<u16> {
        return vec![data];
    }
}

impl PointType<u32> for u32 {
    fn decode(data: Vec<u16>) -> u32 {
        let bytes = to_be_bytes(data).try_into().unwrap();
        return u32::from_be_bytes(bytes);
    }

    fn encode(data: u32) -> Vec<u16> {
        return to_u16_vector(&data.to_be_bytes());
    }
}

impl PointType<u64> for u64 {
    fn decode(data: Vec<u16>) -> u64 {
        let bytes = to_be_bytes(data).try_into().unwrap();
        return u64::from_be_bytes(bytes);
    }

    fn encode(data: u64) -> Vec<u16> {
        return to_u16_vector(&data.to_be_bytes());
    }
}

impl PointType<u128> for u128 {
    fn decode(data: Vec<u16>) -> u128 {
        let bytes = to_be_bytes(data).try_into().unwrap();
        return u128::from_be_bytes(bytes);
    }

    fn encode(data: u128) -> Vec<u16> {
        return to_u16_vector(&data.to_be_bytes());
    }
}

impl PointType<f32> for f32 {
    fn decode(data: Vec<u16>) -> f32 {
        let bytes = to_be_bytes(data).try_into().unwrap();
        return f32::from_be_bytes(bytes);
    }

    fn encode(data: f32) -> Vec<u16> {
        return to_u16_vector(&data.to_be_bytes());
    }
}
