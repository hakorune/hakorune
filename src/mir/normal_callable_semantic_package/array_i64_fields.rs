//! Whole-Box element-integer census for `me.<ArrayBox field>` receivers.
//!
//! `ArrayBox` storage physically accepts any tagged value, so the declared
//! type name alone cannot prove an i64 element contract — the field would
//! admit a foreign handle through `set`/`push` and a `get` result would not
//! be i64. This issuer seals a field only when a whole-Box census holds
//! across every selected method of the owning Box:
//!
//! - the sole `me.<field>` store is a `new ArrayBox()` inside `birth`
//!   (stored-field-initializer prologues appear as exactly such stores),
//!   with empty construction arguments;
//! - every `me.<field>` expression is that provider store, a receiver of an
//!   allowed-selector call (`get/1`, `set/2`, `push/1`, `has/1`,
//!   `length/0`), or a proven alias initializer `local a = me.<field>`;
//! - every alias binding's uses stay receivers of allowed-selector calls;
//! - every `set`/`push` value argument is integer-source: integer literal,
//!   unary minus or integer binary over integer operands, a binding whose
//!   initializer and every rebind are integer, a `me.<numeric field>` read,
//!   or a `get` on a field inside the current proven set. The proven set is
//!   the greatest fixpoint of that rule, which is sound because elements
//!   only ever enter through these writes.
//!
//! Anything else — a foreign selector, a `me.<field>` argument/return/store
//! escape, a me-alias field use, a non-integer write value, a second
//! provider, or a non-`new ArrayBox()` store — excludes the field. No
//! partial claim is issued; the receiver predicate returns `None` and the
//! consumer keeps the manifest `Dynamic` result.
use super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use super::*;
use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, BodyExpressionShapeV1, BodyMeReceiverV1,
    CallableSemanticSourceLedgerView, ResolvedAssignmentTargetV1, ResolvedBinaryOperatorV1,
    ResolvedLexicalRefV1, ResolvedLiteralSourceV1, ResolvedMethodCallReceiverSourceV1,
    ResolvedUnaryOperatorV1, VerifiedResolvedBodyShapeInventoryV1,
};
use std::collections::{BTreeMap, BTreeSet};

use super::container_field_uses::{FieldUseCensusV1 as FieldCensusV1,
    FieldUseMethodV1 as CensusMethodV1, capture_method_v1, with_method_input_v1 as with_census_input};
pub(crate) use super::container_field_uses::is_self_receiver;

#[derive(Debug, Clone)]
pub(crate) struct ArrayFieldWriteCandidateV1 {
    field: hakorune_mir_defs::CanonicalFieldRefV1,
    call: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
}
impl ArrayFieldWriteCandidateV1 {
    pub(crate) fn field(&self) -> hakorune_mir_defs::CanonicalFieldRefV1 { self.field }
    pub(crate) fn call(&self) -> &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1> { &self.call }
}

#[derive(Debug, Clone)]
pub(crate) struct ArrayFieldGetCandidateV1 {
    field: hakorune_mir_defs::CanonicalFieldRefV1,
    call: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
}
impl ArrayFieldGetCandidateV1 {
    pub(crate) fn field(&self) -> hakorune_mir_defs::CanonicalFieldRefV1 { self.field }
    pub(crate) fn call(&self) -> &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1> { &self.call }
}

#[derive(Debug, Default)]
pub(crate) struct ArrayFieldCensusV1 {
    pub(crate) integer_fields: BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>,
    pub(crate) write_fields: Box<[ArrayFieldWriteCandidateV1]>,
    pub(crate) field_reads: Box<[ArrayFieldGetCandidateV1]>,
}

const INTEGER_SOURCE_DEPTH: u32 = 8;

