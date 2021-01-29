//! Use this module to interact with Bitstamp exchange.

pub mod api;
pub mod credentials;
pub mod generic_api;
pub mod utils;

pub use self::api::BitstampApi;
pub use self::credentials::BitstampCreds;
