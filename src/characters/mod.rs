use serde::{
    Deserialize,
    Serialize,
};
use reqwest::StatusCode;
use crate::{
    query_builder,
    models::Pagination,
    anime::characters::CharacterImages,
};

pub mod anime;
pub mod manga;
pub mod voice_actors;
pub mod pictures;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Character {
    pub mal_id: u64,
    pub url: String,
    pub images: CharacterImages,
    pub name: String,
    pub name_kanji: Option<String>,
    pub nicknames: Vec<String>,
    pub favorites: u64,
    pub about: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RootAll {
    pagination: Pagination,
    #[serde(rename = "data")]
    character_list: Vec<Character>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RootFind {
    #[serde(rename = "data")]
    character: Character,
}

pub async fn all() -> Result<Vec<Character>, StatusCode> {
    let character: Result<RootAll, StatusCode> = query_builder::all("characters").await;

    match character {
        Ok(t) => Ok(t.character_list),
        Err(e) => Err(e),
    }
}

pub async fn find(id: u64) -> Result<Character, StatusCode> {
    let text_id = id.to_string();
    let character_list: Result<RootFind, StatusCode> = query_builder::find("characters", &text_id).await;

    match character_list {
        Ok(t) => Ok(t.character),
        Err(e) => Err(e),
    }
}

pub struct Where<'a> {
    query: Vec<(&'a str, String)>,
}

pub fn filter<'a>() -> Where<'a> {
    Where {
        query: Vec::new(),
    }
}

impl<'a> Where<'a> {
    pub fn page(mut self, input: u64) -> Self {
        self.query.push(("page", input.to_string()));
        self
    }

    pub fn limit(mut self, input: u64) -> Self {
        self.query.push(("limit", input.to_string()));
        self
    }

    pub fn q(mut self, input: &str) -> Self {
        self.query.push(("q", String::from(input)));
        self
    }

    pub fn order_by(mut self, input: &str) -> Self {
        self.query.push(("order_by", String::from(input)));
        self
    }

    pub fn sort(mut self, input: &str) -> Self {
        self.query.push(("sort", String::from(input)));
        self
    }
    
    pub fn letter(mut self, input: &str) -> Self {
        self.query.push(("letter", String::from(input)));
        self
    }

    pub async fn all(mut self) -> Result<Vec<Character>, StatusCode> {
        let val = self.query.remove(0);
        let mut filter = format!("?{}={}", val.0, val.1);

        for (k, v) in self.query.into_iter() {
            filter = format!("{}&{}={}", filter, k, v);
        }

        let character_list: Result<RootAll, StatusCode> = query_builder::filter("characters", &filter).await;

        match character_list {
            Ok(t) => Ok(t.character_list),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_find_id() {
        let result = find(46173).await;
        assert_eq!("Eri Ayase", result.unwrap().name);
    }

    #[tokio::test]
    async fn test_all() {
        let result = all().await;
        assert_eq!("Spike Spiegel", result.unwrap()[0].name);
    }

    #[tokio::test]
    async fn test_filter() {
        let result = filter()
            .q("eri ayase")
            .all()
            .await;
        assert_eq!("Eri Ayase", result.unwrap()[0].name);
    }
}
