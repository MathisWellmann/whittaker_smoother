#![deny(missing_docs, unused_crate_dependencies, unused_imports)]
#![warn(clippy::all)]
#![doc = include_str!("../README.md")]

#[cfg(test)]
mod lines;
#[cfg(test)]
mod series;
mod whittaker_henderson_smoother;

pub use whittaker_henderson_smoother::whittaker_smoother;
