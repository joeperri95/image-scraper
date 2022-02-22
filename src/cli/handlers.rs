use super::opt::Opt;

pub fn handle_args(opt: Opt)
{
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
        Opt::Gallery {client_id, sort, section, window, page, nsfw, show_viral, album_preview} => {

            let mut gallery_url_options = url_builder::GalleryUrl::default();
            if let Some(s) = sort {gallery_url_options.sort = Some(s)};
            if let Some(x) = section {gallery_url_options.section = Some(x)};
            if let Some(w) = window {gallery_url_options.window = Some(w)};
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

            let mut subreddit_url_options = url_builder::SubredditUrl::default();
            
            subreddit_url_options.subreddit = subreddit.clone();
            if let Some(s) = sort {subreddit_url_options.sort = Some(s)};            
            if let Some(w) = window {subreddit_url_options.window = Some(w)};
            if let Some(p) = page {subreddit_url_options.page = Some(p as i64)};
        
            let url = url_builder::build_url(url_builder::UrlTarget::Subreddit {url: subreddit_url_options});
            let mut client = imgur::ImgurClient::new(&client_id);     
            let posts = client.gallery_request(&url).await.unwrap();       
            let urllist = client.get_gallery_image_urls(posts).await;                   
            download::download_files(urllist, &format!("output/{}", subreddit)).await; 

        }
        Opt::Search {client_id, term, sort, window, page} => {
            let mut search_url_option = url_builder::SearchUrl::default();

            search_url_option.term = Some(term.clone());
            if let Some(s) = sort {search_url_option.sort = Some(s)};            
            if let Some(w) = window {search_url_option.window = Some(w)};
            if let Some(p) = page {search_url_option.page = Some(p as i64)};
        
            let url = url_builder::build_url(url_builder::UrlTarget::Search {url: search_url_option});
            let mut client = imgur::ImgurClient::new(&client_id);     
            let posts = client.gallery_request(&url).await.unwrap();       
            let urllist = client.get_gallery_image_urls(posts).await;     
            download::download_files(urllist, &format!("output/{}", term)).await;     
        }
    }
}