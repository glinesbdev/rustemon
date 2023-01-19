use crate::{request::Requester, shared::*, Many, ResponseResult, SearchOptions, Single};
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

pub struct Set;

impl Set {
    pub async fn all() -> ResponseResult<Vec<SetData>> {
        let requester = Requester::new("sets");

        Ok(requester.resolve::<Many<SetData>>().await?.data)
    }

    pub async fn search(options: SearchOptions) -> ResponseResult<Many<SetData>> {
        let mut requester = Requester::new("sets");
        requester.options = options;

        Ok(requester.resolve::<Many<SetData>>().await?)
    }

    pub async fn find(id: &str) -> ResponseResult<SetData> {
        Ok(Requester::new(format!("sets/{id}"))
            .resolve::<Single<SetData>>()
            .await?
            .data)
    }
}
