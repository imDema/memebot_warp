use warp::{self, Filter};
use crate::routes::*;
use crate::handlers::*;

mod routes;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let meme_new = meme_routes::new_meme()
        .and_then(meme_handlers::new_meme);

    let meme_tag = meme_routes::add_tag()
        .and_then(meme_handlers::add_tag);

    let meme_action = meme_routes::action()
        .and_then(meme_handlers::action);

    let meme_all = meme_routes::get_all()
        .and_then( meme_handlers::get_all);

    let meme_by_user = meme_routes::by_user()
        .and_then(meme_handlers::by_user);

    let meme_by_tag = meme_routes::by_tag()
        .and_then(meme_handlers::by_tag);

    let user_new = user_routes::new_user()
        .and_then(handlers::user_handlers::new_user);

    let user_all = user_routes::get_all()
        .and_then(user_handlers::get_all);

    let tag_new = tag_routes::new_tag()
        .and_then(tag_handlers::new_tag);

    let tag_all = tag_routes::get_all()
        .and_then(tag_handlers::get_all);

    let routes =
            user_new
            .or(meme_new)
            .or(meme_action)
            .or(tag_new)
            .or(meme_tag)
            .or(meme_all)
            .or(meme_by_tag)
            .or(meme_by_user)
            .or(user_all)
            .or(tag_all);

    let end = routes.with(warp::log("memebot_warp::api"));

    warp::serve(end)
        .run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
