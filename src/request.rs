use crate::{Many, RequestError, ResponseError, ResponseResult, BASE_URL};
use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client, Response, StatusCode,
};
use std::{collections::HashMap, env};
use tokio::task::JoinHandle;

#[derive(Default)]
pub struct Requester {
    q: String,
    page: u16,
    page_size: u8,
    order_by: String,
    client: Client,
    endpoint: String,
}

impl Requester {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            q: Default::default(),
            page: 1,
            page_size: 250,
            order_by: Default::default(),
            client: Client::new(),
            endpoint: endpoint.into(),
        }
    }

    pub fn page(&mut self, page: u16) {
        self.page = page
    }

    pub fn parse_options(&mut self, options: &HashMap<&str, &str>) {
        if let Some(query) = options.get("q") {
            self.q = query.to_string();
        }

        if let Some(page) = options.get("page") {
            if let Ok(page) = page.parse() {
                self.page = page
            }
        }

        if let Some(page_size) = options.get("pageSize") {
            if let Ok(size) = page_size.parse() {
                self.page_size = size;
            }
        }

        if let Some(order) = options.get("orderBy") {
            self.order_by = order.to_string();
        }
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
            "{}?query={}&page={}&pageSize={}&orderBy={}",
            self.endpoint, self.q, self.page, self.page_size, self.order_by
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
        T: 'static + serde::de::DeserializeOwned + Send + Default,
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
