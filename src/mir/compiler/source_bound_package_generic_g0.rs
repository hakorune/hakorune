//! Continuation sealing shared by the source-bound package.
//!
//! Keeping this exhaustive route projection in a sibling preserves the
//! package's source-size boundary. It reuses the existing lifecycle families;
//! it does not issue a Generic-specific token or header.

use super::capability::{CanonicalLoopFamilyPlanV1, ResolvedOwnerHeaderFamilyV1};
use super::source_bound_package::{CanonicalSourceContinuationV1, SourceBindingErrorV1};
use super::source_bound_plan::ExactCanonicalPreflightPlanV1;
use crate::mir::module_invocation_identity::ModuleInvocationFamilyV1;
use crate::mir::module_invocation_policy::ModuleInvocationPolicyV1;

pub(super) fn seal_continuation<'a>(
    plan: &ExactCanonicalPreflightPlanV1<'a>,
) -> Result<CanonicalSourceContinuationV1<'a>, SourceBindingErrorV1> {
    match plan {
        ExactCanonicalPreflightPlanV1::APlus(plan) => single(
            plan.seal_resolved_owner_header_v1()
                .map_err(SourceBindingErrorV1::Header)?,
            ResolvedOwnerHeaderFamilyV1::CurrentCanonicalAPlus,
            ModuleInvocationFamilyV1::CanonicalAPlus,
            None,
        ),
        ExactCanonicalPreflightPlanV1::BindingSsaTrivial(plan) => single(
            plan.seal_resolved_owner_header_v1()
                .map_err(SourceBindingErrorV1::Header)?,
            ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa,
            ModuleInvocationFamilyV1::BindingSsaTrivial,
            None,
        ),
        ExactCanonicalPreflightPlanV1::Loop(plan) => match plan {
            CanonicalLoopFamilyPlanV1::DirectAccum(plan) => single(
                plan.seal_resolved_owner_header_v1()
                    .map_err(SourceBindingErrorV1::Header)?,
                ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa,
                ModuleInvocationFamilyV1::BindingSsaTrivial,
                None,
            ),
            CanonicalLoopFamilyPlanV1::NestedPredicate(plan) => single(
                plan.seal_resolved_owner_header_v1()
                    .map_err(SourceBindingErrorV1::Header)?,
                ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa,
                ModuleInvocationFamilyV1::BindingSsaTrivial,
                None,
            ),
            CanonicalLoopFamilyPlanV1::GenericG0(plan) => {
                let physical_arity = usize::try_from(plan.physical_callable_lane_count())
                    .map_err(|_| SourceBindingErrorV1::PhysicalArityOverflow)?;
                single(
                    plan.seal_resolved_owner_header_v1()
                        .map_err(SourceBindingErrorV1::Header)?,
                    ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa,
                    ModuleInvocationFamilyV1::BindingSsaTrivial,
                    Some(physical_arity),
                )
            }
        },
        ExactCanonicalPreflightPlanV1::BindingSsaAcyclic(plan) => {
            Ok(CanonicalSourceContinuationV1::Callable {
                source: plan.module(),
                policy: ModuleInvocationPolicyV1::policy_for_family(
                    ModuleInvocationFamilyV1::BindingSsaAcyclic,
                ),
            })
        }
        ExactCanonicalPreflightPlanV1::BindingSsaRecursive(plan) => {
            Ok(CanonicalSourceContinuationV1::Callable {
                source: plan.module(),
                policy: ModuleInvocationPolicyV1::policy_for_family(
                    ModuleInvocationFamilyV1::BindingSsaRecursive,
                ),
            })
        }
    }
}

fn single<'a>(
    header: super::capability::VerifiedResolvedOwnerHeaderV1,
    expected_family: ResolvedOwnerHeaderFamilyV1,
    family: ModuleInvocationFamilyV1,
    physical_arity: Option<usize>,
) -> Result<CanonicalSourceContinuationV1<'a>, SourceBindingErrorV1> {
    debug_assert_eq!(header.family(), expected_family);
    Ok(CanonicalSourceContinuationV1::Single {
        physical_arity: physical_arity.unwrap_or_else(|| header.arity()),
        header,
        policy: ModuleInvocationPolicyV1::policy_for_family(family),
    })
}
