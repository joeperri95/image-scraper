use structopt::StructOpt;
use std::path::{PathBuf};
use tokio::fs;
use tokio::task::JoinHandle;
use scraper::{Html, Selector};
use reqwest::{self, RequestBuilder};

mod imgur;
mod gallery;

#[derive(StructOpt, Debug)]
#[structopt(name="imgur-scraper")]
enum Opt{
    Random {
        #[structopt(short="i", long)]
        iterations: usize,
    },
    Search {
        term: String,
    },
    Subreddit {
        subreddit: String,
    },
    Gallery {
        client_id: String,

        #[structopt(short="s", long="sort", parse(from_str))]
        sort: Option<gallery::Sort>,

        #[structopt(short="x", long, parse(from_str))]
        section: Option<gallery::Section>,

        #[structopt(short="w", long, parse(from_str))]
        window: Option<gallery::Window>,

        #[structopt(short="p", long)]
        page: Option<usize>,

        #[structopt(short="n", long)]
        nsfw: Option<bool>,

        #[structopt(short="v", long)]
        show_viral: Option<bool>,

        #[structopt(short="a", long)]
        album_preview: Option<bool>,
    },
    Viral,
}

#[tokio::main]
async fn main() {
    
    let opt= Opt::from_args();
    
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
        Opt::Subreddit {subreddit} => {
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

// create a tokio task that downloads a file over http
async fn spawn_downloader(url: String, filename: String) -> JoinHandle<()> {
    tokio::task::spawn(async move {
        let resp = reqwest::get(url).await.unwrap();
        fs::write(filename, resp.bytes().await.unwrap()).await.unwrap();
    })
}

// take a list of urls and spawwn tasks to download each of them
async fn download_images(urls: Vec<String>, dir: &str) {
    let mut tasks: Vec<JoinHandle<()>> = Vec::new();
    for url in urls.into_iter()
    {
       // extract the actual resource from the url
       let stem = url.split('/').last().unwrap();

       let mut path = PathBuf::new(); 
       path.push(dir);
       
       if !path.is_dir() {
         fs::create_dir(&path).await.unwrap();
       }

       path.push(stem);

       // await the task creation not its conclusion
       println!("{}", path.as_path().display());
       tasks.push(spawn_downloader(url, path.as_path().display().to_string()).await);
    }

    for task in tasks
    {
        task.await.unwrap();
    }
}

// scrape image links from an imgur gallery
async fn scrape(url: &str) -> Vec<String> {
    let response = reqwest::get(url).await.unwrap();
    let document = Html::parse_document(&response.text().await.unwrap());
    let selector = Selector::parse("img").unwrap();
    let video_selector = Selector::parse("source").unwrap();
    let mut urllist = Vec::new();

    // Collect image links from the webpage
    for i in document.select(&selector)
    {
        if let Some(tag) = i.value().attr("src")
        {
            urllist.push(String::from("https:") + tag);
        }
    }

    // Get videos from webpage
    for i in document.select(&video_selector)
    {
        if let Some(source_type) = i.value().attr("type")
        {
            if source_type == "video/mp4" {  
                if let Some(tag) = i.value().attr("src")
                {
                    urllist.push(String::from(tag));
                }
            }
        }
    }

    urllist
}

async fn get_gallery_image_urls(req: RequestBuilder) -> Result<Vec<String>,Box<dyn std::error::Error>>
{
    let response: imgur::ApiResponse = req.send().await?.json().await?;

    let mut list = Vec::new();
      for i in response.data 
      {
          if i.is_album == false {
              let link = format!("{}", i.link).replace(r#"""#,""); 
              list.push(link);
          }
      }

      Ok(list)
}
