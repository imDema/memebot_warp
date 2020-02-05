use memebot_backend::models::*;
use super::common::{reply_with_status, StatusCode};

pub async fn new_meme(meme: NewMeme) ->Result<impl warp::Reply, warp::Rejection> {
    memebot_backend::create_meme(meme)
        .map(|_| reply_with_status(StatusCode::Created))
        .map_err(|_why| warp::reject::reject())
}

pub async fn add_tag(meme_tag: MemeTag) -> Result<impl warp::Reply, warp::Rejection> {
    memebot_backend::add_meme_tag(meme_tag.memeid, meme_tag.tagid)
        .map(|_| reply_with_status(StatusCode::Created))
        .map_err(|_why| warp::reject::reject())
}

pub async fn get_all() -> Result<impl warp::Reply, warp::Rejection> {
    memebot_backend::all_memes()
        .map(|memes|{
            warp::reply::json(&memes)
        })
        .map_err(|_| warp::reject::reject())
}

pub async fn by_user(userid: i32) -> Result<impl warp::Reply, warp::Rejection> {
    memebot_backend::memes_by_userid(userid)
        .map(|memes|{
            warp::reply::json(&memes)
        })
        .map_err(|_| warp::reject::reject())
}

pub async fn by_tag(tagid: i32) -> Result<impl warp::Reply, warp::Rejection> {
    memebot_backend::memes_by_userid(tagid)
        .map(|memes|{
            warp::reply::json(&memes)
        })
        .map_err(|_| warp::reject::reject())
}

pub async fn action(action: NewAction) -> Result<impl warp::Reply, warp::Rejection> {
    memebot_backend::new_action(action)
        .map(|_|{
            reply_with_status(StatusCode::Created)
        })
        .map_err(|_| warp::reject::reject())
}