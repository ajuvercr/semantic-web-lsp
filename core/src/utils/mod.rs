mod fetch;
pub use fetch::{fetch, Resp};
pub use reqwest::Error as ReqwestError;
pub use reqwest::StatusCode as ReqwestStatusCode;
