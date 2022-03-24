use reqwest::StatusCode;
use serde::{
    Deserialize,
    Serialize,
};
use crate::{
    anime::RootFind,
    query_builder,
    models::Person,
};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "data")]
    pub staff_list: Vec<Staff>
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Staff {
    pub person: Person,
    pub positions: Vec<String>,
}

pub async fn get_staff_id(id: u64) -> Result<Vec<Staff>, StatusCode> {
    let text_id = id.to_string();
    let anime_list: Result<RootFind, StatusCode> = query_builder::find("anime", &text_id).await;

    match anime_list {
        Ok(_) => {
            let params = format!("{}/staff", &text_id);
            let staff_list: Result<Root, StatusCode> = query_builder::filter("anime", &params).await;
            
            match staff_list {
                Ok(t) => Ok(t.staff_list),
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
    async fn test_get_staff_id() {
        let result = get_staff_id(15051).await;
        assert_eq!("Saitou, Shigeru", result.unwrap()[0].person.name);
    }
}
