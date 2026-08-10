//! Typed V2 closure over the one neutral JoinSig engine.

use super::super::ids::{LoopBindingKeyV1, LoopNodeKeyV1};
use super::super::schema_v2::LoopValueClassV2;
use super::super::typed_schema_v2::VerifiedLoopRecipeV2;
use super::flow::elaborate_view;
use super::model::{LoopJoinSigRejectReasonV1, VerifiedLoopAfterBinding, VerifiedLoopJoinSigV2};
use super::recipe_view_v2::LoopRecipeV2JoinView;

struct LoopJoinSigElaboratorV2;

impl LoopJoinSigElaboratorV2 {
    fn elaborate(
        verified: &VerifiedLoopRecipeV2,
    ) -> Result<VerifiedLoopJoinSigV2, LoopJoinSigRejectReasonV1> {
        elaborate_view(&LoopRecipeV2JoinView::verified(verified))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopJoinClosureRejectV2 {
    RootCarrierCardinality { root: LoopNodeKeyV1, found: usize },
    JoinSig(LoopJoinSigRejectReasonV1),
}

/// One exact V2 JoinSig and the After derived from its sole root carrier.
///
/// The closure is deliberately non-`Clone` and has no `into_parts`: V2
/// callers can only retain or borrow the same-Recipe logical control proof.
#[derive(Debug)]
pub(crate) struct VerifiedLoopJoinClosureV2 {
    join_sig: VerifiedLoopJoinSigV2,
    after: VerifiedLoopAfterBinding<LoopValueClassV2>,
}

impl VerifiedLoopJoinClosureV2 {
    /// Lend the JoinSig-owned logical transfer rows without exposing raw
    /// JoinSig or After parts to downstream physical code.
    pub(crate) fn logical_transfer_view(
        &self,
    ) -> Result<
        super::transfer_view_v2::LoopJoinLogicalTransferViewV2<'_>,
        super::transfer_view_v2::LoopJoinLogicalTransferRejectV2,
    > {
        super::transfer_view_v2::issue(self, &self.join_sig)
    }

    pub(crate) fn join_sig(&self) -> &VerifiedLoopJoinSigV2 {
        &self.join_sig
    }

    pub(crate) fn after_loop_key(&self) -> LoopNodeKeyV1 {
        self.after.loop_key()
    }

    pub(crate) fn after_binding(&self) -> LoopBindingKeyV1 {
        self.after.binding()
    }

    pub(crate) fn after_class(&self) -> LoopValueClassV2 {
        self.after.class()
    }
}

/// Derive the V2 logical control closure without accepting raw owner, key,
/// JoinSig, or After inputs from the compiler profile.
pub(crate) fn issue_sole_root_carrier_join_closure_v2(
    recipe: &VerifiedLoopRecipeV2,
) -> Result<VerifiedLoopJoinClosureV2, LoopJoinClosureRejectV2> {
    let root = recipe.root_loop();
    let mut root_carriers = recipe
        .as_recipe()
        .carriers
        .iter()
        .filter(|carrier| carrier.owner_loop == root);
    let Some(carrier) = root_carriers.next() else {
        return Err(LoopJoinClosureRejectV2::RootCarrierCardinality { root, found: 0 });
    };
    let additional = root_carriers.count();
    if additional != 0 {
        return Err(LoopJoinClosureRejectV2::RootCarrierCardinality {
            root,
            found: additional + 1,
        });
    }

    let join_sig =
        LoopJoinSigElaboratorV2::elaborate(recipe).map_err(LoopJoinClosureRejectV2::JoinSig)?;
    let after = join_sig
        .require_after_binding_internal(root, carrier.binding, carrier.class)
        .map_err(LoopJoinClosureRejectV2::JoinSig)?;
    Ok(VerifiedLoopJoinClosureV2 { join_sig, after })
}
