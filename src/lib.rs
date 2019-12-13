extern crate dirs;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate regex;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod accounts;
pub mod client;
pub mod config;
pub mod gmail;
pub mod utils;
