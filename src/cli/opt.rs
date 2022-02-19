use structopt::StructOpt;
use crate::gallery;

#[derive(StructOpt, Debug)]
#[structopt(name="imgur-scraper")]
pub enum Opt{
    Random {
        #[structopt(short="i", long)]
        iterations: usize,
    },
    SearchOld {
        term: String,
    },
    Subreddit {
        client_id: String,

        subreddit: String,

        #[structopt(short="s", long="sort", parse(from_str))]
        sort: Option<gallery::Sort>,

        #[structopt(short="w", long, parse(from_str))]
        window: Option<gallery::Window>,

        #[structopt(short="p", long)]
        page: Option<usize>,
    },
    Search {
        client_id: String,

        term: String,

        #[structopt(short="s", long="sort", parse(from_str))]
        sort: Option<gallery::Sort>,

        #[structopt(short="w", long, parse(from_str))]
        window: Option<gallery::Window>,

        #[structopt(short="p", long)]
        page: Option<usize>,
    },
    SubredditOld {
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
