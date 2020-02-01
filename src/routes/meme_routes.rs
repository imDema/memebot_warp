use warp::{filters::BoxedFilter,Filter,path};
use memebot_backend::models::*;

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "v1" / "meme" / ..)
        .boxed()
}

pub fn new_meme() -> BoxedFilter<(NewMeme, )> {
    warp::post()
        .and(path_prefix())
        .and(path("new"))
        .and(warp::body::content_length_limit(4 * 1024))
        .and(warp::body::json())
        .boxed()
}

pub fn add_tag() -> BoxedFilter<(MemeTag, )> {
    warp::post()
    .and(path_prefix())
    .and(path("add_tag"))
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

pub fn by_user() -> BoxedFilter<(i32, )> {
    warp::get()
        .and(path_prefix())
        .and(path("user"))
        .and(warp::path::param::<i32>())
        .boxed()
}

pub fn by_tag() -> BoxedFilter<(i32, )> {
    warp::get()
        .and(path_prefix())
        .and(path("tag"))
        .and(warp::path::param::<i32>())
        .boxed()
}

pub fn action() -> BoxedFilter::<(NewAction, )> {
    warp::post()
        .and(path_prefix())
        .and(path("action"))
        .and(warp::body::json())
        .boxed()
}