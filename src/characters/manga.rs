use serde::{
    Deserialize,
    Serialize
};
use reqwest::StatusCode;
use crate::{
    query_builder,
    anime::recommendations::Meta,
    characters::RootFind,
};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "data")]
    pub manga_list: Vec<CharacterManga>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CharacterManga {
    pub role: String,
    pub manga: Meta,
}

pub async fn get_character_manga_id(id: u64) -> Result<Vec<CharacterManga>, StatusCode> {
    let text_id = id.to_string();
    let character_list: Result<RootFind, StatusCode> = query_builder::find("characters", &text_id).await;

    match character_list {
        Ok(_) => {
            let params = format!("{}/manga", &text_id);
            let character_manga: Result<Root, StatusCode> = query_builder::filter("characters", &params).await;

            match character_manga {
                Ok(t) => Ok(t.manga_list),
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
    async fn test_get_character_manga_id() {
        let result = get_character_manga_id(46173).await;
        assert_eq!("Love Live", result.unwrap()[0].manga.title);
    }
}
