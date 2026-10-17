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
};

#[path = "instance_construction_provider.rs"]
mod provider;
#[path = "instance_construction_scalar.rs"]
mod scalar;
use scalar::{me_field_read_row, prior_store_rhs, scalar_literal_operand};

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
        /// The canonical child class this parameter feeds when the store's
        /// field declares a user object type — sealed here from the field
        /// declaration and exact membership, never re-derived downstream.
        /// `None` stays on scalar/builtin/undeclared field stores.
        provided: Option<CanonicalObjectIdV1>,
    },
    /// Source-issued provider `new` plan consumed by the construction writer.
    /// The retained Static sources keep their original issuer identity. Builtin class
    /// providers keep `object`/`arguments` empty (`None`); a user-class
    /// provider carries the canonical child identity and its sealed
    /// literal arguments for the emitted Birth call. `owned_fields`
    /// names the child's declared typed owned residences — empty for a
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
        static_sources: Box<[std::rc::Rc<super::ProviderStaticArgumentSourceV1>]>,
        owned_fields: Box<[OwnedFieldChildV1]>,
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

#[path = "instance_construction_body.rs"]
mod body;
pub(crate) use body::{ConstructionChildCallSourceLoanV1, ConstructionCompletionV1, ConstructionSourceStructureV1, ConstructionSourceStepV1};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConstructionPlanV1 {
    source: Rc<ConstructionSourceStructureV1>,
}
pub(crate) type ConstructionEligibilityV1 = Result<ConstructionPlanV1, ConstructionUnavailableV1>;
pub(crate) type ConstructionSourceEligibilityV1 = Result<Rc<ConstructionSourceStructureV1>, ConstructionUnavailableV1>;
impl ConstructionPlanV1 {
    pub(crate) fn steps(&self) -> &[ConstructionSourceStepV1] { self.source.steps() }
    pub(crate) fn object(&self) -> CanonicalObjectIdV1 { self.source.object }
    pub(crate) fn constructor(&self) -> Option<&(crate::parser::ConstructorSourceIdV1, FunctionOwnerIdV1)> {
        self.source.constructor.as_ref()
    }
    pub(crate) fn field_demands(&self) -> &[HomeDemandV1] { &self.source.field_demands }
    pub(crate) fn stores(&self) -> &[ConstructionStoreV1] { &self.source.stores }
    pub(crate) const fn reclaims_unpublished_outer_storage(&self) -> bool { true }
    pub(super) fn qualify_ordered_unit_calls_v1(
        source: &ConstructionSourceEligibilityV1,
        mut qualify: impl FnMut(&Rc<ConstructionSourceStructureV1>, &OwnedExprSiteV1) -> Result<bool, String>,
    ) -> Result<ConstructionEligibilityV1, String> {
        let source = match source { Ok(source) => source, Err(reason) => return Ok(Err(*reason)) };
        if let Err(reason) = source.require_initialized_fields() { return Ok(Err(reason)); }
        let mut saw_call = false;
        for step in source.steps() {
            match step {
                ConstructionSourceStepV1::Store(_) if saw_call => {
                    // Existing literal folds do not prove reads across opaque
                    // effects. Keep this source form unavailable until its owner does.
                    return Ok(Err(ConstructionUnavailableV1::BodyCoverageUnsupported));
                }
                ConstructionSourceStepV1::StoredChildCall(call) => {
                    saw_call = true;
                    if !qualify(source, call.site())? { return Ok(Err(ConstructionUnavailableV1::BodyCoverageUnsupported)); }
                }
                _ => {}
            }
        }
        Ok(Ok(Self { source: Rc::clone(source) }))
    }

    pub(super) fn without_child_calls(source: &ConstructionSourceEligibilityV1) -> ConstructionEligibilityV1 {
        let source = source.as_ref().map_err(|reason| *reason)?;
        source.require_initialized_fields()?;
        if source.steps.iter().any(|step| matches!(step, ConstructionSourceStepV1::StoredChildCall(_))) {
            return Err(ConstructionUnavailableV1::BodyCoverageUnsupported);
        }
        Ok(Self { source: Rc::clone(source) })
    }
}

