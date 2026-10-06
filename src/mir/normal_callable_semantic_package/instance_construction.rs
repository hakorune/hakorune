//! Source-issued construction obligations; not a runtime cleanup implementation.
//!
//! Eligibility is deliberately distinct from source validity. Every eligible
//! plan retains outer-storage reclamation on allocation Normal / construction
//! Fault, even when every field demand is Trivial. A store commits only on its
//! Normal edge. No MIR type, event absence or non-escape result issues this plan.

use super::{OwnedFieldChildKindV1, OwnedFieldChildV1};
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};
use std::collections::{BTreeMap, BTreeSet};

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, BodyExpressionShapeV1, BodyMeReceiverV1, FunctionOwnerIdV1,
    HomeDemandV1, OwnedExprSiteV1, ResolvedAssignmentFormV1, ResolvedAssignmentSourceV1,
    ResolvedAssignmentTargetV1, ResolvedBinaryOperatorV1, ResolvedLexicalRefV1,
    ResolvedLiteralSourceV1, ResolvedMethodCallReceiverSourceV1,
    VerifiedResolvedBodyShapeInventoryV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConstructionUnavailableV1 {
    SourceRelationMissing,
    FieldContractUnsupported,
    BodyCoverageUnsupported,
    InitializationContractMissing,
    OverrideUnsupported,
}

/// Exact RHS accepted under the constructor declaration loan.
///
/// This is carried only by the existing construction-store plan to its
/// selected physical consumer. It is neither a general expression form nor a
/// second semantic receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ConstructionStoreRhsV1 {
    LiteralI64(i64),
    Parameter {
        site: SourceExprSiteV1,
        binding: BindingRefV1,
    },
    /// Coverage-only: the plan records the provider `new` site; the
    /// existing new-expression owner produces the value. Builtin class
    /// providers keep `object`/`arguments` empty (`None`); a user-class
    /// provider carries the canonical child identity and its sealed
    /// literal arguments for the emitted Birth call. `owned_fields`
    /// names the child's declared `ArrayBox` residences — empty for a
    /// `PlainI64NoHook` child — for full teardown when child Birth
    /// completed but its parent store faults. Child Birth owns its
    /// own partial fields; its caller reclaims unpublished storage only.
    /// `caller` is this birth constructor's canonical key: the exact
    /// caller half of the `(caller, site)` publication rows a sealed
    /// `QualifiedStaticCall` argument consumes.
    ProviderConstruction {
        site: SourceExprSiteV1,
        class: Box<str>,
        object: Option<ProviderConstructionChildV1>,
        arguments: Box<[super::OrdinaryNewTrivialArgumentV1]>,
        owned_fields: Box<[CanonicalFieldRefV1]>,
        caller: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    },
}

/// A user-class provider child's sealed constructor disposition —
/// issued at plan time so emission never re-decides. `BirthIndexed`
/// defers to the per-site `birth_site_index` recipe; `NoBirthZero` is
/// the arity-0 fieldless zero-init arm — the same disposition
/// `no_birth_constructor_disposition` seals for claim `new` sites.
/// `None` stays on the builtin provider arm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProviderConstructionChildV1 {
    BirthIndexed(CanonicalObjectIdV1),
    NoBirthZero(CanonicalObjectIdV1),
}

