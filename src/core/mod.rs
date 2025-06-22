use crate::core::error::LarkAPIError;

pub mod api_req;
pub mod api_resp;
pub mod app_ticket_manager;
pub mod cache;
pub mod config;
pub mod constants;
pub mod error;
pub mod http;
pub mod req_option;
pub mod req_translator;
pub mod request_builder;
pub mod token_manager;
pub mod utils;

pub type SDKResult<T> = Result<T, LarkAPIError>;
