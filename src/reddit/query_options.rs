use std::{fmt};

#[derive(Debug, PartialEq, Eq)]
pub enum Section
{
    Hot,
    Top,
    New,
    Rising,
}

impl Section
{
    pub fn from(s: &str) -> Result<Self, String>
    {
         let mut target = String::from(s);
          target.make_ascii_lowercase();
            
          match target.as_str() {
           "hot" => {Ok(Section::Hot)}
           "top" => {Ok(Section::Top)}
           "new" => {Ok(Section::New)}
           "rising" => {Ok(Section::Rising)}
            _ => {
            // deafult value
                Err("Valid section options are <hot|top|new|rising>".to_string())
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
            Section::New => write!(f, "new"),
            Section::Rising => write!(f, "rising"),
        }
    }
}

// The window is the time window in the top gallery
// This can be used to find the top of the day, week, etc

#[derive(Debug, PartialEq, Eq )]
pub enum Window
{
    Hour,
    Day,
    Week,
    Month,
    Year,
    All,
}

impl Window
{
    pub fn from(s: &str) -> Result<Self,String>
    {
        let mut target = String::from(s);
        target.make_ascii_lowercase();
        match target.as_str() {
            "now" => {Ok(Window::Hour)},
            "hour" => {Ok(Window::Hour)},
            "day" => {Ok(Window::Day)},
            "week" => {Ok(Window::Week)},
            "month" => {Ok(Window::Month)},
            "year" => {Ok(Window::Year)},
            "all" => {Ok(Window::All)},
            _ => {Err("Valid window options are <now|day|week|month|year|all>".to_string())},
        }
    }
}

impl fmt::Display for Window 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            Window::Hour => write!(f, "now"),
            Window::Day => write!(f, "day"),
            Window::Week => write!(f, "week"),
            Window::Month => write!(f, "month"),
            Window::Year => write!(f, "year"),
            Window::All => write!(f, "all"),
        }
    }
}

// Sort options when conducting a search

#[derive(Debug, PartialEq, Eq)]
pub enum Sort
{
    Hot,
    Top,
    New,
    Relevant,
    Comments,
}

impl Sort
{
    pub fn from(s: &str) -> Result<Self, String>
    {
         let mut target = String::from(s);
          target.make_ascii_lowercase();
            
          match target.as_str() {
           "hot" => {Ok(Sort::Hot)}
           "top" => {Ok(Sort::Top)}
           "new" => {Ok(Sort::New)}
           "relevant" => {Ok(Sort::Relevant)}
           "comments" => {Ok(Sort::Comments)}
            _ => {
            // deafult value
                Err("Valid section options are <hot|top|new|relevant|comments>".to_string())
            }
          }
    }
}

impl fmt::Display for Sort
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            Sort::Hot => write!(f, "hot"),
            Sort::Top => write!(f, "top"),
            Sort::New => write!(f, "new"),
            Sort::Relevant => write!(f, "relevant"),
            Sort::Comments => write!(f, "comments"),
        }
    }
}
