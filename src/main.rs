use structopt::StructOpt;

mod imgur;
mod gallery;
mod download;
mod url_builder;
mod reddit;
mod conditioner;

use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

mod models {pub mod image_post; pub mod album; pub mod image;pub mod listing;}
mod cli {pub mod opt;}
use cli::opt::Opt;

#[tokio::main]
async fn main() {

    let opt= Opt::from_args();    
    
    match opt
    {
        Opt::Random {iterations} => {
                        
            let mut total = 0;
            for _ in 0..iterations
            {               
                let url = imgur::get_random_imgur_url().await.unwrap();
                println!("{:?}", url);
                download::download_file(&url, "output").await;

                total += 1;
                println!("{} of {} complete", total, iterations);
            
            }            
        }        
        Opt::Reddit {subreddit} =>
        {            
            match reddit::get_posts(&subreddit).await
            {
                Ok(files) => {
                    let mut selected_files: HashMap<String, String> = HashMap::new();
                    for (name, url) in files
                    {
                        let mime_type = conditioner::get_content_type(&url).await;
                        if conditioner::SELECTED_MIME_TYPES.contains(&mime_type)
                        {
                            if let Some(ext) = conditioner::get_file_extension_from_mime_type(&mime_type)
                            {
                                selected_files.insert(name + &ext, url);
                            }
                        }
                        
                    }

                    download::download_files_named(selected_files, &format!("output/{}", subreddit)).await;
                },
                Err(e) => {
                    println!("Could not get posts from subreddit {}", subreddit);
                },
            };
        }
        Opt::Gallery {client_id, sort, section, window, page, nsfw, show_viral, album_preview} => {

            let mut gallery_url_options = url_builder::GalleryUrl::default();
            if let Some(s) = sort {gallery_url_options.sort = Some(gallery::Sort::from(s.as_str()).unwrap())};
            if let Some(x) = section {gallery_url_options.section = Some(gallery::Section::from(x.as_str()).unwrap())};
            if let Some(w) = window {gallery_url_options.window = Some(gallery::Window::from(w.as_str()).unwrap())};
            if let Some(p) = page {gallery_url_options.page = Some(p as i64)};
            if let Some(n) = nsfw {gallery_url_options.mature = Some(n)};
            if let Some(v) = show_viral {gallery_url_options.viral = Some(v)};
            if let Some(a) = album_preview {gallery_url_options.album_previews = Some(a)};
                    
            let url = url_builder::build_url(url_builder::UrlTarget::Gallery {url: gallery_url_options});            
            let mut client = imgur::ImgurClient::new(&client_id);     
            let posts = client.gallery_request(&url).await.unwrap();       
            let urllist = client.get_gallery_image_urls(posts).await;             
            download::download_files(urllist, "output").await; 

        }
        Opt::Subreddit {client_id, subreddit, sort, window, page} => {

            let mut subreddit_url_options = url_builder::SubredditUrl
            {
                subreddit: subreddit.clone(),
                sort: None,
                page: None,
                window: None,
            };
         
            if let Some(s) = sort {subreddit_url_options.sort = Some(gallery::Sort::from(s.as_str()).unwrap())};            
            if let Some(w) = window {subreddit_url_options.window = Some(gallery::Window::from(w.as_str()).unwrap())};
            if let Some(p) = page {subreddit_url_options.page = Some(p as i64)};
        
            let url = url_builder::build_url(url_builder::UrlTarget::Subreddit {url: subreddit_url_options});
            let mut client = imgur::ImgurClient::new(&client_id);     
            let posts = client.gallery_request(&url).await.unwrap();       
            let urllist = client.get_gallery_image_urls(posts).await;                   
            download::download_files(urllist, &format!("output/{}", subreddit)).await; 

        }
        Opt::Search {client_id, term, sort, window, page} => {
            let mut search_url_option = url_builder::SearchUrl{
                term: Some(term.clone()),
                sort: None,
                page: None,
                window: None,
            };

            if let Some(s) = sort {search_url_option.sort = Some(gallery::Sort::from(s.as_str()).unwrap())};            
            if let Some(w) = window {search_url_option.window = Some(gallery::Window::from(w.as_str()).unwrap())};
            if let Some(p) = page {search_url_option.page = Some(p as i64)};
        
            let url = url_builder::build_url(url_builder::UrlTarget::Search {url: search_url_option});
            let mut client = imgur::ImgurClient::new(&client_id);     
            let posts = client.gallery_request(&url).await.unwrap();       
            let urllist = client.get_gallery_image_urls(posts).await;     
            download::download_files(urllist, &format!("output/{}", term)).await;     
        }
    };
}
