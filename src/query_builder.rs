use reqwest::StatusCode;
use serde::de::DeserializeOwned;

const API_URL: &str = "https://api.jikan.moe";
const API_VER: &str = "v4";

async fn build<T>(url: String) -> Result<T, StatusCode>
where
    T: DeserializeOwned,
{
    let response = reqwest::get(url).await;
    match &response {
        Ok(r) => {
            if r.status() != StatusCode::OK {
                return Err(r.status());
            }
        },
        Err(e) => {
            if e.is_status() {
                return Err(e.status().unwrap());
            } else {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }

    let content = response.unwrap().json::<T>().await;
    match content {
        Ok(s) => Ok(s),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn all<T>(call: &str) -> Result<T, StatusCode>
where
    T: DeserializeOwned,
{
    let url = format!("{}/{}/{}", API_URL, API_VER, call);
    build(url).await
}

pub async fn find<T>(call: &str, id: &str) -> Result<T, StatusCode>
where
    T: DeserializeOwned,
{
    let url = format!("{}/{}/{}/{}", API_URL, API_VER, call, id);
    build(url).await
}

pub async fn filter<T>(call: &str, params: &str) -> Result<T, StatusCode>
where
    T: DeserializeOwned,
{
    let url = format!("{}/{}/{}/{}", API_URL, API_VER, call, params);
    build(url).await
}
