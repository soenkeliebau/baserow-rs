use std::str::FromStr;
use reqwest::Url;

static LIST_TABLES_URL: &str = "https://api.baserow.io/api/database/tables/all-tables/";
static LIST_TABLE_FIELDS_URL: &str = "https://api.baserow.io/api/database/fields/table/";
static CREATE_RECORD_URL: &str = "https://api.baserow.io/api/database/rows/table/";
static LIST_RECORD_URL: &str = "https://api.baserow.io/api/database/rows/table/";

struct BaserowUrls {
    base_url: Url,
}

impl BaserowUrls {
    const CLOUD_URL: &'static str = "https://api.baserow.io/";
    const LIST_TABLES_URL: &'static str = "/api/database/tables/all-tables/";
    const LIST_TABLE_FIELDS_URL: &'static str = "/api/database/fields/table/";
    const CREATE_RECORD_URL: &'static str = "/api/database/rows/table/";
    const LIST_RECORD_URL: &'static str = "/api/database/rows/table/";

    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: Url::from_str(base_url).unwrap(),
        }
    }

    pub fn get_list_tables_url(&self) -> Url {
        self.base_url.join(LIST_TABLES_URL).unwrap()
    }
}

impl Default for BaserowUrls {
    fn default() -> Self {
        Self {
            base_url: Url::from_str(Self::CLOUD_URL).unwrap(),
        }
    }
}
