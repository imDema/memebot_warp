use warp::{filters::BoxedFilter,Filter,path};
use memebot_backend::models::*;

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "v1" / "tag" / ..)
        .boxed()
}

pub fn new_tag() -> BoxedFilter<(String, )> {
    warp::get()
        .and(path_prefix())
        .and(path("new"))
        .and(warp::path::param::<String>())
        .boxed()
}

pub fn get_all() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(path("all"))
        .boxed()
}
