#[macro_use]
extern crate log;

pub mod cli;

mod config;
mod error;
mod notifiers;
mod reactor;
mod watcher;
