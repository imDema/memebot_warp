use warp::{self, Filter, path};
use memebot_backend::models::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn_pool = memebot_backend::establish_connection_pool();

    let cp = conn_pool.clone();
    let new_action = path!("action" / "new")
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::json())
        .and_then(move |action: NewAction| {
            let conn = cp.clone().get().unwrap();
            async move {
            memebot_backend::new_action(&conn, action)
                .map(|()| reply_with_status(StatusCode::Created))
                .map_err(|_why| warp::reject::reject())
            }
        });

    let memes = path("memes");

    let cp = conn_pool.clone();
    let new_meme = memes.and(path("new"))
        .and(warp::body::content_length_limit(4 * 1024))
        .and(warp::body::json())
        .and_then(move |meme: NewMeme| {
            let conn = cp.clone().get().unwrap();
            async move {
            memebot_backend::create_meme(&conn, meme)
                .map(|()| reply_with_status(StatusCode::Created))
                .map_err(|_why| warp::reject())
            }
        });

    let cp = conn_pool.clone();
    let tag_meme = memes.and(path("add_tag"))
        .and(warp::body::json())
        .and_then(move |meme_tag: MemeTag| {
            let conn = cp.clone().get().unwrap();
            async move {
                memebot_backend::add_meme_tag(&conn, meme_tag.memeid, meme_tag.tagid)
                    .map(|_| reply_with_status(StatusCode::Created))
                    .map_err(|_err| warp::reject())
            }
        });

    let cp = conn_pool.clone();
    let memes_all = memes
        .and(path("all"))
        .map(move || {
            let conn = cp.clone().get().unwrap();
            let memes = memebot_backend::all_memes(&conn).unwrap();
            warp::reply::json(&memes)
        });

        
    let cp = conn_pool.clone();
    let memes_user = memes.and(path("user"))
        .and(path::param())
        .map(move |userid: i32| {
            let conn = cp.clone().get().unwrap();
            let memes = memebot_backend::memes_by_userid(&conn, userid).unwrap();
            warp::reply::json(&memes)
        });

    let cp = conn_pool.clone();
    let memes_tag = memes.and(path("tag"))
        .and(path::param())
        .map(move |tagid: i32| {
            let conn = cp.clone().get().unwrap();
            let memes = memebot_backend::memes_by_tagid(&conn, tagid).unwrap();
            warp::reply::json(&memes)
        });

    let users = path("users");
        
    let cp = conn_pool.clone();
    let new_user = users.and(path("new"))
        .and(warp::body::content_length_limit(4 * 1024))
        .and(warp::body::json())
        .and_then(move |user: NewUser| {
            let conn = cp.clone().get().unwrap();
            async move {
            memebot_backend::create_user(&conn, user)
                .map(|()| reply_with_status(StatusCode::Created))
                .map_err(|_why| warp::reject::reject())
            }
        });

    let cp = conn_pool.clone();
    let users_all = users.and(path("all"))
        .map(move ||{
            let conn = cp.clone().get().unwrap();
            let users = memebot_backend::all_users(&conn).unwrap();
            warp::reply::json(&users)
        });

    let tags = path("tags");

    let cp = conn_pool.clone();
    let new_tag = tags.and(path("new"))
        .and(path::param())
        .and_then(move |name: String| {
            let conn = cp.clone().get().unwrap();
            async move {
            memebot_backend::create_tag(&conn, &name)
                .map(|tag| warp::reply::json(&tag))
                .map_err(|_err| warp::reject::reject())
            }
        });


    let routes = warp::post()
        .and( //POST
            new_user
            .or(new_meme)
            .or(new_action)
            .or(new_tag)
            .or(tag_meme)
        )
        .or( //GET
            memes_all
            .or(memes_tag)
            .or(memes_user)
            .or(users_all)
        );

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}


enum StatusCode {
    ServerError,
    Created,
    Ok,
}

#[inline]
fn reply_with_status(sc: StatusCode) -> impl warp::reply::Reply {
    use warp::http::StatusCode as WarpCode;
    let rep = warp::reply();

    let proper_status_code = match sc {
        StatusCode::ServerError => WarpCode::INTERNAL_SERVER_ERROR,
        StatusCode::Created => WarpCode::CREATED,
        StatusCode::Ok => WarpCode::OK,
    };

    warp::reply::with_status(rep, proper_status_code)
}