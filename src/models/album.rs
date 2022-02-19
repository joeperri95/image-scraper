use rand::prelude::*;
use std::{thread, time};
use serde_json::{Value};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct Album
{
    pub id: String,
    pub title: String,
    pub description: String,
    pub datetime: i64,
    pub cover: String,
    pub cover_width: i64,
    pub cover_height: i64,
    pub account_url: String,
    pub account_id: String,
    pub privacy: String,
    pub layout: String,
    pub views: i64,
    pub link: String,
    pub favorite: bool,
    pub nsfw: bool,
    pub section: String,
    pub order: i64,
    pub deletehash: Option<String>,
    pub image_count: i64,
    pub images: Vec<Image>,
    pub in_gallery: bool,

}