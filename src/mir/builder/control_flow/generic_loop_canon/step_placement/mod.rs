//! Generic-loop step placement owner.
//!
//! This module flattens the old deep placement path while keeping facts and
//! plan responsibilities separated.

pub(in crate::mir::builder) mod facts;
pub(crate) mod plan;
