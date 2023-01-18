use crate::{
    responses::errors::{RequestError, ResponseError},
    ResponseResult,
};
use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client, IntoUrl, StatusCode,
};
use std::{env, marker::PhantomData};
use url::Url;

#[derive(Debug)]
pub struct Request<T> {
    q: Option<String>,
    page: Option<u16>,
    page_size: Option<u8>,
    order_by: Option<String>,
    select: Option<String>,
    endpoint: String,
    client: Client,
    phantom: PhantomData<T>,
}

impl<T> Default for Request<T> {
    fn default() -> Self {
        Self {
            q: Default::default(),
            page: Default::default(),
            page_size: Default::default(),
            order_by: Default::default(),
            select: Default::default(),
            endpoint: Default::default(),
            client: Client::default(),
            phantom: PhantomData,
        }
    }
}

impl<T> Request<T>
where
    T: serde::de::DeserializeOwned,
{
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            ..Default::default()
        }
    }

    pub fn query(&mut self, query: impl Into<String>) -> &mut Self {
        self.q = Some(query.into());
        self
    }

    pub fn page(&mut self, page: u16) -> &mut Self {
        self.page = Some(page);
        self
    }

    pub fn page_size(&mut self, size: u8) -> &mut Self {
        self.page_size = Some(size);
        self
    }

    pub fn order_by(&mut self, order: impl Into<String>) -> &mut Self {
        self.order_by = Some(order.into());
        self
    }

    pub fn select(&mut self, select: impl Into<String>) -> &mut Self {
        self.select = Some(select.into());
        self
    }

    pub async fn find(&self, id: impl Into<String>) -> ResponseResult<T> {
        let url = match self.build_url(&format!(
            "https://api.pokemontcg.io/v2/{}/{}",
            self.endpoint,
            id.into()
        )) {
            Ok(url) => url,
            Err(error) => {
                return Err(ResponseError::from(error));
            }
        };

        Ok(self.send(url).await?)
    }

    pub async fn search(&self) -> ResponseResult<T> {
        let url = match self.build_url(&format!("https://api.pokemontcg.io/v2/{}", self.endpoint)) {
            Ok(url) => url,
            Err(error) => {
                return Err(ResponseError::from(error));
            }
        };

        println!("{url}");

        Ok(self.send(url).await?)
    }

    fn build_url(&self, base: &str) -> Result<Url, url::ParseError> {
        Url::parse_with_params(
            base,
            &[
                ("q", self.q.clone().unwrap_or_default()),
                ("page", self.page.unwrap_or_else(|| 1).to_string()),
                (
                    "pageSize",
                    self.page_size.unwrap_or_else(|| 255).to_string(),
                ),
                ("orderBy", self.order_by.clone().unwrap_or_default()),
                ("select", self.select.clone().unwrap_or_default()),
            ],
        )
    }

    async fn send(&self, url: impl IntoUrl) -> ResponseResult<T> {
        let api_key = match env::var("POKEMON_API_KEY") {
            Ok(key) => key,
            Err(err) => {
                return Err(ResponseError::from(format!(
                    "{}: POKEMON_API_KEY",
                    err.to_string()
                )));
            }
        };

        let response = self
            .client
            .get(url)
            .header("X-Api-Key", api_key)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        let response_status = response.status();

        let result: ResponseResult<T> = match response_status {
            StatusCode::OK => match response.json::<T>().await {
                Ok(parsed) => Ok(parsed),
                Err(err) => Err(ResponseError::from(err)),
            },
            StatusCode::BAD_REQUEST
            | StatusCode::PAYMENT_REQUIRED
            | StatusCode::FORBIDDEN
            | StatusCode::NOT_FOUND
            | StatusCode::TOO_MANY_REQUESTS
            | StatusCode::UNAUTHORIZED
            | StatusCode::INTERNAL_SERVER_ERROR
            | StatusCode::BAD_GATEWAY
            | StatusCode::SERVICE_UNAVAILABLE
            | StatusCode::GATEWAY_TIMEOUT => Err(ResponseError::from(RequestError::new(
                response.text().await?,
                response_status,
            ))),
            _ => {
                let error = format!(
                    "Unknown error: please contact crate owners: {}",
                    env!("CARGO_PKG_AUTHORS")
                );
                Err(ResponseError::from(error))
            }
        };

        result
    }
}
