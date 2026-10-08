//! Receiver field-call admission inside `scan_statement_flow`.
//!
//! `me.<field>.m(..)` on a declared `ArrayBox` field of the self-rooted `me`
//! is a covered prefix statement when the (selector, arity) pair resolves
//! in the generated `CORE_METHOD_CONTRACT_ROWS_V2` manifest and every
//! argument subtree is Home-neutral. Statement position admits `NoValue`
//! result rows only; `local x = ..` installs the manifest result class —
//! `I64Value`/`BoolValue` as scalar `Trivial`, `Dynamic`/`StringValue` as
//! `BoundValue` (a produced value carrying no release obligation).
//!
//! `get/1` is the one exception to manifest-result installation: when the
//! issuer's whole-Box element-integer census sealed the field and the
//! index observes an Integer leaf, the `Dynamic` manifest result upgrades
//! to `I64Value` — the sole authority for `me.<field>.get` element-i64.
//! Without the census seal nothing changes; the result stays `Dynamic`.
//!
//! No ledger row is minted: the raw lane's `Callee::Method{RuntimeData}` /
//! `ArrayElementWrite` emission already owns the instruction, and neither
//! is lifecycle-required. Anything outside the manifest contract keeps
//! `PrefixNotCovered`.
use super::*;
use crate::mir::core_method_result_kind::{
    lookup_core_method_result_row_v2, CoreMethodResultKindV1,
};
use crate::mir::resolved_semantics::{BodyExpressionShapeV1, VerifiedResolvedBodyShapeInventoryV1};

/// The shared proof for one `me.<field>.m(..)` call site: the sealed
/// method-call row's receiver must be a `FieldAccess` rooted at this
/// frame's self-rooted `me`, the field's declared type must prove
/// `ArrayBox` via the caller's `container_field` predicate, the
/// (selector, arity) pair must resolve in the generated manifest, and
/// every argument subtree must be Home-neutral. Returns the manifest
/// result kind only when every obligation holds.
fn proven_field_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    call_site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    homes: &[BindingRefV1],
    container_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    array_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    // The issuer's `formal.<field>` index proof: the scanner supplies the
    // exact site, receiver site, and the receiver's resolved parameter
    // binding; the predicate alone decides the unique-declaration proof.
    formal_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    scalar_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    view_use: &mut impl FnMut(&OwnedExprSiteV1, BorrowedViewUseRequestV1<'_>) -> Result<bool, E>,
) -> Result<Option<CoreMethodResultKindV1>, E> {
    let Some(shape) = input.body_shape() else {
        return Ok(None);
    };
    let Some(call) = input.function().method_call(call_site) else {
        return Ok(None);
    };
    // The FieldAccess shape row is the receiver proof — the `receiver`
    // disposition only names non-lexical (`Other`), the exact receiver
    // site is what gets checked.
    let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
        shape.expression_shape(call.receiver_site())
    else {
        return Ok(None);
    };
    let Some(home) = field_write::self_rooted_me(shape, object, locals) else {
        return Ok(None);
    };
    if !container_field(
        &OwnedExprSiteV1::new(input.owner(), call.receiver_site().clone()),
        object,
        home,
        home,
        field,
    )? {
        return Ok(None);
    }
    let Some(row) = lookup_core_method_result_row_v2(
        "ArrayBox",
        call.selector(),
        call.arity(),
    ) else {
        return Ok(None);
    };
    for argument in call.arguments() {
        if !argument_subtree_neutral(
            input,
            shape,
            argument.site(),
            locals,
            homes,
            scalar_field,
            formal_i64_field,
            view_use,
        )? {
            return Ok(None);
        }
    }
    // `get/1` upgrades the manifest `Dynamic` result to i64 only when the
    // issuer's whole-Box census sealed this field's element stores as
    // integer and the index observes an Integer leaf — an Integer literal
    // or Integer-class binding, a proven `me.<numeric field>` read, or a
    // `formal.<field>` read whose parameter binding and unique integer
    // declaration the issuer proves, the same leaves the
    // `integer_source` contract credits. Anything else keeps the manifest
    // result — the call stays covered, the value stays dynamic.
    let mut index_i64 = |ordinal: usize| -> Result<bool, E> {
        let site = call.arguments()[ordinal].site();
        if matches!(
            locals.observe(site),
            Some(local_flow::OrdinaryObservation::Integer(_))
                | Some(local_flow::OrdinaryObservation::TrivialLocal(
                    _,
                    Some(local_flow::SourceScalarKind::Integer),
                ))
        ) {
            return Ok(true);
        }
        let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
            shape.expression_shape(site)
        else {
            return Ok(false);
        };
        if let Some(home) = field_write::self_rooted_me(shape, object, locals) {
            return scalar_field(
                &OwnedExprSiteV1::new(input.owner(), site.clone()),
                object,
                home,
                home,
                field,
            );
        }
        // `formal.<field>`: the object resolves to a lexical binding the
        // issuer may prove a parameter — the predicate owns both the
        // binding-kind check and the unique-declaration proof.
        let Some(BodyExpressionShapeV1::Variable {
            resolved: crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding),
            ..
        }) = shape.expression_shape(object)
        else {
            return Ok(false);
        };
        formal_i64_field(
            &OwnedExprSiteV1::new(input.owner(), site.clone()),
            object,
            *binding,
            *binding,
            field,
        )
    };
    let kind = match (call.selector(), call.arity()) {
        ("get", 1)
            if row.result_kind == CoreMethodResultKindV1::Dynamic
                && index_i64(0)?
                && array_i64_field(
                    &OwnedExprSiteV1::new(input.owner(), call.receiver_site().clone()),
                    object,
                    home,
                    home,
                    field,
                )? =>
        {
            CoreMethodResultKindV1::I64Value
        }
        _ => row.result_kind,
    };
    Ok(Some(kind))
}

