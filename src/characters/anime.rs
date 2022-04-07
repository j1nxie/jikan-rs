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
    pub anime_list: Vec<CharacterAnime>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CharacterAnime {
    pub role: String,
    pub anime: Meta,
}

pub async fn get_character_anime_id(id: u64) -> Result<Vec<CharacterAnime>, StatusCode> {
    let text_id = id.to_string();
    let character_list: Result<RootFind, StatusCode> = query_builder::find("characters", &text_id).await;

    match character_list {
        Ok(_) => {
            let params = format!("{}/anime", &text_id);
            let character_anime: Result<Root, StatusCode> = query_builder::filter("characters", &params).await;

            match character_anime {
                Ok(t) => Ok(t.anime_list),
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
    async fn test_get_character_anime_id() {
        let result = get_character_anime_id(46173).await;
        assert_eq!("Bokura no Live Kimi to no Life", result.unwrap()[0].anime.title);
    }
}
