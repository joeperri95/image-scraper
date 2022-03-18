use crate::{conditioner::{self, condition_filename}, models};
use std::collections::{HashMap};

// get hot posts from the specified subreddit
// gather a list of images and videos
pub async fn get_posts(subreddit: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>>
{
    let url = &format!("https://reddit.com/r/{}/hot/.json", subreddit);
    let client = reqwest::Client::new();
    let mut req = client.request(reqwest::Method::GET, url);
    req = req.header(reqwest::header::USER_AGENT, "example");    
    
    let response: models::listing::Listing = req.send().await?.json().await?;   
    
    let mut files_to_download = HashMap::new();

    for post in response.data.children
    {
        //println!("{} {}\n{:?}", post.kind, post.data.title.clone().unwrap(), post.data);
        if let Some(preview) = post.data.preview.clone()
        {
            if let Some(vid) = preview.reddit_video_preview.clone()
            {
                let title = conditioner::condition_filename(&post.data.title.unwrap().to_string());
                let link = conditioner::trim_quotation_marks(&vid.fallback_url.clone().unwrap().to_string());
                files_to_download.insert(title, link);            
            }
            else {
                let title = conditioner::condition_filename(&post.data.title.unwrap().to_string());
                let link = conditioner::trim_quotation_marks(&post.data.url.clone().unwrap().to_string());
                files_to_download.insert(title, link);            
            }
        }
        else {
            let title = conditioner::condition_filename(&post.data.title.unwrap().to_string());
            let link = conditioner::trim_quotation_marks(&post.data.url.clone().unwrap().to_string());
            files_to_download.insert(title, link);            
        }        
    }
    
    Ok(files_to_download)
}        

// remove quotes from url
pub fn clean_link(link: &str) -> String
{
    link.to_string().replace(r#"""#,"")
}

// convert the post title to a friendly filename
pub fn clean_title(title: &str) -> String
{        
    let mut res: String = title.chars()
        .filter(|x| x.is_ascii())
        .filter(|x| !x.is_ascii_punctuation())        
        .collect();

    res = String::from(res.trim_end());
    res.replace(' ', "_").to_ascii_lowercase()
}

pub struct RedditClient
{

}

impl RedditClient
{
    
}