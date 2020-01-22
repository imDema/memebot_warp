use warp::{self, path, Filter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let new_user = warp::path!("new_user" / String)
        .map(|user: String| {
            let u = memebot_backend::models::NewUser::new(&user);
            warp::reply::json(&u)
            }
        );

    let routes = warp::get().and(
        new_user
    );

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
