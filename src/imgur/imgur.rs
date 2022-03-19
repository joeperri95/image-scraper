use reqwest::Error;

use crate::imgur::models;

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