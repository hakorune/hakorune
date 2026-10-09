//! Select the original Static I64 Loop before the generic V1 facts issuer.
//! The producer is semantic-only; this route stays closed until its Home and
//! physical SSA session can consume the verified V2 product.

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1;
use crate::mir::normal_callable_semantic_package::VerifiedStaticLoopTaggedEntrySourceV1;
use crate::mir::resolved_semantics::{
    OwnedExprSiteV1, ResolvedLexicalRefV1, ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1,
    SourceNodeSiteV1, SourcePathSegmentV1 as Segment, SourceStmtSiteV1,
};

use super::super::normal_callable_loop_source_facts::produce_static_i64_loop_semantic_v2;
use super::super::normal_callable_loop_source_facts::VerifiedStaticI64LoopSemanticV2;

#[derive(Debug)]
pub(in crate::mir) struct StaticI64LoopFunctionEntryV2 {
    semantic: VerifiedStaticI64LoopSemanticV2,
    tagged: VerifiedStaticLoopTaggedEntrySourceV1,
}

impl StaticI64LoopFunctionEntryV2 {
    pub(in crate::mir) fn semantic(&self) -> &VerifiedStaticI64LoopSemanticV2 {
        &self.semantic
    }

    pub(in crate::mir) fn tagged_formal(&self) -> crate::mir::resolved_semantics::BindingRefV1 {
        self.tagged.formal()
    }
}

pub(in crate::mir) fn take_at_function_entry_v2(
    input: ResolvedFunctionLoweringInputV1<'_>,
    claims: &OrdinaryNewClaimLedgerV1,
) -> Result<Option<StaticI64LoopFunctionEntryV2>, String> {
    let mut selected = input
        .function()
        .loop_sites()
        .filter(|site| claims.expects_loop_static_source_loan_v1(input.owner(), site));
    let Some(loop_site) = selected.next() else {
        return Ok(None);
    };
    if selected.next().is_some() {
        return Err(
            "[freeze:contract][callable-loop/static-i64-v2/duplicate-selected-loop]".to_owned(),
        );
    }
    take_selected_semantic_product(input, claims, loop_site)?
        .ok_or_else(|| {
            "[freeze:contract][callable-loop/static-i64-v2/source-unavailable]".to_owned()
        })
        .map(Some)
}

pub(in crate::mir) fn stop_after_selected_semantic_product(
    input: ResolvedFunctionLoweringInputV1<'_>,
    claims: &OrdinaryNewClaimLedgerV1,
    loop_site: &SourceStmtSiteV1,
) -> Result<(), String> {
    if take_selected_semantic_product(input, claims, loop_site)?.is_some() {
        return Err(
            "[freeze:contract][callable-loop/static-i64-v2/physical-unavailable]".to_owned(),
        );
    }
    Ok(())
}

fn take_selected_semantic_product(
    input: ResolvedFunctionLoweringInputV1<'_>,
    claims: &OrdinaryNewClaimLedgerV1,
    loop_site: &SourceStmtSiteV1,
) -> Result<Option<StaticI64LoopFunctionEntryV2>, String> {
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
        return if claims.expects_loop_static_source_loan_v1(input.owner(), loop_site) {
            Err(reject())
        } else {
            Ok(None)
        };
    }
    let header = claims
        .take_loop_static_source_call_loan_v1(loop_site, &header_site)
        .transpose()?;
    let body = claims
        .take_loop_static_source_call_loan_v1(loop_site, &body_site)
        .transpose()?;
    let (header, body) = match (header, body) {
        (None, None) if !claims.expects_loop_static_source_loan_v1(input.owner(), loop_site) => {
            return Ok(None)
        }
        (Some(header), Some(body)) => (header, body),
        _ => return Err(reject()),
    };
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
    let entry = claims
        .take_loop_entry_static_i64_source_loan_v1(loop_site, n_declaration)
        .ok_or_else(reject)??;
    let tail = claims
        .take_loop_tail_static_i64_source_loan_v1(loop_site)
        .ok_or_else(reject)??;
    let completion = claims
        .completion_for_owner(input.owner())
        .ok_or_else(reject)?;
    let product = produce_static_i64_loop_semantic_v2(
        input, loop_site, completion, entry, header, body, tail,
    )?;
    let home = claims
        .take_loop_static_home_neutral_v1(loop_site)
        .ok_or_else(reject)??;
    if !home.corroborates(&product) {
        return Err("[freeze:contract][callable-loop/static-home-effect-mismatch]".to_owned());
    }
    let tagged = claims
        .take_loop_static_tagged_entry_v1(loop_site, product.source_calls().0.declaration())
        .ok_or_else(reject)??;
    if !tagged.corroborates(product.source_calls().0) {
        return Err("[freeze:contract][callable-loop/static-tagged-entry-mismatch]".to_owned());
    }
    Ok(Some(StaticI64LoopFunctionEntryV2 {
        semantic: product,
        tagged,
    }))
}
