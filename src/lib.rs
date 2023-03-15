//! # Dendrite Generic Back-End
//!
//! Crate `generic_backend` is a [Rust](https://www.rust-lang.org) generic back-end binary that connects to [AxonServer](https://axoniq.io/product-overview/axon-server).
//!
//! See the GitHub project [rustigaan/generic-backend](https://github.com/rustigaan/generic-backend) for an example of how to use this code.

pub mod application;
pub mod example_api;
pub mod example_command;
pub mod example_event;
pub mod example_query;
pub mod proto_dendrite_config;
pub mod proto_example;
