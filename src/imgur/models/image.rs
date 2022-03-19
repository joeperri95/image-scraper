use serde_json::{Value};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Image
{
    pub id: Value,
    pub title: Value,
    pub description: Value,
    pub datetime: Option<Value>,
    pub file_type: Option<Value>,
    pub animated: Option<Value>,
    pub width: Option<Value>,
    pub height: Option<Value>,
    pub size: Option<Value>,
    pub views: Option<Value>,
    pub bandwidth: Option<Value>,
    pub deletehash: Option<Value>,
    pub name: Option<Value>,
    pub section: Option<Value>,
    pub link: Value,
    pub gifv: Option<Value>,
    pub mp4: Option<Value>,
    pub mp4_size: Option<Value>,
    pub looping: Option<Value>,
    pub favorite: Option<Value>,
    pub nsfw: Option<Value>,
    pub vote: Option<Value>,
    pub in_gallery: Option<Value>,
}