//! Use this module to interact with Bitstamp exchange.

pub mod api;
pub mod credentials;
pub mod generic_api;
pub mod utils;

pub use self::api::GdaxApi;
pub use self::credentials::GdaxCreds;
