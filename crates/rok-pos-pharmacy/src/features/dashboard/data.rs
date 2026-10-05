//! Where the dashboard's figures come from.
//!
//! A source's own records arrive here and leave as domain entities. The source
//! has its own shape, its own strings and its own idea of an amount; the
//! conversion happens once, at this boundary.

pub mod models;
pub mod repository;
pub mod repository_impl;
pub mod story;
