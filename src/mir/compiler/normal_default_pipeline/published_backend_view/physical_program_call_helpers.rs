//! Forwarding shim — the callee->callable-key projection owner moved to
//! `crate::mir::ssot::callable_key` (R6-S0 boundary S0-A).

pub(crate) use crate::mir::ssot::callable_key::{
    ordinary_call_receiver, ordinary_callable_key,
};
