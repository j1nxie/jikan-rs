use serde::{
    Deserialize,
    Serialize,
};
use reqwest::StatusCode;
use crate::{
    query_builder,
    anime::{
        RootFind,
        AnimeImages,
    },
};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "data")]
    pub recommendation_list: Vec<Recommendation>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Recommendation {
    pub entry: Meta,
    pub url: String,
    pub votes: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Meta {
    pub mal_id: u64,
    pub url: String,
    pub images: AnimeImages,
    pub title: String,
}

pub async fn get_recommendations_id(id: u64) -> Result<Vec<Recommendation>, StatusCode> {
    let text_id = id.to_string();
    let anime_list: Result<RootFind, StatusCode> = query_builder::find("anime", &text_id).await;

    match anime_list {
        Ok(_) => {
            let params = format!("{}/recommendations", &text_id);
            let recommendation_list: Result<Root, StatusCode> = query_builder::filter("anime", &params).await;

            match recommendation_list {
                Ok(t) => Ok(t.recommendation_list),
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
    async fn test_get_recommendations_id() {
        let result = get_recommendations_id(15051).await;
        assert_eq!("K-On!", result.unwrap()[0].entry.title);
    }
}