/// `formal.<field>` leaf authority: `field` names exactly one non-weak
/// numeric-integer declaration across the package's ordinary-box coverage.
/// An opaque parameter binding carries no class of its own — the read's
/// declared type is proven by name alone: every `block_id`-style field
/// visible to this package is `i64`. Two box sources declaring the same
/// field name — even both integer — decline, as does a weak or
/// non-integer declaration.
pub(crate) fn coverage_unique_i64_field(
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    coverage: &crate::parser::ParserOrdinaryBoxSourceCoverageV1,
    field: &str,
) -> bool {
    coverage_unique_i64_field_ref(constructors, coverage, field).is_some()
}

/// Retain the canonical declaration selected by the same complete-name census.
pub(crate) fn coverage_unique_i64_field_ref(
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    coverage: &crate::parser::ParserOrdinaryBoxSourceCoverageV1,
    field: &str,
) -> Option<hakorune_mir_defs::CanonicalFieldRefV1> {
    coverage_unique_i64_field_source(constructors, coverage, field).map(|source| source.field())
}

/// Same unique declaration census, retaining metadata before any consumer projects it.
pub(crate) fn coverage_unique_i64_field_source(
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    coverage: &crate::parser::ParserOrdinaryBoxSourceCoverageV1,
    field: &str,
) -> Option<std::rc::Rc<super::LocalFieldReadSourceV1>> {
    let mut found = None;
    for row in coverage.rows() {
        let declarations = constructors.with_source_object_definition(row, |object, definition| {
            definition.fields().iter().enumerate().filter(|(_, declaration)| declaration.name == field)
                .map(|(ordinal, declaration)| (declaration.is_weak, declaration.declared_type_name.clone(),
                    hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(object, ordinal)))
                .collect::<Vec<_>>()
        }).ok()?;
        for (weak, declared, canonical) in declarations {
            if found.is_some() || weak || !declared.as_deref().is_some_and(
                crate::mir::numeric_substrate::is_numeric_integer_type_name)
            { return None; }
            found = Some(super::LocalFieldReadSourceV1::from_declaration(
                canonical?, crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Scalar,
                Some(declared?.into_boxed_str()),
            ));
        }
    }
    found
}

/// The full `formal.<field>` index proof the issuer's field-call arm
/// consults: `binding` must be an exact `Parameter` of the owning
/// callable — a receiver, local, or alias keeps the existing leaves —
/// and `field` must satisfy `coverage_unique_i64_field`.
pub(crate) fn formal_i64_index_field(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    coverage: &crate::parser::ParserOrdinaryBoxSourceCoverageV1,
    binding: BindingRefV1,
    field: &str,
) -> bool {
    ledger
        .binding(binding)
        .is_some_and(|row| matches!(row.kind(), BindingKindV1::Parameter { .. }))
        && coverage_unique_i64_field(constructors, coverage, field)
}

