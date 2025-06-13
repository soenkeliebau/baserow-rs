use crate::baserow_config::Database;
use crate::field_types::{TableField, cleanup_name};
use convert_case::Case::Snake;
use convert_case::{Case, Casing};
use quote::__private::TokenStream;
use quote::{format_ident, quote};
use reqwest::Client as ReqwestClient;
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

static LIST_TABLES_URL: &str = "https://api.baserow.io/api/database/tables/all-tables/";
static LIST_TABLE_FIELDS_URL: &str = "https://api.baserow.io/api/database/fields/table/";

#[derive(Serialize, Deserialize, Debug)]
pub struct BaserowConfig {
    token: String,
    database: usize,
}

pub struct Generator {
    client: ReqwestClient,
}

#[derive(Serialize, Deserialize, Debug)]
struct Table {
    pub id: usize,
    pub name: String,
    pub order: usize,
    pub database_id: usize,
    pub fields: Option<Vec<TableField>>,
}

#[allow(dead_code)]
pub enum Identifier {
    UnsignedNumber { id: Option<usize> },
    SignedNumber { id: Option<isize> },
    FloatNumber { id: Option<f64> },
    Text { id: Option<String> },
}

impl Identifier {
    #[allow(dead_code)]
    pub fn get_string(&self) -> Option<String> {
        match self {
            Identifier::SignedNumber { id } => id.as_ref().map(|id| id.to_string()),
            Identifier::Text { id } => id.clone(),
            Identifier::UnsignedNumber { id } => id.as_ref().map(|id| id.to_string()),
            Identifier::FloatNumber { id } => id.as_ref().map(|id| id.to_string()),
        }
    }
}

impl Table {
    pub fn extend_with_fields(&mut self, fields: Vec<TableField>) {
        self.fields = Some(fields);
    }

    pub fn get_struct_name(&self) -> String {
        self.name.to_case(Case::Pascal)
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

impl Generator {
    pub fn new(token: &str) -> Self {
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

    async fn list_tables(&self) -> Vec<Table> {
        let mut tables = self
            .client
            .get(LIST_TABLES_URL)
            .send()
            .await
            .unwrap()
            .json::<Vec<Table>>()
            .await
            .unwrap();

        for table in &mut tables {
            if let Some(table_fields) = self.list_table_fields(&table.id).await {
                table.extend_with_fields(table_fields);
            }
        }

        tables
    }

    async fn list_table_fields(&self, table_id: &usize) -> Option<Vec<TableField>> {
        if let Ok(response) = self
            .client
            .get(format!("{LIST_TABLE_FIELDS_URL}{table_id}/"))
            .send()
            .await
        {
            Some(response.json::<Vec<TableField>>().await.unwrap())
        } else {
            None
        }
    }

    pub async fn generate_structs(&self, databases: &Vec<Database>, target_path: &Path) {
        let mut mod_file = File::create(target_path.join("mod.rs")).expect("Unable to create file");

        // Pull list of all tables accessible with our token, these will be across multiple databases
        // in order to not do this multiple times we'll filter down to the tables we are interested
        // in for every iteration below
        let tablelist = self.list_tables().await;

        for database in databases {
            let module_name = cleanup_name(&database.name).to_case(Snake);
            // Write entry for file in mod.rs
            mod_file
                .write_all(format!("pub mod {};", module_name).as_bytes())
                .unwrap();
            mod_file.write_all("\n".as_bytes()).unwrap();

            let mut structs = quote! {
            use baserow_client::client::{BaserowObject, Identifier};
            use serde::de::Visitor;
            use serde::{de, Deserialize, Deserializer, Serialize};
            use std::fmt;
            use std::fmt::Write;
            use std::str::FromStr;
            use std::string::ToString;
            use strum_macros::{Display, EnumString};
            use chrono::DateTime;
                    };

            // Create module file for this database
            let mut code_file = File::create(target_path.join(format!("{}.rs", module_name)))
                .expect("Unable to create file");

            // Filter list to tables for the database we are looking at in this iteration
            for table in &tablelist
                .iter()
                .filter(|t| t.database_id.eq(&database.id))
                .collect::<Vec<&Table>>()
            {
                // Gather information to be used during generation
                let struct_name = format_ident!("{}", table.get_struct_name());
                let fields = generate_fields(table.fields.as_ref(), &table.name);
                let extra_structs =
                    generate_extra_structs(table.fields.as_ref(), &table.get_struct_name());
                let primary_field = get_primary_field(table.fields.as_ref());
                let primary_field_id = format!("field_{}", primary_field.get_id());
                let primary_id_function = generate_primary_id_fn(primary_field, &table.name);
                let table_id = table.id;

                // Generate code
                structs.extend(quote! {
                    #[derive(Serialize, Deserialize, Debug, Clone)]
                    pub struct #struct_name {
                        #fields
                    }

                    #extra_structs

                    impl BaserowObject for #struct_name {
                        fn get_static_table_id() -> usize {
                            #table_id
                        }

                        fn get_table_id(&self) -> usize {
                            Self::get_static_table_id()
                        }

                        fn get_id(&self) -> Identifier {
                            #primary_id_function
                        }

                        fn get_table_id_field(&self) -> String {
                            #primary_field_id.to_string()
                        }
                }});
            }
            structs.extend(generate_deserializers());

            // Print formated code to stdout
            let syntax_tree = syn::parse_file(&structs.to_string()).unwrap();
            code_file.write_all(prettyplease::unparse(&syntax_tree).as_bytes()).unwrap();
            code_file.flush().unwrap();
            mod_file.flush().unwrap();
        }
    }
}

fn get_primary_field(fields: Option<&Vec<TableField>>) -> &TableField {
    if let Some(fields) = fields {
        let filtered_primary_fields = fields
            .iter()
            .filter(|field| field.is_primary())
            .collect::<Vec<&TableField>>();
        if filtered_primary_fields.len() != 1 {
            panic!("got more or less than one primary field!");
        }
        filtered_primary_fields.first().unwrap()
    } else {
        panic!("got no fields to determine primary field");
    }
}

fn generate_deserializers() -> TokenStream {
    quote! {
        fn isize_or_null<'de, D>(deserializer: D) -> Result<Option<isize>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IsizeOrNull;

        impl<'de> Visitor<'de> for IsizeOrNull {
            type Value = Option<isize>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("number or null")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Some(isize::from_str(value).unwrap()))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }
        }

