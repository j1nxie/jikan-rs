use serde::{
    Deserialize,
    Serialize
};
use reqwest::StatusCode;
use crate::{
    query_builder,
    anime::{
        RootFind,
        userupdates::UserMeta,
        Pagination,
    },
};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "data")]
    pub review_list: Vec<Review>,
    pub pagination: Pagination,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Review {
    pub user: UserMeta,
    pub mal_id: u64,
    pub url: String,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub votes: u64,
    pub date: String,
    pub review: String,
    pub episodes_watched: u64,
    pub scores: Scores,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Scores {
    pub overall: u64,
    pub story: u64,
    pub animation: u64,
    pub sound: u64,
    pub character: u64,
    pub enjoyment: u64,
}

pub async fn get_reviews_id(id: u64) -> Result<Vec<Review>, StatusCode> {
    let text_id = id.to_string();
    let anime_list: Result<RootFind, StatusCode> = query_builder::find("anime", &text_id).await;

    match anime_list {
        Ok(_) => {
            let params = format!("{}/reviews", &text_id);
            let review_list: Result<Root, StatusCode> = query_builder::filter("anime", &params).await;

            match review_list {
                Ok(t) => Ok(t.review_list),
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
    async fn test_get_reviews_id() {
        let result = get_reviews_id(15051).await;
        println!("{}", result.unwrap()[0].review);
        // no deterministic way to check for correctness
        // only confirms that the GET didn't 400
    }
}