/// Integer-source proof for one write value argument, mirroring the named
/// Array `integer_source` contract and adding three Box-census leaves: a
/// `me.<numeric field>` read on the same declaration, a `get` on a field
/// inside the current proven set (its elements are already i64 by the
/// fixpoint rule), and a `formal.<field>` read whose object is an exact
/// parameter binding — the caller's `formal_i64_field` predicate owns the
/// unique-declaration proof for that name.
#[allow(clippy::too_many_arguments)]
pub(crate) fn integer_source_at(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    shape: &VerifiedResolvedBodyShapeInventoryV1,
    numeric_fields: &BTreeSet<Box<str>>,
    proven: &BTreeSet<Box<str>>,
    get_calls: &BTreeMap<SourceExprSiteV1, Box<str>>,
    formal_i64_field: &dyn Fn(&str) -> bool,
    site: &SourceExprSiteV1,
    visited: &mut BTreeSet<BindingRefV1>,
    depth: u32,
) -> bool {
    if depth > INTEGER_SOURCE_DEPTH {
        return false;
    }
    if matches!(
        ledger.literal_source(site),
        Some(ResolvedLiteralSourceV1::Integer(_) | ResolvedLiteralSourceV1::TypedInteger { .. })
    ) {
        return true;
    }
    if let Some(field) = get_calls.get(site) {
        return proven.contains(field.as_ref());
    }
    if let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
        shape.expression_shape(site)
    {
        if is_self_receiver(ledger, shape, object) && numeric_fields.contains(field.as_ref()) {
            return true;
        }
        // `formal.<field>`: the object must resolve to an exact parameter
        // binding of this callable — a local alias or `me`-rooted receiver
        // keeps the existing leaves — and the caller's predicate alone
        // decides the unique-declaration proof.
        let Some(BodyExpressionShapeV1::Variable {
            resolved: ResolvedLexicalRefV1::Local(binding),
            ..
        }) = shape.expression_shape(object)
        else {
            return false;
        };
        return ledger
            .binding(*binding)
            .is_some_and(|row| matches!(row.kind(), BindingKindV1::Parameter { .. }))
            && formal_i64_field(field);
    }
    if let Some(source) = ledger.unary_source(site) {
        return source.operator() == ResolvedUnaryOperatorV1::Minus
            && integer_source_at(
                ledger,
                shape,
                numeric_fields,
                proven,
                get_calls,
                formal_i64_field,
                source.operand(),
                visited,
                depth + 1,
            );
    }
    if let Some(source) = ledger.binary_source(site) {
        if !matches!(
            source.operator(),
            ResolvedBinaryOperatorV1::Add
                | ResolvedBinaryOperatorV1::Subtract
                | ResolvedBinaryOperatorV1::Multiply
                | ResolvedBinaryOperatorV1::Divide
                | ResolvedBinaryOperatorV1::Modulo
                | ResolvedBinaryOperatorV1::BitAnd
                | ResolvedBinaryOperatorV1::BitOr
                | ResolvedBinaryOperatorV1::BitXor
                | ResolvedBinaryOperatorV1::Shl
                | ResolvedBinaryOperatorV1::Shr
        ) {
            return false;
        }
        return integer_source_at(
            ledger,
            shape,
            numeric_fields,
            proven,
            get_calls,
            formal_i64_field,
            source.lhs(),
            visited,
            depth + 1,
        ) && integer_source_at(
            ledger,
            shape,
            numeric_fields,
            proven,
            get_calls,
            formal_i64_field,
            source.rhs(),
            visited,
            depth + 1,
        );
    }
    let Some(ResolvedLexicalRefV1::Local(binding)) = ledger.variable_ref(site) else {
        return false;
    };
    if !visited.insert(binding) {
        return true;
    }
    let mut initializers = ledger
        .initializer_relations()
        .filter(|row| row.binding() == binding);
    let Some(initializer) = initializers.next() else {
        return false;
    };
    if initializers.next().is_some() {
        return false;
    }
    let Some(initializer_site) = initializer.initializer_site() else {
        return false;
    };
    if !integer_source_at(
        ledger,
        shape,
        numeric_fields,
        proven,
        get_calls,
        formal_i64_field,
        initializer_site,
        visited,
        depth + 1,
    ) {
        return false;
    }
    shape
        .assignment_sources()
        .iter()
        .filter(|source| {
            matches!(
                ledger.assignment_target(source.target_site()),
                Some(ResolvedAssignmentTargetV1::BindingRebind(actual)) if *actual == binding
            )
        })
        .all(|source| {
            integer_source_at(
                ledger,
                shape,
                numeric_fields,
                proven,
                get_calls,
                formal_i64_field,
                source.value_site(),
                visited,
                depth + 1,
            )
        })
}

