use std::fmt;

// The gallery section to select
// Hot is an algorithmically determined gallery of recently popular images 
// Top is absolute upvotes in a period of time
// User is the most recent submissions to the gallery

#[derive(Debug, PartialEq, Eq)]
pub enum Section
{
    Hot,
    Top,
    User,
}

impl From<&str> for Section
{
    fn from(s: &str) -> Self
    {
         let mut target = String::from(s);
          target.make_ascii_lowercase();
            
          match target.as_str() {
           "hot" => {Section::Hot}
           "top" => {Section::Top}
           "user" => {Section::User}
            _ => {
            // deafult value
             Section::Hot 
            }
          }
    }
}

impl fmt::Display for Section
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            Section::Hot => write!(f, "hot"),
            Section::Top => write!(f, "top"),
            Section::User => write!(f, "user"),
        }
    }
}

// This sort enum determines how the results should be sorted
// Rising can only be used in the user gallery

#[derive(Debug, PartialEq, Eq)]
pub enum Sort
{
    Viral,
    Top,
    Time,
    Rising,
}

impl From<&str> for Sort
{
    fn from(s: &str) -> Self
    {
       let mut target = String::from(s); 
       target.make_ascii_lowercase();
       match target.as_str()
       {
            "viral" => {Sort::Viral}
            "top" => { Sort::Top }
            "time" => {Sort::Time}
            "rising" => { Sort::Rising }
            _ => {
               // default value
               Sort::Viral
            }
       }
    }
}

impl fmt::Display for Sort 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            Sort::Viral => write!(f, "viral"),
            Sort::Top => write!(f, "top"),
            Sort::Time => write!(f, "time"),
            Sort::Rising => write!(f, "rising"),
        }
    }
}

// The window is the time window in the top gallery
// This can be used to find the top of the day, week, etc

#[derive(Debug, PartialEq, Eq )]
pub enum Window
{
    Day,
    Week,
    Month,
    Year,
    All,
}

impl From<&str> for Window
{
    fn from(s: &str) -> Self
    {
        let mut target = String::from(s);
        target.make_ascii_lowercase();
        match target.as_str() {
            "day" => {Window::Day},
            "week" => {Window::Week},
            "month" => {Window::Month},
            "year" => {Window::Year},
            "all" => {Window::All},
            _ => {Window::Day},
        }
    }
}

impl fmt::Display for Window 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            Window::Day => write!(f, "day"),
            Window::Week => write!(f, "week"),
            Window::Month => write!(f, "month"),
            Window::Year => write!(f, "year"),
            Window::All => write!(f, "all"),
        }
    }
}

#[derive(Debug)]
pub struct Gallery
{
    pub section: Section,
    pub sort: Sort,
    pub page: usize,
    pub window: Window,
    pub client_id: String,
    pub show_viral: bool,
    pub nsfw: bool,
    pub album_previews: bool,
}

#[derive(Debug)]
pub enum GalleryError
{
    InvalidCombination(&'static str),
}

impl Gallery
{
    pub fn new(client_id: String) -> Gallery
    {
        // Create default url 
        Gallery
        {
            client_id,
            section: Section::Hot,
            sort: Sort::Viral,
            page: 1,
            window: Window::Day,
            show_viral: true,
            nsfw: false,
            album_previews: true,
        }
    }

    pub fn section(mut self, section: Section) -> Result<Gallery, GalleryError>
    {
       if self.sort == Sort::Rising && section != Section::User
       {
            Err(GalleryError::InvalidCombination("Sorting by rising is only valid when in the user section"))
       }
        else{
           self.section = section; 
           Ok(self)
        }
    }

    pub fn sort(mut self, sort: Sort) -> Result<Gallery, GalleryError>
    {
       if sort == Sort::Rising && self.section != Section::User
       {
            Err(GalleryError::InvalidCombination("Sorting by rising is only valid when in the user section"))
       }
        else{
           self.sort = sort; 
           Ok(self)
        }
    }

    pub fn page(mut self, page: usize) -> Gallery
    {
        self.page = page;
        self
    }

    pub fn window(mut self, window: Window) -> Result<Gallery, GalleryError>
    {
        if self.section != Section::Top
        {
            Err(GalleryError::InvalidCombination("Window is only a valid setting in the top section"))
        }
        else
            {
                self.window = window;
                Ok(self)
            }
        }

    pub fn nsfw(mut self, nsfw: bool) -> Gallery
    {
        self.nsfw = nsfw;
        self
    }

    pub fn show_viral(mut self, viral: bool) -> Gallery
    {
        self.show_viral = viral;
        self
    }

    pub fn album_previews(mut self, previews: bool) -> Gallery
    {
        self.album_previews = previews;
        self
    }

    pub fn build_url(&self) -> reqwest::RequestBuilder
    {
        let client = reqwest::Client::new();
        let req = client.request(reqwest::Method::GET, format!("https://api.imgur.com/3/gallery/{}/{}/{}/{}?{}&{}&{}", 
                self.section,
                self.sort,
                self.window,
                self.page,
                self.show_viral,
                self.nsfw,
                self.album_previews
        ));
        req.header(reqwest::header::AUTHORIZATION, format!("Client-ID {}", self.client_id))
    }
}