/// `true` when every sealed expression row under `root` is Home-neutral
/// as a builtin call argument: no `new`/call/map/array/block nodes, `me`
/// only as a FieldAccess receiver, `me.<field>` reads proven scalar by
/// the issuer predicate, and variable leaves observing `Trivial`, `Null`,
/// or a `BoundValue` that is not a live `homes` member (a `BoundValue`
/// result of a nullable receiver call is a `homes` member — passing it
/// into a builtin call is an untracked ownership transfer).
fn argument_subtree_neutral<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    shape: &VerifiedResolvedBodyShapeInventoryV1,
    root: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    homes: &[BindingRefV1],
    scalar_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    formal_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    view_use: &mut impl FnMut(&OwnedExprSiteV1, BorrowedViewUseRequestV1<'_>) -> Result<bool, E>,
) -> Result<bool, E> {
    let prefix = root.node().segments();
    let subtree: Vec<&BodyExpressionShapeV1> = shape
        .expressions()
        .iter()
        .filter(|expression| {
            let segments = expression.site().node().segments();
            segments.len() >= prefix.len() && segments.starts_with(prefix)
        })
        .collect();
    if subtree.is_empty() {
        return Ok(false);
    }
    let field_receivers: std::collections::BTreeSet<&SourceExprSiteV1> = subtree
        .iter()
        .filter_map(|expression| match expression {
            BodyExpressionShapeV1::FieldAccess { object, .. } => Some(object),
            _ => None,
        })
        .collect();
    for expression in subtree {
        match expression {
            BodyExpressionShapeV1::MapLiteral { .. }
            | BodyExpressionShapeV1::ArrayLiteral { .. }
            | BodyExpressionShapeV1::QualifiedReceiver { .. }
            | BodyExpressionShapeV1::MethodCall { .. }
            | BodyExpressionShapeV1::BlockExpr { .. } => return Ok(false),
            BodyExpressionShapeV1::Variable { site, .. } => match locals.observe(site) {
                Some(
                    local_flow::OrdinaryObservation::Integer(_)
                    | local_flow::OrdinaryObservation::Bool(_)
                    | local_flow::OrdinaryObservation::Null
                    | local_flow::OrdinaryObservation::TrivialLocal(..),
                ) => {}
                Some(local_flow::OrdinaryObservation::BoundValue(binding))
                    if !homes.contains(&binding) => {}
                // A `Handle` leaf is neutral only when the sealed draft
                // admits a dominated-view value use at this exact site —
                // the issuer's predicate is the consult, the draft stays
                // the sole admission authority.
                _ => {
                    if !view_use(
                        &OwnedExprSiteV1::new(input.owner(), site.clone()),
                        BorrowedViewUseRequestV1::Operand,
                    )? {
                        return Ok(false);
                    }
                }
            },
            BodyExpressionShapeV1::Me { site, .. } => {
                if !field_receivers.contains(site) {
                    return Ok(false);
                }
            }
            BodyExpressionShapeV1::FieldAccess {
                site,
                object,
                field,
            } => {
                if let Some(home) = field_write::self_rooted_me(shape, object, locals) {
                    if !scalar_field(
                        &OwnedExprSiteV1::new(input.owner(), site.clone()),
                        object,
                        home,
                        home,
                        field,
                    )? {
                        return Ok(false);
                    }
                    continue;
                }
                // `formal.<field>` reads are neutral only when the issuer
                // proves the parameter binding and the unique integer
                // declaration — a proven-i64 read yields a scalar.
                let Some(BodyExpressionShapeV1::Variable {
                    resolved: crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding),
                    ..
                }) = shape.expression_shape(object)
                else {
                    return Ok(false);
                };
                if !formal_i64_field(
                    &OwnedExprSiteV1::new(input.owner(), site.clone()),
                    object,
                    *binding,
                    *binding,
                    field,
                )? {
                    return Ok(false);
                }
            }
            BodyExpressionShapeV1::Other { site, kind } => match &**kind {
                "UnaryOp" | "BinaryOp" => {}
                "Literal" => {
                    if !matches!(
                        locals.observe(site),
                        Some(
                            local_flow::OrdinaryObservation::Integer(_)
                                | local_flow::OrdinaryObservation::Bool(_)
                                | local_flow::OrdinaryObservation::Null
                        )
                    ) {
                        return Ok(false);
                    }
                }
                _ => return Ok(false),
            },
        }
    }
    Ok(true)
}

