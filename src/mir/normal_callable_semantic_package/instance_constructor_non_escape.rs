//! Birth-body admission, not Home Flow or a runtime cleanup plan.
//!
//! All receiver occurrences come from the sealed expression inventory. Local
//! aliases use a monotone may-alias set: source traversal order is not execution
//! order, so reassignment never proves that a binding stopped aliasing `me`.

use std::collections::BTreeSet;

use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, BodyExpressionShapeV1, BodyMeReceiverV1, ResolvedAssignmentFormV1,
    ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, SourceExprSiteV1,
    VerifiedResolvedBodyShapeInventoryV1, VerifiedResolvedFunctionV1,
    VerifiedSemanticOwnerForestV1,
};

#[derive(Debug)]
pub(in crate::mir) enum BirthReceiverNonEscapeIssueV1 {
    OwnerMismatch,
    ReceiverMissingOrDuplicate,
    Capture { _binding: BindingRefV1 },
    UnprovenUse { _site: SourceExprSiteV1 },
}

pub(super) fn verify_birth_receiver_non_escape_v1(
    function: &VerifiedResolvedFunctionV1,
    shape: &VerifiedResolvedBodyShapeInventoryV1,
    forest: &VerifiedSemanticOwnerForestV1,
) -> Result<(), BirthReceiverNonEscapeIssueV1> {
    if shape.owner() != function.owner() || forest.roots() != [function.owner()] {
        return Err(BirthReceiverNonEscapeIssueV1::OwnerMismatch);
    }
    let receivers: Vec<_> = function
        .bindings()
        .filter(|(_, binding)| binding.kind() == BindingKindV1::Receiver)
        .map(|(binding, _)| binding)
        .collect();
    let [receiver] = receivers.as_slice() else {
        return Err(BirthReceiverNonEscapeIssueV1::ReceiverMissingOrDuplicate);
    };

    // Only explicit local copies/rebinds can propagate an alias. Aggregates,
    // calls, block results and unknown expressions are not assumed transparent.
    let mut alias_edges = Vec::new();
    let mut local_targets = BTreeSet::new();
    for row in function.expression_source().initializers() {
        if let Some(site) = row.initializer_site() {
            if is_local(function, row.binding()) {
                alias_edges.push((site.clone(), row.binding()));
            }
        }
    }
    for row in shape.assignment_sources() {
        if row.form() != ResolvedAssignmentFormV1::Plain {
            continue;
        }
        if let Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) =
            function.assignment_target(row.target_site())
        {
            if is_local(function, *binding) {
                alias_edges.push((row.value_site().clone(), *binding));
                local_targets.insert(row.target_site().clone());
            }
        }
    }

    let mut aliases = BTreeSet::from([*receiver]);
    let mut occurrences = BTreeSet::new();
    loop {
        let before = (aliases.len(), occurrences.len());
        for expression in shape.expressions() {
            let source = match expression {
                BodyExpressionShapeV1::Me {
                    site,
                    receiver: BodyMeReceiverV1::Lexical(binding),
                } => Some((site, *binding)),
                BodyExpressionShapeV1::Variable {
                    site,
                    resolved: ResolvedLexicalRefV1::Local(binding),
                } => Some((site, *binding)),
                _ => None,
            };
            if let Some((site, binding)) = source {
                if aliases.contains(&binding) {
                    occurrences.insert(site.clone());
                }
            }
        }
        for (site, binding) in &alias_edges {
            if occurrences.contains(site) {
                aliases.insert(*binding);
            }
        }
        if before == (aliases.len(), occurrences.len()) {
            break;
        }
    }

    // Captures live in child owners, not necessarily the root expression list.
    for upvar in forest.upvars() {
        if aliases.contains(&upvar.source()) {
            return Err(BirthReceiverNonEscapeIssueV1::Capture {
                _binding: upvar.source(),
            });
        }
    }

    let mut admitted = local_targets;
    admitted.extend(alias_edges.into_iter().map(|(site, _)| site));
    for expression in shape.expressions() {
        if let BodyExpressionShapeV1::FieldAccess { object, .. } = expression {
            admitted.insert(object.clone());
        }
    }
    // The generic child-relation list is not exhaustive. Every occurrence must
    // instead have one of the explicit, admitted source roles above. In
    // particular method receivers, arguments, return/store values and compound
    // expressions never disappear because a generic child relation is missing.
    if let Some(site) = occurrences.difference(&admitted).next() {
        return Err(BirthReceiverNonEscapeIssueV1::UnprovenUse {
            _site: site.clone(),
        });
    }
    Ok(())
}

fn is_local(function: &VerifiedResolvedFunctionV1, binding: BindingRefV1) -> bool {
    matches!(
        function.binding(binding).map(|record| record.kind()),
        Some(BindingKindV1::Local { .. })
    )
}
