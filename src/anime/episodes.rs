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
pub struct RootList {
    #[serde(rename = "data")]
    pub episode_list: Vec<Episode>,
    pub pagination: Pagination,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RootEpisode {
    #[serde(rename = "data")]
    pub episode: Episode,
}


#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Episode {
    pub mal_id: u64,
    pub url: Option<String>,
    pub title: String,
    pub title_japanese: Option<String>,
    pub title_romanji: Option<String>,
    pub duration: Option<u64>,
    pub aired: Option<String>,
    pub filler: bool,
    pub recap: bool,
    pub forum_url: Option<String>,
}

// TODO: implement page query

pub async fn get_episodes_id(id: u64) -> Result<Vec<Episode>, StatusCode> {
    let text_id = id.to_string();
    let anime_list: Result<RootFind, StatusCode> = query_builder::find("anime", &text_id).await;

    match anime_list {
        Ok(_) => {
            let params = format!("{}/episodes", &text_id);
            let episode_list: Result<RootList, StatusCode> = query_builder::filter("anime", &params).await;

            match episode_list {
                Ok(t) => Ok(t.episode_list),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

pub async fn get_episode_by_id(anime_id: u64, episode_id: u64) -> Result<Episode, StatusCode> {
    let text_anime_id = anime_id.to_string();
    let text_episode_id = episode_id.to_string();
    let anime_list: Result<RootFind, StatusCode> = query_builder::find("anime", &text_anime_id).await;

    match anime_list {
        Ok(_) => {
            let params = format!("{}/episodes/{}", &text_anime_id, &text_episode_id);
            let episode: Result<RootEpisode, StatusCode> = query_builder::filter("anime", &params).await;

            match episode {
                Ok(t) => Ok(t.episode),
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
    async fn test_get_episodes_id() {
        let result = get_episodes_id(15051).await;
        assert_eq!("Come True! Our Dreams!", result.unwrap()[0].title);
    }

    #[tokio::test]
    async fn test_get_episode_by_id() {
        let result = get_episode_by_id(15051, 1).await;
        assert_eq!("Come True! Our Dreams!", result.unwrap().title);
    }
}
