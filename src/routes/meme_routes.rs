use warp::{filters::BoxedFilter,Filter,path};
use memebot_backend::models::*;
use crate::json_body;

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "v1" / "meme" / ..)
        .boxed()
}

pub fn new_meme() -> BoxedFilter<(NewMeme, )> {
    warp::post()
        .and(path_prefix())
        .and(path("new"))
        .and(json_body!())
        .boxed()
}

pub fn add_tag() -> BoxedFilter<(MemeTag, )> {
    warp::post()
    .and(path_prefix())
    .and(path("add_tag"))
    .and(json_body!())
    .boxed()
}

pub fn action() -> BoxedFilter::<(NewAction, )> {
    warp::post()
        .and(path_prefix())
        .and(path("action"))
        .and(json_body!())
        .boxed()
}

pub fn get_memes() -> BoxedFilter<()>{
    warp::get()
        .and(path_prefix())
        .boxed()
}

pub fn all() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(path("all"))
        .boxed()
}

pub fn order() -> BoxedFilter<(String, )> {
    warp::path("order")
        .and(warp::path::param::<String>())
        .boxed()
}

pub fn filter_userid() -> BoxedFilter<(i32, )> {
    warp::path("user")
        .and(warp::path::param::<i32>())
        .boxed()
}

pub fn filter_tagid() -> BoxedFilter<(i32,)> {
    warp::path("tag")
        .and(warp::path::param::<i32>())
        .boxed()
}