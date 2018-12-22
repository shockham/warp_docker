use warp::Filter;
use warp::path;

fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([0, 0, 0, 0], 3030));
}
