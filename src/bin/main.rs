#[macro_use]
extern crate warp;

#[macro_use]
extern crate larp;

#[macro_use]
extern crate serde_derive;

extern crate serde;

use warp::Filter;
use larp::Route;
use warp::Reply;

#[derive(Debug, Serialize, Deserialize)]
struct Post {
  pub title: String,
  pub typo: u8,
}

fn get() -> Route<impl Reply> {
  let world = get!()
    .map(|| "hello, world");

  let name = get!(String)
    .map(|name| format!("hello, {}", name));


  route!("hello", world, name)
}

fn post() -> Route<impl Reply> {
  let world = post!()
    .map(|p: Post| format!("{:?}", p));

  route!("post", world)
}

fn put() -> Route<impl Reply> {
  let world = put!()
    .map(|p: Post| warp::reply::json(&p));

  route!("put", world)
}

fn delete() -> Route<impl Reply> {
  let world = del!()
    .map(|| "hello, delete");

  route!("delete", world)
}

fn main() {
  let route = route!(/, get(), post(), put(), delete());

  warp::serve(route)
    .run(([127, 0, 0, 1], 3000))
}