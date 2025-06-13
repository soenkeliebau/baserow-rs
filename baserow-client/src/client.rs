use crate::url_builder::{Error as UrlBuilderError, UrlBuilder};
use reqwest::Client as ReqwestClient;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use snafu::{ensure, OptionExt, ResultExt, Snafu};

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display("Url operation failed:  {source}"))]
    UrlBuilder { source: UrlBuilderError },
    #[snafu(display("Failed to {msg}:  {source}"))]
    Reqwest { source: reqwest::Error, msg: String },

    #[snafu(display("Failed to {msg}:  {source}"))]
    ReqwestWithUrl {
        source: reqwest::Error,
        msg: String,
        url: String,
    },

    #[snafu(display("Invalid header value specified [{value}]:  {source}"))]
    Header {
        source: http::header::InvalidHeaderValue,
        value: String,
    },
    #[snafu(display("Failed to serialize {msg}: {source})"))]
    SerializeRequest {
        source: serde_json::Error,
        msg: String,
    },
    #[snafu(display("Object has no id, cannot update"))]
    NoIdentifier {},
    #[snafu(display("Server returned status [{status}]: {msg}"))]
    ResponseStatus {
        status: String,
        msg: String,
    },
}

pub struct Client {
    client: ReqwestClient,
    url_builder: UrlBuilder,
}

pub trait BaserowObject {
    fn get_static_table_id() -> usize;
    fn get_table_id(&self) -> usize;
    fn get_id(&self) -> Identifier;
    fn get_table_id_field(&self) -> String;
}

pub enum Identifier {
    UnsignedNumber { id: Option<usize> },
    SignedNumber { id: Option<isize> },
    FloatNumber { id: Option<f64> },
    Text { id: Option<String> },
}

impl Identifier {
    pub fn get_string(&self) -> Option<String> {
        match self {
            Identifier::SignedNumber { id } => id.as_ref().map(|id| id.to_string()),
            Identifier::Text { id } => id.clone(),
            Identifier::UnsignedNumber { id } => id.as_ref().map(|id| id.to_string()),
            Identifier::FloatNumber { id } => id.as_ref().map(|id| id.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchResult<T> {
    pub count: usize,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
struct IdOnly {
    pub id: usize,
}

impl Client {
    pub fn new(token: &str, base_url: Option<&str>) -> Result<Self, Error> {
        // Build default headers to be included with every request later on
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Token {}", token))
                .context(HeaderSnafu { value: token })?,
        );
        // This really shouldn't fail, but since we have the error variant .. why not?
        default_headers.insert(
            ACCEPT,
            HeaderValue::from_str("application/json").context(HeaderSnafu {
                value: "application/json",
            })?,
        );

        Ok(Self {
            client: ReqwestClient::builder()
                .default_headers(default_headers)
                .build()
                .context(ReqwestSnafu {
                    msg: "build client",
                })?,
            url_builder: UrlBuilder::new(base_url).context(UrlBuilderSnafu {})?,
        })
    }

    pub async fn list<T>(&self) -> Result<Vec<T>, Error>
    where
        T: BaserowObject + Serialize + DeserializeOwned,
    {
        let url = self
            .url_builder
            .get_list_records_url(T::get_static_table_id())
            .context(UrlBuilderSnafu)?;
        println!("Calling url: {}", url);
        let response = self
            .client
            .get(url.as_ref())
            .send()
            .await
            .context(ReqwestWithUrlSnafu {
                msg: "send list request",
                url: url.as_ref(),
            })?;
        ensure!(response.status().is_success(), ResponseStatusSnafu {status: response.status().to_string(),msg: response.text().await.unwrap()});

            Ok(response
                .json::<SearchResult<T>>()
                .await
                .context(ReqwestSnafu {
                    msg: "deserialize list response",
                })?
                .results)
            }

    pub async fn create<T>(&self, obj: &T) -> Result<(), Error>
    where
        T: BaserowObject + Serialize,
    {
        let url = self
            .url_builder
            .get_create_record_url(obj.get_table_id())
            .context(UrlBuilderSnafu {})?;

        let request = self
            .client
            .post(url.as_ref())
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(obj).context(SerializeRequestSnafu {
                msg: obj.get_table_id().to_string(),
            })?)
            .build()
            .context(ReqwestSnafu {
                msg: "build create request",
            })?;

        println!("Request\n{:?}", request);
        let response = self
            .client
            .execute(request)
            .await
            .context(ReqwestWithUrlSnafu {
                msg: "send create request",
                url: url.as_ref(),
            })?;

        println!("{:?}", response);
        Ok(())
    }

    pub async fn update<T>(&self, obj: &T) -> Result<(), Error>
    where
        T: BaserowObject + Serialize,
    {
        // Need to find the rowid for the object first
        let id: String = obj.get_id().get_string().context(NoIdentifierSnafu)?;

        let url = self
            .url_builder
            .get_find_record_url(obj.get_table_id(), &obj.get_table_id_field(), &id)
            .context(UrlBuilderSnafu)?;

        let search_result = self
            .client
            .get(url.as_ref())
            .send()
            .await
            .context(ReqwestWithUrlSnafu {
                msg: "send update request",
                url: url.as_ref(),
            })?
            .json::<SearchResult<IdOnly>>()
            .await
            .context(ReqwestSnafu {
                msg: "deserialize search response",
            })?;

        if !search_result.count.eq(&1) {
            panic!("Should only have found one object for primary id!");
        }

        let id = search_result.results.first().unwrap().id;

        let url = self
            .url_builder
            .get_update_record_url(obj.get_table_id(), id)
            .context(UrlBuilderSnafu)?;

        let response = self
            .client
            .patch(url.as_ref())
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(obj).unwrap())
            .send()
            .await
            .context(ReqwestWithUrlSnafu {
                msg: "send update request",
                url: url.as_ref(),
            })?;
        println!("{:?}", response);
        println!("{:?}", response.text().await.unwrap());
        Ok(())
    }
}
