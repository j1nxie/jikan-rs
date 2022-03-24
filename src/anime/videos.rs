use serde::{
    Deserialize,
    Serialize,
};
use reqwest::StatusCode;
use crate::{
    query_builder,
    anime::{
        RootFind,
        TrailerBase,
        news::CommonImages,
    },
};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "data")]
    pub video_list: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Video {
    pub promo: Vec<Promo>,
    pub episodes: Vec<Episode>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Promo {
    pub title: String,
    pub trailer: TrailerBase,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Episode {
    pub mal_id: u64,
    pub url: String,
    pub title: String,
    pub episode: String,
    pub images: CommonImages,
}

pub async fn get_videos_id(id: u64) -> Result<Video, StatusCode> {
    let text_id = id.to_string();
    let anime_list: Result<RootFind, StatusCode> = query_builder::find("anime", &text_id).await;

    match anime_list {
        Ok(_) => {
            let params = format!("{}/videos", text_id);
            let video_list: Result<Root, StatusCode> = query_builder::filter("anime", &params).await;

            match video_list {
                Ok(t) => Ok(t.video_list),
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
    async fn test_get_videos_id() {
        let result = get_videos_id(15051).await;
        assert_eq!("PV Blu-ray Box version", result.unwrap().promo[0].title);
    }
}
