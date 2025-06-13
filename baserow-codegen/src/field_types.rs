use convert_case::Case::Pascal;
use convert_case::{Case, Casing};
use quote::__private::TokenStream;
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharedFields {
    pub id: usize,
    pub table_id: usize,
    pub name: String,
    pub order: usize,
    pub primary: bool,
    pub read_only: bool,
    pub immutable_type: Option<bool>,
    pub immutable_properties: bool,
    pub description: Option<String>,
    pub database_id: usize,
    pub workspace_id: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum TableField {
    #[serde(rename = "text")]
    Text {
        #[serde(flatten)]
        shared_fields: SharedFields,
        text_default: Option<String>,
    },
    #[serde(rename = "long_text")]
    LongText {
        #[serde(flatten)]
        shared_fields: SharedFields,
        long_text_enable_rich_text: Option<bool>,
    },
    #[serde(rename = "url")]
    Url {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "email")]
    Email {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "number")]
    Number {
        #[serde(flatten)]
        shared_fields: SharedFields,
        number_decimal_places: isize,
        number_negative: bool,
        number_prefix: String,
        number_suffix: String,
        number_default: Option<String>,
    },
    #[serde(rename = "rating")]
    Rating {
        #[serde(flatten)]
        shared_fields: SharedFields,
        max_value: isize,
        color: String,
        style: String,
    },
    #[serde(rename = "boolean")]
    Boolean {
        #[serde(flatten)]
        shared_fields: SharedFields,
        boolean_default: bool,
    },
    #[serde(rename = "date")]
    Date {
        #[serde(flatten)]
        shared_fields: SharedFields,
        date_format: String,
        date_include_time: bool,
        date_time_format: String,
        date_show_tzinfo: bool,
        date_force_timezone: Option<String>,
    },
    #[serde(rename = "last_modified")]
    LastModified {
        #[serde(flatten)]
        shared_fields: SharedFields,
        date_format: String,
        date_include_time: bool,
        date_time_format: String,
        date_show_tzinfo: bool,
        date_force_timezone: Option<String>,
    },
    #[serde(rename = "last_modified_by")]
    LastModifiedBy {
        #[serde(flatten)]
        shared_fields: SharedFields,
        available_collaborators: Vec<Collaborator>,
    },
    #[serde(rename = "created_on")]
    CreatedOn {
        #[serde(flatten)]
        shared_fields: SharedFields,
        date_format: String,
        date_include_time: bool,
        date_time_format: String,
        date_show_tzinfo: bool,
        date_force_timezone: Option<String>,
    },
    #[serde(rename = "created_by")]
    CreatedBy {
        #[serde(flatten)]
        shared_fields: SharedFields,
        available_collaborators: Vec<Collaborator>,
    },
    #[serde(rename = "duration")]
    Duration {
        #[serde(flatten)]
        shared_fields: SharedFields,
        duration_format: String,
    },
    #[serde(rename = "link_row")]
    LinkRow {
        #[serde(flatten)]
        shared_fields: SharedFields,
        link_row_table_id: Option<isize>,
        link_row_related_field_id: Option<isize>,
        link_row_table: Option<isize>,
        link_row_related_field: Option<isize>,
        link_row_limit_selection_view_id: Option<isize>,
        link_row_table_primary_field: Box<TableField>,
        link_row_multiple_relationships: bool,
    },
    #[serde(rename = "file")]
    File {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "single_select")]
    SingleSelect {
        #[serde(flatten)]
        shared_fields: SharedFields,
        select_options: Vec<SelectOption>,
        single_select_default: Option<isize>,
    },
    #[serde(rename = "multiple_select")]
    MultipleSelect {
        #[serde(flatten)]
        shared_fields: SharedFields,
        select_options: Vec<SelectOption>,
        multiple_select_default: Option<Vec<isize>>,
    },
    #[serde(rename = "phone_number")]
    PhoneNumber {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "formula")]
    Formula {
        #[serde(flatten)]
        shared_fields: SharedFields,
        date_time_format: Option<String>,
        array_formula_type: Option<String>,
        date_include_time: Option<bool>,
        error: Option<String>,
        number_suffix: String,
        date_force_timezone: Option<String>,
        nullable: bool,
        number_separator: String,
        select_options: Option<Vec<SelectOption>>,
        date_show_tzinfo: Option<bool>,
        number_prefix: String,
        date_format: Option<String>,
        number_decimal_places: Option<isize>,
        available_collaborators: Option<Vec<Collaborator>>,
        duration_format: Option<String>,
        formula: String,
        formula_type: String,
    },
    #[serde(rename = "count")]
    Count {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "rollup")]
    Rollup {
        #[serde(flatten)]
        shared_fields: SharedFields,
        date_time_format: Option<String>,
        array_formula_type: Option<String>,
        date_include_time: Option<bool>,
        error: Option<String>,
        number_suffix: String,
        date_force_timezone: Option<String>,
        nullable: bool,
        number_separator: String,
        date_show_tzinfo: Option<bool>,
        number_prefix: String,
        date_format: Option<String>,
        number_decimal_places: Option<isize>,
        duration_format: Option<String>,
        through_field_id: Option<isize>,
        target_field_id: Option<isize>,
        rollup_function: String,
        formula_type: String,
    },
    #[serde(rename = "lookup")]
    Lookup {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "multiple_collaborators")]
    MultipleCollaborators {
        #[serde(flatten)]
        shared_fields: SharedFields,
        available_collaborators: Vec<Collaborator>,
        notify_user_when_added: bool,
    },
    #[serde(rename = "uuid")]
    Uuid {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "autonumber")]
    AutoNumber {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "password")]
    Password {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
    #[serde(rename = "ai")]
    Ai {
        #[serde(flatten)]
        shared_fields: SharedFields,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collaborator {
    id: isize,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectOption {
    pub id: isize,
    pub value: String,
    pub color: String,
}

pub fn cleanup_name(dirty_name: &str) -> String {
    let first_stage = dirty_name.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', '/', '@'][..], "" );
    // Check if starts with number, that is not a legal rust identifier
    if first_stage[0..1].parse::<u8>().is_ok() {
        format!("a{}", first_stage)
    } else {
        first_stage
    }
}

impl TableField {
    fn clean_name(dirty_name: &str) -> String {
        let clean_name = cleanup_name(dirty_name);
        let clean_name = clean_name.to_case(Case::Snake);
        if clean_name.eq("type") {
            "ty".to_string()
        } else {
            clean_name
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            TableField::Text { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::LongText { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::Url { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::Email { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::Number { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::Rating { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::Boolean { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::Date { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::LastModified { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::LastModifiedBy { shared_fields, .. } => {
                Self::clean_name(&shared_fields.name)
            }
            TableField::CreatedOn { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::CreatedBy { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::Duration { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::LinkRow { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::File { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::SingleSelect { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::MultipleSelect { shared_fields, .. } => {
                Self::clean_name(&shared_fields.name)
            }
            TableField::PhoneNumber { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::Formula { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::Count { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::Rollup { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::Lookup { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::MultipleCollaborators { shared_fields, .. } => {
                Self::clean_name(&shared_fields.name)
            }
            TableField::Uuid { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::AutoNumber { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::Password { shared_fields, .. } => Self::clean_name(&shared_fields.name),
            TableField::Ai { shared_fields, .. } => Self::clean_name(&shared_fields.name),
        }
    }

    pub fn get_original_name(&self) -> &String {
        match self {
            TableField::Text { shared_fields, .. } => &shared_fields.name,
            TableField::LongText { shared_fields, .. } => &shared_fields.name,
            TableField::Url { shared_fields, .. } => &shared_fields.name,
            TableField::Email { shared_fields, .. } => &shared_fields.name,
            TableField::Number { shared_fields, .. } => &shared_fields.name,
            TableField::Rating { shared_fields, .. } => &shared_fields.name,
            TableField::Boolean { shared_fields, .. } => &shared_fields.name,
            TableField::Date { shared_fields, .. } => &shared_fields.name,
            TableField::LastModified { shared_fields, .. } => &shared_fields.name,
            TableField::LastModifiedBy { shared_fields, .. } => &shared_fields.name,
            TableField::CreatedOn { shared_fields, .. } => &shared_fields.name,
            TableField::CreatedBy { shared_fields, .. } => &shared_fields.name,
            TableField::Duration { shared_fields, .. } => &shared_fields.name,
            TableField::LinkRow { shared_fields, .. } => &shared_fields.name,
            TableField::File { shared_fields, .. } => &shared_fields.name,
            TableField::SingleSelect { shared_fields, .. } => &shared_fields.name,
            TableField::MultipleSelect { shared_fields, .. } => &shared_fields.name,
            TableField::PhoneNumber { shared_fields, .. } => &shared_fields.name,
            TableField::Formula { shared_fields, .. } => &shared_fields.name,
            TableField::Count { shared_fields, .. } => &shared_fields.name,
            TableField::Rollup { shared_fields, .. } => &shared_fields.name,
            TableField::Lookup { shared_fields, .. } => &shared_fields.name,
            TableField::MultipleCollaborators { shared_fields, .. } => &shared_fields.name,
            TableField::Uuid { shared_fields, .. } => &shared_fields.name,
            TableField::AutoNumber { shared_fields, .. } => &shared_fields.name,
            TableField::Password { shared_fields, .. } => &shared_fields.name,
            TableField::Ai { shared_fields, .. } => &shared_fields.name,
        }
    }

    #[allow(dead_code)]
    pub fn get_description(&self) -> &Option<String> {
        match self {
            TableField::Text { shared_fields, .. } => &shared_fields.description,
            TableField::LongText { shared_fields, .. } => &shared_fields.description,
            TableField::Url { shared_fields, .. } => &shared_fields.description,
            TableField::Email { shared_fields, .. } => &shared_fields.description,
            TableField::Number { shared_fields, .. } => &shared_fields.description,
            TableField::Rating { shared_fields, .. } => &shared_fields.description,
            TableField::Boolean { shared_fields, .. } => &shared_fields.description,
            TableField::Date { shared_fields, .. } => &shared_fields.description,
            TableField::LastModified { shared_fields, .. } => &shared_fields.description,
            TableField::LastModifiedBy { shared_fields, .. } => &shared_fields.description,
            TableField::CreatedOn { shared_fields, .. } => &shared_fields.description,
            TableField::CreatedBy { shared_fields, .. } => &shared_fields.description,
            TableField::Duration { shared_fields, .. } => &shared_fields.description,
            TableField::LinkRow { shared_fields, .. } => &shared_fields.description,
            TableField::File { shared_fields, .. } => &shared_fields.description,
            TableField::SingleSelect { shared_fields, .. } => &shared_fields.description,
            TableField::MultipleSelect { shared_fields, .. } => &shared_fields.description,
            TableField::PhoneNumber { shared_fields, .. } => &shared_fields.description,
            TableField::Formula { shared_fields, .. } => &shared_fields.description,
            TableField::Count { shared_fields, .. } => &shared_fields.description,
            TableField::Rollup { shared_fields, .. } => &shared_fields.description,
            TableField::Lookup { shared_fields, .. } => &shared_fields.description,
            TableField::MultipleCollaborators { shared_fields, .. } => &shared_fields.description,
            TableField::Uuid { shared_fields, .. } => &shared_fields.description,
            TableField::AutoNumber { shared_fields, .. } => &shared_fields.description,
            TableField::Password { shared_fields, .. } => &shared_fields.description,
            TableField::Ai { shared_fields, .. } => &shared_fields.description,
        }
    }

    pub fn get_extra_structs(&self, table_name: &str) -> Option<TokenStream> {
        match self {
            TableField::SingleSelect { select_options, .. } => {
                let mut variants = TokenStream::new();
                for option in select_options {
                    let serialized_name = &option.value;
                    let rust_variant_name =
                        format_ident!("{}", cleanup_name(&serialized_name.to_case(Pascal)).replace(' ', ""));
                    variants.extend(quote! {
                        #[serde(rename = #serialized_name)]
                        #[strum(serialize = #serialized_name)]
                        #rust_variant_name {color: String, id: usize},
                    });
                }

                let rust_name =
                    cleanup_name(&format!("{}{}", table_name.to_case(Pascal), self.get_original_name().to_case(Pascal)));
                let rust_name = format_ident!("{}", rust_name);
                Some(quote! {
                    #[derive(Serialize, Deserialize, Debug, Clone, EnumString, Display)]
                    #[serde(tag = "value")]
                    pub enum #rust_name {
                        #variants
                    }
                })
            }
            _ => None,
        }
    }

    pub fn get_deserializer(&self) -> Option<TokenStream> {
        match self {
            TableField::Number {
                number_decimal_places,
                number_negative,
                ..
            } => {
                if number_decimal_places.gt(&0) {
                    Some(quote! {, deserialize_with = "float_or_null"})
                } else if *number_negative {
                    Some(quote! {, deserialize_with = "isize_or_null"})
                } else {
                    Some(quote! {, deserialize_with = "usize_or_null"})
                }
            }
            TableField::Count { .. } => Some(quote! {, deserialize_with = "usize_or_null"}),
            TableField::AutoNumber { .. } => Some(quote! {, deserialize_with = "usize_or_null"}),
            _ => None,
        }
    }

    pub fn get_rust_type(&self, table_name: &str) -> String {
        match self {
            TableField::Text { .. } => "String".to_string(),
            TableField::LongText { .. } => "String".to_string(),
            TableField::Url { .. } => "String".to_string(),
            TableField::Email { .. } => "String".to_string(),
            TableField::Number {
                number_decimal_places,
                number_negative,
                ..
            } => {
                if number_decimal_places.gt(&0) {
                    "f64".to_string()
                } else if *number_negative {
                    "isize".to_string()
                } else {
                    "usize".to_string()
                }
            }
            TableField::Rating { .. } => "String".to_string(),
            TableField::Boolean { .. } => "bool".to_string(),
            TableField::Date { .. } => "DateTime<Utc>".to_string(),
            TableField::LastModified { .. } => "String".to_string(),
            TableField::LastModifiedBy { .. } => "String".to_string(),
            TableField::CreatedOn { .. } => "String".to_string(),
            TableField::CreatedBy { .. } => "String".to_string(),
            TableField::Duration { .. } => "Duration".to_string(),
            TableField::LinkRow { .. } => "String".to_string(),
            TableField::File { .. } => "String".to_string(),
            TableField::SingleSelect { .. } => {
                cleanup_name(&format!("{}{}", table_name.to_case(Pascal), self.get_original_name().to_case(Pascal)))
            },
            TableField::MultipleSelect { .. } => "String".to_string(),
            TableField::PhoneNumber { .. } => "String".to_string(),
            TableField::Formula { .. } => "String".to_string(),
            TableField::Count { .. } => "usize".to_string(),
            TableField::Rollup { .. } => "String".to_string(),
            TableField::Lookup { .. } => "String".to_string(),
            TableField::MultipleCollaborators { .. } => "String".to_string(),
            TableField::Uuid { .. } => "String".to_string(),
            TableField::AutoNumber { .. } => "usize".to_string(),
            TableField::Password { .. } => "String".to_string(),
            TableField::Ai { .. } => "String".to_string(),
        }
    }

    pub fn is_primary(&self) -> bool {
        match self {
            TableField::Text { shared_fields, .. } => shared_fields.primary,
            TableField::LongText { shared_fields, .. } => shared_fields.primary,
            TableField::Url { shared_fields, .. } => shared_fields.primary,
            TableField::Email { shared_fields, .. } => shared_fields.primary,
            TableField::Number { shared_fields, .. } => shared_fields.primary,
            TableField::Rating { shared_fields, .. } => shared_fields.primary,
            TableField::Boolean { shared_fields, .. } => shared_fields.primary,
            TableField::Date { shared_fields, .. } => shared_fields.primary,
            TableField::LastModified { shared_fields, .. } => shared_fields.primary,
            TableField::LastModifiedBy { shared_fields, .. } => shared_fields.primary,
            TableField::CreatedOn { shared_fields, .. } => shared_fields.primary,
            TableField::CreatedBy { shared_fields, .. } => shared_fields.primary,
            TableField::Duration { shared_fields, .. } => shared_fields.primary,
            TableField::LinkRow { shared_fields, .. } => shared_fields.primary,
            TableField::File { shared_fields, .. } => shared_fields.primary,
            TableField::SingleSelect { shared_fields, .. } => shared_fields.primary,
            TableField::MultipleSelect { shared_fields, .. } => shared_fields.primary,
            TableField::PhoneNumber { shared_fields, .. } => shared_fields.primary,
            TableField::Formula { shared_fields, .. } => shared_fields.primary,
            TableField::Count { shared_fields, .. } => shared_fields.primary,
            TableField::Rollup { shared_fields, .. } => shared_fields.primary,
            TableField::Lookup { shared_fields, .. } => shared_fields.primary,
            TableField::MultipleCollaborators { shared_fields, .. } => shared_fields.primary,
            TableField::Uuid { shared_fields, .. } => shared_fields.primary,
            TableField::AutoNumber { shared_fields, .. } => shared_fields.primary,
            TableField::Password { shared_fields, .. } => shared_fields.primary,
            TableField::Ai { shared_fields, .. } => shared_fields.primary,
        }
    }

    pub fn get_id(&self) -> usize {
        match self {
            TableField::Text { shared_fields, .. } => shared_fields.id,
            TableField::LongText { shared_fields, .. } => shared_fields.id,
            TableField::Url { shared_fields, .. } => shared_fields.id,
            TableField::Email { shared_fields, .. } => shared_fields.id,
            TableField::Number { shared_fields, .. } => shared_fields.id,
            TableField::Rating { shared_fields, .. } => shared_fields.id,
            TableField::Boolean { shared_fields, .. } => shared_fields.id,
            TableField::Date { shared_fields, .. } => shared_fields.id,
            TableField::LastModified { shared_fields, .. } => shared_fields.id,
            TableField::LastModifiedBy { shared_fields, .. } => shared_fields.id,
            TableField::CreatedOn { shared_fields, .. } => shared_fields.id,
            TableField::CreatedBy { shared_fields, .. } => shared_fields.id,
            TableField::Duration { shared_fields, .. } => shared_fields.id,
            TableField::LinkRow { shared_fields, .. } => shared_fields.id,
            TableField::File { shared_fields, .. } => shared_fields.id,
            TableField::SingleSelect { shared_fields, .. } => shared_fields.id,
            TableField::MultipleSelect { shared_fields, .. } => shared_fields.id,
            TableField::PhoneNumber { shared_fields, .. } => shared_fields.id,
            TableField::Formula { shared_fields, .. } => shared_fields.id,
            TableField::Count { shared_fields, .. } => shared_fields.id,
            TableField::Rollup { shared_fields, .. } => shared_fields.id,
            TableField::Lookup { shared_fields, .. } => shared_fields.id,
            TableField::MultipleCollaborators { shared_fields, .. } => shared_fields.id,
            TableField::Uuid { shared_fields, .. } => shared_fields.id,
            TableField::AutoNumber { shared_fields, .. } => shared_fields.id,
            TableField::Password { shared_fields, .. } => shared_fields.id,
            TableField::Ai { shared_fields, .. } => shared_fields.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::field_types::TableField;
    use std::fs;
    use convert_case::Case::Pascal;
    use convert_case::{Case, Casing};

    #[test]
    fn test_deserialize() {
        let contents = fs::read_to_string("testdata/field_types1.json")
            .expect("Should have been able to read the file");

        let json: Vec<TableField> =
            serde_json::from_str(&contents).expect("file should be proper JSON");

        let contents = fs::read_to_string("testdata/field_types2.json")
            .expect("Should have been able to read the file");

        let json: Vec<TableField> =
            serde_json::from_str(&contents).expect("file should be proper JSON");
    }
    
    #[test]
    fn test_to_case() {
        let input = "jim.halfpenny@stackable.tech";
        
        let first= input.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', '/', '@'][..], " ");
        
        println!("{}", first.to_case(Pascal));
    }
}
