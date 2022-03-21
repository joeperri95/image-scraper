use rand::prelude::*;
use std::{thread, time};

use crate::conditioner::fix_file_extension;

fn generate_random_hash(iterations: usize) -> Option<String>
{

    let chars = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let mut hash = String::new();
    let mut rng = thread_rng();

    for _i in 0..iterations
    {
        hash += &chars.get(rng.gen_range(0..chars.len()))?.to_string();
    }

    Some(hash)
}

// Construct a url for a random imgur image
fn generate_imgur_url() -> Result<String, String>
{
    let mut url = "https://i.imgur.com/".to_string();

     // More jpg hits than png
     // Most png files get transcoded to jpeg anyway
     // Need to save file with the right extension

    let hash;
    match generate_random_hash(5)
    {
        Some (h) => {hash = h}
        None => {return Err("Could not generate random hash".to_string())}
    }
    url += &hash;
    url += ".jpg";
    Ok(url)
}

// Construct a url for a random imgur image
pub async fn get_random_imgur_url() -> Result<String, Box<dyn std::error::Error>>
{

    let mut url = generate_imgur_url()?;
    let mut req = reqwest::get(url.clone()).await?;
    let mut count = 0;
    while req.url().path() == "/removed.png" || req.status().is_client_error()
    {
        
        url = generate_imgur_url()?; 
        req = reqwest::get(url.clone()).await?;
        
        count += 1;
        println!("url: {} attempt: {}", url, count);
         
        // sleep for a bit to be nice to imgur
        thread::sleep(time::Duration::from_millis(10));
    }

    // fix extensions
    let name = url.replace(".jpg", "");
    url = fix_file_extension(&name, &url).await?;
    Ok(url)
}