//! One selected Static I64 V2 semantic product before the Home walk.
//! The existing Recipe producer remains the only topology/key verifier;
//! this owner moves its affine source loans once and keeps the same product
//! for Home source observation and later function-entry consumption.

use super::loop_static_source_loan::IssuedLoopStaticSourceLoansV1;
use crate::mir::builder::{produce_static_i64_loop_semantic_v2, StaticI64LoopFunctionEntryV2};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{
    OwnedExprSiteV1, ResolvedLexicalRefV1, ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1,
    SourceNodeSiteV1, SourcePathSegmentV1 as Segment, SourceStmtSiteV1,
};
use std::collections::BTreeSet;

impl super::OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn take_preissued_static_i64_product_v2(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Option<Result<StaticI64LoopFunctionEntryV2, String>> {
        self.preissued_static_i64_products
            .borrow_mut()
            .remove(&owner)
    }
}

pub(super) fn issue_selected_product_v2(
    input: ResolvedFunctionLoweringInputV1<'_>,
    completion: &VerifiedFunctionCompletionV1,
    loans: &mut IssuedLoopStaticSourceLoansV1,
) -> Result<Option<StaticI64LoopFunctionEntryV2>, String> {
    let mut selected = input.function().loop_sites().filter(|site| {
        loans
            .selected_sites
            .contains(&(input.owner(), (*site).clone()))
    });
    let Some(loop_site) = selected.next() else {
        return Ok(None);
    };
    if selected.next().is_some() {
        return Err(
            "[freeze:contract][callable-loop/static-i64-v2/duplicate-selected-loop]".into(),
        );
    }
    let reject = || "[freeze:contract][callable-loop/static-i64-v2/source-unavailable]".to_owned();
    let site = |relative: &[Segment]| {
        let mut path = loop_site.node().segments().to_vec();
        path.extend_from_slice(relative);
        SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(path))
    };
    let header_site =
        OwnedExprSiteV1::new(input.owner(), site(&[Segment::LoopCondition, Segment::Rhs]));
    let body_site = OwnedExprSiteV1::new(
        input.owner(),
        site(&[Segment::LoopBody(0), Segment::IfCondition, Segment::Rhs]),
    );
    let has_current_owner_call = |site: &OwnedExprSiteV1| {
        input.function().method_calls().any(|(observed, call)| {
            observed == site.site()
                && call.receiver() == ResolvedMethodCallReceiverSourceV1::CurrentOwner
        })
    };
    if !has_current_owner_call(&header_site) || !has_current_owner_call(&body_site) {
        return Err(reject());
    }
    let header = loans
        .rows
        .remove(&(loop_site.clone(), header_site))
        .ok_or_else(reject)??;
    let body = loans
        .rows
        .remove(&(loop_site.clone(), body_site))
        .ok_or_else(reject)??;
    let n_binding = match input.function().variable_ref(&site(&[
        Segment::LoopBody(0),
        Segment::IfCondition,
        Segment::Lhs,
    ])) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => return Err(reject()),
    };
    let mut n_initializers = input
        .function()
        .expression_source()
        .initializers()
        .filter(|row| row.binding() == n_binding);
    let n_declaration = n_initializers.next().ok_or_else(reject)?.declaration_site();
    if n_initializers.next().is_some() {
        return Err(reject());
    }
    let entry = loans
        .entry_rows
        .remove(&(loop_site.clone(), n_declaration.clone()))
        .ok_or_else(reject)??;
    let tail = loans.tail_rows.remove(loop_site).ok_or_else(reject)??;
    let product = produce_static_i64_loop_semantic_v2(
        input, loop_site, completion, entry, header, body, tail,
    )?;
    let expected: BTreeSet<SourceStmtSiteV1> = [
        product.roles().return_site.clone(),
        product.tail_call().return_site().clone(),
    ]
    .into_iter()
    .collect();
    if expected.len() != 2
        || completion.explicit_sites().len() != 2
        || completion
            .explicit_sites()
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>()
            != expected
        || completion.implicit_body_end().is_some()
    {
        return Err("[freeze:contract][callable-loop/static-i64-v2/exit-set-drift]".into());
    }
    let home = loans.home_rows.remove(loop_site).ok_or_else(reject)??;
    if !home.corroborates(&product) {
        return Err("[freeze:contract][callable-loop/static-home-effect-mismatch]".into());
    }
    let tagged = loans
        .tagged_rows
        .remove(&(
            loop_site.clone(),
            product.source_calls().0.declaration().clone(),
        ))
        .ok_or_else(reject)??;
    if !tagged.corroborates(product.source_calls().0) {
        return Err("[freeze:contract][callable-loop/static-tagged-entry-mismatch]".into());
    }
    Ok(Some(StaticI64LoopFunctionEntryV2::from_verified_source(
        product, tagged,
    )))
}