/// Issue the element-integer field set for one exact Box source row.
///
/// The returned set names `me.<field>` receivers whose `get/1` result may
/// claim i64. A field outside the set keeps the manifest `Dynamic` result —
/// `set`/`push` coverage is unchanged because the manifest already admits
/// them independently of the element contract.
pub(crate) fn issue_array_i64_fields_v1(
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    box_source: &crate::parser::ParserOrdinaryBoxSourceRowV1,
) -> Result<ArrayFieldCensusV1, OrdinaryNewCoSealIssueV1> {
    let mut fields: BTreeSet<Box<str>> = BTreeSet::new();
    let mut numeric_fields: BTreeSet<Box<str>> = BTreeSet::new();
    let mut canonical: BTreeMap<Box<str>, hakorune_mir_defs::CanonicalFieldRefV1> = BTreeMap::new();
    constructors
        .with_source_object_definition(box_source, |object, definition| {
            for (ordinal, declaration) in definition.fields().iter().enumerate() {
                let Some(field) = hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(
                    object, ordinal,
                ) else {
                    continue;
                };
                match declaration.declared_type_name.as_deref() {
                    _ if definition.is_array_field(ordinal) => {
                        fields.insert(declaration.name.clone().into_boxed_str());
                        canonical.insert(declaration.name.clone().into_boxed_str(), field);
                    }
                    Some(name)
                        if crate::mir::numeric_substrate::is_numeric_integer_type_name(name) =>
                    {
                        numeric_fields.insert(declaration.name.clone().into_boxed_str());
                    }
                    _ => {}
                }
            }
        })
        .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)?;
    if fields.is_empty() {
        return Ok(ArrayFieldCensusV1::default());
    }
    // Every selected callable owned by this Box joins the census; a method
    // outside the selected set is never emitted and cannot write. The Box's
    // `birth` row joins through the constructor batch even when it is not a
    // selected key — its stored-field-initializer prologue hosts the sole
    // admitted provider store.
    let birth_row = constructors
        .birth_row_for(box_source)
        .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)?;
    let source = batch.source_ast();
    let mut census: BTreeMap<Box<str>, FieldCensusV1> = fields
        .iter()
        .map(|field| (field.clone(), FieldCensusV1::default()))
        .collect();
    let mut get_calls: BTreeMap<CensusMethodV1, BTreeMap<SourceExprSiteV1, Box<str>>> =
        BTreeMap::new();
    let mut methods: Vec<CensusMethodV1> = Vec::new();
    let mut birth_selected = false;
    for key in selected.keys() {
        let SelectedNormalCallableKeyV1::Cataloged(catalog_key) = key else {
            continue;
        };
        let is_box_method = matches!(
            catalog_key.namespace(),
            crate::mir::builder::SameModuleCallableNamespaceV1::InstanceBoxMethod
                | crate::mir::builder::SameModuleCallableNamespaceV1::BirthConstructor
        ) && catalog_key.owner() == box_source.name();
        if !is_box_method {
            continue;
        }
        let Some(slot) = selected.batch_slot(key) else {
            continue;
        };
        let is_birth = catalog_key.namespace()
            == crate::mir::builder::SameModuleCallableNamespaceV1::BirthConstructor;
        birth_selected |= is_birth;
        methods.push(CensusMethodV1::Selected { slot, is_birth });
    }
    if birth_row.is_some() && !birth_selected {
        methods.push(CensusMethodV1::BirthRow);
    }
    for method in &methods {
        with_census_input(batch, birth_row, source, *method, |input| {
            capture_method_v1(input, *method, &fields, &mut census, true)
        })??;
    }
    // Container semantics remain here, after the shared exact-use walk.
    let mut write_args: BTreeMap<Box<str>, Vec<(CensusMethodV1, SourceExprSiteV1)>> = BTreeMap::new();
    for (field, rows) in &mut census {
        for (method, call) in &rows.calls {
            match (call.selector(), call.arity()) {
                ("get", 1) => { get_calls.entry(*method).or_default().insert(call.site().clone(), field.clone()); }
                ("set", 2) => { write_args.entry(field.clone()).or_default().push((*method, call.arguments()[1].site().clone())); }
                ("push", 1) => { write_args.entry(field.clone()).or_default().push((*method, call.arguments()[0].site().clone())); }
                ("has", 1) | ("length", 0) => {}
                _ => rows.vetoed = true,
            }
        }
    }
    // Provider leg: exactly one `me.<field>` store, inside `birth`, and the
    // value is a bare `new ArrayBox()`.
    let mut proven: BTreeSet<Box<str>> = BTreeSet::new();
    for (field, rows) in &census {
        if rows.vetoed {
            continue;
        }
        let [(method, value_site)] = rows.stores.as_slice() else {
            continue;
        };
        if !method.is_birth() {
            continue;
        }
        let provider_ok = with_census_input(batch, birth_row, source, *method, |input| {
            let ledger = input.forest().callable_source_ledger(input.owner()).ok()?;
            let construction = ledger.construction_source(value_site)?;
            Some(
                construction.class() == "ArrayBox"
                    && construction.arguments().is_empty()
                    && construction.field_initializers().is_empty(),
            )
        })? == Some(true);
        if provider_ok {
            proven.insert(field.clone());
        }
    }
    let provider_fields = proven.clone();
    // Integer-write fixpoint: a field stays proven while every write value
    // is integer-source with `get` results on proven fields credited.
    let coverage = batch.ordinary_box_coverage();
    let formal_i64_field =
        |field: &str| coverage_unique_i64_field(constructors, coverage, field);
    loop {
        let mut shrink = BTreeSet::new();
        'fields: for field in proven.iter().cloned().collect::<Vec<_>>() {
            for (method, arg) in write_args.get(&field).into_iter().flatten() {
                let ok = with_census_input(batch, birth_row, source, *method, |input| {
                    let ledger = input.forest().callable_source_ledger(input.owner()).ok()?;
                    let shape = input.body_shape()?;
                    let method_gets = get_calls.get(method).cloned().unwrap_or_default();
                    let mut visited = BTreeSet::new();
                    Some(integer_source_at(
                        &ledger,
                        shape,
                        &numeric_fields,
                        &proven,
                        &method_gets,
                        &formal_i64_field,
                        arg,
                        &mut visited,
                        0,
                    ))
                })?;
                if ok != Some(true) {
                    shrink.insert(field.clone());
                    continue 'fields;
                }
            }
        }
        if shrink.is_empty() {
            break;
        }
        for field in shrink {
            proven.remove(&field);
        }
    }
    let integer_fields = proven
        .iter()
        .filter_map(|field| canonical.get(field.as_ref()).copied())
        .collect();

    // Retain all writes for the source SUM graph. The field must have one
    // exact birth provider and every use must be accounted for. Element
    // classes are determined by the graph, not by this census.
    let mut write_fields = Vec::new();
    let mut field_reads = Vec::new();
    for (field, rows) in &census {
        let has_provider = provider_fields.contains(field);
        if !has_provider || rows.vetoed || rows.foreign || rows.captured_receiver || rows.calls.is_empty() {
            continue;
        }
        let mut writes = Vec::new();
        let mut reads = Vec::new();
        let mut unsupported = false;
        for (_, call) in &rows.calls {
            match (call.selector(), call.arity()) {
                ("push", 1) | ("set", 2) => writes.push(std::sync::Arc::clone(call)),
                ("get", 1) => reads.push(std::sync::Arc::clone(call)),
                ("has", 1) | ("length", 0) => {}
                _ => unsupported = true,
            }
        }
        if unsupported || writes.is_empty() {
            continue;
        }
        let Some(canonical) = canonical.get(field.as_ref()).copied() else {
            continue;
        };
        write_fields.extend(writes.into_iter().map(|call| ArrayFieldWriteCandidateV1 {
            field: canonical,
            call,
        }));
        field_reads.extend(reads.into_iter().map(|call| ArrayFieldGetCandidateV1 {
            field: canonical,
            call,
        }));
    }
    write_fields.sort_by(|left, right| {
        left.field.cmp(&right.field).then_with(|| left.call.site().cmp(right.call.site()))
    });
    field_reads.sort_by(|left, right| {
        left.field.cmp(&right.field).then_with(|| left.call.site().cmp(right.call.site()))
    });
    Ok(ArrayFieldCensusV1 {
        integer_fields,
        write_fields: write_fields.into_boxed_slice(),
        field_reads: field_reads.into_boxed_slice(),
    })
}
