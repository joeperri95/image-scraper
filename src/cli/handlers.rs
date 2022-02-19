use super::opt::Opt;

pub fn handle_args(opt: Opt)
{
    match opt
    {
        Opt::Random {iterations} => {
            
            let mut tasks = Vec::default();
            for i in 0..iterations
            {
                tasks.push(tokio::task::spawn(async move{
                    println!("Task {} started", i);
                    let index = i;
                    let filename = format!("output/image-{}.png", index);
                    imgur::download_random_imgur_file(filename.as_str()).await.unwrap();
                    println!("Task {} complete", index);
                }));
            }
            for task in tasks{
                task.await.unwrap();
            }
        }
        Opt::Viral => {
            let url = "https://imgur.com/gallery".to_owned(); 
            let urllist = scrape(&url).await;
            download_images(urllist, "output/viral").await;
        },
        Opt::Gallery {client_id, sort, section, window, page, nsfw, show_viral, album_preview} => {
        let mut url = gallery::Gallery::new(client_id);

        if let Some(s) = sort {url =url.sort(s).unwrap()};
        if let Some(x) = section {url = url.section(x).unwrap()};
        if let Some(w) = window {url = url.window(w).unwrap()};
        if let Some(p) = page {url = url.page(p)};
        if let Some(n) = nsfw {url = url.nsfw(n)};
        if let Some(v) = show_viral {url = url.show_viral(v)};
        if let Some(a) = album_preview {url = url.album_previews(a)};
            
        let req = url.build_url();
        let urllist = get_gallery_image_urls(req).await; 
        download_images(urllist.unwrap(), "output").await; 
        
        }
        Opt::Subreddit {client_id, subreddit, sort, window, page} => {
            let mut url: SubredditGallery = subreddit_gallery::SubredditGallery::new(client_id, &subreddit);

            if let Some(s) = sort {url =url.sort(s)};            
            if let Some(w) = window {url = url.window(w)};
            if let Some(p) = page {url = url.page(p)};
        
            let req = url.build_url();            
            println!("{:?}", req);
            let urllist = get_gallery_image_urls(req).await; 
            println!("{:?}", urllist);
            download_images(urllist.unwrap(), &format!("output/{}", &subreddit)).await; 
            
        }
        Opt::SubredditOld {subreddit} => {
            let url = "https://imgur.com/r/".to_owned() + &subreddit;
            let urllist = scrape(&url).await;
            download_images(urllist, &format!("output/{}", &subreddit)).await;
        },
        Opt::Search {term} => {
            // let url = "https://imgur.com/gallery/hot/viral";
            let url = "https://imgur.com/search?q=".to_owned() + &term;
            let urllist = scrape(&url).await;
            download_images(urllist, &format!("output/{}", &term)).await;
        }
    };
}