/// Called within the exact declaration loan. Provider arguments, when present,
/// retain the published Birth key and qualified static-call source membership.
/// Source retention alone never grants a complete construction execution plan.
pub(super) fn issue_construction_source_v1(
    object_id: CanonicalObjectIdV1,
    source: &crate::parser::ParserOrdinaryBoxSourceRowV1,
    declaration: &ASTNode,
    birth: Option<(
        &crate::parser::ConstructorSourceIdV1,
        ResolvedFunctionLoweringInputV1<'_>,
    )>,
    provider_static_sources: Option<(
        &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        &[std::rc::Rc<super::instance_constructor_semantic::provider_source::ProviderStaticArgumentSourceV1>],
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
) -> ConstructionSourceEligibilityV1 {
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
    let mut plan = ConstructionSourceStructureV1 {
        object: object_id,
        completion: ConstructionCompletionV1::InitializedFields,
        field_demands: vec![HomeDemandV1::Trivial; fields.len()].into_boxed_slice(),
        stores: Box::new([]),
        constructor: None,
        steps: Vec::new().into_boxed_slice(),
    };
    let Some((source_id, input)) = birth else {
        if !fields.is_empty() {
            let definition = definitions.get(object_id.declaration_index() as usize)
                .ok_or(U::SourceRelationMissing)?;
            if definition.destruction_disposition()
                != crate::mir::function::ObjectDestructionDispositionV1::PlainI64NoHook
                || definition.fields().len() != field_decls.len()
                || definition.fields().iter().zip(field_decls).any(|(canonical, field)| {
                    canonical.name != field.name || canonical.is_weak
                        || canonical.declared_type_name.as_deref() != Some("i64")
                        || field.declared_type_name.as_deref() != Some("i64")
                })
            {
                return Err(U::InitializationContractMissing);
            }
            plan.completion = ConstructionCompletionV1::AllocationOnlyI64 {
                fields: field_decls.iter().enumerate().map(|(ordinal, declaration)| {
                    let field = CanonicalFieldRefV1::from_declaration_ordinal(object_id, ordinal)
                        .ok_or(U::SourceRelationMissing)?;
                    Ok((declaration.name.as_str().into(), field))
                }).collect::<Result<Vec<_>, U>>()?.into_boxed_slice(),
            };
        }
        return Ok(Rc::new(plan));
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
    let mut steps = Vec::new();
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
        if matches!(statement.node(), ASTNode::MethodCall { .. }) {
            let call = body::capture_child_call_source_v1(
                input, shape, *receiver, object_id, fields, field_decls,
                objects, &stores, statement.site(),
            )?;
            // Retain the complete original call subtree as a source request.
            // No expression in it is executable merely from this coverage.
            expressions.extend(function.expression_sites().filter(|site|
                site.node().segments().starts_with(statement.site().node().segments())).cloned());
            steps.push(ConstructionSourceStepV1::StoredChildCall(call));
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
            ASTNode::Variable { .. } => {
                let binding = shape
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
                            Some(*binding)
                        }
                        _ => None,
                    })
                    .ok_or(U::BodyCoverageUnsupported)?;
                // A parameter stored into a field that declares a user
                // object class is the provided-object arm: exact
                // membership resolves the declared class to its canonical
                // identity — the same lookup the provider `new` arm owns.
                // Numeric, `ArrayBox`, builtin and undeclared fields keep
                // the scalar-provided arm.
                let provided = match field_decls[ordinal].declared_type_name.as_deref() {
                    Some(declared)
                        if crate::mir::declared_type_storage
                            ::exact_numeric_storage_for_declared_type(declared)
                            .is_none()
                            && declared != "ArrayBox"
                            && !crate::box_trait::is_builtin_box(declared) =>
                    {
                        let mut resolved = objects
                            .iter()
                            .filter_map(|(own, id)| (own.name() == declared).then_some(*id));
                        let child = resolved.next().ok_or(U::SourceRelationMissing)?;
                        if resolved.next().is_some() {
                            return Err(U::SourceRelationMissing);
                        }
                        Some(child)
                    }
                    _ => None,
                };
                ConstructionStoreRhsV1::Parameter {
                    site: row.value_site().clone(),
                    binding,
                    provided,
                }
            }
            ASTNode::New { .. } => provider::issue_provider_rhs_v1(
                object_id, &input, row.value_site(), provider_static_sources,
                birth_rows, objects, definitions, &mut expressions,
            )?,
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
        let fault_discharge = body::prior_owned_fields_v1(&stores);
        steps.push(ConstructionSourceStepV1::Store(statement.site().clone()));
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
    let mut provided = BTreeSet::new();
    for store in &stores {
        if let ConstructionStoreRhsV1::Parameter { binding, provided: Some(_), .. } = store.rhs() {
            if !provided.insert(*binding) { return Err(U::FieldContractUnsupported); }
        }
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
                // class authority and it must resolve to a releasable
                // user class — self-reference included: the emitted
                // teardown call graph recurses on the instance, not the
                // type. The disposition bound matches the release op's
                // child admission exactly.
                ConstructionStoreRhsV1::Parameter { .. } => {
                    if crate::runtime::CoreBoxId::from_name(name).is_some() {
                        false
                    } else {
                        let mut resolved = objects
                            .iter()
                            .filter_map(|(own, id)| (own.name() == name).then_some(*id));
                        match (resolved.next(), resolved.next()) {
                            (Some(child), None) => definitions
                                .get(child.declaration_index() as usize)
                                .map(|definition| definition.destruction_disposition())
                                .is_some_and(|disposition| {
                                    matches!(
                                        disposition,
                                        crate::mir::function::ObjectDestructionDispositionV1::PlainI64NoHook
                                            | crate::mir::function::ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook
                                            | crate::mir::function::ObjectDestructionDispositionV1::OwnedObjectFieldsNoHook
                                    )
                                }),
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
    plan.steps = steps.into_boxed_slice();
    Ok(Rc::new(plan))
}

#[path = "instance_construction_execution.rs"]
mod execution;
pub(crate) use execution::{AllocationOnlyNewExecutionV1, NewExecutionProfileRefV1, new_execution_profile_v1};
