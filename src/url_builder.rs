use super::gallery::{Sort, Section, Window};

#[derive(Default, PartialEq, Eq)]
pub struct SubredditUrl
{
    pub subreddit: String,
    pub sort: Option<Sort>,
    pub page: Option<i64>,
    pub window: Option<Window>,
}

#[derive(Default, PartialEq, Eq)]
pub struct GalleryUrl
{
    pub section: Option<Section>,
    pub sort: Option<Sort>,
    pub page: Option<i64>,
    pub window: Option<Window>,
    pub viral: Option<bool>,
    pub mature: Option<bool>,
    pub album_previews: Option<bool>,
}

#[derive(Default, PartialEq, Eq)]
pub struct SearchUrl
{
    pub term: Option<String>,
    pub sort: Option<Sort>,
    pub page: Option<i64>,
    pub window: Option<Window>,
}

#[derive(PartialEq, Eq)]
pub enum UrlTarget
{
    Subreddit{url: SubredditUrl},
    Gallery{url: GalleryUrl},
    Search{url: SearchUrl},
}

pub fn build_url(target: UrlTarget) -> String
{
    match target {
        UrlTarget::Subreddit {url} => {
            let mut final_url = format!("https://api.imgur.com/3/gallery/r/{}", url.subreddit);
            if let Some(sort) = url.sort {final_url += &format!("/{}", sort)};
            if let Some(window) = url.window {final_url += &format!("/{}", window)};
            if let Some(page) = url.page {final_url += &format!("/{}", page)};

            final_url
        }
        UrlTarget::Gallery {url} => {
            let mut final_url = "https://api.imgur.com/3/gallery".to_string();
            let mut query_prefix = '?';

            if let Some(section) = url.section {final_url += &format!("/{}", section)};
            if let Some(sort) = url.sort {final_url += &format!("/{}", sort)};
            if let Some(window) = url.window {final_url += &format!("/{}", window)};
            if let Some(page) = url.page {final_url += &format!("/{}", page)};
            if let Some(viral) = url.viral {                
                final_url += &format!("{}show_viral={}", query_prefix , viral);
                query_prefix = '&';
            };
            if let Some(mature) = url.mature {
                final_url += &format!("{}mature={}", query_prefix, mature);
                query_prefix = '&';
            };
            if let Some(album_previews) = url.album_previews {
                final_url += &format!("{}album_previews={}", query_prefix, album_previews);
            };

            final_url
        }
        UrlTarget::Search {url} => {
            let mut final_url = String::from("https://api.imgur.com/3/gallery/search/");
            
            if let Some(sort) = url.sort {final_url += &format!("/{}", sort)};
            if let Some(window) = url.window {final_url += &format!("/{}", window)};
            if let Some(page) = url.page {final_url += &format!("/{}", page)};
            if let Some(term) = url.term {
                final_url += &format!("?q={}", term);                
            };
            final_url
        }
    }
}
