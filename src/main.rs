use warp::{self, Filter, path};
use memebot_backend::models::*;

mod routes;
mod handlers;

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

    let cp = conn_pool.clone();
    let new_meme = routes::meme_routes::new_meme()
        .and_then(move |meme: NewMeme| {
            let conn = cp.clone().get().unwrap();
            async move {
            memebot_backend::create_meme(&conn, meme)
                .map(|()| reply_with_status(StatusCode::Created))
                .map_err(|_why| warp::reject())
            }
        });

    let cp = conn_pool.clone();
    let tag_meme = routes::meme_routes::add_tag()
        .and_then(move |meme_tag: MemeTag| {
            let conn = cp.clone().get().unwrap();
            async move {
                memebot_backend::add_meme_tag(&conn, meme_tag.memeid, meme_tag.tagid)
                    .map(|_| reply_with_status(StatusCode::Created))
                    .map_err(|_err| warp::reject())
            }
        });

    let cp = conn_pool.clone();
    let memes_all = routes::meme_routes::get_all()
        .map(move || {
            let conn = cp.clone().get().unwrap();
            let memes = memebot_backend::all_memes(&conn).unwrap();
            warp::reply::json(&memes)
        });

        
    let cp = conn_pool.clone();
    let memes_user = routes::meme_routes::by_user()
        .map(move |userid: i32| {
            let conn = cp.clone().get().unwrap();
            let memes = memebot_backend::memes_by_userid(&conn, userid).unwrap();
            warp::reply::json(&memes)
        });

    let cp = conn_pool.clone();
    let memes_tag = routes::meme_routes::by_tag()
        .map(move |tagid: i32| {
            let conn = cp.clone().get().unwrap();
            let memes = memebot_backend::memes_by_tagid(&conn, tagid).unwrap();
            warp::reply::json(&memes)
        });

    let cp = conn_pool.clone();
    let new_user = routes::user_routes::new_user()
        .and_then(handlers::user_handlers::new_user);

    let cp = conn_pool.clone();
    let users_all = routes::user_routes::get_all()
        .map(move ||{
            let conn = cp.clone().get().unwrap();
            let users = memebot_backend::all_users(&conn).unwrap();
            warp::reply::json(&users)
        });

    let cp = conn_pool.clone();
    let new_tag = routes::tag_routes::new_tag()
        .and_then(move |name: String| {
            let conn = cp.clone().get().unwrap();
            async move {
            memebot_backend::create_tag(&conn, &name)
                .map(|tag| warp::reply::json(&tag))
                .map_err(|_err| warp::reject::reject())
            }
        });

    let cp = conn_pool.clone();
    let tags_all = routes::tag_routes::get_all()
    .map(move || {
        let conn = cp.clone().get().unwrap();
        let tags = memebot_backend::all_tags(&conn).unwrap();
        warp::reply::json(&tags)
    });

    let routes =
            new_user
            .or(new_meme)
            .or(new_action)
            .or(new_tag)
            .or(tag_meme)
            .or(memes_all)
            .or(memes_tag)
            .or(memes_user)
            .or(users_all)
            .or(tags_all);

    let end = routes.with(warp::log("api"));

    warp::serve(end)
        .run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
