use memebot_backend::models::*;
use memebot_backend::meme_query::MemeQuery;
use super::common::{reply_with_status, StatusCode};

pub async fn new_meme(meme: NewMeme) ->Result<impl warp::Reply, warp::Rejection> {
    memebot_backend::create_meme(meme)
        .map(|_| reply_with_status(StatusCode::Created))
        .map_err(|_why| warp::reject())
}

pub async fn add_tag(meme_tag: MemeTag) -> Result<impl warp::Reply, warp::Rejection> {
    memebot_backend::add_meme_tag(meme_tag.memeid, meme_tag.tagid)
        .map(|_| reply_with_status(StatusCode::Created))
        .map_err(|_why| warp::reject())
}

pub async fn action(action: NewAction) -> Result<impl warp::Reply, warp::Rejection> {
    memebot_backend::new_action(action)
        .map(|_| reply_with_status(StatusCode::Created))
        .map_err(|_| warp::reject())
}

pub fn init_query() -> MemeQuery {
    MemeQuery::new()
}

pub fn order(query: MemeQuery, by: String) -> MemeQuery {
    match by.as_str() {
        "heat" => query.order_hot(),
        "score" => query.order_rating(),
        "date" => query.order_date(),
        _ => query,
    }
}

pub fn filter_userid(query: MemeQuery, userid: i32) -> MemeQuery {
    query.by_user(userid)
}

pub fn filter_tagid(query: MemeQuery, tagid: i32) -> MemeQuery {
    query.by_tag(tagid)
}

pub async fn exec(query: MemeQuery) -> Result<impl warp::Reply, warp::Rejection> {
    query.execute()
        .map(|memes| warp::reply::json(&memes))
        .map_err(|_| warp::reject())
}