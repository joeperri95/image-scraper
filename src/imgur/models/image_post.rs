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
pub struct ImagePosts
{
    pub data: Vec<ImagePost>,
}
