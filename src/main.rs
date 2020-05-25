// prasiolite-bb: a message board
// Copyright (C) 2020 QuietMisdreavus
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::net::SocketAddr;

use tracing_subscriber::{FmtSubscriber, EnvFilter};

use warp::Filter;
use warp::filters::{method, path, query};
use warp::reject::{not_found, Rejection};
use warp::reply::Reply;

mod db;

const SERVER_ADDR: &'static str = "127.0.0.1:3030";

macro_rules! routes {
    ($r:expr) => { $r };
    ($first:expr, $($next:expr),+ $(,)?) => {
        $first $(.or($next))+
    };
}

fn get_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    method::get().and(routes![
        // index route; list all forums
        path::end()
            .map(get_index),

        // get a forum
        warp::path!("forum" / u32)
            .and(query::query())
            .and_then(get_forum),

        // get a thread
        warp::path!("forum" / u32 / "topic" / u32)
            .and(query::query())
            .map(get_thread),
    ])
}

fn get_index() -> String {
    use std::fmt::Write;

    let mut output = String::new();

    output.push_str("prasiolite-bb test data\n");
    output.push('\n');
    output.push_str("-----\n");

    for forum in db::get_forums() {
        output.push('\n');

        writeln!(output, "forum \"{}\"", forum.name).unwrap();
        writeln!(output, "  {}", forum.description).unwrap();
    }

    output
}

async fn get_forum(forum_id: u32, query: HashMap<String, String>) -> Result<String, Rejection> {
    use std::fmt::Write;

    let forum = if let Some(f) = db::get_forum(forum_id) {
        f
    } else {
        return Err(not_found());
    };

    let sort: db::Sorting = query.get("sort").map_or(db::Sorting::Descending, |s| (&**s).into());

    let topics = db::get_topics(forum_id, sort);

    let mut output = String::new();

    writeln!(output, "forum: {}", forum.name).unwrap();
    writeln!(output, "{}", forum.description).unwrap();

    output.push_str("\n");
    output.push_str("-----\n");
    output.push_str("\n");

    for t in topics {
        output.push_str("\n");
        writeln!(output, "\"{}\"", t.name).unwrap();
        writeln!(output, "  by: {}", t.author).unwrap();
        writeln!(output, "  opened: {}", t.opened).unwrap();
        writeln!(output, "  last post: {}", t.last_post).unwrap();
        writeln!(output, "  posts: {}", t.post_count).unwrap();
    }

    Ok(output)
}

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

    let routes = get_routes();

    let addr: SocketAddr = SERVER_ADDR.parse().unwrap();
    println!();
    println!("running server on {}", addr);

    warp::serve(routes)
        .run(addr)
        .await;
}