/// A bare `me.<ArrayBox field>.m(..)` expression statement is covered when
/// the call is manifest-proven and `NoValue` — a produced value discarded
/// silently is never admitted.
pub(super) fn observe_statement_field_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    homes: &[BindingRefV1],
    container_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    array_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    formal_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    scalar_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    view_use: &mut impl FnMut(&OwnedExprSiteV1, BorrowedViewUseRequestV1<'_>) -> Result<bool, E>,
) -> Result<bool, E> {
    if !matches!(statement.node(), ASTNode::MethodCall { .. }) {
        return Ok(false);
    }
    let call_site = SourceExprSiteV1::from_node(statement.site().node().clone());
    Ok(matches!(
        proven_field_call(
            input,
            &call_site,
            locals,
            homes,
            container_field,
            array_i64_field,
            formal_i64_field,
            scalar_field,
            view_use,
        )?,
        Some(CoreMethodResultKindV1::NoValue)
    ))
}

/// A `local x = me.<ArrayBox field>.m(..)` initializer is covered when the
/// call is manifest-proven and returns a value — the caller installs the
/// binding by the returned result class (`NoValue` never binds).
pub(super) fn observe_local_field_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    homes: &[BindingRefV1],
    container_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    array_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    formal_i64_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    scalar_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
    view_use: &mut impl FnMut(&OwnedExprSiteV1, BorrowedViewUseRequestV1<'_>) -> Result<bool, E>,
) -> Result<Option<CoreMethodResultKindV1>, E> {
    let Some(kind) = proven_field_call(
        input,
        site,
        locals,
        homes,
        container_field,
        array_i64_field,
        formal_i64_field,
        scalar_field,
        view_use,
    )?
    else {
        return Ok(None);
    };
    Ok(match kind {
        CoreMethodResultKindV1::NoValue => None,
        other => Some(other),
    })
}