impl ProviderConstructionChildV1 {
    pub(crate) const fn object(&self) -> CanonicalObjectIdV1 {
        match self {
            Self::BirthIndexed(object) | Self::NoBirthZero(object) => *object,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConstructionStoreV1 {
    assignment: ResolvedAssignmentSourceV1,
    field: CanonicalFieldRefV1,
    receiver_site: SourceExprSiteV1,
    receiver_binding: BindingRefV1,
    rhs: ConstructionStoreRhsV1,
    /// Prior Normal-committed residences, sealed newest-first from the
    /// constructor's source-ordered stores. The current store is excluded.
    fault_discharge: Box<[OwnedFieldChildV1]>,
    /// `me` receiver sites this store's sealed RHS reads through
    /// (read-after-write field reads and folded scalar operands). They
    /// carry no physical field read — take time observes them against
    /// the receiver object.
    me_reads: Box<[SourceExprSiteV1]>,
}

impl ConstructionStoreV1 {
    pub(crate) const fn assignment(&self) -> &ResolvedAssignmentSourceV1 {
        &self.assignment
    }

    pub(crate) const fn field(&self) -> CanonicalFieldRefV1 {
        self.field
    }

    pub(crate) const fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    pub(crate) const fn receiver_binding(&self) -> BindingRefV1 {
        self.receiver_binding
    }

    pub(crate) const fn rhs(&self) -> &ConstructionStoreRhsV1 {
        &self.rhs
    }

    pub(crate) fn fault_discharge(&self) -> &[OwnedFieldChildV1] {
        &self.fault_discharge
    }

    pub(crate) fn me_reads(&self) -> &[SourceExprSiteV1] {
        &self.me_reads
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConstructionPlanV1 {
    // Issued once by the enclosing branded semantic batch, including NoBirth.
    object: CanonicalObjectIdV1,
    field_demands: Box<[HomeDemandV1]>,
    stores: Box<[ConstructionStoreV1]>,
    // Store sites are local to this existing constructor owner. Keep it after
    // the affine New target is consumed; Box identity alone cannot qualify them.
    constructor: Option<(crate::parser::ConstructorSourceIdV1, FunctionOwnerIdV1)>,
}

pub(crate) type ConstructionEligibilityV1 = Result<ConstructionPlanV1, ConstructionUnavailableV1>;

impl ConstructionPlanV1 {
    pub(crate) const fn object(&self) -> CanonicalObjectIdV1 {
        self.object
    }

    pub(crate) fn constructor(
        &self,
    ) -> Option<&(crate::parser::ConstructorSourceIdV1, FunctionOwnerIdV1)> {
        self.constructor.as_ref()
    }

    pub(crate) fn field_demands(&self) -> &[HomeDemandV1] {
        &self.field_demands
    }

    pub(crate) fn stores(&self) -> &[ConstructionStoreV1] {
        &self.stores
    }

    /// Independent of field demand/count. This is an obligation, not proof
    /// that a backend already implements reclamation.
    pub(crate) const fn reclaims_unpublished_outer_storage(&self) -> bool {
        true
    }
}

/// Called only within the exact parser declaration loan at semantic issuance.
///
/// `provider_static_claims` is the caller's published `birth` key paired
/// with the package's qualified static-call membership index: the sole
/// authority for admitting `Alias.m(..)` results as provider `new`
/// arguments. Rows outside a Birth carry `None` — they own no provider
/// arguments at all.
pub(super) fn issue_construction_plan(
    object_id: CanonicalObjectIdV1,
    source: &crate::parser::ParserOrdinaryBoxSourceRowV1,
    declaration: &ASTNode,
    birth: Option<(
        &crate::parser::ConstructorSourceIdV1,
        ResolvedFunctionLoweringInputV1<'_>,
    )>,
    provider_static_claims: Option<(
        &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        &super::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1,
    )>,
    // Published Birth inventory keyed by final Box ordinal: the same
    // membership `birth_for` consults, loaned by the catalog caller so
    // a provider child seals `BirthIndexed` vs `NoBirthZero` here —
    // emission never re-derives it.
    birth_rows: &BTreeMap<usize, BTreeSet<u32>>,
    objects: &[(
        crate::parser::ParserOrdinaryBoxSourceRowV1,
        CanonicalObjectIdV1,
    )],
    definitions: &[crate::mir::function::CanonicalObjectDefinitionV1],
) -> ConstructionEligibilityV1 {
    use ConstructionUnavailableV1 as U;
    // Parser normalization moves declared defaults into Birth stores. The
    // sealed source trigger retains its role at the named residence join;
    // generated stores satisfy the same arm rules as handwritten ones.
    let _ = source.has_stored_field_initializer();
    let ASTNode::BoxDeclaration {
        fields,
        field_decls,
        weak_fields,
        delegates,
        extends,
        invariants,
        transitions,
        type_parameters,
        is_sync,
        is_static,
        static_init,
        attrs,
        ..
    } = declaration
    else {
        return Err(U::SourceRelationMissing);
    };
    if fields.len() != field_decls.len()
        || fields
            .iter()
            .zip(field_decls)
            .any(|(name, field)| name != &field.name)
        || !weak_fields.is_empty()
        || !delegates.is_empty()
        || !extends.is_empty()
        || !invariants.is_empty()
        || !transitions.is_empty()
        || !type_parameters.is_empty()
        || *is_sync
        || *is_static
        || static_init.is_some()
        || !attrs.is_empty()
        || field_decls
            .iter()
            .any(|field| field.is_weak || field.default_value.is_some())
    {
        return Err(U::FieldContractUnsupported);
    }
    let names: BTreeSet<_> = fields.iter().collect();
    if names.len() != fields.len() {
        return Err(U::SourceRelationMissing);
    }
    let mut plan = ConstructionPlanV1 {
        object: object_id,
        field_demands: vec![HomeDemandV1::Trivial; fields.len()].into_boxed_slice(),
        stores: Box::new([]),
        constructor: None,
    };
    let Some((source_id, input)) = birth else {
        return if fields.is_empty() {
            Ok(plan)
        } else {
            Err(U::InitializationContractMissing)
        };
    };
    plan.constructor = Some((source_id.clone(), input.owner()));
    let function = input.function();
    let shape = input.body_shape().ok_or(U::SourceRelationMissing)?;
    if shape.owner() != input.owner() || input.forest().owners().count() != 1 {
        return Err(U::BodyCoverageUnsupported);
    }
    let ASTNode::FunctionDeclaration {
        uses,
        contracts,
        attrs,
        ..
    } = input.source().root()
    else {
        return Err(U::SourceRelationMissing);
    };
    if !uses.is_empty() || !contracts.is_empty() || !attrs.is_empty() {
        return Err(U::BodyCoverageUnsupported);
    }
    let receivers: Vec<_> = function
        .bindings()
        .filter(|(_, row)| row.kind() == BindingKindV1::Receiver)
        .map(|(binding, _)| binding)
        .collect();
    let [receiver] = receivers.as_slice() else {
        return Err(U::SourceRelationMissing);
    };
    let body = input
        .source()
        .root_body()
        .map_err(|_| U::SourceRelationMissing)?;
    let mut stores = Vec::new();
    let mut initialized = BTreeSet::new();
    let mut statements = BTreeSet::new();
    let mut expressions = BTreeSet::new();
    for index in 0..body.statements().len() {
        let statement = input
            .source()
            .body_stmt(&body, index)
            .map_err(|_| U::SourceRelationMissing)?;
        statements.insert(statement.site().clone());
        if matches!(statement.node(), ASTNode::Return { value: None, .. })
            && index + 1 == body.statements().len()
        {
            continue;
        }
        let ASTNode::Assignment { .. } = statement.node() else {
            return Err(U::BodyCoverageUnsupported);
        };
        let mut rows = shape
            .assignment_sources()
            .iter()
            .filter(|row| row.statement_site() == statement.site());
        let row = rows.next().ok_or(U::SourceRelationMissing)?;
        if rows.next().is_some() || row.form() != ResolvedAssignmentFormV1::Plain {
            return Err(U::SourceRelationMissing);
        }
        let Some(ResolvedAssignmentTargetV1::FieldWrite { receiver: object }) =
            function.assignment_target(row.target_site())
        else {
            return Err(U::BodyCoverageUnsupported);
        };
        let field = shape
            .expressions()
            .iter()
            .find_map(|expression| match expression {
                BodyExpressionShapeV1::FieldAccess {
                    site,
                    object: exact,
                    field,
                } if site == row.target_site() && exact == object => Some(field),
                _ => None,
            })
            .ok_or(U::SourceRelationMissing)?;
        if !shape.expressions().iter().any(|expression| {
            matches!(expression,
            BodyExpressionShapeV1::Me { site, receiver: BodyMeReceiverV1::Lexical(binding) }
                if site == object && binding == receiver)
        }) {
            return Err(U::BodyCoverageUnsupported);
        }
        let ordinal = fields
            .iter()
            .position(|name| name == field.as_ref())
            .ok_or(U::SourceRelationMissing)?;
        // Replacement requires old-value release semantics; not first-store
        // proof. One bounded exception: when the box declares stored-field
        // initializers, the parser prepends `me.<field> = <default>` to every
        // birth, so a handwritten scalar store to a defaulted field observes
        // a second store for the same ordinal. The overwritten value is a
        // plain scalar never observed before constructor return — release is
        // trivial, and the plan emits both stores in source order. Object
        // field re-stores still need real release semantics and stay closed.
        if !initialized.insert(ordinal)
            && !(source.has_stored_field_initializer()
                && matches!(
                    field_decls[ordinal].declared_type_name.as_deref(),
                    Some("i64") | Some("usize")
                ))
        {
            return Err(U::BodyCoverageUnsupported);
        }
        let rhs = input
            .source()
            .expr_at(&OwnedExprSiteV1::new(
                input.owner(),
                row.value_site().clone(),
            ))
            .map_err(|_| U::SourceRelationMissing)?;
        // `me` receiver sites this store's sealed RHS reads through —
        // consumed at take time against the receiver object, never as a
        // physical field read.
        let mut me_reads = Vec::new();
        let rhs = match rhs.node() {
            ASTNode::Literal {
                value: LiteralValue::Integer(value),
                ..
            } => ConstructionStoreRhsV1::LiteralI64(*value),
            ASTNode::Variable { .. } => shape
                .expressions()
                .iter()
                .find_map(|expression| match expression {
                    BodyExpressionShapeV1::Variable {
                        site,
                        resolved: ResolvedLexicalRefV1::Local(binding),
                    } if site == row.value_site()
                        && matches!(
                            function.binding(*binding).map(|record| record.kind()),
                            Some(BindingKindV1::Parameter { .. })
                        ) =>
                    {
                        Some(ConstructionStoreRhsV1::Parameter {
                            site: row.value_site().clone(),
                            binding: *binding,
                        })
                    }
                    _ => None,
                })
                .ok_or(U::BodyCoverageUnsupported)?,
            ASTNode::New { .. } => {
                let construction = function
                    .expression_source()
                    .construction(row.value_site())
                    .ok_or(U::SourceRelationMissing)?;
                if !construction.field_initializers().is_empty() {
                    return Err(U::FieldContractUnsupported);
                }
                let class = construction.class();
                let mut object = None;
                let mut arguments: Box<
                    [crate::mir::normal_callable_semantic_package::
                        OrdinaryNewTrivialArgumentV1],
                > = Box::new([]);
                let mut owned_fields = Vec::new();
                if crate::runtime::CoreBoxId::from_name(class).is_none() {
                    // A user-class provider constructs its child through the
                    // canonical Birth path: exact membership resolves the
                    // child object, and the child can never be the parent
                    // itself. The admitted child bound is one level of
                    // owned `ArrayBox` fields — the in-flight teardown then
                    // discharges each sealed residence before the child
                    // storage; deeper nesting stays unsupported.
                    let mut resolved = objects.iter().filter_map(|(own, id)| {
                        (own.name() == class).then_some((own, *id))
                    });
                    let Some((child_source, child)) = resolved.next() else {
                        return Err(U::FieldContractUnsupported);
                    };
                    let child_definition = definitions
                        .get(child.declaration_index() as usize);
                    let child_disposition =
                        child_definition.map(|definition| definition.destruction_disposition());
                    if resolved.next().is_some()
                        || child == object_id
                        || !matches!(
                            child_disposition,
                            Some(
                                crate::mir::function::ObjectDestructionDispositionV1::PlainI64NoHook
                                    | crate::mir::function::ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook
                            )
                        )
                    {
                        return Err(U::FieldContractUnsupported);
                    }
                    // Seal the child's constructor disposition once —
                    // `birth_for` parity: a Birth row at this arity makes
                    // it `BirthIndexed`; no Birth row at all + arity 0 +
                    // fieldless is `NoBirthZero` (its `construction_for`
                    // is the empty plan — a NoBirth class with fields is
                    // `InitializationContractMissing`). Any other shape
                    // (birth arity mismatch, arity>0 no-birth, non-fieldless
                    // no-birth) declines rather than emitting an
                    // uninitializable or uncallable child.
                    let arity = u32::try_from(construction.arguments().len())
                        .map_err(|_| U::SourceRelationMissing)?;
                    let child_arities =
                        birth_rows.get(&child_source.final_box_ordinal());
                    let child_ctor = if child_arities
                        .is_some_and(|arities| arities.contains(&arity))
                    {
                        ProviderConstructionChildV1::BirthIndexed(child)
                    } else if child_arities.is_none()
                        && arity == 0
                        && child_definition.is_some_and(|definition| {
                            definition.fields().is_empty()
                        })
                    {
                        ProviderConstructionChildV1::NoBirthZero(child)
                    } else {
                        return Err(U::FieldContractUnsupported);
                    };
                    if child_disposition
                        == Some(
                            crate::mir::function::ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook,
                        )
                    {
                        for (ordinal, field) in child_definition
                            .expect("child definition checked above")
                            .fields()
                            .iter()
                            .enumerate()
                        {
                            if field.declared_type_name.as_deref() == Some("ArrayBox") {
                                owned_fields.push(
                                    CanonicalFieldRefV1::from_declaration_ordinal(
                                        child, ordinal,
                                    )
                                    .ok_or(U::SourceRelationMissing)?,
                                );
                            }
                        }
                    }
                    let new_site =
                        OwnedExprSiteV1::new(input.owner(), row.value_site().clone());
                    let mut sealed = Vec::with_capacity(construction.arguments().len());
                    for (arg_ordinal, arg_site) in
                        construction.arguments().iter().enumerate()
                    {
                        let node = input
                            .source()
                            .expr_at(&OwnedExprSiteV1::new(
                                input.owner(),
                                arg_site.clone(),
                            ))
                            .map_err(|_| U::SourceRelationMissing)?;
                        let kind = match node.node() {
                            ASTNode::Literal {
                                value: LiteralValue::Integer(value),
                                ..
                            } => {
                                super::OrdinaryNewTrivialArgumentKindV1::Integer(
                                    *value,
                                )
                            }
                            ASTNode::Literal {
                                value: LiteralValue::Bool(value),
                                ..
                            } => {
                                super::OrdinaryNewTrivialArgumentKindV1::Bool(*value)
                            }
                            ASTNode::MethodCall { .. } => {
                                // A qualified `Alias.m(..)` actual joins only
                                // through the package claim index: the row
                                // corroborates the resolver's own
                                // `QualifiedUnbound` receiver shape, the
                                // claim proves the `StaticBoxMethod` target
                                // and its `ExactI64` result, and every inner
                                // actual must seal to an Integer/Bool literal
                                // with i64 evidence at the callee's
                                // required-i64 ordinals — the same discipline
                                // `issue_qualified_static_local_call` owns.
                                let Some((caller_key, claims)) = provider_static_claims else {
                                    return Err(U::FieldContractUnsupported);
                                };
                                let Some(call) = function.method_call(arg_site) else {
                                    return Err(U::FieldContractUnsupported);
                                };
                                if call.receiver()
                                    != ResolvedMethodCallReceiverSourceV1::QualifiedUnbound
                                    || call.site() != arg_site
                                {
                                    return Err(U::FieldContractUnsupported);
                                }
                                let Some((claim, target)) =
                                    claims.claim_target(caller_key, arg_site)
                                else {
                                    return Err(U::FieldContractUnsupported);
                                };
                                if call.arguments().len() != target.arity() as usize {
                                    return Err(U::FieldContractUnsupported);
                                }
                                let mut sealed_arguments =
                                    Vec::with_capacity(call.arguments().len());
                                for argument in call.arguments() {
                                    let (kind, i64_evidence) = match input
                                        .function()
                                        .expression_source()
                                        .literal(argument.site())
                                    {
                                        Some(ResolvedLiteralSourceV1::Integer(value)) => (
                                            super::QualifiedStaticCallArgumentKindV1::Integer(
                                                *value,
                                            ),
                                            true,
                                        ),
                                        Some(ResolvedLiteralSourceV1::Bool(value)) => (
                                            super::QualifiedStaticCallArgumentKindV1::Bool(*value),
                                            false,
                                        ),
                                        _ => return Err(U::FieldContractUnsupported),
                                    };
                                    if claim.required_i64_arguments().contains(&argument.ordinal())
                                        && !i64_evidence
                                    {
                                        return Err(U::FieldContractUnsupported);
                                    }
                                    sealed_arguments.push(kind);
                                    expressions.insert(argument.site().clone());
                                }
                                expressions.insert(call.receiver_site().clone());
                                super::OrdinaryNewTrivialArgumentKindV1::QualifiedStaticCall {
                                    target: target.clone(),
                                    arguments: sealed_arguments.into_boxed_slice(),
                                }
                            }
                            _ => return Err(U::FieldContractUnsupported),
                        };
                        sealed.push(super::OrdinaryNewTrivialArgumentV1::new(
                            input.owner(),
                            new_site.clone(),
                            arg_ordinal as u32,
                            arg_site.clone(),
                            kind,
                        ));
                        expressions.insert(arg_site.clone());
                    }
                    object = Some(child_ctor);
                    arguments = sealed.into_boxed_slice();
                } else if !construction.arguments().is_empty() {
                    return Err(U::FieldContractUnsupported);
                }
                ConstructionStoreRhsV1::ProviderConstruction {
                    site: row.value_site().clone(),
                    class: class.into(),
                    object,
                    arguments,
                    owned_fields: owned_fields.into_boxed_slice(),
                    caller: provider_static_claims
                        .map(|(caller, _)| caller.clone())
                        .ok_or(U::SourceRelationMissing)?,
                }
            }
            ASTNode::FieldAccess { .. } => {
                // `me.<field>` RHS read: the sealed store ledger is the
                // sole writer inside this constructor, so the read
                // resolves to the newest already-sealed store of the
                // same field (read-after-write). No physical field
                // read is emitted — birth `ObjectFieldGet` is
                // unconditionally rejected as `unowned-exact-field-read`.
                // `Parameter`/object-field stores stay unsupported: the
                // `Parameter` arm's site must be an unconsumed variable
                // site of that binding, and the read's `me` site is not.
                let (field, me_site) = me_field_read_row(
                    shape,
                    *receiver,
                    row.value_site(),
                    object_id,
                    fields,
                )?;
                expressions.insert(me_site.clone());
                me_reads.push(me_site);
                match prior_store_rhs(&stores, field) {
                    Some(ConstructionStoreRhsV1::LiteralI64(value)) => {
                        ConstructionStoreRhsV1::LiteralI64(*value)
                    }
                    _ => return Err(U::BodyCoverageUnsupported),
                }
            }
            ASTNode::BinaryOp { .. } => {
                // Scalar arithmetic on `me.<field>` reads folds to a
                // `LiteralI64` store — literal-only folding keeps the
                // birth free of physical field reads and `BinOp` MIR.
                // At least one operand must read a field: pure
                // literal-literal arithmetic stays declined, and
                // trap/overflow operators or `Parameter` operands stay
                // unsupported rather than changing runtime semantics.
                let binary = function
                    .expression_source()
                    .binary(row.value_site())
                    .ok_or(U::SourceRelationMissing)?;
                let (lhs, lhs_sites, lhs_reads) = scalar_literal_operand(
                    &input, shape, *receiver, object_id, fields, &stores, binary.lhs(),
                )?;
                let (rhs_value, rhs_sites, rhs_reads) = scalar_literal_operand(
                    &input, shape, *receiver, object_id, fields, &stores, binary.rhs(),
                )?;
                if lhs_reads.is_empty() && rhs_reads.is_empty() {
                    return Err(U::BodyCoverageUnsupported);
                }
                expressions.extend(lhs_sites.into_iter().chain(rhs_sites));
                me_reads.extend(lhs_reads.into_iter().chain(rhs_reads));
                let folded = match binary.operator() {
                    ResolvedBinaryOperatorV1::Add => lhs.checked_add(rhs_value),
                    ResolvedBinaryOperatorV1::Subtract => lhs.checked_sub(rhs_value),
                    ResolvedBinaryOperatorV1::Multiply => lhs.checked_mul(rhs_value),
                    _ => None,
                }
                .ok_or(U::BodyCoverageUnsupported)?;
                ConstructionStoreRhsV1::LiteralI64(folded)
            }
            _ => return Err(U::BodyCoverageUnsupported),
        };
        expressions.extend([
            row.target_site().clone(),
            object.clone(),
            row.value_site().clone(),
        ]);
        let field = CanonicalFieldRefV1::from_declaration_ordinal(object_id, ordinal)
            .ok_or(U::SourceRelationMissing)?;
        let fault_discharge = stores
            .iter()
            .rev()
            .filter_map(|prior: &ConstructionStoreV1| {
                let kind = match prior.rhs() {
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object: Some(child),
                        ..
                    } => OwnedFieldChildKindV1::Object(child.object()),
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object: None,
                        class,
                        ..
                    } if class.as_ref() == "ArrayBox" => OwnedFieldChildKindV1::Array,
                    ConstructionStoreRhsV1::Parameter { .. } => {
                        // The existing Provided residence is an exact birth
                        // formal store into a declared owned user-class field.
                        // The final field-contract check below corroborates
                        // unique/non-self PlainI64NoHook class authority before
                        // this constructor plan can escape the issuer.
                        let name = field_decls
                            .get(prior.field().declaration_ordinal() as usize)?
                            .declared_type_name
                            .as_deref()?;
                        if matches!(name, "i64" | "usize") {
                            return None;
                        }
                        let mut resolved = objects
                            .iter()
                            .filter_map(|(own, id)| (own.name() == name).then_some(*id));
                        let (Some(child), None) = (resolved.next(), resolved.next()) else {
                            return None;
                        };
                        OwnedFieldChildKindV1::Object(child)
                    }
                    _ => return None,
                };
                Some(OwnedFieldChildV1 {
                    field: prior.field(),
                    kind,
                })
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        stores.push(ConstructionStoreV1 {
            assignment: row.clone(),
            field,
            receiver_site: object.clone(),
            receiver_binding: *receiver,
            rhs,
            fault_discharge,
            me_reads: me_reads.into_boxed_slice(),
        });
    }
    // Reject residual syntax/child owners; SequenceItem or absent Call events
    // alone do not classify ArrayLiteral, FromCall, Lambda or nested acquisition.
    if shape.statements().len() != statements.len()
        || shape
            .statements()
            .iter()
            .any(|row| !statements.contains(row.site()))
        || shape.assignment_sources().len() != stores.len()
        || function
            .expression_sites()
            .any(|site| !expressions.contains(site))
    {
        return Err(U::BodyCoverageUnsupported);
    }
    if initialized.len() != fields.len() {
        return Err(U::InitializationContractMissing);
    }
    let mut demands = vec![HomeDemandV1::Trivial; fields.len()];
    for (ordinal, field) in field_decls.iter().enumerate() {
        let store = stores
            .iter()
            .find(|store| store.field().declaration_ordinal() as usize == ordinal)
            .ok_or(U::InitializationContractMissing)?;
        let supported = match field.declared_type_name.as_deref() {
            Some("i64") | Some("usize") => matches!(
                store.rhs(),
                ConstructionStoreRhsV1::LiteralI64(_) | ConstructionStoreRhsV1::Parameter { .. }
            ),
            Some(name) => match store.rhs() {
                ConstructionStoreRhsV1::ProviderConstruction {
                    class, object, ..
                } => {
                    class.as_ref() == name
                        && object.is_some()
                            == (crate::runtime::CoreBoxId::from_name(name).is_none())
                }
                // Caller-provided object store: birth formals are
                // unannotated, so the declared field class is the sole
                // class authority and it must resolve to a non-self
                // `PlainI64NoHook` user class — the same child bound a
                // `ProviderConstruction` carries.
                ConstructionStoreRhsV1::Parameter { .. } => {
                    if crate::runtime::CoreBoxId::from_name(name).is_some() {
                        false
                    } else {
                        let mut resolved = objects
                            .iter()
                            .filter_map(|(own, id)| (own.name() == name).then_some(*id));
                        match (resolved.next(), resolved.next()) {
                            (Some(child), None) if child != object_id => {
                                definitions
                                    .get(child.declaration_index() as usize)
                                    .map(|definition| definition.destruction_disposition())
                                    == Some(
                                        crate::mir::function::ObjectDestructionDispositionV1::PlainI64NoHook,
                                    )
                            }
                            _ => false,
                        }
                    }
                }
                _ => false,
            },
            None => matches!(
                store.rhs(),
                ConstructionStoreRhsV1::ProviderConstruction { .. }
            ),
        };
        if !supported {
            return Err(U::FieldContractUnsupported);
        }
        let object_field = !matches!(
            field.declared_type_name.as_deref(),
            Some("i64") | Some("usize")
        );
        if object_field {
            demands[ordinal] = HomeDemandV1::Handle;
        }
    }
    plan.field_demands = demands.into_boxed_slice();
    plan.stores = stores.into_boxed_slice();
    Ok(plan)
}

/// Newest already-sealed store for `field` in this constructor.
fn prior_store_rhs(
    stores: &[ConstructionStoreV1],
    field: CanonicalFieldRefV1,
) -> Option<&ConstructionStoreRhsV1> {
    stores
        .iter()
        .rev()
        .find(|store| store.field() == field)
        .map(|store| store.rhs())
}

/// Corroborate a `me.<field>` read site: the shape FieldAccess row
/// claims the site, its receiver is this constructor's `me` binding,
/// and the field name belongs to the declaring box. Returns the
/// canonical field plus the `me` receiver site for coverage.
fn me_field_read_row(
    shape: &VerifiedResolvedBodyShapeInventoryV1,
    receiver: BindingRefV1,
    access_site: &SourceExprSiteV1,
    object_id: CanonicalObjectIdV1,
    fields: &[String],
) -> Result<(CanonicalFieldRefV1, SourceExprSiteV1), ConstructionUnavailableV1> {
    use ConstructionUnavailableV1 as U;
    let (object, field) = shape
        .expressions()
        .iter()
        .find_map(|expression| match expression {
            BodyExpressionShapeV1::FieldAccess { site, object, field }
                if site == access_site =>
            {
                Some((object, field))
            }
            _ => None,
        })
        .ok_or(U::SourceRelationMissing)?;
    if !shape.expressions().iter().any(|expression| {
        matches!(expression,
        BodyExpressionShapeV1::Me { site, receiver: BodyMeReceiverV1::Lexical(binding) }
            if site == object && *binding == receiver)
    }) {
        return Err(U::BodyCoverageUnsupported);
    }
    let ordinal = fields
        .iter()
        .position(|name| name == field.as_ref())
        .ok_or(U::SourceRelationMissing)?;
    let field = CanonicalFieldRefV1::from_declaration_ordinal(object_id, ordinal)
        .ok_or(U::SourceRelationMissing)?;
    Ok((field, object.clone()))
}

/// One BinOp operand sealed to an i64 literal: a direct integer
/// literal or a `me.<field>` read resolving to a prior `LiteralI64`
/// store. `Parameter`/object-field reads stay unsupported so folding
/// never replaces runtime semantics. Returned sites are the coverage
/// rows the caller registers (operand site plus any `me` site) and the
/// `me` receiver sites the store must observe at take time.
fn scalar_literal_operand(
    input: &ResolvedFunctionLoweringInputV1<'_>,
    shape: &VerifiedResolvedBodyShapeInventoryV1,
    receiver: BindingRefV1,
    object_id: CanonicalObjectIdV1,
    fields: &[String],
    stores: &[ConstructionStoreV1],
    site: &SourceExprSiteV1,
) -> Result<(i64, Vec<SourceExprSiteV1>, Vec<SourceExprSiteV1>), ConstructionUnavailableV1>
{
    use ConstructionUnavailableV1 as U;
    let node = input
        .source()
        .expr_at(&OwnedExprSiteV1::new(input.owner(), site.clone()))
        .map_err(|_| U::SourceRelationMissing)?;
    let mut sites = vec![site.clone()];
    match node.node() {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            ..
        } => Ok((*value, sites, Vec::new())),
        ASTNode::FieldAccess { .. } => {
            let (field, me_site) =
                me_field_read_row(shape, receiver, site, object_id, fields)?;
            sites.push(me_site.clone());
            match prior_store_rhs(stores, field) {
                Some(ConstructionStoreRhsV1::LiteralI64(value)) => {
                    Ok((*value, sites, vec![me_site]))
                }
                _ => Err(U::BodyCoverageUnsupported),
            }
        }
        _ => Err(U::BodyCoverageUnsupported),
    }
}
