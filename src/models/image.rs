use rand::prelude::*;
use std::{thread, time};
use serde_json::{Value};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct Image
{
    pub id: String,
    pub title: String,
    pub description: String,
    pub datetime: i64,
    pub file_type: String,
    pub animated: bool,
    pub width: i64,
    pub height: i64,
    pub size: i64,
    pub views: i64,
    pub bandwidth: i64,
    pub deletehash: Option<String>,
    pub name: Option<String>,
    pub section: String,
    pub link: String,
    pub gifv: Option<String>,
    pub mp4: Option<String>,
    pub mp4_size: Option<i64>,
    pub looping: Option<bool>,
    pub favorite: bool,
    pub nsfw: bool,
    pub vote: String,
    pub in_gallery: bool,
}