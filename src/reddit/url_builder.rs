use crate::reddit::query_options::{Section, Sort, Window};

#[derive(Default, PartialEq, Eq)]
pub struct SubredditUrl
{
    pub subreddit: String,
    pub section: Option<Section>,    
    pub window: Option<Window>,
    pub before: Option<String>,
    pub after: Option<String>,
}

#[derive(Default, PartialEq, Eq)]
pub struct SearchUrl
{
    pub term: String,
    pub sort: Option<Sort>,    
    pub window: Option<Window>,
    pub before: Option<String>,
    pub after: Option<String>,
}

#[derive(PartialEq, Eq)]
pub enum UrlTarget
{
    Subreddit{url: SubredditUrl},
    Search{url: SearchUrl},
}

pub fn build_url(target: UrlTarget) -> String
{
    match target {
        UrlTarget::Subreddit {url} => {
            
            let mut final_url = format!("https://reddit.com/r/{}", url.subreddit);
            let mut query_prefix = '?';

            if let Some(section) = url.section {final_url += &format!("/{}", section)};                   
           
            final_url += "/.json";

            if let Some(window) = url.window {
                final_url += &format!("{}t={}", query_prefix, window);
                query_prefix = '&';
            };
            if let Some(before) = url.before {
                final_url += &format!("{}before={}", query_prefix, before);                
                query_prefix = '&';
            };
            if let Some(after) = url.after {
                final_url += &format!("{}after={}", query_prefix, after);                                
            };

            final_url
        }
        UrlTarget::Search {url} => {
            let mut final_url = format!("https://reddit.com/search/.json?q={}", url.term);
            
            if let Some(sort) = url.sort {
                final_url += &format!("&sort={}", sort);
            };
            if let Some(window) = url.window {
                final_url += &format!("&t={}", window);
            };             
            if let Some(before) = url.before {
                final_url += &format!("&before={}", before);                
            };
            if let Some(after) = url.after {
                final_url += &format!("&after={}", after);                
            };

            final_url
        }
    }
}
