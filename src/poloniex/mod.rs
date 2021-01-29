//! Use this module to interact with Poloniex exchange.

pub mod api;
pub mod credentials;
pub mod generic_api;
pub mod utils;

pub use self::api::PoloniexApi;
pub use self::api::{MoveOrderOption, PlaceOrderOption};
pub use self::credentials::PoloniexCreds;
