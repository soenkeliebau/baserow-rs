use crate::url_builder::UrlBuilder;
use reqwest::Client as ReqwestClient;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

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
    pub fn new(token: &str, base_url: Option<&str>) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Token {}", token)).unwrap(),
        );
        default_headers.insert(ACCEPT, HeaderValue::from_str("application/json").unwrap());

        Self {
            client: ReqwestClient::builder()
                .default_headers(default_headers)
                .build()
                .unwrap(),
            url_builder: UrlBuilder::new(base_url),
        }
    }

    pub async fn list<T>(&self) -> Vec<T>
    where
        T: BaserowObject + Serialize + DeserializeOwned,
    {
        self.client
            .get(
                self.url_builder
                    .get_list_records_url(T::get_static_table_id()),
            )
            .send()
            .await
            .unwrap()
            .json::<SearchResult<T>>()
            .await
            .unwrap()
            .results
    }

    pub async fn create<T>(&self, obj: &T)
    where
        T: BaserowObject + Serialize,
    {
        let request = self
            .client
            .post(self.url_builder.get_create_record_url(obj.get_table_id()))
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(obj).unwrap())
            .build()
            .unwrap();

        println!("Request\n{:?}", request);
        let response = self.client.execute(request).await.unwrap();

        println!("{:?}", response);
        println!("{:?}", response.text().await.unwrap())
    }

    pub async fn update<T>(&self, obj: &T)
    where
        T: BaserowObject + Serialize,
    {
        // Need to find the rowid for the object first
        let id: String = obj.get_id().get_string().unwrap();

        println!("{:?}", self.client);
        let mut search_result = self
            .client
            .get(self.url_builder.get_find_record_url(
                obj.get_table_id(),
                &obj.get_table_id_field(),
                &id,
            ))
            .send()
            .await
            .unwrap()
            .json::<SearchResult<IdOnly>>()
            .await
            .unwrap();

        if !search_result.count.eq(&1) {
            panic!("Should only have found one object for primary id!");
        }

        let id = search_result.results.first().unwrap().id;

        let response = self
            .client
            .patch(
                self.url_builder
                    .get_update_record_url(obj.get_table_id(), id),
            )
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(obj).unwrap())
            .send()
            .await
            .unwrap();
        println!("{:?}", response);
        println!("{:?}", response.text().await.unwrap());
    }
}
