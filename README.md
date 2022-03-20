# Image Scraper

Mass download images, gifs and mp4 files from imgur and reddit.

Useful to gather semantically related media for data science applications.

## Subcommands

### Subreddit

Scrape media from the specified subreddit

Usage
```
cargo run -- reddit subreddit [-x section] [-w window] [-c count]

-x section
    The section specifies how posts are sorted.  
    Allowed options are <hot|new|top|rising>

-w window
    When using the top section, the window parameter 
    specifies the time window to use. Has no effect with  
    other section arguments.
    Allowed options are <hour|day|week|month|year|all> 

-c count 
    Not currently implemented

subreddit
    The subreddit to scrape
```

### Reddit Search

Perform a search for a term on reddit and scrape the resulting media

Usage
```
cargo run -- reddit-search term [-s sort] [-w window] [-c count]

-s sort
    This parameter specifies how posts are sorted.  
    Allowed options are <hot|new|top|rising|relevant|comments>

-w window
    When sorting by top, relevance, or the most comments, the window parameter 
    specifies the time window to use. Has no effect with  
    other sort arguments.
    Allowed options are <hour|day|week|month|year|all> 

-c count 
    Not currently implemented

term
    Your search term
```

### Gallery

Scrape images from the public imgur gallery.


Usage
```
cargo run -- gallery [-x section] [-s sort] [-w window] [-p page] [-n nsfw] [-a album-preview] [-v show-viral] client-id

-x section
    Select which section of the gallery to access.
    Allowed options are <hot|top|user>

-s sort
    This parameter specifies how posts are sorted.  
    Allowed options are <viral|top|time|rising>

-w window
    When sorting by top, the date range of the request.
    Has no effect with all other sort arguments.
    Allowed options are <day|week|month|year|all>

-p page
    Not currently implemented

client-id
    Your imgur api client ID

Deprecated options

-n nsfw
-a album-preview
-v show-viral
```

### Random

Randomly generate imgur urls and download them.

Imgur used to use a 5 digit hash for each image. The random subcommand will randomly generate these 5 digit hashes and retrieve the image at that location. 

Imgur has since moved to 7 digit hashes which have a very low hit rate. 

CAUTION: This subcommand includes images which are not in the public gallery and may return some unsavoury content.

Usage
```
cargo run -- random -i iterations

-i iterations
    The number of files to download
```

### Subreddit Gallery

Download images from a particular subreddit gallery on imgur

Usage
```
cargo run -- subreddit-gallery [-s sort] [-w window] [-p page] subreddit client-id

-s sort
    This parameter specifies how posts are sorted.  
    Allowed options are <viral|top|time|rising>

-w window
    When sorting by top, the date range of the request.
    Has no effect with all other sort arguments.
    Allowed options are <day|week|month|year|all>

-p page
    Not currently implemented

subreddit
    The subreddit to scrape

client-id
    Your imgur api client ID
```


### Imgur Search

Make a search query and download the results

Usage
```
cargo run -- imgur-search [-s sort] [-w window] [-p page] term client-id

-s sort
    This parameter specifies how posts are sorted.  
    Allowed options are <viral|top|time|rising>

-w window
    When sorting by top, the date range of the request.
    Has no effect with all other sort arguments.
    Allowed options are <day|week|month|year|all>

-p page
    Not currently implemented

term
    Your search term

client-id
    Your imgur api client ID
```