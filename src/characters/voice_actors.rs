use serde::{
    Deserialize,
    Serialize,
};
use reqwest::StatusCode;
use crate::{
    query_builder,
    characters::RootFind,
    models::Person,
};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "data")]
    pub voice_actor_list: Vec<VoiceActor>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct VoiceActor {
    pub language: String,
    pub person: Person,
}

pub async fn get_voice_actor_id(id: u64) -> Result<Vec<VoiceActor>, StatusCode> {
    let text_id = id.to_string();
    let character_list: Result<RootFind, StatusCode> = query_builder::find("characters", &text_id).await;

    match character_list {
        Ok(_) => {
            let params = format!("{}/voices", &text_id);
            let voice_actor_list: Result<Root, StatusCode> = query_builder::filter("characters", &params).await;

            match voice_actor_list {
                Ok(t) => Ok(t.voice_actor_list),
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
    async fn test_get_voice_actor_id() {
        let result = get_voice_actor_id(46173).await;
        assert_eq!("Nanjou, Yoshino", result.unwrap()[0].person.name);
    }
}
