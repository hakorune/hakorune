//! Complete V1 root After closure from one verified Recipe and JoinSig.

use super::super::ids::LoopNodeKeyV1;
use super::super::verify::VerifiedLoopRecipeV1;
use super::flow::LoopJoinSigElaboratorV1;
use super::model::{
    LoopJoinPortV1, LoopJoinSigRejectReasonV1, VerifiedLoopAfterBindingV1, VerifiedLoopJoinSigV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopJoinClosureRejectV1 {
    RootCarrierCardinality { root: LoopNodeKeyV1, found: usize },
    JoinSig(LoopJoinSigRejectReasonV1),
}

/// The entire root After set is issued with the JoinSig that proved it.
/// A consumer cannot request a convenient subset and treat it as complete.
#[derive(Debug)]
pub(crate) struct VerifiedLoopJoinClosureV1 {
    join_sig: VerifiedLoopJoinSigV1,
    after: Box<[VerifiedLoopAfterBindingV1]>,
}

impl VerifiedLoopJoinClosureV1 {
    pub(crate) fn join_sig(&self) -> &VerifiedLoopJoinSigV1 {
        &self.join_sig
    }

    pub(crate) fn after(&self) -> &[VerifiedLoopAfterBindingV1] {
        &self.after
    }

    pub(in crate::mir::loop_recipe_contract) fn into_parts(
        self,
    ) -> (VerifiedLoopJoinSigV1, Box<[VerifiedLoopAfterBindingV1]>) {
        (self.join_sig, self.after)
    }
}

pub(crate) fn issue_root_carrier_join_closure_v1(
    recipe: &VerifiedLoopRecipeV1,
) -> Result<VerifiedLoopJoinClosureV1, LoopJoinClosureRejectV1> {
    let root = recipe.root_loop();
    let carriers = recipe
        .as_recipe()
        .carriers
        .iter()
        .filter(|carrier| carrier.owner_loop == root)
        .collect::<Vec<_>>();
    if carriers.is_empty() {
        return Err(LoopJoinClosureRejectV1::RootCarrierCardinality { root, found: 0 });
    }
    let join_sig =
        LoopJoinSigElaboratorV1::elaborate(recipe).map_err(LoopJoinClosureRejectV1::JoinSig)?;
    let after_rows = join_sig
        .as_sig()
        .port_bindings
        .iter()
        .filter(|row| row.loop_key == root && row.port == LoopJoinPortV1::After)
        .collect::<Vec<_>>();
    if after_rows.len() != carriers.len()
        || after_rows.iter().any(|row| {
            !carriers
                .iter()
                .any(|carrier| carrier.binding == row.binding && carrier.class == row.class)
        })
    {
        return Err(LoopJoinClosureRejectV1::JoinSig(
            LoopJoinSigRejectReasonV1::PortBindingSetMismatch {
                loop_key: root,
                port: LoopJoinPortV1::After,
            },
        ));
    }
    let after = carriers
        .into_iter()
        .map(|carrier| {
            join_sig
                .require_after_binding(root, carrier.binding, carrier.class)
                .map_err(LoopJoinClosureRejectV1::JoinSig)
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();
    Ok(VerifiedLoopJoinClosureV1 { join_sig, after })
}
