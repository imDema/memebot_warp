pub enum StatusCode {
    ServerError,
    Created,
    Ok,
}

#[inline]
pub fn reply_with_status(sc: StatusCode) -> impl warp::reply::Reply {
    use warp::http::StatusCode as WarpCode;
    let rep = warp::reply();

    let proper_status_code = match sc {
        StatusCode::ServerError => WarpCode::INTERNAL_SERVER_ERROR,
        StatusCode::Created => WarpCode::CREATED,
        StatusCode::Ok => WarpCode::OK,
    };

    warp::reply::with_status(rep, proper_status_code)
}