// Make sure the extension is good
// Check mime types
// Make sure file names are friendly

use std::collections::{HashSet, HashMap};

lazy_static! {
    pub static ref SELECTED_MIME_TYPES: HashSet<String> = {
        let mut m = HashSet::new();
        m.insert("image/jpeg".to_string());
        m.insert("image/png".to_string());
        m.insert("image/gif".to_string());
        m.insert("video/mp4".to_string());
        m
    }; 
}

// Return the results of a head request 
pub async fn get_content_type(url: &str) -> String {
    let client = reqwest::Client::new();
    let req = client.head(url);
    let resp = req.send().await.expect("Request for url failed");

    let mime_type = String::from(resp.headers()["content-type"].to_str().unwrap());

    mime_type
}

pub fn get_file_extension_from_mime_type(mime: &str) -> Option<String>
{
    match mime {
        "image/jpeg" => {Some(".jpg".to_string())}
        "image/png" => {Some(".png".to_string())}
        "image/gif" => {Some(".gif".to_string())}
        "video/mp4" => {Some(".mp4".to_string())}
        _ => {None}
    }
}

pub async fn fix_file_extension(name: &str, url: &str) -> Option<String>
{
    let mime_type = get_content_type(&url).await;
    if SELECTED_MIME_TYPES.contains(&mime_type)
    {
        if let Some(ext) = get_file_extension_from_mime_type(&mime_type)
        {
            return Some(name.to_owned() + &ext);
        }
    } 

    None
}

pub async fn fix_file_extensions(files: HashMap<String, String>) -> HashMap<String, String>
{
    let mut fixed_files: HashMap<String, String> = HashMap::new();
    
    for (name, url) in files
    {
        if let Some(name) = fix_file_extension(&name, &url).await
        {
            fixed_files.insert(name, url);
        }
    }

    fixed_files
}

// remove quotes from string
pub fn trim_quotation_marks(link: &str) -> String
{
    link.to_string().replace(r#"""#,"")
}

// convert a string friendly filename
// remove unicode
// remove punctuation
// replace whitespace with underscores
pub fn condition_filename(title: &str) -> String
{        
    let mut res: String = title.chars()
        .filter(|x| x.is_ascii())
        .filter(|x| !x.is_ascii_punctuation())        
        .collect();

    res = String::from(res.trim_end());
    res.replace(' ', "_").to_ascii_lowercase()
}
