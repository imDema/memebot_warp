use warp::{self, Filter};
use warp::path::end;
use crate::routes::*;
use crate::handlers::*;

mod routes;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let mq = || meme_routes::get_memes()
        .map(meme_handlers::init_query);
    
    let meme_queries = 
        mq().and(meme_routes::all()).and(end()) // All memes

        .or(mq().and(meme_routes::all()) // All memes ordered
            .and(meme_routes::order())
            .map(meme_handlers::order)).and(end())
        .unify()

        .or(mq().and(meme_routes::filter_userid()) // Memes from user
            .map(meme_handlers::filter_userid)).and(end())
        .unify()

        .or(mq().and(meme_routes::filter_userid()) // Memes from user ordered
            .map(meme_handlers::filter_userid)
            .and(meme_routes::order())
            .map(meme_handlers::order)).and(end())
        .unify()

        .or(mq().and(meme_routes::filter_tagid()) // Memes with tag
            .map(meme_handlers::filter_tagid)).and(end())
        .unify()

        .or(mq().and(meme_routes::filter_tagid()) // Memes with tag ordered
            .map(meme_handlers::filter_tagid)
            .and(meme_routes::order())
            .map(meme_handlers::order)).and(end())
        .unify()
    .and_then(meme_handlers::exec);

    let meme_new = meme_routes::new_meme()
        .and_then(meme_handlers::new_meme);

    let meme_tag = meme_routes::add_tag()
        .and_then(meme_handlers::add_tag);

    let meme_action = meme_routes::action()
        .and_then(meme_handlers::action);

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
            .or(meme_queries)
            .or(user_all)
            .or(tag_all);

    let end = routes.with(warp::log("memebot_warp::api"));

    warp::serve(end)
        .run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
