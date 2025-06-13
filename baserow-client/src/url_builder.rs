use reqwest::Url;
use snafu::{ResultExt, Snafu};
use std::str::FromStr;

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display("Unable to build url for {action}:  {source}"))]
    BuildUrl {
        source: url::ParseError,
        action: String,
    },
}

pub struct UrlBuilder {
    base_url: Url,
}

impl UrlBuilder {
    // The default url for Baserow cloud
    const CLOUD_URL: &'static str = "https://api.baserow.io/";
    // API stubs to build needed endpoints from for requests
    const RECORD_URL: &'static str = "/api/database/rows/table/";

    pub fn new(base_url: Option<&str>) -> Result<Self, Error> {
        match base_url {
            None => Ok(Self::default()),
            Some(url) => Ok(Self {
                base_url: Url::from_str(url).context(BuildUrlSnafu { action: "base url" })?,
            }),
        }
    }

    pub fn get_record_url(&self) -> Url {
        self.base_url.join(Self::RECORD_URL).unwrap()
    }

    pub fn get_list_records_url(&self, table_id: usize) -> Result<Url, Error> {
        self
            .get_record_url()
            .join(&table_id.to_string())
            .context(BuildUrlSnafu {
                action: "listing records",
            })
    }

    pub fn get_create_record_url(&self, table_id: usize) -> Result<Url, Error> {
        self
            .get_record_url()
            .join(&table_id.to_string())
            .context(BuildUrlSnafu {
                action: "creating record",
            })
    }

    pub fn get_find_record_url(
        &self,
        table_id: usize,
        field_id: &str,
        id: &str,
    ) -> Result<Url, Error> {
        self
            .get_create_record_url(table_id)?
            .join(&format!("?filter__{}__equal={}", field_id, id))
            .context(BuildUrlSnafu {
                action: "finding record by id",
            })
    }

    pub fn get_update_record_url(&self, table_id: usize, record_id: usize) -> Result<Url, Error> {
        self
            .get_create_record_url(table_id)?
            .join(&record_id.to_string())
            .context(BuildUrlSnafu {
                action: "updating record by id",
            })
    }
}

impl Default for UrlBuilder {
    fn default() -> Self {
        Self {
            // This unwrap is okay, if we ever hit that it is an error in the code, 
            // as the parsed url is hard-coded
            base_url: Url::from_str(Self::CLOUD_URL).unwrap(),
        }
    }
}
