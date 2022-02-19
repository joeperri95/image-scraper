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