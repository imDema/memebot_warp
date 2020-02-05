use memebot_backend::models::NewUser;
use super::common::{reply_with_status, StatusCode};

pub async fn new_user(user: NewUser) -> Result<impl warp::Reply, warp::Rejection> {
    memebot_backend::create_user(user)
        .map(|_| reply_with_status(StatusCode::Created))
        .map_err(|_why| warp::reject::reject())
}

pub async fn get_all() -> Result<impl warp::Reply, warp::Rejection> {
    memebot_backend::all_users()
        .map(|users|{
            warp::reply::json(&users)
        })
        .map_err(|_| warp::reject::reject())
}
