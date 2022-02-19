use std::path::{PathBuf};
use tokio::fs;
use tokio::task::JoinHandle;

// create a tokio task that downloads a file over http
pub async fn spawn_downloader(url: String, filename: String) -> JoinHandle<()> {
    tokio::task::spawn(async move {
        let resp = reqwest::get(url).await.expect("Request for url failed");
        fs::write(filename, resp.bytes().await.unwrap()).await.unwrap();
    })
}

// take a list of urls and spawwn tasks to download each of them
pub async fn download_files(urls: Vec<String>, dir: &str) {
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