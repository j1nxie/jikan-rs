use reqwest::StatusCode;
use serde::{
    Deserialize,
    Serialize,
};
use crate::{
    query_builder,
    models::Pagination,
    anime::RootFind,
};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Root {
    pub pagination: Pagination,
    #[serde(rename = "data")]
    pub news_list: Vec<News>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct News {
    pub mal_id: u64,
    pub url: String,
    pub title: String,
    pub date: String,
    pub author_username: String,
    pub author_url: String,
    pub forum_url: String,
    pub images: CommonImages,
    pub comments: u64,
    pub excerpt: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CommonImages {
    pub jpg: Jpg,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Jpg {
    pub image_url: Option<String>,
}

// TODO: implement page query

pub async fn get_news_id(id: u64) -> Result<Vec<News>, StatusCode> {
    let text_id = id.to_string();
    let anime_list: Result<RootFind, StatusCode> = query_builder::find("anime", &text_id).await;

    match anime_list {
        Ok(_) => {
            let params = format!("{}/news", &text_id);
            let news_list: Result<Root, StatusCode> = query_builder::filter("anime", &params).await;

            match news_list {
                Ok(t) => Ok(t.news_list),
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
    async fn test_get_news_id() {
        let result = get_news_id(15051).await;
        assert_eq!("'Love Live!' Franchise Announces New TV Anime Series", result.unwrap()[0].title);
    }
}
