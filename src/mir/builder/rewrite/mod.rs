//! Narrow rewrite helpers for explicit, source-level special cases.
//!
//! The optional Known/Unique instance-to-Global optimization was retired by
//! `MIR-CALL-SAME-MODULE-REWRITE-KNOWN-POLICY-RETIRE-I0`.  Canonical method
//! lowering now keeps the incoming typed `Method(Some(receiver))` target.
//! Only the explicit early str-like compatibility route remains here.

pub mod special;
