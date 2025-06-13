use reqwest::Url;
use std::str::FromStr;

pub struct UrlBuilder {
    base_url: Url,
}

impl UrlBuilder {
    // The default url for Baserow cloud
    const CLOUD_URL: &'static str = "https://api.baserow.io/";
    // API stubs to build needed endpoints from for requests
    const LIST_TABLES_URL: &'static str = "/api/database/tables/all-tables/";
    const LIST_TABLE_FIELDS_URL: &'static str = "/api/database/fields/table/";
    const RECORD_URL: &'static str = "/api/database/rows/table/";

    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: Url::from_str(base_url).unwrap(),
        }
    }

    pub fn get_list_tables_url(&self) -> Url {
        self.base_url.join(Self::LIST_TABLES_URL).unwrap()
    }

    pub fn get_record_url(&self) -> Url {
        self.base_url.join(Self::RECORD_URL).unwrap()
    }

    pub fn get_list_table_fields_url(&self) -> Url {
        self.base_url.join(Self::LIST_TABLE_FIELDS_URL).unwrap()
    }

    pub fn get_find_record_url(&self, table_id: usize, field_id: usize) -> Url {
        self.get_record_url()
            .join(&format!("?filter__{}__equal={}", table_id, field_id))
            .unwrap()
    }

    pub fn get_create_record_url(&self, table_id: usize, record_id: usize) -> Url {
        self.get_record_url()
            .join(&table_id.to_string())
            .unwrap()
            .join(&record_id.to_string())
            .unwrap()
    }
}

impl Default for UrlBuilder {
    fn default() -> Self {
        Self {
            base_url: Url::from_str(Self::CLOUD_URL).unwrap(),
        }
    }
}
