use rand::prelude::*;
use std::{thread, time};
use scraper::{Html, Selector};

use crate::models;

pub async fn get_gallery_image_urls(posts: models::image_post::ImagePosts) -> Vec<String>
{
    
    let mut list = Vec::new();
        
    for i in posts.data 
      {
        println!("{:?}", i);
          if i.is_album == false {
              let mut link = format!("{}", i.link).replace(r#"""#,"");

              // imgur transcodes gifs to mp4 and a jpeg
              // they create a jpeg with a trailing h
              
              link = format!("{}", link).replace("h.gif",".mp4");
              list.push(link);
          }
      }

      list
}

// Construct a url for a random imgur image
fn generate_imgur_url() -> String
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
     // Need to save file with the right extension

     url += ".jpg";
     url
}

// Construct a url for a random imgur image
pub async fn get_random_imgur_url() -> Result<String, Box<dyn std::error::Error>>
{

    let mut url = generate_imgur_url();
    let mut req = reqwest::get(url.clone()).await?;

    while req.url().path() == "/removed.png" || req.status().is_client_error()
    {
        url = generate_imgur_url(); 
        req = reqwest::get(url.clone()).await?;

        // sleep for a bit to be nice to imgur
        thread::sleep(time::Duration::from_millis(10));
    }

     Ok(url)
}

// scrape image links from an imgur gallery
pub async fn scrape(url: &str) -> Vec<String> {
    let response = reqwest::get(url).await.unwrap();
    let document = Html::parse_document(&response.text().await.unwrap());
    let selector = Selector::parse("img").unwrap();
    let video_selector = Selector::parse("source").unwrap();
    let mut urllist = Vec::new();

    // Collect image links from the webpage
    for i in document.select(&selector)
    {
        if let Some(tag) = i.value().attr("src")
        {
            urllist.push(String::from("https:") + tag);
        }
    }

    // Get videos from webpage
    for i in document.select(&video_selector)
    {
        if let Some(source_type) = i.value().attr("type")
        {
            if source_type == "video/mp4" {  
                if let Some(tag) = i.value().attr("src")
                {
                    urllist.push(String::from(tag));
                }
            }
        }
    }

    urllist
}