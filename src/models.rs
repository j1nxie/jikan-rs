use serde::{
    Deserialize,
    Serialize,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateRange {
    pub from: Option<String>,
    pub to: Option<String>,
    pub prop: DateProp,
    pub string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateProp {
    pub from: Date,
    pub to: Date,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Date {
    day: Option<u16>,
    month: Option<u16>,
    year: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pagination {
    pub last_visible_page: u64,
    pub has_next_page: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Images {
    pub image_url: String,
    pub small_image_url: String,
    pub large_image_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MalUrl {
    pub mal_id: u64,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Person {
    pub mal_id: u32,
    pub url: String,
    pub images: PeopleImages,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PeopleImages {
    jpg: Jpg,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Jpg {
    pub image_url: String,
}
