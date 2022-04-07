use serde::{
    Deserialize,
    Serialize,
};
use reqwest::StatusCode;
use crate::{
    query_builder,
    models::{
        DateRange,
        MalUrl,
        Pagination,
    },
};

pub mod characters;
pub mod staff;
pub mod episodes;
pub mod news;
pub mod forum;
pub mod videos;
pub mod pictures;
pub mod statistics;
pub mod moreinfo;
pub mod recommendations;
pub mod userupdates;
pub mod reviews;
pub mod relations;
pub mod themes;
pub mod external;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Anime {
    pub mal_id: u64,
    pub url: String,
    pub images: Option<AnimeImages>,
    pub trailer: Option<TrailerBase>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Vec<String>,
    #[serde(rename = "type")]
    pub anime_type: String,
    pub source: String,
    pub episodes: Option<u16>,
    pub status: String,
    pub airing: bool,
    pub aired: DateRange,
    pub rating: Option<String>,
    pub score: Option<f32>,
    pub scored_by: Option<u32>,
    pub rank: Option<f32>,
    pub popularity: Option<u32>,
    pub members: Option<u32>,
    pub favorites: Option<u32>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub season: Option<String>,
    pub year: Option<u32>,
    pub broadcast: Broadcast,
    pub producers: Vec<MalUrl>,
    pub licensors: Vec<MalUrl>,
    pub studios: Vec<MalUrl>,
    pub genres: Vec<MalUrl>,
    pub explicit_genres: Vec<MalUrl>,
    pub themes: Vec<MalUrl>,
    pub demographics: Vec<MalUrl>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimeImages {
    jpg: Images,
    webp: Images,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Images {
    image_url: String,
    small_image_url: String,
    large_image_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrailerBase {
    youtube_id: Option<String>,
    url: Option<String>,
    embed_url: Option<String>,
    images: TrailerImages,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrailerImages {
    image_url: Option<String>,
    small_image_url: Option<String>,
    medium_image_url: Option<String>,
    large_image_url: Option<String>,
    maximum_image_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Broadcast {
    day: Option<String>,
    time: Option<String>,
    timezone: Option<String>,
    string: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RootAll {
    pagination: Pagination,
    #[serde(rename = "data")]
    anime_list: Vec<Anime>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RootFind {
    #[serde(rename = "data")]
    anime: Anime,
}

pub async fn all() -> Result<Vec<Anime>, StatusCode> {
    let anime: Result<RootAll, StatusCode> = query_builder::all("anime").await;

    match anime {
        Ok(t) => Ok(t.anime_list),
        Err(e) => Err(e),
    }
}

pub async fn find(id: u64) -> Result<Anime, StatusCode> {
    let text_id = id.to_string();
    let anime_list: Result<RootFind, StatusCode> = query_builder::find("anime", &text_id).await;

    match anime_list {
        Ok(t) => Ok(t.anime),
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

    pub fn q(mut self, input: &'a str) -> Self {
        self.query.push(("q", String::from(input)));
        self
    }

    pub fn r#type(mut self, input: &'a str) -> Self {
        self.query.push(("type", String::from(input)));
        self
    }

    pub fn score(mut self, input: f64) -> Self {
        self.query.push(("score", input.to_string()));
        self
    }

    pub fn min_score(mut self, input: f64) -> Self {
        self.query.push(("min_score", input.to_string()));
        self
    }

    pub fn max_score(mut self, input: f64) -> Self {
        self.query.push(("max_score", input.to_string()));
        self
    }

    pub fn status(mut self, input: &'a str) -> Self {
        self.query.push(("status", String::from(input)));
        self
    }

    pub fn rating(mut self, input: &'a str) -> Self {
        self.query.push(("rating", String::from(input)));
        self
    }

    pub fn sfw(mut self, input: bool) -> Self {
        self.query.push(("sfw", input.to_string()));
        self
    }

    pub fn genres(mut self, input: &'a str) -> Self {
        self.query.push(("genres", String::from(input)));
        self
    }

    pub fn genres_exclude(mut self, input: &'a str) -> Self {
        self.query.push(("genres_exclude", String::from(input)));
        self
    }

    pub fn order_by(mut self, input: &'a str) -> Self {
        self.query.push(("order_by", String::from(input)));
        self
    }

    pub fn sort(mut self, input: &'a str) -> Self {
        self.query.push(("sort", String::from(input)));
        self
    }

    pub fn letter(mut self, input: &'a str) -> Self {
        self.query.push(("letter", String::from(input)));
        self
    }

    pub fn producer(mut self, input: &'a str) -> Self {
        self.query.push(("producer", String::from(input)));
        self
    }

    pub async fn all(mut self) -> Result<Vec<Anime>, StatusCode> {
        let val = self.query.remove(0);
        let mut filter = format!("?{}={}", val.0, val.1);

        for (k, v) in self.query.into_iter() {
            filter = format!("{}&{}={}", filter, k, v);
        }

        let anime_list: Result<RootAll, StatusCode> = query_builder::filter("anime", &filter).await;

        match anime_list {
            Ok(t) => Ok(t.anime_list), 
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_find_id() {
        let result = find(15051).await;
        assert_eq!("Love Live! School Idol Project", result.unwrap().title);
    }

    #[tokio::test]
    async fn test_all() {
        let result = all().await;
        assert_eq!("Cowboy Bebop", result.unwrap()[0].title);
    }
    
    #[tokio::test]
    async fn test_filter() {
        let result = filter()
            .q("love live!")
            .sfw(true)
            .all()
            .await;
        println!("{}", result.unwrap()[0].title);
        // no deterministic way to check for correctness
        // this can only confirm that the filter didn't return a 400
    }
}
