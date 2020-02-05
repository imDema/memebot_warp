use warp::{filters::BoxedFilter,Filter,path};
use memebot_backend::models::*;
use crate::json_body;

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "v1" / "user" / ..)
        .boxed()
}

pub fn new_user() -> BoxedFilter<(NewUser, )> {
    warp::post()
        .and(path_prefix())
        .and(path("new"))
        .and(json_body!())
        .boxed()
}

pub fn get_all() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(path("all"))
        .boxed()
}
