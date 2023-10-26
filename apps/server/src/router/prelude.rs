#[derive(Clone, Copy, Debug)]
pub struct MyCtx {}

#[derive(thiserror::Error, serde::Serialize, specta::Type, Debug)]
#[error("{0}")]
/// Router error
pub struct Error(&'static str);

pub const R:  rspc::Rspc<MyCtx, Error> = rspc::Rspc::new();

pub type RouterBuilder = rspc::RouterBuilder<MyCtx>;
pub type Router = rspc::Router<MyCtx>;
