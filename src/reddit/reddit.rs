use crate::{conditioner::{self}, reddit::models};
use std::collections::{HashMap};

pub struct RedditClient
{
    pub user_agent: String
}

impl RedditClient
{
    pub fn new(user_agent: &str) -> RedditClient
    {
        RedditClient {
            user_agent: String::from(user_agent),
        }
    }

    pub async fn get_posts(&self, url: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>>
    {        
        let client = reqwest::Client::new();
        let mut req = client.request(reqwest::Method::GET, url);
        req = req.header(reqwest::header::USER_AGENT, &self.user_agent);    
        
        let response = req.send().await?;   
        let listing: models::listing::Listing = response.json().await?;
        
        let mut files_to_download = HashMap::new();

        for post in listing.data.children
        {            
            let link;

            let title = match &post.data.title
            {
                Some (t) => {conditioner::condition_filename(&t.to_string())}
                None => {
                    continue;
                }
            };
            
            // If there is a reddit preview we will likely want that over a link to some external site

            if let Some(preview) = post.data.preview.clone()
            {
                if let Some(vid) = preview.reddit_video_preview.clone()
                { 
                    if let Some(v) = vid.fallback_url.clone()
                    {
                        link = conditioner::trim_quotation_marks(&v.to_string());
                        files_to_download.insert(title, link);
                    }
                    continue;
                }
            }
 
            if let Some(v) = post.data.url.clone()
            {
                link = conditioner::trim_quotation_marks(&v.to_string());
                files_to_download.insert(title, link);
            }
        }
        
        Ok(files_to_download)
    }        
}