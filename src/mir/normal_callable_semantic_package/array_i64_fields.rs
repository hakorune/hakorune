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

/// Per-field census accumulated across the owning Box's selected methods.
#[derive(Default)]
struct FieldCensusV1 {
    /// `me.<field>` store `(method, value_site)` rows.
    stores: Vec<(CensusMethodV1, SourceExprSiteV1)>,
    /// `set`/`push` value-argument sites as `(method, site)`.
    write_args: Vec<(CensusMethodV1, SourceExprSiteV1)>,
    /// A foreign occurrence excluded the field.
    vetoed: bool,
}

/// One method under the census: a selected batch slot, or the owning
/// Box's `birth` row reached through the constructor batch — `birth`
/// carries the stored-field-initializer prologue that hosts the sole
/// admitted provider store, and it is not always a selected key.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum CensusMethodV1 {
    Selected { slot: u32, is_birth: bool },
    BirthRow,
}

impl CensusMethodV1 {
    const fn is_birth(&self) -> bool {
        match self {
            Self::Selected { is_birth, .. } => *is_birth,
            Self::BirthRow => true,
        }
    }
}

/// `true` when `object` resolves to this frame's own `me` receiver binding.
pub(crate) fn is_self_receiver(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    shape: &VerifiedResolvedBodyShapeInventoryV1,
    object: &SourceExprSiteV1,
) -> bool {
    matches!(
        shape.expression_shape(object),
        Some(BodyExpressionShapeV1::Me {
            receiver: BodyMeReceiverV1::Lexical(me),
            ..
        }) if ledger
            .binding(*me)
            .is_some_and(|record| record.kind() == BindingKindV1::Receiver)
    )
}

/// Resolves the tracked-field receiver for one call site: either
/// `me.<field>` directly, or a local alias whose sole initializer is
/// `me.<field>`.
fn field_receiver(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    shape: &VerifiedResolvedBodyShapeInventoryV1,
    aliases: &BTreeMap<BindingRefV1, Box<str>>,
    fields: &BTreeSet<Box<str>>,
    call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
) -> Option<Box<str>> {
    if let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
        shape.expression_shape(call.receiver_site())
    {
        if is_self_receiver(ledger, shape, object) && fields.contains(field.as_ref()) {
            return Some(field.clone());
        }
        return None;
    }
    let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) =
        call.receiver()
    else {
        return None;
    };
    aliases.get(&binding).cloned()
}

/// `me.<field>` aliases for one method: bindings whose single initializer
/// is a `me.<field>` read on a tracked field, plus the bindings initialized
/// from bare `me` (any field use on those vetoes the touched field).
fn method_aliases(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    shape: &VerifiedResolvedBodyShapeInventoryV1,
    fields: &BTreeSet<Box<str>>,
    census: &mut BTreeMap<Box<str>, FieldCensusV1>,
) -> (
    BTreeMap<BindingRefV1, Box<str>>,
    BTreeSet<SourceExprSiteV1>,
    BTreeSet<BindingRefV1>,
) {
    let mut aliases: BTreeMap<BindingRefV1, Box<str>> = BTreeMap::new();
    let mut alias_sites: BTreeSet<SourceExprSiteV1> = BTreeSet::new();
    let mut duplicate: BTreeSet<BindingRefV1> = BTreeSet::new();
    let mut me_aliases: BTreeSet<BindingRefV1> = BTreeSet::new();
    for relation in ledger.initializer_relations() {
        let Some(site) = relation.initializer_site() else {
            continue;
        };
        match shape.expression_shape(site) {
            Some(BodyExpressionShapeV1::FieldAccess { object, field, .. })
                if is_self_receiver(ledger, shape, object) && fields.contains(field.as_ref()) =>
            {
                if aliases.insert(relation.binding(), field.clone()).is_some() {
                    duplicate.insert(relation.binding());
                }
                alias_sites.insert(site.clone());
            }
            Some(BodyExpressionShapeV1::Me { .. }) => {
                me_aliases.insert(relation.binding());
            }
            _ => {}
        }
    }
    for binding in duplicate {
        if let Some(field) = aliases.remove(&binding) {
            census.entry(field).or_default().vetoed = true;
        }
    }
    for source in shape.assignment_sources() {
        let Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) =
            ledger.assignment_target(source.target_site())
        else {
            continue;
        };
        if let Some(field) = aliases.remove(binding) {
            census.entry(field).or_default().vetoed = true;
        }
        if me_aliases.contains(binding) {
            for field in fields {
                census.entry(field.clone()).or_default().vetoed = true;
            }
        }
    }
    (aliases, alias_sites, me_aliases)
}

