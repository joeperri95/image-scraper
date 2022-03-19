use std::{fmt};

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

impl Section
{
    pub fn from(s: &str) -> Result<Self, String>
    {
         let mut target = String::from(s);
          target.make_ascii_lowercase();
            
          match target.as_str() {
           "hot" => {Ok(Section::Hot)}
           "top" => {Ok(Section::Top)}
           "user" => {Ok(Section::User)}
            _ => {
            // deafult value
                Err("Valid section options are <hot|top|user>".to_string())
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

impl Sort
{
    pub fn from(s: &str) -> Result<Self, String>
    {
       let mut target = String::from(s); 
       target.make_ascii_lowercase();
       match target.as_str()
       {
            "viral" => {Ok(Sort::Viral)}
            "top" => { Ok(Sort::Top)}
            "time" => {Ok(Sort::Time)}
            "rising" => { Ok(Sort::Rising) }
            _ => {
               Err("Valid sort options are <viral|top|time|rising>".to_string())
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

impl Window
{
    pub fn from(s: &str) -> Result<Self,String>
    {
        let mut target = String::from(s);
        target.make_ascii_lowercase();
        match target.as_str() {
            "day" => {Ok(Window::Day)},
            "week" => {Ok(Window::Week)},
            "month" => {Ok(Window::Month)},
            "year" => {Ok(Window::Year)},
            "all" => {Ok(Window::All)},
            _ => {Err("Valid window options are <day|week|month|year|all>".to_string())},
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