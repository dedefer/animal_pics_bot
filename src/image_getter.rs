use std::{collections::HashMap, error::Error};

use rand::random;
use serde::Deserialize;

pub struct ImageGetter {
    client: reqwest::Client,
    token: String,
}

impl ImageGetter {
    pub fn new(token: String) -> ImageGetter {
        ImageGetter {
            client: reqwest::Client::new(),
            token,
        }
    }

    pub async fn get_image_url(&self, q: &str) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
        let pre_resp: Photos = self.client.get("https://api.pexels.com/v1/search/")
            .header("Authorization", self.token.as_str())
            .query(&[
                ("query", q),
                ("per_page", "1"),
            ]).send().await?
            .json().await?;

        if pre_resp.total_results == 0 {
            return Ok(None);
        }

        let page = random::<usize>() % (
                pre_resp.total_results / 80 + if pre_resp.total_results % 80 > 0 { 1 } else { 0 }
            ) as usize + 1;

        let mut photos: Photos = self.client.get("https://api.pexels.com/v1/search/")
        .header("Authorization", self.token.as_str())
        .query(&[
            ("query", q),
            ("per_page", "80"),
            ("page", page.to_string().as_str()),
        ]).send().await?
        .json().await?;

        let photo_idx = rand::random::<usize>() % photos.photos.len();

        Ok(photos.photos[photo_idx].src.remove("large"))
    }
}

#[derive(Clone, Deserialize, Debug)]
struct Photos {
    total_results: i32,
    page: i32,
    per_page: i32,
    photos: Vec<Photo>,
    next_page: Option<String>,
    prev_page: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
struct Photo {
    id: i32,
    width: i32,
    height: i32,
    url: String,
    photographer: String,
    photographer_url: String,
    photographer_id: i32,
    avg_color: String,
    src: HashMap<String, String>,
    liked: bool,
}