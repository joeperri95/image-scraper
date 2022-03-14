use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name="imgur-scraper")]
pub enum Opt{
    Random {
        #[structopt(short="i", long)]
        iterations: usize,
    }, 
    Reddit {
        subreddit: String,
    },
    Subreddit {
        client_id: String,

        subreddit: String,

        #[structopt(short="s", long="sort")]
        sort: Option<String>,

        #[structopt(short="w", long)]
        window: Option<String>,

        #[structopt(short="p", long)]
        page: Option<usize>,
    },
    Search {
        client_id: String,

        term: String,

        #[structopt(short="s", long="sort")]        
        sort: Option<String>,

        #[structopt(short="w", long)]
        window: Option<String>,

        #[structopt(short="p", long)]
        page: Option<usize>,
    },
    Gallery {
        client_id: String,

        #[structopt(short="s", long="sort")]        
        sort: Option<String>,

        #[structopt(short="x", long)]
        section: Option<String>,

        #[structopt(short="w", long)]
        window: Option<String>,

        #[structopt(short="p", long)]
        page: Option<usize>,

        #[structopt(short="n", long)]
        nsfw: Option<bool>,

        #[structopt(short="v", long)]
        show_viral: Option<bool>,

        #[structopt(short="a", long)]
        album_preview: Option<bool>,
    },    
}
