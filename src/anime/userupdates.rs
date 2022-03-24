use serde::{
    Deserialize,
    Serialize
};
use reqwest::StatusCode;
use crate::{
    query_builder,
    anime::{
        RootFind,
        Pagination,
    },
};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "data")]
    pub user_list: Vec<User>,
    pub pagination: Pagination,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub user: UserMeta,
    pub score: Option<u64>,
    pub status: String,
    pub episodes_seen: Option<u64>,
    pub episodes_total: Option<u64>,
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UserMeta {
    pub username: String,
    pub url: String,
    pub images: UserImages,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UserImages {
    pub jpg: Images,
    pub webp: Images,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Images {
    pub image_url: String,
}

// TODO: implement page query

pub async fn get_userupdates_id(id: u64) -> Result<Vec<User>, StatusCode> {
    let text_id = id.to_string();
    let anime_list: Result<RootFind, StatusCode> = query_builder::find("anime", &text_id).await;

    match anime_list {
        Ok(_) => {
            let params = format!("{}/userupdates", &text_id);
            let user_list: Result<Root, StatusCode> = query_builder::filter("anime", &params).await;

            match user_list {
                Ok(t) => Ok(t.user_list),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_get_userupdates_id() {
        let result = get_userupdates_id(15051).await;
        println!("{}", result.unwrap()[0].user.username);
        // no deterministic way to check correctness
        // only confirms that the GET didn't return a 400
    }
}
