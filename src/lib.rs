#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(clippy::uninlined_format_args)]

pub mod card;
pub mod client;
pub mod core;
// pub mod message;
pub mod custom_bot;
pub mod event;
pub mod prelude;
pub mod service;
