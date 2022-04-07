use serde::{
    Deserialize,
    Serialize,
};
use reqwest::StatusCode;
use crate::{
    query_builder,
    characters::RootFind,
};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "data")]
    pub images: Vec<Pictures>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Pictures {
    pub image_url: Option<String>,
    pub large_image_url: Option<String>,
}

pub async fn get_pictures_id(id: u64) -> Result<Vec<Pictures>, StatusCode> {
    let text_id = id.to_string();
    let character_list: Result<RootFind, StatusCode> = query_builder::find("characters", &text_id).await;

    match character_list {
        Ok(_) => {
            let params = format!("{}/pictures", &text_id);
            let picture_list: Result<Root, StatusCode> = query_builder::filter("characters", &params).await;

            match picture_list {
                Ok(t) => Ok(t.images),
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
    async fn test_get_pictures_id() {
        let result = get_pictures_id(46173).await;
        println!("{}", result.unwrap()[0].image_url.as_ref().unwrap());
    }
}
