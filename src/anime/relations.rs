use serde::{
    Deserialize,
    Serialize,
};
use reqwest::StatusCode;
use crate::{
    query_builder,
    anime::{
        RootFind,
        MalUrl,
    }
};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "data")]
    pub relation_list: Vec<Relation>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Relation {
    pub relation: String,
    pub entry: Vec<MalUrl>,
}

pub async fn get_relations_id(id: u64) -> Result<Vec<Relation>, StatusCode> {
    let text_id = id.to_string();
    let anime_list: Result<RootFind, StatusCode> = query_builder::find("anime", &text_id).await;

    match anime_list {
        Ok(_) => {
            let params = format!("{}/relations", &text_id);
            let relation_list: Result<Root, StatusCode> = query_builder::filter("anime", &params).await;

            match relation_list {
                Ok(t) => Ok(t.relation_list),
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
    async fn test_get_relations_id() {
        let result = get_relations_id(15051).await;
        assert_eq!("Love Live!", result.unwrap()[0].entry[0].name);
    }
}
