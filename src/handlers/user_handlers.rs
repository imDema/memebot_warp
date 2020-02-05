use warp::{filters::BoxedFilter,Filter,path};
use memebot_backend::models::*;
use super::common::{reply_with_status, StatusCode};

pub async fn new_user(user: NewUser) -> Result<impl warp::Reply, warp::Rejection> {
    tokio::spawn(async {
    memebot_backend::create_user(user)
        .map(|()| reply_with_status(StatusCode::Created)
        )}).await.map_err(|_why| warp::reject::reject())

}

pub async fn get_all() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(path("all"))
        .boxed()
}
