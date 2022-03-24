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
    pub theme_list: Themes,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Themes {
    pub openings: Vec<String>,
    pub endings: Vec<String>,
}

pub async fn get_themes_id(id: u64) -> Result<Themes, StatusCode> {
    let text_id = id.to_string();
    let anime_list: Result<RootFind, StatusCode> = query_builder::find("anime", &text_id).await;

    match anime_list {
        Ok(_) => {
            let params = format!("{}/themes", &text_id);
            let theme_list: Result<Root, StatusCode> = query_builder::filter("anime", &params).await;

            match theme_list {
                Ok(t) => Ok(t.theme_list),
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
    async fn test_get_themes_id() {
        let result = get_themes_id(15051).await;
        assert_eq!("\"Bokura wa Ima no Naka de (僕らは今のなかで)\" by μ's", result.unwrap().openings[0]);
    }
}
