//! Caller-zero loop-node physical admission issuer (M10b-I0-P2-E).
//!
//! One `IssuedLoopNodeWinnerV1` (recipe + resolver context) is co-sealed
//! Builder-free into the sole physical input set the canonical segment
//! pipeline consumes: semantic context -> operation/effect demand ->
//! prepared operation program -> prepared physical layout -> entry input
//! set -> internal declaration rows. The `GenericG0` arm stays
//! function-level and is a typed terminal here, never a fallback.
//!
//! This module mints no Recipe keys, no physical identities, and no
//! Builder state; it only composes already-verified products.

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::loop_node_winner_spine::{IssuedLoopNodeWinnerV1, LoopNodeWinnerRecipeV1};
use crate::mir::loop_recipe_contract::{
    LoopJoinSigRejectReasonV1, LoopOperationPhysicalDemandRejectV1,
    LoopPhysicalLayoutRejectV1, PreparedLoopPhysicalLayoutV1,
    VerifiedLoopContinuationContractV1, VerifiedLoopInitializedLocalInputSourceSetV1,
    VerifiedLoopOperationEffectProductV1, VerifiedLoopOperationPhysicalDemandV1,
    VerifiedLoopSemanticContextV1,
};
use crate::mir::resolved_semantics::ResolvedLoopRegionLookupErrorV1;

/// Complete Builder-free admission product. The physicalize edge consumes
/// exactly this; nothing may be re-issued or re-derived downstream.
#[derive(Debug)]
pub(crate) struct VerifiedLoopNodePhysicalAdmissionV1 {
    layout: PreparedLoopPhysicalLayoutV1,
    inputs: VerifiedLoopInitializedLocalInputSourceSetV1,
}

impl VerifiedLoopNodePhysicalAdmissionV1 {
    pub(crate) fn layout(&self) -> &PreparedLoopPhysicalLayoutV1 {
        &self.layout
    }

    pub(crate) fn inputs(&self) -> &VerifiedLoopInitializedLocalInputSourceSetV1 {
        &self.inputs
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        PreparedLoopPhysicalLayoutV1,
        VerifiedLoopInitializedLocalInputSourceSetV1,
    ) {
        (self.layout, self.inputs)
    }
}

#[derive(Debug)]
pub(crate) enum LoopNodePhysicalAdmissionRejectV1 {
    /// Resolver could not re-issue the selected loop's source/context pair.
    LoopContext(ResolvedLoopRegionLookupErrorV1),
    /// The issued site does not resolve to the same loop source.
    LoopContextSiteMismatch,
    /// The GenericG0 arm is function-level; the node edge has no authority.
    FunctionLevelFamily,
    /// The recipe carries no root carrier for the After contract.
    RootCarrierMissing,
    /// JoinSig declined the root After binding.
    AfterBinding(LoopJoinSigRejectReasonV1),
    /// The neutral operation demand rejected the co-sealed product.
    Demand(LoopOperationPhysicalDemandRejectV1),
    /// The physical layout rejected the prepared program.
    Layout(LoopPhysicalLayoutRejectV1),
}

/// Issue the physical admission for one issued winner recipe.
///
/// The issuer requires the `IssuedLoopNodeWinnerV1` context — the loop
/// statement site and the window-lease frame minted before selection —
/// and re-issues only the resolver-owned scope/region pair for that
/// exact site. Every other product is consumed, never re-derived.
pub(crate) fn issue_loop_node_physical_admission_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    issued: IssuedLoopNodeWinnerV1,
) -> Result<VerifiedLoopNodePhysicalAdmissionV1, LoopNodePhysicalAdmissionRejectV1> {
    use LoopNodePhysicalAdmissionRejectV1 as Reject;

    let (site, frame, recipe) = issued.into_parts();
    let (operations, inputs) = match recipe {
        LoopNodeWinnerRecipeV1::DirectAccum(product) => product.into_parts(),
        LoopNodeWinnerRecipeV1::NestedPredicate(product) => {
            let (operations, inputs, _handoff) = product.into_parts();
            (operations, inputs)
        }
        LoopNodeWinnerRecipeV1::LoopTrue(product) => {
            let (_receipt, operations, inputs) = product.into_parts();
            (operations, inputs)
        }
        LoopNodeWinnerRecipeV1::LoopCond(product) => {
            let (_receipt, operations, inputs) = product.into_parts();
            (operations, inputs)
        }
        LoopNodeWinnerRecipeV1::GenericG0(_) => return Err(Reject::FunctionLevelFamily),
    };
    let (loop_source, scope_region) = input
        .function()
        .resolved_loop_source_context(&site)
        .map_err(Reject::LoopContext)?;
    if loop_source.site() != &site {
        return Err(Reject::LoopContextSiteMismatch);
    }
    let context = VerifiedLoopSemanticContextV1::from_parts(
        input.owner(),
        input.function().function_origin(),
        input.function().source_kind(),
        site,
        frame,
        scope_region,
    );
    issue_admission_from_products(context, operations, inputs)
}

fn issue_admission_from_products(
    context: VerifiedLoopSemanticContextV1,
    operations: VerifiedLoopOperationEffectProductV1,
    inputs: VerifiedLoopInitializedLocalInputSourceSetV1,
) -> Result<VerifiedLoopNodePhysicalAdmissionV1, LoopNodePhysicalAdmissionRejectV1> {
    use LoopNodePhysicalAdmissionRejectV1 as Reject;

    let owner = operations.core().owner();
    let recipe = operations.core().recipe().as_recipe();
    let root = recipe.root_loop;
    let carrier = recipe
        .carriers
        .iter()
        .find(|carrier| carrier.owner_loop == root)
        .ok_or(Reject::RootCarrierMissing)?;
    let after = operations
        .core()
        .join_sig()
        .require_after_binding(root, carrier.binding, carrier.class)
        .map_err(Reject::AfterBinding)?;
    let continuation = VerifiedLoopContinuationContractV1::from_after(owner, after);
    let layout = VerifiedLoopOperationPhysicalDemandV1::issue(context, operations, continuation)
        .and_then(|demand| demand.prepare_all())
        .map_err(Reject::Demand)?
        .prepare_physical_layout()
        .map_err(Reject::Layout)?;
    Ok(VerifiedLoopNodePhysicalAdmissionV1 { layout, inputs })
}
