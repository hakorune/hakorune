//! Exact primitive returned-value corroboration in the completed-index owner.
//! This prerequisite grants no caller Home, transfer or publication permission.
#[cfg(test)]
use super::super::OrdinaryNewResultClaimV1;
use super::super::OwnedFieldChildV1;
use super::super::{result_class_claim, OrdinaryNewClaimLedgerV1, OrdinaryNewTrivialArgumentV1};
use crate::mir::function::ObjectDestructionDispositionV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    TerminalRelationV1, TerminalReturnedSourceV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1};
use hakorune_mir_defs::CanonicalObjectIdV1;
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;
use result_class_claim::{ResultOriginWitnessV1, ResultValueOriginV1, ResultWitnessStepV1};
use std::rc::Rc;

/// Snapshot of the same returned-New source, borrowed before or after affine take.
/// This is not a cleanup reducer or an invocation/result permission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::normal_callable_semantic_package) struct ObjectReturnTeardownDescriptorV1 {
    object: CanonicalObjectIdV1,
    destruction: ObjectDestructionDispositionV1,
    children: Option<Box<[OwnedFieldChildV1]>>,
}
impl ObjectReturnTeardownDescriptorV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn object(
        &self,
    ) -> CanonicalObjectIdV1 {
        self.object
    }
    /// Preserve the original Plain/owned-field distinction at local acquisition.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn owned_children(
        &self,
    ) -> Option<&[OwnedFieldChildV1]> {
        self.children.as_deref()
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

    #[cfg(test)]
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
        /// Original per-New actuals retain SAME child-relation loans across calls.
        /// These are source provenance, not field refinement or support permission.
        arguments: Box<[OrdinaryNewTrivialArgumentV1]>,
    },
}

impl VerifiedObjectReturnLeafV1 {
    /// The old direct End owns every child. A source Forward child needs
    /// the still-unconnected paired residence transport and conditional End.
    pub(super) fn requires_anchored_field_end_v1(&self) -> bool {
        match self {
            Self::Null => false,
            Self::Fresh { arguments, .. } => arguments.iter()
                .flat_map(|argument| argument.child_relations())
                .flat_map(|relation| relation.witnesses())
                .any(|witness| matches!(witness.origin(), ResultValueOriginV1::ForwardFormal { .. })),
        }
    }
}

/// An immutable source dependency, not an ownership or field-storage receipt.
/// The original relation and witness remain shared through every call hop.
#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) struct ObjectReturnFieldSupportV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) relation: Rc<result_class_claim::SourceConstructionChildRelationV1>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) witness: Rc<ResultOriginWitnessV1>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) formal: BindingRefV1,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) ordinal: u32,
}
impl PartialEq for ObjectReturnFieldSupportV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.relation, &other.relation) && Rc::ptr_eq(&self.witness, &other.witness)
            && self.formal == other.formal && self.ordinal == other.ordinal
    }
}
impl Eq for ObjectReturnFieldSupportV1 {}

