use serde::{
    Deserialize,
    Serialize,
};
use crate::{
    anime::RootFind,
    query_builder,
    models::Person,
};
use reqwest::StatusCode;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "data")]
    pub character_list: Vec<Character>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Character {
    #[serde(rename = "character")]
    pub character_details: CharacterDetails,
    pub role: Option<String>,
    pub voice_actors: Vec<VoiceActor>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CharacterDetails {
    pub mal_id: u32,
    pub url: String,
    pub images: CharacterImages,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CharacterImages {
    pub jpg: Images,
    pub webp: Images,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Images {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct VoiceActor {
    pub person: Person,
    pub language: String,
}

pub async fn get_characters_id(id: u64) -> Result<Vec<Character>, StatusCode> {
    let text_id = id.to_string();
    let anime_list: Result<RootFind, StatusCode> = query_builder::find("anime", &text_id).await;

    match anime_list {
        Ok(_) => {
            let params = format!("{}/characters", &text_id);
            let character_list: Result<Root, StatusCode> = query_builder::filter("anime", &params).await;
            match character_list {
                Ok(t) => Ok(t.character_list),
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
    async fn test_get_characters_id() {
        let result = get_characters_id(15051).await;
        assert_eq!("Ayase, Eri", result.unwrap()[0].character_details.name);
    }
}
