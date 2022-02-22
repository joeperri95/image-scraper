use serde_json::{Value};
use serde::Deserialize;
use serde::Serialize;

use super::image::Image;

#[derive(Debug, Serialize, Deserialize)]
pub struct Album
{
    pub id: Value,
    pub title: Value,
    pub description: Value,
    pub datetime: Option<Value>,
    pub cover: Option<Value>,
    pub cover_width: Option<Value>,
    pub cover_height: Option<Value>,
    pub account_url: Option<Value>,
    pub account_id: Option<Value>,
    pub privacy: Option<Value>,
    pub layout: Option<Value>,
    pub views: Option<Value>,
    pub link: Option<Value>,
    pub favorite: Option<Value>,
    pub nsfw: Option<Value>,
    pub section: Option<Value>,
    pub order: Option<Value>,
    pub deletehash: Option<Value>,
    pub image_count: Option<Value>,
    pub images: Vec<Image>,
    pub in_gallery: Option<Value>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumResponse
{
    pub data: Album,
    pub success: bool,
    pub status: usize,
}
