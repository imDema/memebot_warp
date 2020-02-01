use warp::{filters::BoxedFilter,Filter,path};
use memebot_backend::models::*;

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "v1" / "user" / ..)
        .boxed()
}

pub fn new_user() -> BoxedFilter<(NewUser, )> {
    warp::post()
        .and(path_prefix())
        .and(path("new"))
        .and(warp::body::content_length_limit(4 * 1024))
        .and(warp::body::json())
        .boxed()
}

pub fn get_all() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(path("all"))
        .boxed()
}
