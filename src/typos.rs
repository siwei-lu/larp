use warp::filters::BoxedFilter;

pub type Route<T> = BoxedFilter<(T,)>;