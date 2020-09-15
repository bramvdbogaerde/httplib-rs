use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct QueryParams {
    params: HashMap<String, String>,
}

impl Default for QueryParams {
    fn default() -> Self {
        QueryParams {
            params: HashMap::new(),
        }
    }
}

impl QueryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<K: Into<String>, V: std::fmt::Display>(&mut self, key: K, value: V) -> &mut Self {
        self.params.insert(key.into(), format!("{}", value));
        self
    }
}

impl std::fmt::Display for QueryParams {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "?")?;
        for (key, value) in self.params.iter() {
            write!(formatter, "{}={}&", key, value)?;
        }

        Ok(())
    }
}

pub enum Body<R> {
    Valued {
        value: R,
    },

    Empty
}

impl<R> Body<R>
where
    R: Serialize,
{
    pub fn wrap(value: R) -> Body<R> {
        Body::Valued { value }
    }

    pub fn empty() -> Body<R> {
        Body::Empty
    }
}

pub enum HttpError {
    Undefined,
}

#[derive(Clone, Debug)]
pub struct Client {
    base: String,
}

impl Client {
    pub fn new<S: Into<String>>(base: S) -> Client {
        Client { base: base.into() }
    }

    pub async fn get<P: AsRef<str>, T: DeserializeOwned>(
        &mut self,
        url: P,
        query_params: QueryParams,
    ) -> Result<T, HttpError> {
        let url = format!("{}/{}{}", self.base, url.as_ref(), query_params);
        surf::get(url.as_str())
            .recv_json()
            .await
            .map_err(|_| HttpError::Undefined)
    }

    pub async fn post<P: AsRef<str>, T: DeserializeOwned, R: Serialize>(
        &mut self,
        _body: Body<R>,
        _query_params: QueryParams,
    ) -> Result<T, HttpError> {
        todo!()
    }
}
