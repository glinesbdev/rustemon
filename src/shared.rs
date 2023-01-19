use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(default)]
pub struct Legality {
    pub standard: String,
    pub expanded: String,
    pub unlimited: String,
}
