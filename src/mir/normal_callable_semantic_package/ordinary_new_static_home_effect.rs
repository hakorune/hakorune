//! ClosedCallable Home-effect proof for the selected Static I64 Loop.
//!
//! This is a whole-body, transitive source proof. It borrows the package's
//! original Static target/actual issuer; it never infers Home neutrality from
//! an I64 result, absent `new`, a source comment, or a physical carrier.

use std::collections::BTreeSet;

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, UnaryOperator};
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableCatalogBrandV1, SelectedNormalCallableKeyV1,
};
use crate::mir::callable_parameter_contract::{
    CallableParameterContractKindV1, CallableParameterDeclarationModeV1,
};
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::resolved_semantics::{OwnedExprSiteV1, SourceStmtSiteV1};

use super::super::model::OwnedCallableParameterContractDeclarationV1;
use super::super::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1;
use super::lexical_instance_call::PreparedBorrowedFormalIngressV1;
use super::loop_static_source_loan::issue_static_source_seal_v1;

#[derive(Debug)]
pub(in crate::mir) struct VerifiedClosedStaticLoopHomeNeutralV1 {
    loop_site: SourceStmtSiteV1,
    caller: CanonicalSameModuleCallableKeyV1,
    brand: SameModuleCallableCatalogBrandV1,
}

impl VerifiedClosedStaticLoopHomeNeutralV1 {
    pub(in crate::mir) fn corroborates(
        &self,
        product: &crate::mir::builder::VerifiedStaticI64LoopSemanticV2,
    ) -> bool {
        let (entry, header, body) = product.source_calls();
        let tail = product.tail_call();
        self.loop_site == product.roles().loop_site
            && [
                entry.original(),
                header.original(),
                body.original(),
                tail.original(),
            ]
            .into_iter()
            .all(|source| {
                self.brand.is_same(source.catalog_brand()) && self.caller == *source.caller()
            })
    }
}

pub(super) fn issue_closed_static_loop_home_neutral_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    claims: &QualifiedStaticCallClaimIndexV1,
    incoming: &PreparedBorrowedFormalIngressV1,
    caller: &CanonicalSameModuleCallableKeyV1,
    loop_site: &SourceStmtSiteV1,
) -> Result<VerifiedClosedStaticLoopHomeNeutralV1, String> {
    let reject = || "[freeze:contract][callable-loop/static-home-effect-unavailable]".to_owned();
    let mut visiting = BTreeSet::new();
    let mut proven = BTreeSet::new();
    prove_closed_static_body(
        batch,
        selected,
        contracts,
        claims,
        incoming,
        caller,
        &mut visiting,
        &mut proven,
    )?;
    let slot = selected
        .batch_slot(&SelectedNormalCallableKeyV1::Cataloged(caller.clone()))
        .ok_or_else(reject)?;
    let owns_loop = batch
        .with_lowering_input(slot, |input| {
            input.function().loop_sites().any(|site| site == loop_site)
        })
        .map_err(|_| reject())?;
    if !owns_loop {
        return Err(reject());
    }
    Ok(VerifiedClosedStaticLoopHomeNeutralV1 {
        loop_site: loop_site.clone(),
        caller: caller.clone(),
        brand: claims.catalog_brand().clone(),
    })
}

