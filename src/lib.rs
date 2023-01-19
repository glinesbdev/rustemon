pub use crate::card::*;
pub use crate::responses::errors::*;
pub use crate::responses::*;
pub use crate::shared::*;

pub mod card;
mod request;
pub mod responses;
pub mod set;
pub mod shared;

pub type ResponseResult<T> = std::result::Result<T, ResponseError>;

const BASE_URL: &str = "https://api.pokemontcg.io/v2";
