use crate::shared::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SetImage {
    pub symbol: String,
    pub logo: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct SetData {
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
