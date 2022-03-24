use serde::{
    Deserialize,
    Serialize
};
use reqwest::StatusCode;
use crate::{
    query_builder,
    anime::RootFind,
};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "data")]
    pub statistics: Statistics,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Statistics {
    pub watching: u64,
    pub completed: u64,
    pub on_hold: u64,
    pub dropped: u64,
    pub plan_to_watch: u64,
    pub total: u64,
    pub scores: Vec<Scores>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Scores {
    pub score: u64,
    pub votes: u64,
    pub percentage: f64,
}

pub async fn get_statistics_id(id: u64) -> Result<Statistics, StatusCode> {
    let text_id = id.to_string();
    let anime_list: Result<RootFind, StatusCode> = query_builder::find("anime", &text_id).await;

    match anime_list {
        Ok(_) => {
            let params = format!("{}/statistics", &text_id);
            let statistics: Result<Root, StatusCode> = query_builder::filter("anime", &params).await;

            match statistics {
                Ok(t) => Ok(t.statistics),
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
    async fn test_get_statistics_id() {
        let result = get_statistics_id(15051).await;
        println!("{}", result.unwrap().watching);
        // no deterministic way to check for correctness
        // this can only confirm that the GET didn't return a 400
    }
}
