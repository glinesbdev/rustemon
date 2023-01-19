use crate::{Many, RequestError, ResponseError, ResponseResult, BASE_URL, MAX_PAGE_SIZE};
use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client, Response, StatusCode,
};
use std::env;
use tokio::task::JoinHandle;

pub struct SearchOptions {
    q: String,
    page: u16,
    page_size: u8,
    order_by: String,
    select: String,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            q: Default::default(),
            page: 1,
            page_size: MAX_PAGE_SIZE,
            order_by: Default::default(),
            select: Default::default(),
        }
    }
}

impl SearchOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn query(&mut self, query: impl Into<String>) -> &mut Self {
        self.q = query.into();
        self
    }

    pub fn page(&mut self, page: u16) -> &mut Self {
        self.page = page;
        self
    }

    pub fn page_size(&mut self, page_size: u8) -> &mut Self {
        self.page_size = if page_size > MAX_PAGE_SIZE {
            MAX_PAGE_SIZE
        } else {
            page_size
        };
        self
    }

    pub fn order_by(&mut self, order_by: impl Into<String>) -> &mut Self {
        self.order_by = order_by.into();
        self
    }

    pub fn select(&mut self, select: impl Into<String>) -> &mut Self {
        self.select = select.into();
        self
    }
}

#[derive(Default)]
pub struct Requester {
    options: SearchOptions,
    client: Client,
    endpoint: String,
}

impl Requester {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            options: SearchOptions::new(),
            client: Client::new(),
            endpoint: endpoint.into(),
        }
    }

    pub fn page(&mut self, page: u16) {
        self.options.page = page
    }

    pub fn parse_options(&mut self, options: SearchOptions) {
        self.options = options
    }

    pub async fn resolve<T: serde::de::DeserializeOwned>(&self) -> ResponseResult<T> {
        let response = self.api_response().await?;
        let response_status = response.status();

        match response_status {
            StatusCode::OK => match response.json::<T>().await {
                Ok(parsed) => Ok(parsed),
                Err(err) => {
                    return Err(ResponseError::from(err));
                }
            },
            _ => {
                return Err(ResponseError::from(RequestError::new(
                    response.text().await?,
                    response_status,
                )));
            }
        }
    }

    fn api_key(&self) -> Option<String> {
        match env::var("POKEMON_API_KEY") {
            Ok(key) => Some(key),
            Err(_) => None,
        }
    }

    fn build_url(&self) -> String {
        format!(
            "{}?query={}&page={}&pageSize={}&orderBy={}&select={}",
            self.endpoint,
            self.options.q,
            self.options.page,
            self.options.page_size,
            self.options.order_by,
            self.options.select
        )
    }

    async fn api_response(&self) -> reqwest::Result<Response> {
        let response = self
            .client
            .get(format!("{}/{}", BASE_URL, self.build_url()))
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json");

        match self.api_key() {
            Some(api_key) => response.header("X-Api-Key", api_key).send().await,
            None => response.send().await,
        }
    }
}

pub struct MultiRequester;

impl MultiRequester {
    pub async fn resolve_n_pages<T>(endpoint: &'static str, pages: u16) -> ResponseResult<Vec<T>>
    where
        T: serde::de::DeserializeOwned + Send + Default + 'static,
    {
        let mut threads: Vec<JoinHandle<Vec<T>>> = Vec::new();

        for page in (1..pages).into_iter() {
            let thread: JoinHandle<Vec<T>> = tokio::spawn(async move {
                let mut requester = Requester::new(endpoint);
                requester.page(page);

                let results = match requester.resolve::<Many<T>>().await {
                    Ok(results) => results.data,
                    Err(_) => Vec::new(),
                };

                results
            });

            threads.push(thread);
        }

        let results = futures::future::join_all(threads)
            .await
            .into_iter()
            .flat_map(|card_data| -> Vec<T> {
                match card_data {
                    Ok(cards) => cards,
                    Err(_) => Vec::new(),
                }
            })
            .collect();

        Ok(results)
    }
}
