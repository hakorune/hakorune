//! Pure decisions shared by future lowering-time fact publishers.
//!
//! This module owns no fact storage, Builder session, MIR instruction, or
//! producer-specific evidence. It deliberately stays below `TypeContext` so a
//! later monotone store can consume the decision without treating mutable maps
//! as its policy authority.

mod type_decision;

pub use type_decision::{
    PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1, TypeFactDecisionV1,
};

#[cfg(test)]
mod type_decision_tests;
