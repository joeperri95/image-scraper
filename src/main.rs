use structopt::StructOpt;

mod download;
mod conditioner;

#[macro_use]
extern crate lazy_static;

mod imgur {pub mod imgur; pub mod query_options; pub mod random; pub mod url_builder; mod models {pub mod image_post; pub mod album; pub mod image;}}
mod reddit {pub mod reddit; pub mod query_options; pub mod url_builder; mod models{pub mod listing;}}
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
                let url = imgur::random::get_random_imgur_url().await.unwrap();                
                download::download_file(&url, "output").await;

                total += 1;
                println!("{} of {} complete", total, iterations);            
            }            
        }        
        Opt::Reddit {subreddit, section, window, count} =>
        {            
            let mut gallery_url_options = reddit::url_builder::SubredditUrl 
            {
                subreddit: subreddit.clone(),
                section: None,
                window: None,
                before: None,
                after: None,
            };
            
            if let Some(x) = section {gallery_url_options.section = Some(reddit::query_options::Section::from(x.as_str()).unwrap())};
            if let Some(w) = window {gallery_url_options.window = Some(reddit::query_options::Window::from(w.as_str()).unwrap())};
            
            if let Some(_c) = count {
                // not yet implemented
            };
            
            let url = reddit::url_builder::build_url(reddit::url_builder::UrlTarget::Subreddit {url: gallery_url_options});

            let reddit = reddit::reddit::RedditClient::new("skrrt");
            match reddit.get_posts(&url).await
            {
                Ok(files) => {
                    let selected_files = conditioner::fix_file_extensions(files).await;                    
                    download::download_files_named(selected_files, &format!("output/{}", subreddit)).await;
                },
                Err(err) => {
                    println!("Could not get posts from subreddit {}", subreddit);
                    println!("Error Message: {}", err);
                },
            };
        }
        Opt::RedditSearch {term, sort, window, count} =>
        {            
            let mut gallery_url_options = reddit::url_builder::SearchUrl
            {
                term: term.clone(),
                sort: None,
                window: None,
                before: None,
                after: None,
            };
            
            if let Some(s) = sort {gallery_url_options.sort = Some(reddit::query_options::Sort::from(s.as_str()).unwrap())};
            if let Some(w) = window {gallery_url_options.window = Some(reddit::query_options::Window::from(w.as_str()).unwrap())};
           
            if let Some(_c) = count {
                // not yet implemented
            };
            
            let url = reddit::url_builder::build_url(reddit::url_builder::UrlTarget::Search {url: gallery_url_options});

            let reddit = reddit::reddit::RedditClient::new("skrrt");
            match reddit.get_posts(&url).await
            {
                Ok(files) => {
                    let selected_files = conditioner::fix_file_extensions(files).await;                    
                    download::download_files_named(selected_files, &format!("output/{}", term)).await;
                },
                Err(err) => {
                    println!("Could not get posts for search term {}", term);
                    println!("Error Message: {}", err);
                },
            };
        }
        Opt::Gallery {client_id, sort, section, window, page, nsfw, show_viral, album_preview} => {

            let mut gallery_url_options = imgur::url_builder::GalleryUrl::default();
            if let Some(s) = sort {gallery_url_options.sort = Some(imgur::query_options::Sort::from(s.as_str()).unwrap())};
            if let Some(x) = section {gallery_url_options.section = Some(imgur::query_options::Section::from(x.as_str()).unwrap())};
            if let Some(w) = window {gallery_url_options.window = Some(imgur::query_options::Window::from(w.as_str()).unwrap())};
            if let Some(p) = page {gallery_url_options.page = Some(p as i64)};
            if let Some(n) = nsfw {gallery_url_options.mature = Some(n)};
            if let Some(v) = show_viral {gallery_url_options.viral = Some(v)};
            if let Some(a) = album_preview {gallery_url_options.album_previews = Some(a)};
                    
            let url = imgur::url_builder::build_url(imgur::url_builder::UrlTarget::Gallery {url: gallery_url_options});            
            let mut client = imgur::imgur::ImgurClient::new(&client_id);
            let posts = client.gallery_request(&url).await.unwrap();       
            let urllist = client.get_gallery_image_urls(posts).await;             
            download::download_files(urllist, "output").await; 

        }
        Opt::Subreddit {client_id, subreddit, sort, window, page} => {

            let mut subreddit_url_options = imgur::url_builder::SubredditUrl
            {
                subreddit: subreddit.clone(),
                sort: None,
                page: None,
                window: None,
            };
         
            if let Some(s) = sort {subreddit_url_options.sort = Some(imgur::query_options::Sort::from(s.as_str()).unwrap())};            
            if let Some(w) = window {subreddit_url_options.window = Some(imgur::query_options::Window::from(w.as_str()).unwrap())};
            if let Some(p) = page {subreddit_url_options.page = Some(p as i64)};
        
            let url = imgur::url_builder::build_url(imgur::url_builder::UrlTarget::Subreddit {url: subreddit_url_options});
            let mut client = imgur::imgur::ImgurClient::new(&client_id);     
            let posts = client.gallery_request(&url).await.unwrap();       
            let urllist = client.get_gallery_image_urls(posts).await;                   
            download::download_files(urllist, &format!("output/{}", subreddit)).await; 

        }
        Opt::Search {client_id, term, sort, window, page} => {
            let mut search_url_option = imgur::url_builder::SearchUrl{
                term: Some(term.clone()),
                sort: None,
                page: None,
                window: None,
            };

            if let Some(s) = sort {search_url_option.sort = Some(imgur::query_options::Sort::from(s.as_str()).unwrap())};            
            if let Some(w) = window {search_url_option.window = Some(imgur::query_options::Window::from(w.as_str()).unwrap())};
            if let Some(p) = page {search_url_option.page = Some(p as i64)};
        
            let url = imgur::url_builder::build_url(imgur::url_builder::UrlTarget::Search {url: search_url_option});
            let mut client = imgur::imgur::ImgurClient::new(&client_id);     
            let posts = client.gallery_request(&url).await.unwrap();       
            let urllist = client.get_gallery_image_urls(posts).await;     
            download::download_files(urllist, &format!("output/{}", term)).await;     
        }
    };
}