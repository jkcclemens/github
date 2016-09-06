#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate hyper;

pub mod github;
pub mod users;
pub mod organizations;
pub mod repositories;

#[cfg(test)]
mod tests;