impl OrdinaryNewClaimLedgerV1 {
    /// Corroborate only the exact immutable leaf named by the original Facts.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_object_return_leaf_v1(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
        witness: &Rc<ResultOriginWitnessV1>,
    ) -> Result<Option<VerifiedObjectReturnLeafV1>, String> {
        self.checked_original_return_leaf_v1(key, witness, None)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn checked_mixed_return_leaf_v1(
        &self,
        loan: &result_class_claim::MixedLocalCallSourceLoanV1,
        witness: &Rc<ResultOriginWitnessV1>,
    ) -> Result<Option<VerifiedObjectReturnLeafV1>, String> {
        self.callable_result_classes.recheck_mixed_local_roots_v1(loan).map_err(freeze)?;
        if witness.site().owner() != loan.callee_owner()
            || !loan.roots().iter().any(|root| Rc::ptr_eq(root, witness))
        { return Err(freeze("mixed-return/foreign-leaf")); }
        let leaf = self.checked_original_return_leaf_v1(loan.key(), witness, Some(loan.fresh_class()))?;
        if matches!(&leaf, Some(VerifiedObjectReturnLeafV1::Fresh { teardown, .. }) if teardown.object() != loan.object()) {
            return Err(freeze("mixed-return/leaf-object"));
        }
        Ok(leaf)
    }

    /// Borrow the original returned-New leaf before or after affine take.
    /// Its existing owner validates the complete relation census and formals.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn returned_new_field_supports_v1(
        &self, site: &OwnedExprSiteV1,
    ) -> Result<Vec<ObjectReturnFieldSupportV1>, String> {
        let mut original = self.callable_result_classes.source_rows().flat_map(|(key, rows)|
            rows.iter().flat_map(move |row| row.witnesses().iter().map(move |witness| (key, witness))))
            .filter(|(_, witness)| witness.site() == site
                && matches!(witness.step(), ResultWitnessStepV1::FreshConstruction));
        let (key, witness) = original.next().ok_or_else(|| freeze("object-return/field-support-leaf-missing"))?;
        if original.next().is_some() { return Err(freeze("object-return/field-support-leaf-duplicate")); }
        let leaf = self.checked_object_return_leaf_v1(key, witness)?
            .ok_or_else(|| freeze("object-return/field-support-leaf-unavailable"))?;
        self.object_return_leaf_supports_v1(&leaf)
    }

    pub(super) fn object_return_leaf_supports_v1(
        &self,
        leaf: &VerifiedObjectReturnLeafV1,
    ) -> Result<Vec<ObjectReturnFieldSupportV1>, String> {
        let VerifiedObjectReturnLeafV1::Fresh { site, arguments, .. } = leaf else {
            return Ok(Vec::new());
        };
        self.check_return_argument_relations_v1(site, arguments)?;
        let mut supports = Vec::new();
        for relation in arguments.iter().flat_map(|argument| argument.child_relations()) {
            for witness in relation.witnesses() {
                let ResultValueOriginV1::ForwardFormal { ordinal } = witness.origin() else {
                    continue;
                };
                let binding = match witness.step() {
                    ResultWitnessStepV1::Formal { binding, ordinal: source } if source == ordinal => *binding,
                    ResultWitnessStepV1::Call { callee, substitution: Some(row), .. }
                        if row.caller_ordinal == *ordinal && row.argument_site.owner() == site.owner()
                            && matches!(callee.origin(), ResultValueOriginV1::ForwardFormal { ordinal } if *ordinal == row.callee_ordinal) => row.binding,
                    _ => return Err(freeze("object-return/field-support-witness")),
                };
                let catalog = self.original_parameter_contracts.as_deref()
                    .ok_or_else(|| freeze("object-return/field-support-catalog"))?;
                let mut rows = catalog.iter().filter(|row| row.owner == site.owner());
                let contract = rows.next().ok_or_else(|| freeze("object-return/field-support-owner"))?;
                if rows.next().is_some() { return Err(freeze("object-return/field-support-owner")); }
                let mut parameters = contract.parameters.iter().filter(|row| row.ordinal == *ordinal);
                let parameter = parameters.next().ok_or_else(|| freeze("object-return/field-support-formal"))?;
                if parameters.next().is_some() || parameter.binding != binding || binding.owner() != site.owner()
                    || witness.site() != relation.actual()
                    || !matches!(parameter.kind, crate::mir::callable_parameter_contract::CallableParameterContractKindV1::DeclaredObject(_))
                { return Err(freeze("object-return/field-support-formal")); }
                supports.push(ObjectReturnFieldSupportV1 {
                    relation: Rc::clone(relation), witness: Rc::clone(witness), formal: binding, ordinal: *ordinal,
                });
            }
        }
        Ok(supports)
    }

    /// Recheck the complete original relation census, not just the loans still
    /// present in an argument row. Dropping a loan cannot erase a dependency.
    pub(in crate::mir::normal_callable_semantic_package) fn check_return_argument_relations_v1(
        &self,
        site: &OwnedExprSiteV1,
        arguments: &[OrdinaryNewTrivialArgumentV1],
    ) -> Result<(), String> {
        let original: Vec<_> = self.callable_result_classes
            .construction_child_relation_loans()
            .filter(|relation| relation.outer() == site)
            .collect();
        if arguments.iter().enumerate().any(|(ordinal, argument)|
            argument.owner() != site.owner() || argument.new_site() != site
                || argument.ordinal() as usize != ordinal)
        {
            return Err(freeze("object-return/leaf-argument-identity"));
        }
        let retained: Vec<_> = arguments.iter()
            .flat_map(|argument| argument.child_relations().iter()
                .map(move |relation| (argument, relation)))
            .collect();
        let reordered = arguments.iter().any(|argument| {
            let expected: Vec<_> = original.iter()
                .filter(|relation| relation.formal_ordinal() == argument.ordinal())
                .collect();
            expected.len() != argument.child_relations().len()
                || expected.iter().zip(argument.child_relations())
                    .any(|(source, loan)| !Rc::ptr_eq(source, loan))
        });
        if reordered || retained.len() != original.len()
            || original.iter().any(|source| retained.iter()
                .filter(|(_, relation)| Rc::ptr_eq(source, relation)).count() != 1)
            || retained.iter().any(|(argument, relation)|
                relation.outer() != site || relation.actual().owner() != site.owner()
                    || relation.actual().site() != argument.site()
                    || relation.formal_ordinal() != argument.ordinal())
        {
            return Err(freeze("object-return/leaf-child-relation-identity"));
        }
        Ok(())
    }

    fn checked_original_return_leaf_v1(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
        witness: &Rc<ResultOriginWitnessV1>,
        mixed_class: Option<&str>,
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
        let Some(class) = mixed_class.or_else(|| self
            .callable_result_classes
            .get(key)
            .and_then(|row| row.class()))
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
                self.with_result_new_source_v1(site, |source| {
                    if source.site != site || source.class != class {
                        return Err(freeze("object-return/leaf-claim-identity"));
                    }
                    let (Ok(construction), Ok(prefix), Ok(arguments)) =
                        (source.construction, source.prefix, source.arguments)
                    else {
                        return Ok(None);
                    };
                    if construction.object() != source.object
                        || prefix.required_unwind() != site
                        || prefix.normal_homes() != homes.homes()
                        || arguments.len() != source.arity
                    {
                        return Err(format!(
                            "{} key={key:?} site={site:?} object_matches={} required={:?} prior={:?} normal={:?} expected={:?} arguments={}/{}",
                            freeze("object-return/leaf-prefix-identity"),
                            construction.object() == source.object, prefix.required_unwind(),
                            prefix.prior_homes(), prefix.normal_homes(), homes.homes(), arguments.len(), source.arity,
                        ));
                    }
                    self.check_return_argument_relations_v1(site, arguments)?;
                    Ok(Some(VerifiedObjectReturnLeafV1::Fresh {
                        site: site.clone(),
                        arguments: arguments.clone(),
                        teardown: ObjectReturnTeardownDescriptorV1 {
                            object: source.object,
                            destruction: source.destruction,
                            children: source.children.map(Into::into),
                        },
                    }))
                })
                .map(Option::flatten)
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