/// Census one selected method of the Box.
fn census_method(
    input: ResolvedFunctionLoweringInputV1<'_>,
    method: CensusMethodV1,
    fields: &BTreeSet<Box<str>>,
    census: &mut BTreeMap<Box<str>, FieldCensusV1>,
    get_calls: &mut BTreeMap<CensusMethodV1, BTreeMap<SourceExprSiteV1, Box<str>>>,
) -> Result<(), OrdinaryNewCoSealIssueV1> {
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)?;
    let Some(shape) = input.body_shape() else {
        return Err(OrdinaryNewCoSealIssueV1::BatchLoan);
    };
    let (aliases, alias_sites, me_aliases) = method_aliases(&ledger, shape, fields, census);
    let mut store_sites: BTreeSet<SourceExprSiteV1> = BTreeSet::new();
    let mut receiver_sites: BTreeSet<SourceExprSiteV1> = BTreeSet::new();
    for source in shape.assignment_sources() {
        let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
            shape.expression_shape(source.target_site())
        else {
            continue;
        };
        if is_self_receiver(&ledger, shape, object) && fields.contains(field.as_ref()) {
            store_sites.insert(source.target_site().clone());
            census
                .entry(field.clone())
                .or_default()
                .stores
                .push((method, source.value_site().clone()));
        }
    }
    for (_, call) in ledger.method_calls() {
        let Some(field) = field_receiver(&ledger, shape, &aliases, fields, call) else {
            continue;
        };
        receiver_sites.insert(call.receiver_site().clone());
        match (call.selector(), call.arity()) {
            ("get", 1) => {
                get_calls
                    .entry(method)
                    .or_default()
                    .insert(call.site().clone(), field);
            }
            ("set", 2) => {
                census
                    .entry(field)
                    .or_default()
                    .write_args
                    .push((method, call.arguments()[1].site().clone()));
            }
            ("push", 1) => {
                census
                    .entry(field)
                    .or_default()
                    .write_args
                    .push((method, call.arguments()[0].site().clone()));
            }
            // Non-writing receivers keep the element contract intact.
            ("has", 1) | ("length", 0) => {}
            _ => {
                census.entry(field).or_default().vetoed = true;
            }
        }
    }
    // Every `me.<field>` occurrence must be accounted for; every alias use
    // must be an allowed receiver; every `me`-alias field use vetoes.
    for expression in shape.expressions() {
        match expression {
            BodyExpressionShapeV1::FieldAccess {
                site, object, field,
            } => {
                if is_self_receiver(&ledger, shape, object)
                    && fields.contains(field.as_ref())
                    && !store_sites.contains(site)
                    && !receiver_sites.contains(site)
                    && !alias_sites.contains(site)
                {
                    census.entry(field.clone()).or_default().vetoed = true;
                }
                if let Some(BodyExpressionShapeV1::Variable { resolved, .. }) =
                    shape.expression_shape(object)
                {
                    if let ResolvedLexicalRefV1::Local(binding) = resolved {
                        if me_aliases.contains(binding) && fields.contains(field.as_ref()) {
                            census.entry(field.clone()).or_default().vetoed = true;
                        }
                    }
                }
            }
            BodyExpressionShapeV1::Variable { site, resolved } => {
                let ResolvedLexicalRefV1::Local(binding) = resolved else {
                    continue;
                };
                if let Some(field) = aliases.get(binding) {
                    if !receiver_sites.contains(site) {
                        census.entry(field.clone()).or_default().vetoed = true;
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
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
    let mut found = false;
    for row in coverage.rows() {
        let declared = constructors
            .with_source_object_definition(row, |_, definition| {
                let mut hit = None;
                for declaration in definition.fields() {
                    if declaration.name == field && !declaration.is_weak {
                        if hit.is_some() {
                            return Some(None);
                        }
                        hit = Some(declaration.declared_type_name.clone());
                    }
                }
                Some(hit.flatten())
            })
            .ok()
            .flatten()
            .flatten();
        if !declared.is_some_and(|name| {
            crate::mir::numeric_substrate::is_numeric_integer_type_name(&name)
        }) {
            continue;
        }
        if found {
            return false;
        }
        found = true;
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
) -> Result<BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>, OrdinaryNewCoSealIssueV1> {
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
                    Some("ArrayBox") if !declaration.is_weak => {
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
        return Ok(BTreeSet::new());
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
            census_method(input, *method, &fields, &mut census, &mut get_calls)
        })??;
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
    if proven.is_empty() {
        return Ok(BTreeSet::new());
    }
    // Integer-write fixpoint: a field stays proven while every write value
    // is integer-source with `get` results on proven fields credited.
    let coverage = batch.ordinary_box_coverage();
    let formal_i64_field =
        |field: &str| coverage_unique_i64_field(constructors, coverage, field);
    loop {
        let mut shrink = BTreeSet::new();
        'fields: for field in proven.iter().cloned().collect::<Vec<_>>() {
            for (method, arg) in &census[&field].write_args {
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
    Ok(proven
        .iter()
        .filter_map(|field| canonical.get(field.as_ref()).copied())
        .collect())
}

/// Borrow the lowering input for one census method: the batch loan for a
/// selected slot, or the constructor row's own sealed input for `birth`.
fn with_census_input<R>(
    batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    birth: Option<&super::VerifiedInstanceConstructorSemanticRowV1>,
    source: &crate::ast::ASTNode,
    method: CensusMethodV1,
    f: impl FnOnce(ResolvedFunctionLoweringInputV1<'_>) -> R,
) -> Result<R, OrdinaryNewCoSealIssueV1> {
    match method {
        CensusMethodV1::Selected { slot, .. } => batch
            .with_lowering_input(slot, f)
            .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan),
        CensusMethodV1::BirthRow => {
            let row = birth.ok_or(OrdinaryNewCoSealIssueV1::BatchLoan)?;
            let input = row
                .lowering_input(source)
                .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)?;
            Ok(f(input))
        }
    }
}