#[allow(clippy::too_many_arguments)]
fn prove_closed_static_body(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    claims: &QualifiedStaticCallClaimIndexV1,
    incoming: &PreparedBorrowedFormalIngressV1,
    key: &CanonicalSameModuleCallableKeyV1,
    visiting: &mut BTreeSet<CanonicalSameModuleCallableKeyV1>,
    proven: &mut BTreeSet<CanonicalSameModuleCallableKeyV1>,
) -> Result<(), String> {
    let reject = || "[freeze:contract][callable-loop/static-home-effect-unavailable]".to_owned();
    if proven.contains(key) {
        return Ok(());
    }
    if !visiting.insert(key.clone()) {
        return Err(reject());
    }
    let slot = selected
        .batch_slot(&SelectedNormalCallableKeyV1::Cataloged(key.clone()))
        .ok_or_else(reject)?;
    let mut rows = contracts.iter().filter(|row| row.batch_slot == slot);
    let contract = rows.next().ok_or_else(reject)?;
    if rows.next().is_some()
        || contract.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
        || contract.parameters.len() != key.arity() as usize
        || contract.parameters.iter().any(|formal| {
            !matches!(
                formal.kind,
                CallableParameterContractKindV1::OpaqueHandle
                    | CallableParameterContractKindV1::ExactTrivial(_)
            )
        })
        || contract.parameters.iter().any(|formal| {
            formal.kind == CallableParameterContractKindV1::OpaqueHandle
                && !incoming.checked_static_input(formal.binding)
        })
    {
        return Err(reject());
    }
    let targets = batch
        .with_lowering_input(slot, |input| {
            if input.owner() != contract.owner {
                return Err(reject());
            }
            let ASTNode::FunctionDeclaration {
                body,
                return_type_name,
                ..
            } = input.source().root()
            else {
                return Err(reject());
            };
            // The current result solver accepts an annotated I64 without reading
            // its body. This bounded issuer only uses body-proven results.
            if return_type_name.is_some() {
                return Err(reject());
            }
            let mut syntax_calls = 0usize;
            for statement in body {
                scan_home_neutral_syntax(statement, &mut syntax_calls)?;
            }
            let calls: Vec<_> = input.function().method_calls().collect();
            if calls.len() != syntax_calls {
                return Err(reject());
            }
            let mut targets = Vec::with_capacity(calls.len());
            for (site, call) in calls {
                let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
                if !matches!(
                    input.source().expr_at(&owned).map_err(|_| reject())?.node(),
                    ASTNode::MethodCall { .. }
                ) {
                    return Err(reject());
                }
                let seal = issue_static_source_seal_v1(
                    claims, incoming, selected, contracts, key, input, &owned,
                )?
                .ok_or_else(reject)?;
                if seal.original().argument_sites().len() != call.arity() as usize {
                    return Err(reject());
                }
                targets.push(seal.original().target().clone());
            }
            Ok(targets)
        })
        .map_err(|_| reject())??;
    for target in targets {
        prove_closed_static_body(
            batch, selected, contracts, claims, incoming, &target, visiting, proven,
        )?;
    }
    visiting.remove(key);
    proven.insert(key.clone());
    Ok(())
}

fn scan_home_neutral_syntax(node: &ASTNode, calls: &mut usize) -> Result<(), String> {
    let reject = || "[freeze:contract][callable-loop/static-home-effect-unavailable]".to_owned();
    match node {
        ASTNode::Literal {
            value:
                LiteralValue::Integer(_) | LiteralValue::TypedInteger { .. } | LiteralValue::Bool(_),
            ..
        }
        | ASTNode::Variable { .. } => Ok(()),
        ASTNode::MethodCall {
            object, arguments, ..
        } if matches!(object.as_ref(), ASTNode::Me { .. }) => {
            *calls = calls.checked_add(1).ok_or_else(reject)?;
            for argument in arguments {
                scan_home_neutral_syntax(argument, calls)?;
            }
            Ok(())
        }
        ASTNode::UnaryOp {
            operator: UnaryOperator::Minus | UnaryOperator::Not,
            operand,
            ..
        } => scan_home_neutral_syntax(operand, calls),
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } if matches!(
            operator,
            BinaryOperator::Add
                | BinaryOperator::Subtract
                | BinaryOperator::Multiply
                | BinaryOperator::Divide
                | BinaryOperator::Modulo
                | BinaryOperator::Equal
                | BinaryOperator::NotEqual
                | BinaryOperator::Less
                | BinaryOperator::Greater
                | BinaryOperator::LessEqual
                | BinaryOperator::GreaterEqual
                | BinaryOperator::And
                | BinaryOperator::Or
        ) =>
        {
            scan_home_neutral_syntax(left, calls)?;
            scan_home_neutral_syntax(right, calls)
        }
        ASTNode::Local { initial_values, .. } => {
            for value in initial_values {
                scan_home_neutral_syntax(value.as_ref().ok_or_else(reject)?, calls)?;
            }
            Ok(())
        }
        ASTNode::Assignment { target, value, .. }
            if matches!(target.as_ref(), ASTNode::Variable { .. }) =>
        {
            scan_home_neutral_syntax(value, calls)
        }
        ASTNode::Return {
            value: Some(value), ..
        } => scan_home_neutral_syntax(value, calls),
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            scan_home_neutral_syntax(condition, calls)?;
            for statement in then_body {
                scan_home_neutral_syntax(statement, calls)?;
            }
            if let Some(else_body) = else_body {
                for statement in else_body {
                    scan_home_neutral_syntax(statement, calls)?;
                }
            }
            Ok(())
        }
        ASTNode::Loop {
            condition, body, ..
        } => {
            scan_home_neutral_syntax(condition, calls)?;
            for statement in body {
                scan_home_neutral_syntax(statement, calls)?;
            }
            Ok(())
        }
        ASTNode::Break { .. } | ASTNode::Continue { .. } => Ok(()),
        _ => Err(reject()),
    }
}
