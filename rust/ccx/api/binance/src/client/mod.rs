mod config;
mod limits;
mod rest;
mod signer;
mod websocket;
mod websocket2;
use serde::Deserialize;

pub use self::config::*;
pub use self::limits::*;
pub use self::rest::*;
pub use self::signer::*;
pub use self::websocket::*;
pub use self::websocket2::*;

#[derive(Debug, Deserialize)]
struct BinanceContentError {
    #[allow(dead_code)]
    pub code: i16,
    #[allow(dead_code)]
    pub msg: String,
}
