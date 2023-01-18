use crate::{set::Set, shared::Legality};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct AncientTrait {
    pub name: String,
    pub text: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Ability {
    pub name: String,
    pub text: String,
    #[serde(rename = "type")]
    pub ability_type: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Attack {
    pub cost: Vec<String>,
    pub name: String,
    pub text: String,
    pub damage: String,
    pub converted_energy_cost: u8,
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Weakness {
    #[serde(rename = "type")]
    pub weakness_type: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Resistance {
    #[serde(rename = "type")]
    pub resistance_type: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct TcgPlayerPriceData {
    pub low: f32,
    pub mid: f32,
    pub high: f32,
    pub market: f32,
    pub direct_low: f32,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct TcgPlayerPrice {
    pub normal: TcgPlayerPriceData,
    pub holofoil: TcgPlayerPriceData,
    #[serde(rename = "1stEditionHolofoil")]
    pub first_edition_holofoil: TcgPlayerPriceData,
    #[serde(rename = "1stEditionNormal")]
    pub first_edition_normal: TcgPlayerPriceData,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct TcgPlayer {
    pub url: String,
    pub updated_at: String,
    pub prices: TcgPlayerPrice,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct CarkMarketPrice {
    pub average_sell_price: f32,
    pub low_price: f32,
    pub trend_price: f32,
    pub german_pro_low: f32,
    pub suggested_price: f32,
    pub reverse_holo_sell: f32,
    pub reverse_holo_low: f32,
    pub reverse_holo_trend: f32,
    pub low_price_ex_plus: f32,
    pub avg1: f32,
    pub avg7: f32,
    pub avg30: f32,
    pub reverse_holo_avg1: f32,
    pub reverse_holo_avg7: f32,
    pub reverse_holo_avg30: f32,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct CardMarket {
    pub url: String,
    pub updated_at: String,
    pub prices: CarkMarketPrice,
}

#[derive(Deserialize, Debug, Default)]
pub struct CardImage {
    pub small: String,
    pub large: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub supertype: String,
    pub subtypes: Vec<String>,
    pub level: String,
    pub hp: String,
    pub types: Vec<String>,
    pub evolves_from: String,
    pub evolves_to: Vec<String>,
    pub rules: Vec<String>,
    pub ancient_trait: AncientTrait,
    pub abilities: Vec<Ability>,
    pub attacks: Vec<Attack>,
    pub weaknesses: Vec<Weakness>,
    pub resistances: Vec<Resistance>,
    pub retreat_cost: Vec<String>,
    pub converted_retreat_cost: u8,
    pub set: Set,
    pub number: String,
    pub artist: String,
    pub rarity: String,
    pub flavor_text: String,
    pub national_pokedex_numbers: Vec<u16>,
    pub legalities: Legality,
    pub regulation_mark: String,
    pub images: CardImage,
    pub tcgplayer: TcgPlayer,
    pub cardmarket: CardMarket,
}
