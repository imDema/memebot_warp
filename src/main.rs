use warp::{self, Filter};
use memebot_backend::models::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn_pool = memebot_backend::establish_connection_pool();

    let cp = conn_pool.clone();
    let new_user = warp::path!("new_user")
        .and(warp::body::content_length_limit(4 * 1024))
        .and(warp::body::json())
        .and_then(move |user: NewUser| {
            let conn = cp.clone().get().unwrap();
            async move {
            memebot_backend::create_user(&conn, user)
                .map(|()| warp::reply())
                .map_err(|_why| warp::reject::reject())
            }
        });
    
    let cp = conn_pool.clone();
    let new_meme = warp::path!("new_meme")
        .and(warp::body::content_length_limit(4 * 1024))
        .and(warp::body::json())
        .and_then(move |meme: NewMeme| {
            let conn = cp.clone().get().unwrap();
            async move {
            memebot_backend::create_meme(&conn, meme)
                .map(|()| warp::reply())
                .map_err(|_why| warp::reject::reject())
            }
        });

    let cp = conn_pool.clone();
    let new_action = warp::path("new_action")
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::json())
        .and_then(move |action: NewAction| {
            let conn = cp.clone().get().unwrap();
            async move {
            memebot_backend::new_action(&conn, action)
                .map(|()| warp::reply())
                .map_err(|_why| warp::reject::reject())
            }
        });

    let cp = conn_pool.clone();
    let all = warp::path("all")
        .map(move || {
            let conn = cp.clone().get().unwrap();
            let coso = AllTest::new(memebot_backend::all_users(&conn).unwrap(), memebot_backend::all_memes(&conn).unwrap());
            warp::reply::json(&coso)
        });

    let memes = warp::path("memes");

    let cp = conn_pool.clone();
    let memes_user = memes.and(warp::path("user"))
        .and(warp::path::param())
        .map(move |userid: i32| {
            let conn = cp.clone().get().unwrap();
            let memes = memebot_backend::memes_by_userid(&conn, userid).unwrap();
            warp::reply::json(&memes)
        });

    let cp = conn_pool.clone();
    let memes_tag = memes.and(warp::path("tag"))
        .and(warp::path::param())
        .map(move |tagid: i32| {
            let conn = cp.clone().get().unwrap();
            let memes = memebot_backend::memes_by_tagid(&conn, tagid).unwrap();
            warp::reply::json(&memes)
        });

    let routes = warp::post()
        .and( //POST
            new_user
            .or(new_meme)
            .or(new_action)
        )
        .or( //GET
            all
            .or(memes_tag)
            .or(memes_user)
        );

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
