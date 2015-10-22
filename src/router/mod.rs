//! A `Router` assigns `Middleware` to paths and resolves them per request
pub use self::http_router::HttpRouter;
pub use self::router::{Router, Route, RouteResult};
pub use self::matcher::Matcher;
pub use self::into_matcher::FORMAT_PARAM;

pub mod http_router;
pub mod router;
pub mod chainable_router;
mod matcher;
mod into_matcher;
