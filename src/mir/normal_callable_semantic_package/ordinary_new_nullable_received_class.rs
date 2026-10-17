//! Source class of the exact nullable initializer, before execution permission.
//! Self and claim-local receivers share the same selected NullableObject claim.
use super::*;
use crate::mir::resolved_semantics::{
    ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, ResolvedMethodCallReceiverSourceV1,
};

pub(in crate::mir::normal_callable_semantic_package) fn nullable_received_result_class(
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    classes: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    candidates: &[OrdinaryNewCandidate],
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    home: BindingRefV1,
) -> Option<Box<str>> {
    let function = input.function();
    if home.owner() != input.owner()
        || !matches!(function.binding(home)?.kind(), BindingKindV1::Local { .. })
        || function.assignment_targets().any(|(_, target)| {
            matches!(target,
            ResolvedAssignmentTargetV1::BindingRebind(binding) if *binding == home)
        })
    {
        return None;
    }
    let mut initializers = function
        .expression_source()
        .initializers()
        .filter(|row| row.binding() == home);
    let initializer = initializers.next()?;
    if initializers.next().is_some()
        || function.declaration_binding(initializer.declaration_site()) != Some(home)
    {
        return None;
    }
    let site = initializer.initializer_site()?;
    let call = function.method_call(site)?;
    if call.owner() != input.owner() || call.site() != site {
        return None;
    }
    let key = match call.receiver() {
        ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding))
            if receiver.is_some_and(|(root, _)| root == binding) =>
        {
            let (root, own_box) = receiver?;
            let mut callers = batch
                .declarations()
                .filter(|row| row.owner() == input.owner());
            let caller = callers.next()?;
            let crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(caller_key) =
                selected.key_for_batch_slot(caller.batch_slot())?
            else {
                return None;
            };
            if callers.next().is_some()
                || caller_key.namespace()
                    != crate::mir::builder::SameModuleCallableNamespaceV1::InstanceBoxMethod
                || caller_key.owner() != own_box.name()
            {
                return None;
            }
            if root.owner() != input.owner()
                || function.declaration_binding(&SourceBindingSiteV1::Receiver) != Some(root)
                || !matches!(function.binding(root)?.kind(), BindingKindV1::Receiver)
            {
                return None;
            }
            let (key, slot) = super::super::super::lexical_instance_call::unique_instance_target(
                selected,
                own_box.name(),
                call.selector(),
                call.arity(),
            )?;
            let mut declarations = batch.declarations().filter(|row| row.batch_slot() == slot);
            let declaration = declarations.next()?;
            if declarations.next().is_some()
                || declaration.owner().compilation_brand() != input.owner().compilation_brand()
            {
                return None;
            }
            key
        }
        _ => {
            lexical_claim_local_target(
                selected,
                batch,
                candidates,
                input,
                &OwnedExprSiteV1::new(input.owner(), site.clone()),
            )?
            .0
        }
    };
    match classes.get(&key) {
        Some(super::super::super::OrdinaryNewResultClassV1::NullableObject(class)) => {
            Some(class.clone())
        }
        _ => None,
    }
}

#[cfg(test)]
#[path = "ordinary_new_nullable_received_class_tests.rs"]
mod tests;
