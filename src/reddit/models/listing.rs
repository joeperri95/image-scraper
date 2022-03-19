use serde_json::{Value};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Listing
{
   pub kind: Value,
   pub data: ListingData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingData
{
   pub after: Value,
   pub dist: Value,
   pub modhash: Value,   pub geo_filter: Value,
   pub children: Vec<RedditPost>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditPost
{
   pub kind: Value,
   pub data: RedditPostData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditPostData
{
    pub archived : Option<Value>,
    pub author : Option<Value>,
    pub author_fullname : Option<Value>,
    pub author_is_blocked : Option<Value>,
    pub banned_at_utc : Option<Value>,
    pub banned_by : Option<Value>,
    pub can_gild : Option<Value>,
    pub can_mod_post : Option<Value>,
    pub category : Option<Value>,
    pub clicked : Option<Value>,
    pub content_categories : Option<Value>,
    pub contest_mode : Option<Value>,
    pub created : Option<Value>,
    pub created_utc : Option<Value>,
    pub discussion_type : Option<Value>,
    pub distinguished : Option<Value>,
    pub domain : Option<Value>,
    pub downs : Option<Value>,
    pub edited : Option<Value>,
    pub hidden : Option<Value>,
    pub hide_score : Option<Value>,
    pub id : Option<Value>,
    pub is_created_from_ads_ui : Option<Value>,
    pub is_crosspostable : Option<Value>,
    pub is_meta : Option<Value>,
    pub is_original_content : Option<Value>,
    pub is_reddit_media_domain : Option<Value>,
    pub is_robot_indexable : Option<Value>,
    pub is_self : Option<Value>,
    pub is_video : Option<Value>,
    pub likes : Option<Value>,
    pub media : Option<Value>,
    pub media_embed : Option<Value>,
    pub media_only : Option<Value>,
    pub name : Option<Value>,
    pub no_follow : Option<Value>,
    pub num_comments : Option<Value>,
    pub num_crossposts : Option<Value>,
    pub num_reports : Option<Value>,
    pub over_18 : Option<Value>,
    pub parent_whitelist_status : Option<Value>,
    pub permalink : Option<Value>,
    pub pinned : Option<Value>,    
    pub preview : Option<RedditPreview>,
    pub pwls : Option<Value>,
    pub quarantine : Option<Value>,
    pub removal_reason : Option<Value>,
    pub removed_by : Option<Value>,
    pub removed_by_category : Option<Value>,
    pub report_reasons : Option<Value>,
    pub saved : Option<Value>,
    pub score : Option<Value>,
    pub secure_media : Option<Value>,
    pub secure_media_embed : Option<Value>,
    pub selftext : Option<Value>,
    pub selftext_html : Option<Value>,
    pub send_replies : Option<Value>,
    pub subreddit : Option<Value>,
    pub subreddit_id : Option<Value>,
    pub subreddit_name_prefixed : Option<Value>,
    pub subreddit_subscribers : Option<Value>,
    pub subreddit_type : Option<Value>,
    pub suggested_sort : Option<Value>,
    pub thumbnail : Option<Value>,
    pub thumbnail_height : Option<Value>,
    pub thumbnail_width : Option<Value>,
    pub title : Option<Value>,
    pub top_awarded_type : Option<Value>,
    pub total_awards_received : Option<Value>,
    pub treatment_tags : Option<Value>,
    pub ups : Option<Value>,
    pub upvote_ratio : Option<Value>,
    pub url : Option<Value>,
    pub user_reports : Option<Value>,
    pub view_count : Option<Value>,
    pub visited : Option<Value>,
    pub whitelist_status : Option<Value>,
    pub wls : Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedditPreview
{
    pub reddit_video_preview: Option<RedditVideoPreview>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedditVideoPreview
{
    pub bitrate_kbps : Option<Value>,
    pub dash_url : Option<Value>,
    pub duration : Option<Value>,
    pub fallback_url : Option<Value>,
    pub height : Option<Value>,
    pub hls_url : Option<Value>,
    pub is_gif : Option<Value>,
    pub scrubber_media_url : Option<Value>,
    pub transcoding_status : Option<Value>,
    pub width : Option<Value>,
}