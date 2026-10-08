//! Exact primitive returned-value corroboration in the completed-index owner.
//! This prerequisite grants no caller Home, transfer or publication permission.
use super::super::{result_class_claim, OrdinaryNewClaimLedgerV1};
use super::super::{OrdinaryNewResultClaimV1, OwnedFieldChildV1};
use crate::mir::function::ObjectDestructionDispositionV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    TerminalRelationV1, TerminalReturnedSourceV1,
};
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use hakorune_mir_defs::CanonicalObjectIdV1;
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;
use result_class_claim::{ResultOriginWitnessV1, ResultValueOriginV1, ResultWitnessStepV1};
use std::rc::Rc;

/// Snapshot only of the exact returned New claim, retained before affine take.
/// This is not a cleanup reducer or an invocation/result permission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::normal_callable_semantic_package) struct ObjectReturnTeardownDescriptorV1 {
    object: CanonicalObjectIdV1,
    destruction: ObjectDestructionDispositionV1,
    children: Option<Box<[OwnedFieldChildV1]>>,
}
impl ObjectReturnTeardownDescriptorV1 {
    pub(super) fn object(&self) -> CanonicalObjectIdV1 {
        self.object
    }
    pub(super) fn children(&self) -> &[OwnedFieldChildV1] {
        self.children.as_deref().unwrap_or(&[])
    }

    pub(super) fn supports_direct_teardown(&self, nullable: bool) -> bool {
        match self.destruction {
            ObjectDestructionDispositionV1::PlainI64NoHook => self.children.is_none(),
            ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook
            | ObjectDestructionDispositionV1::OwnedObjectFieldsNoHook => {
                // Nullable child-field release has no accepted envelope yet.
                !nullable && self.children.is_some()
            }
            ObjectDestructionDispositionV1::Unavailable(_) => false,
        }
    }

    fn from_exact_claim(claim: &OrdinaryNewResultClaimV1) -> Self {
        Self {
            object: claim.object(),
            destruction: claim.core.destruction(),
            children: claim.core.children().map(|children| children.into()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::normal_callable_semantic_package) enum VerifiedObjectReturnLeafV1 {
    Null,
    Fresh {
        site: OwnedExprSiteV1,
        teardown: ObjectReturnTeardownDescriptorV1,
    },
}

impl OrdinaryNewClaimLedgerV1 {
    /// Corroborate only the exact immutable leaf named by the original Facts.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_object_return_leaf_v1(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
        witness: &Rc<ResultOriginWitnessV1>,
    ) -> Result<Option<VerifiedObjectReturnLeafV1>, String> {
        let exits = self
            .callable_result_classes
            .outcomes(key)
            .ok_or_else(|| freeze("object-return/foreign-leaf"))?;
        if !exits
            .iter()
            .flat_map(|row| row.witnesses())
            .any(|original| Rc::ptr_eq(original, witness))
        {
            return Err(freeze("object-return/foreign-leaf"));
        }
        let Some(class) = self
            .callable_result_classes
            .get(key)
            .and_then(|row| row.class())
        else {
            return Ok(None);
        };
        if !matches!(
            witness.step(),
            ResultWitnessStepV1::NullLiteral | ResultWitnessStepV1::FreshConstruction
        ) {
            return Ok(None);
        }
        let Some(completion) = self
            .completion_index
            .get(&witness.site().owner())
            .and_then(|row| row.as_ref().ok())
        else {
            return Ok(None);
        };
        if completion.owner() != witness.site().owner() {
            return Err(freeze("object-return/leaf-completion-owner"));
        }
        let mut exact = completion.explicit_sites().iter().filter_map(|exit| {
            match self.terminal_relation_for_owner_at(completion.owner(), exit)? {
                TerminalRelationV1::Value(row) if row.value_site() == witness.site().site() => {
                    Some((exit, row))
                }
                _ => None,
            }
        });
        let Some((exit, terminal)) = exact.next() else {
            return Ok(None);
        };
        if exact.next().is_some()
            || terminal.owner() != witness.site().owner()
            || terminal.return_site() != exit
        {
            return Err(freeze("object-return/leaf-exit-identity"));
        }
        // Both null and fresh exits must retain the original cleanup row.
        // A source-only Completion does not prove an empty cleanup obligation.
        let Some(Ok(homes)) = completion
            .cleanup()
            .root_flow()
            .and_then(|flow| flow.exit_row(exit))
        else {
            return Ok(None);
        };
        match (witness.step(), witness.origin(), terminal.returned()) {
            (
                ResultWitnessStepV1::NullLiteral,
                ResultValueOriginV1::Null,
                TerminalReturnedSourceV1::NullLiteral,
            ) => Ok(Some(VerifiedObjectReturnLeafV1::Null)),
            (
                ResultWitnessStepV1::FreshConstruction,
                ResultValueOriginV1::Fresh(name),
                TerminalReturnedSourceV1::Construction(site),
            ) => {
                if site != witness.site() || name.as_ref() != class {
                    return Err(freeze("object-return/leaf-construction-identity"));
                }
                let claims = self.result_claims.borrow();
                let Some(claim) = claims.get(site) else {
                    return Ok(None);
                };
                if claim.site() != site || claim.class() != class {
                    return Err(freeze("object-return/leaf-claim-identity"));
                }
                let (Ok(construction), Ok(prefix), Ok(arguments)) = (
                    claim.construction(),
                    claim.home_prefix(),
                    claim.argument_rows(),
                ) else {
                    return Ok(None);
                };
                if construction.object() != claim.object()
                    || prefix.required_unwind() != site
                    || prefix.prior_homes() != homes.homes()
                    || arguments.len() != claim.arity()
                {
                    return Err(freeze("object-return/leaf-prefix-identity"));
                }
                Ok(Some(VerifiedObjectReturnLeafV1::Fresh {
                    site: site.clone(),
                    teardown: ObjectReturnTeardownDescriptorV1::from_exact_claim(claim),
                }))
            }
            _ => Err(freeze("object-return/leaf-terminal-mismatch")),
        }
    }
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/{reason}]")
}

#[cfg(test)]
#[path = "ordinary_new_return_leaf_tests.rs"]
mod tests;
