# Larp

A helper for [warp](https://github.com/seanmonstar/warp). Provide some macros to make warp easier.

## Warning

Larp is far from stable. BE CAREFUL if want to use it on your product.

## Install
```toml
[dependencies]
warp = "0.1.9"
larp = "0.0.1"

# optional
serde = "1.0.80"
serde_derive = "1.0.80"
```

## Example

```rust
#[macro_use]
extern crate warp;

#[macro_use]
extern crate larp;

use warp::Filter;

fn main() {
  let route = get!().map(|| "hello, world");
  warp::serve(route).run(([0; 4], 3000))
}
```

Now you can see "hello, world" at [http://localhost:3000](http://localhost:3000).

## API

### get!

Create a route only responding to get request.

```rust
#[macro_use]
extern crate warp;

#[macro_use]
extern crate larp;

use warp::Filter;

fn main() {
  // handle / and response "hello, world"
  let hello_world = get!().map(|| "hello, world");
  
  // handle /hello/:name and response "hello, $name"
  let hello_name = get!("hello" / String)
    .map(|name| format!("hello, {}", name));
    
  // combine some routes
  let route = route!(/, hello_world, hello_name);
  
  warp::serve(route).run(([0; 4], 3000))
}
```

### post!

Create a route only responding to post request. The `Content-Type` of request must be `application/json` and it will be parsed automatically.

```rust
#[macro_use]
extern crate warp;

#[macro_use]
extern crate larp;

// for serializing `Person` struct, you should import `serde` and `serde_derive`
#[macro_use]
extern crate serde_derive;
extern crate serde;

use warp::Filter;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
  pub name: String,
}

fn main() {
  // handle / and response "hello, world"
  let hello_world = post!().map(|| "hello, world");
  
  // handle /person
  // request body must be `{ name: 'someone' }`
  // response body is `{ name: 'someone' }` too
  let hello_person = post!("person")
    .map(|p: Person| warp::reply::json(&p));
    
  // combine some routes
  let route = route!(/, hello_world, hello_person);
  
  warp::serve(route).run(([0; 4], 3000))
}
```

### put!

Create a route only responding to put request. Same as `post!`.

### del!

Create a route only responding to delete request. Same as `get!`.

### route!

Make a subroute and combine some routes.

```rust
// ...

fn main() {
  let hello_world = get!().map(|| "hello, world");
  
  let hello_name = get!("hello" / String)
      .map(|name| format!("hello, {}", name));
      
  let hello_post_world = post!().map(|| "hello, world");
  
  let hello_person = post!("person")
      .map(|p: Person| warp::reply::json(&p));
      
  // handle /get and /get/hello
  let get = route!("get", hello_world, hello_name);
  
  // handle /post and /post/person
  let post = route!("post", hello_post_world, hello_person);
  
  let route = route!(/, get, post);
  warp::serve(route).run(([0; 4], 3000))
}
```
