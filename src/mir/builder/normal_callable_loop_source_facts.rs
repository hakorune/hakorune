//! Source-aware callable-loop Facts/Recipe issuer modules.
//!
//! The GenericLoop and route-specific source products remain owned by their
//! existing issuers. This facade only exposes those owners and keeps tests at
//! the original module boundary; it does not add a semantic route.

#[path = "normal_callable_loop_source_facts/composite.rs"]
mod composite;
#[path = "normal_callable_loop_source_facts/composite_physical.rs"]
mod composite_physical;
#[path = "normal_callable_loop_source_facts/generic.rs"]
mod generic;
#[path = "normal_callable_loop_source_facts/loop_break.rs"]
mod loop_break;
#[path = "normal_callable_loop_source_facts/loop_cond.rs"]
mod loop_cond;
#[path = "normal_callable_loop_source_facts/loop_true.rs"]
mod loop_true;

pub(in crate::mir) use composite::*;
pub(in crate::mir::builder) use composite_physical::*;
pub(in crate::mir::builder) use generic::*;
pub(in crate::mir) use loop_break::*;
pub(in crate::mir::builder) use loop_cond::*;
