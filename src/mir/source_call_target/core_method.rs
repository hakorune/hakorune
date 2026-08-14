//! Fixed source-bound CoreMethod relation for the first S6C cohort.
//!
//! Exact call sites and the sole Loop membership come from the consumed
//! typed-input product. CoreMethod targets come from one existing target
//! issuer session. This module only co-seals those authorities; it does not
//! select by name, issue Recipe keys, or observe MIR/physical identity.

use crate::mir::callable_semantic_batch::{S6CCallSitePairRefV1, VerifiedS6CTypedInputRelationV1};
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::resolved_semantics::{
    CallableSemanticSourceLedgerView, CoreMethodHomeSchemaV1,
    ResolverCoreMethodCallableContractIssuerV1, ResolverCoreMethodCallableContractRejectV1,
    VerifiedCoreMethodInstanceTargetV1, VerifiedResolvedMethodCallSourceV1,
    VerifiedResolverCoreMethodCallableContractV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CSourceBoundCallRoleV1 {
    Length,
    Substring,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum S6CSourceBoundCallRelationRejectV1 {
    MixedManifestBrand,
    MixedSchema,
    MixedRelationBrand,
    DuplicateTargetBrand,
    WrongTargetRole {
        role: S6CSourceBoundCallRoleV1,
        op: CoreMethodOp,
        arity: u32,
    },
    CallSiteCoverage {
        role: S6CSourceBoundCallRoleV1,
        actual: usize,
    },
    Callable {
        role: S6CSourceBoundCallRoleV1,
        reject: ResolverCoreMethodCallableContractRejectV1,
    },
}

/// Borrow-only view over the fixed source-bound relation.
#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CSourceBoundCallRelationRefV1<'a> {
    typed: &'a VerifiedS6CTypedInputRelationV1,
    length: &'a VerifiedResolverCoreMethodCallableContractV1,
    substring: &'a VerifiedResolverCoreMethodCallableContractV1,
}

impl S6CSourceBoundCallRelationRefV1<'_> {
    pub(crate) const fn typed(&self) -> &VerifiedS6CTypedInputRelationV1 {
        self.typed
    }

    pub(crate) const fn length(&self) -> &VerifiedResolverCoreMethodCallableContractV1 {
        self.length
    }

    pub(crate) const fn substring(&self) -> &VerifiedResolverCoreMethodCallableContractV1 {
        self.substring
    }
}

/// Non-Clone fixed relation for `length/0` and `substring/2`.
#[derive(Debug)]
pub(crate) struct VerifiedSourceBoundS6CCallRelationV1 {
    typed: VerifiedS6CTypedInputRelationV1,
    length: VerifiedResolverCoreMethodCallableContractV1,
    substring: VerifiedResolverCoreMethodCallableContractV1,
}

impl VerifiedSourceBoundS6CCallRelationV1 {
    pub(crate) fn with_relation<R>(
        &self,
        callback: impl for<'relation> FnOnce(S6CSourceBoundCallRelationRefV1<'relation>) -> R,
    ) -> R {
        callback(S6CSourceBoundCallRelationRefV1 {
            typed: &self.typed,
            length: &self.length,
            substring: &self.substring,
        })
    }
}

pub(crate) fn issue_source_bound_s6c_call_relation_v1(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    typed: VerifiedS6CTypedInputRelationV1,
    length_target: VerifiedCoreMethodInstanceTargetV1,
    substring_target: VerifiedCoreMethodInstanceTargetV1,
) -> Result<VerifiedSourceBoundS6CCallRelationV1, S6CSourceBoundCallRelationRejectV1> {
    verify_target_pair(&length_target, &substring_target)?;

    let (length, substring) = typed.with_call_sites(|sites| {
        let length_call = exact_call(ledger, sites, S6CSourceBoundCallRoleV1::Length)?;
        let substring_call = exact_call(ledger, sites, S6CSourceBoundCallRoleV1::Substring)?;
        let length = ResolverCoreMethodCallableContractIssuerV1::issue(
            ledger,
            length_call,
            typed.membership(),
            sites.length_placement(),
            length_target,
        )
        .map_err(|reject| S6CSourceBoundCallRelationRejectV1::Callable {
            role: S6CSourceBoundCallRoleV1::Length,
            reject,
        })?;
        let substring = ResolverCoreMethodCallableContractIssuerV1::issue(
            ledger,
            substring_call,
            typed.membership(),
            sites.substring_placement(),
            substring_target,
        )
        .map_err(|reject| S6CSourceBoundCallRelationRejectV1::Callable {
            role: S6CSourceBoundCallRoleV1::Substring,
            reject,
        })?;
        Ok((length, substring))
    })?;

    Ok(VerifiedSourceBoundS6CCallRelationV1 {
        typed,
        length,
        substring,
    })
}

fn verify_target_pair(
    length: &VerifiedCoreMethodInstanceTargetV1,
    substring: &VerifiedCoreMethodInstanceTargetV1,
) -> Result<(), S6CSourceBoundCallRelationRejectV1> {
    if length.manifest_brand() != substring.manifest_brand() {
        return Err(S6CSourceBoundCallRelationRejectV1::MixedManifestBrand);
    }
    if length.schema() != substring.schema()
        || length.schema() != CoreMethodHomeSchemaV1::StringBoxText
    {
        return Err(S6CSourceBoundCallRelationRejectV1::MixedSchema);
    }
    if length.relation_brand() != substring.relation_brand() {
        return Err(S6CSourceBoundCallRelationRejectV1::MixedRelationBrand);
    }
    if length.target_brand() == substring.target_brand() {
        return Err(S6CSourceBoundCallRelationRejectV1::DuplicateTargetBrand);
    }
    require_target_role(
        length,
        S6CSourceBoundCallRoleV1::Length,
        CoreMethodOp::StringLen,
        0,
    )?;
    require_target_role(
        substring,
        S6CSourceBoundCallRoleV1::Substring,
        CoreMethodOp::StringSubstring,
        2,
    )
}

fn require_target_role(
    target: &VerifiedCoreMethodInstanceTargetV1,
    role: S6CSourceBoundCallRoleV1,
    expected_op: CoreMethodOp,
    expected_arity: u32,
) -> Result<(), S6CSourceBoundCallRelationRejectV1> {
    let row = target.row();
    if row.row().op != expected_op || row.arity() != expected_arity {
        return Err(S6CSourceBoundCallRelationRejectV1::WrongTargetRole {
            role,
            op: row.row().op,
            arity: row.arity(),
        });
    }
    Ok(())
}

fn exact_call<'a>(
    ledger: &'a CallableSemanticSourceLedgerView<'_>,
    sites: S6CCallSitePairRefV1<'_>,
    role: S6CSourceBoundCallRoleV1,
) -> Result<&'a VerifiedResolvedMethodCallSourceV1, S6CSourceBoundCallRelationRejectV1> {
    let site = match role {
        S6CSourceBoundCallRoleV1::Length => sites.length_site(),
        S6CSourceBoundCallRoleV1::Substring => sites.substring_site(),
    };
    let matching = ledger
        .method_calls()
        .filter(|(candidate, _)| *candidate == site)
        .map(|(_, call)| call)
        .collect::<Vec<_>>();
    match matching.as_slice() {
        [call] => Ok(*call),
        rows => Err(S6CSourceBoundCallRelationRejectV1::CallSiteCoverage {
            role,
            actual: rows.len(),
        }),
    }
}
