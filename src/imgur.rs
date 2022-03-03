use rand::prelude::*;
use std::{thread, time};
use reqwest::Error;

use crate::models;

fn generate_random_hash(iterations: usize) -> String
{

    let chars = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let mut hash = String::new();
    let mut rng = thread_rng();

    for _i in 0..iterations
    {
        hash += &chars.get(rng.gen_range(0..chars.len())).unwrap().to_string();
    }

    hash
}

// Construct a url for a random imgur image
fn generate_imgur_url() -> String
{
    let mut url = "https://imgur.com/".to_string();

     // More jpg hits than png
     // Most png files get transcoded to jpeg anyway
     // Need to save file with the right extension

    url += &generate_random_hash(5);
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

    // fix extensions
    // This should be done elsewhere but it is easiest to get it fromt the MIME type

    if req.headers()["content-type"] == "image/png"
    {
        url = url.replace(".jpg", ".png");
    }
    else if req.headers()["content-type"] == "image/gif"
    {
        url = url.replace(".jpg", ".gif");
    }
    
    Ok(url)
}

pub struct ImgurClient {
    client_id: String,
}

impl ImgurClient
{
    pub fn new(client_id : &str) -> ImgurClient
    {
        ImgurClient
        {
            client_id : client_id.to_string()
        }       
    }

    pub async fn gallery_request(&mut self, url: &str) -> Result<models::image_post::ImagePosts, Error>
    {
        let client = reqwest::Client::new();
        let mut req = client.request(reqwest::Method::GET, url);
        req = req.header(reqwest::header::AUTHORIZATION, format!("Client-ID {}", self.client_id));
        let response: models::image_post::ImagePosts = req.send().await?.json().await?;                   
        Ok(response)
    }

    pub async fn album_request(&mut self, url: &str) -> Result<models::album::AlbumResponse, Error>
    {
        let client = reqwest::Client::new();
        let mut req = client.request(reqwest::Method::GET, url);
        req = req.header(reqwest::header::AUTHORIZATION, format!("Client-ID {}", self.client_id));
        let response: models::album::AlbumResponse = req.send().await?.json().await?;   
        
        Ok(response)
    }

    pub async fn get_gallery_image_urls(&mut self, posts: models::image_post::ImagePosts) -> Vec<String>
    {
        let mut list = Vec::new();
            
        for i in posts.data 
        {
            println!("{:?}", i);
            if i.is_album == false {
                let mut link = i.link.to_string().replace(r#"""#,"");

                // imgur transcodes gifs to mp4 and a jpeg
                // they create a jpeg with a trailing h
                
                link = link.to_string().replace("h.gif",".mp4");
                list.push(link);
            }
            else {
                         
                let id = format!("{}", i.id).replace(r#"""#,"");
                
                let album_url =  format!("https://api.imgur.com/3/album/{}", id);
                let resp = self.album_request(&album_url).await.unwrap();
                println!("{:?}", resp.data.images);
                for j in &resp.data.images
                {   
                    let mut image_link = format!("{}", j.link).replace(r#"""#,"");

                // imgur transcodes gifs to mp4 and a jpeg
                // they create a jpeg with a trailing h
                    
                    image_link = image_link.to_string().replace("h.gif",".mp4");
                    list.push(image_link);
                }
            }
        }

      list
    }
}