        deserializer.deserialize_any(IsizeOrNull)
    }


    fn usize_or_null<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsizeOrNull;

        impl<'de> Visitor<'de> for UsizeOrNull {
            type Value = Option<usize>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("number or null")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Some(usize::from_str(value).unwrap()))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }
        }

        deserializer.deserialize_any(UsizeOrNull)
    }

    fn float_or_null<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FloatOrNull;

        impl<'de> Visitor<'de> for FloatOrNull {
            type Value = Option<f64>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("number or null")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Some(f64::from_str(value).unwrap()))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }
        }

        deserializer.deserialize_any(FloatOrNull)
    }
        }
}

fn generate_extra_structs(
    fields: Option<&Vec<TableField>>,
    table_name: &str,
) -> Option<TokenStream> {
    if let Some(fields) = fields {
        let mut extra_structs_stream = TokenStream::new();
        for field in fields {
            extra_structs_stream.extend(field.get_extra_structs(table_name));
        }
        Some(extra_structs_stream)
    } else {
        None
    }
}

fn generate_fields(fields: Option<&Vec<TableField>>, table_name: &str) -> Option<TokenStream> {
    if let Some(fields) = fields {
        let mut field_stream = TokenStream::new();
        for field in fields {
            // Prepare some values that most branches of the following code will need
            let field_name = format_ident!("{}", field.get_name().to_case(Case::Snake));
            let field_type = format_ident!("{}", field.get_rust_type(table_name));
            let field_id = format!("field_{}", field.get_id());
            let deserializer = field.get_deserializer();
            field_stream.extend(quote! {
                #[serde(rename = #field_id #deserializer)]
                pub #field_name: Option<#field_type>,
            });
        }
        Some(field_stream)
    } else {
        None
    }
}

fn generate_primary_id_fn(primary_field: &TableField, table_name: &str) -> TokenStream {
    let field_name = format_ident!("{}", primary_field.get_name());

    match primary_field.get_rust_type(table_name).as_ref() {
        "isize" => quote! {
        Identifier::SignedNumber { id: self.#field_name}
        },
        "usize" => quote! {
        Identifier::UnsignedNumber { id: self.#field_name}
        },
        "f64" => quote! {
        Identifier::FloatNumber { id: self.#field_name}
        },
        _ => quote! {
            Identifier::Text { id: Some(match &self.#field_name {
                    None => "".to_string(),
                    Some(name) => name.to_string(),
                })
            }
        },
    }
}
