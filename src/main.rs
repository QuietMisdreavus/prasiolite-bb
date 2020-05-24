use std::collections::HashMap;
use std::net::SocketAddr;

use tracing_subscriber::{FmtSubscriber, EnvFilter};

use warp::Filter;
use warp::filters::{method, path, query};

const SERVER_ADDR: &'static str = "127.0.0.1:3030";

#[tracing::instrument]
fn get_thread(forum: u32, topic: u32, query: HashMap<String, String>) -> String {
    use std::fmt::Write;

    let mut output = String::new();

    writeln!(output, "forum: {}", forum).unwrap();
    writeln!(output, "topic: {}", topic).unwrap();

    if !query.is_empty() {
        writeln!(output, "\nquery string:").unwrap();
        for (k, v) in query {
            writeln!(output, "  {}: {}", k, v).unwrap();
        }
    }

    output
}

#[tokio::main]
async fn main() {
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let index = path::end()
        .map(|| format!("sup"));

    let topic = warp::path!("forum" / u32 / "topic" / u32)
        .and(query::query())
        .map(get_thread);

    let routes = method::get().and(hello.or(index).or(topic));

    let addr: SocketAddr = SERVER_ADDR.parse().unwrap();
    println!();
    println!("running server on {}", addr);

    warp::serve(routes)
        .run(addr)
        .await;
}
