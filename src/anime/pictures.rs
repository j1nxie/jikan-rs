use serde::{
    Deserialize,
    Serialize,
};
use reqwest::StatusCode;
use crate::{
    query_builder,
    anime::{
        RootFind,
        news::CommonImages,
    },
};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "data")]
    pub picture_list: Vec<CommonImages>,
}

pub async fn get_pictures_id(id: u64) -> Result<Vec<CommonImages>, StatusCode> {
    let text_id = id.to_string();
    let anime_list: Result<RootFind, StatusCode> = query_builder::find("anime", &text_id).await;

    match anime_list {
        Ok(_) => {
            let params = format!("{}/pictures", &text_id);
            let picture_list: Result<Root, StatusCode> = query_builder::filter("anime", &params).await;

            match picture_list {
                Ok(t) => Ok(t.picture_list),
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
        let result = get_pictures_id(15051).await;
        assert_eq!("https://cdn.myanimelist.net/images/anime/10/44209.jpg", result.unwrap()[0].jpg.image_url.as_ref().unwrap());
    }
}
