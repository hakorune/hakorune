//! Bounded A-prime exact-I64 physical-demand capability.
//!
//! This module is the bridge between the selected package's semantic loan and
//! a later fresh physical session.  It owns no physical IDs and does not
//! classify a generic Dynamic value.

mod issuer;
mod model;

pub(in crate::mir) use issuer::issue_selected_a_prime_i64_physical_demand;
pub(in crate::mir) use model::{
    APrimeI64PhysicalDemandRejectV1, APrimeI64PhysicalRequirementV1,
    VerifiedAPrimeI64PhysicalDemandV1,
};

#[cfg(test)]
mod tests {
    #[test]
    fn a_prime_demand_stays_builder_free() {
        let source = [include_str!("issuer.rs"), include_str!("model.rs")].join("\n");
        for forbidden in [
            "ValueId",
            "BasicBlockId",
            "MirInstruction",
            "MirBuilder",
            "resolve_i64",
            "FunctionEntryContract",
            "fallback",
            "retry",
        ] {
            assert!(
                !source.contains(forbidden),
                "A-prime demand contains forbidden physical/recovery term: {forbidden}"
            );
        }
    }
}
