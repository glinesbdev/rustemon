use serde::Deserialize;

pub mod errors;

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Single<T> {
    pub data: T,
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Many<T> {
    pub data: Vec<T>,
    pub page: u16,
    pub page_size: u8,
    pub count: u32,
    pub total_count: u32,
}
