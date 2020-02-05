
pub async fn new_tag(tagname: String) -> Result<impl warp::Reply, warp::Rejection> {
    memebot_backend::create_tag(&tagname)
        .map(|tag| warp::reply::json(&tag))
        .map_err(|_why| warp::reject::reject())
}

pub async fn get_all() -> Result<impl warp::Reply, warp::Rejection> {
    memebot_backend::all_tags()
        .map(|tags|{
            warp::reply::json(&tags)
        })
        .map_err(|_| warp::reject::reject())
}
