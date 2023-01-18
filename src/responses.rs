use crate::{card::Card, set::Set};
use serde::Deserialize;

pub mod errors;

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct CardResponse {
    pub data: Option<Card>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct SetResponse {
    pub data: Option<Set>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct CardsResponse {
    pub data: Vec<Card>,
    pub page: u16,
    pub page_size: u8,
    pub count: u32,
    pub total_count: u32,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct SetsResponse {
    pub data: Vec<Set>,
    pub page: u16,
    pub page_size: u8,
    pub count: u32,
    pub total_count: u32,
}
