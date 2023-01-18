use crate::shared::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct SetImage {
    pub symbol: String,
    pub logo: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Set {
    pub id: String,
    pub name: String,
    pub series: String,
    pub printed_total: u16,
    pub total: u16,
    pub legalities: Legality,
    pub ptcgo_code: String,
    pub release_date: String,
    pub updated_at: String,
    pub images: SetImage,
}
