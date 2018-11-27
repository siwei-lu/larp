#[macro_export]
macro_rules! route {
  (/, $first:expr $(, $routes: expr)*) => {
    $first$(.or($routes))*
      .boxed()
  };

  ($path:expr, $first:expr $(, $routes: expr)*) => {
    path!($path)
      .and($first$(.or($routes))*)
      .boxed()
  };
}

#[macro_export]
macro_rules! get {
  () => { warp::get2().and(warp::filters::path::end()) };

  ($($x:tt)*) => {
    path!($($x)*).and(warp::filters::path::end())
  };
}

#[macro_export]
macro_rules! post {
  () => {
    warp::post2()
      .and(warp::filters::path::end())
      .and(warp::body::json())
  };

  ($($x:tt)*) => {
    warp::post2()
      .and(path!($($x)*))
      .and(warp::filters::path::end())
      .and(warp::body::json())
  };
}

#[macro_export]
macro_rules! put {
  () => {
    warp::put2()
      .and(warp::filters::path::end())
      .and(warp::body::json())
  };

  ($($x:tt)*) => {
    warp::put2()
      .and(path!($($x)*))
      .and(warp::filters::path::end())
      .and(warp::body::json())
  };
}

#[macro_export]
macro_rules! del {
  () => { warp::delete2().and(warp::filters::path::end()) };

  ($($x:tt)*) => {
    warp::delete2()
      .and(path!($($x)*))
      .and(warp::filters::path::end())
  };
}