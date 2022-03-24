use serde::{
    Deserialize,
    Serialize,
};
use reqwest::StatusCode;
use crate::{
    query_builder,
    anime::RootFind,
};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "data")]
    pub external_list: Vec<External>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct External {
    pub name: String,
    pub url: String,
}

pub async fn get_external_id(id: u64) -> Result<Vec<External>, StatusCode> {
    let text_id = id.to_string();
    let anime_list: Result<RootFind, StatusCode> = query_builder::find("anime", &text_id).await;

    match anime_list {
        Ok(_) => {
            let params = format!("{}/external", &text_id);
            let external_list: Result<Root, StatusCode> = query_builder::filter("anime", &params).await;

            match external_list {
                Ok(t) => Ok(t.external_list),
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
    async fn test_get_external_id() {
        let result = get_external_id(15051).await;
        assert_eq!("http://www.lovelive-anime.jp/", result.unwrap()[0].url);
    }
}
