//! Use this module to interact with Kraken exchange.
//! See examples for more informations.

pub mod api;
pub mod credentials;
pub mod generic_api;
pub mod utils;

pub use self::api::KrakenApi;
pub use self::credentials::KrakenCreds;
