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
    pub moreinfo: Option<String>,
}

pub async fn get_moreinfo_id(id: u64) -> Result<Option<String>, StatusCode> {
    let text_id = id.to_string();
    let anime_list: Result<RootFind, StatusCode> = query_builder::find("anime", &text_id).await;

    match anime_list {
        Ok(_) => {
            let params = format!("{}/moreinfo", &text_id);
            let moreinfo: Result<Root, StatusCode> = query_builder::filter("anime", &params).await;

            match moreinfo {
                Ok(t) => Ok(t.moreinfo),
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
    async fn test_get_moreinfo_id() {
        let result = get_moreinfo_id(15051).await;
        assert_eq!(None, result.unwrap());
    }
}
