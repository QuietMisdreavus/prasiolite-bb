use warp::Filter;
use warp::filters::{method, path};

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let index = path::end()
        .map(|| format!("sup"));

    let routes = method::get().and(hello.or(index));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
