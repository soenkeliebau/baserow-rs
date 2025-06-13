use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use reqwest::Client as ReqwestClient;
use crate::url_builder::UrlBuilder;

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
        }
    }

    pub async fn list<T>(&self) -> Vec<T>
    where
        T: BaserowObject + Serialize + DeserializeOwned,
    {
        
        let list_url = format!("{CREATE_RECORD_URL}{}/", T::get_static_table_id());

        let response = self
            .client
            .get(list_url)
            .send()
            .await
            .unwrap()
            .json::<SearchResult<T>>()
            .await
            .unwrap();

        Vec::new()
    }

    pub async fn create<T>(&self, obj: &T)
    where
        T: BaserowObject + Serialize,
    {
        let create_url = format!("{CREATE_RECORD_URL}{}/", obj.get_table_id().to_string());

        let request = self
            .client
            .post(create_url)
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
        self.url_builder.get_find_record_url(obj.get_table_id(), obj.get_table_id_field())
        let find_url = format!(
            "{LIST_RECORD_URL}{}/?filter__{}__equal={}",
            obj.get_table_id(),
            obj.get_table_id_field(),
            id
        );

        println!("{:?}", self.client);
        let mut search_result = self
            .client
            .get(find_url)
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
            .patch(self.url_builder.get_create_record_url(obj.get_table_id(), id))
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(obj).unwrap())
            .send()
            .await
            .unwrap();
        println!("{:?}", response);
        println!("{:?}", response.text().await.unwrap());
    }
}
