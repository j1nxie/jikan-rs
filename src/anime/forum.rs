use reqwest::StatusCode;
use serde::{
    Deserialize,
    Serialize,
};
use crate::{
    query_builder,
    anime::RootFind,
};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "data")]
    forum_list: Vec<Forum>
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Forum {
    mal_id: u64,
    url: String,
    title: String,
    date: String,
    author_username: String,
    author_url: String,
    comments: u64,
    last_comment: Comment,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Comment {
    url: String,
    author_username: String,
    author_url: String,
    date: String,
}

// TODO: implement filter queries

pub async fn get_forum_id(id: u64) -> Result<Vec<Forum>, StatusCode> {
    let text_id = id.to_string();
    let anime_list: Result<RootFind, StatusCode> = query_builder::find("anime", &text_id).await;

    match anime_list {
        Ok(_) => {
            let params = format!("{}/forum", &text_id);
            let forum_list: Result<Root, StatusCode> = query_builder::filter("anime", &params).await;

            match forum_list {
                Ok(t) => Ok(t.forum_list),
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
    async fn test_get_forum_id() {
        let result = get_forum_id(15051).await;
        assert_eq!("Love Live! School Idol Project Episode 13 Discussion", result.unwrap()[0].title);
    }
}
