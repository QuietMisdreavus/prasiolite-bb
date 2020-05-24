use std::collections::HashMap;
use std::net::SocketAddr;

use tracing_subscriber::{FmtSubscriber, EnvFilter};

use warp::Filter;
use warp::filters::{method, path, query};

const SERVER_ADDR: &'static str = "127.0.0.1:3030";

macro_rules! routes {
    ($r:expr) => { $r };
    ($first:expr, $($next:expr),+ $(,)?) => {
        $first $(.or($next))+
    };
}

async fn start_server(addr: SocketAddr) {
    let routes = method::get().and(routes![
        // warp "hello world": `/hello/warp` returns "Hello, warp!"
        warp::path!("hello" / String)
            .map(|name| format!("Hello, {}!", name)),

        // index route
        path::end()
            .map(|| format!("sup")),

        // get a thread
        warp::path!("forum" / u32 / "topic" / u32)
            .and(query::query())
            .map(get_thread),
    ]);

    warp::serve(routes)
        .run(addr)
        .await;
}

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

    let addr: SocketAddr = SERVER_ADDR.parse().unwrap();
    println!();
    println!("running server on {}", addr);

    start_server(addr).await;
}
