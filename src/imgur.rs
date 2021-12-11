use rand::prelude::*;
use tokio::fs;
use std::{thread, time};
use serde_json::{Value};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImagePost 
{
    pub id: Value,
    pub title: Value,
    pub description: Option<Value>,
    pub datetime:Value,

    pub file_type: Option<Value>,
    pub width: Option<Value>,
    pub height: Option<Value>,
    pub size: Option<Value>,
    pub views: Option<Value>,
    pub bandwidth: Option<Value>,
    pub vote: Option<Value>,
    pub favorite: Option<Value>,
    pub nsfw: Option<Value>,
    pub section: Option<Value>,
    pub account_url: Option<Value>,
    pub account_id: Option<Value>,
    pub is_ad: Option<Value>,
    pub tags: Option<Value>,
    pub in_most_viral: Option<Value>,
    pub in_gallery: Option<Value>,
    pub link: Value,
    pub comment_count: Option<Value>,
    pub ups: Option<Value>,
    pub downs: Option<Value>,
    pub points: Option<Value>,
    pub score: Option<Value>,
    pub is_album: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse
{
    pub data: Vec<ImagePost>,
}


// Construct a url for a random imgur image
pub fn get_random_imgur_url() -> String
{

     let chars = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
     let mut url = "https://imgur.com/".to_string();
     let mut rng = thread_rng();
 
     for _i in 0..5
     {
         url += &chars.get(rng.gen_range(0..chars.len())).unwrap().to_string();
     }

     // More jpg hits than png
     // Most png files get transcoded to jpeg anyway

     url += ".jpg";
     url
}

// download a random imgur file  
pub async fn download_random_imgur_file(filename: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let mut url = get_random_imgur_url(); 
    let mut req = reqwest::get(url).await?;

    // bad links will redirect to removed.png
    println!("{} {} {}", filename, req.status(), req.url().path());
    while req.url().path() == "/removed.png" || req.status().is_client_error()
    {
        url = get_random_imgur_url(); 
        req = reqwest::get(url).await?;

        // sleep for a bit to be nice to imgur
        thread::sleep(time::Duration::from_millis(10));
    }

    fs::write(filename, req.bytes().await?).await?;
    Ok(())
}

