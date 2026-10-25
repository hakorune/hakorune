//! Ground call-result composition from the original borrowed source rows.
//! Temporary readiness sets schedule proof; the original result cohort owns it.
use super::*;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::BorrowedResultCohortV1;

use super::super::borrowed_formal_source::{
    OriginalBorrowedFormalSourceV1, OriginalFormalObjectBasisV1, OriginalIntegerBasisV1,
    OriginalNewI64InputsLoanV1, OriginalNewSourceLoanV1,
};
use super::super::borrowed_formal_uses::BorrowedFormalUsesDraftV1;
use super::expression::{FieldKeyV1, FieldSourcesV1, ResultExpressionSourceV1};
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::OrdinaryNewFieldWriteClaimsV1;
use crate::mir::resolved_semantics::ResolvedAssignmentFormV1;

#[derive(Debug)]
struct FieldWriterV1 {
    ordinal: usize,
    expression: Rc<ResultExpressionSourceV1>,
    birth: Option<BirthBaseV1>,
    target: Option<FormalFieldWriterTargetV1>,
}
#[derive(Debug, Clone)]
struct FormalFieldWriterTargetV1 {
    original: crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::OriginalFieldWriteSourceLoanV1,
    receiver: OriginalFormalObjectBasisV1,
}
#[derive(Debug)]
enum BirthBaseV1 {
    Literal(i64),
    Parameter(OriginalIntegerBasisV1),
    DynamicI64 {
        plan: crate::mir::normal_callable_semantic_package::instance_construction::ConstructionPlanV1,
        store_index: usize,
        store: crate::mir::normal_callable_semantic_package::instance_construction::ConstructionStoreV1,
        field: hakorune_mir_defs::CanonicalFieldRefV1,
    },
    NewParameter {
        inputs: OriginalNewI64InputsLoanV1,
        fields: Box<[(FieldKeyV1, Rc<crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::DeclaredFieldValueSourceV1>)]>,
    },
}

fn dynamic_i64_birth_base_v1(
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    source_id: Option<&crate::parser::ConstructorSourceIdV1>,
    store: &crate::mir::normal_callable_semantic_package::instance_construction::ConstructionStoreV1,
) -> Option<BirthBaseV1> {
    let source_id = source_id?;
    let mut rows = constructors
        .rows()
        .iter()
        .filter(|row| row.source_id() == source_id);
    let row = rows.next()?;
    if rows.next().is_some() || row.kind() != crate::parser::ConstructorSourceKindV1::Birth {
        return None;
    }
    let plan = row.construction().as_ref().ok()?;
    let mut stores = plan
        .stores()
        .iter()
        .enumerate()
        .filter(|(_, original)| std::ptr::eq(*original, store));
    let (store_index, original) = stores.next()?;
    if stores.next().is_some()
        || plan.store_execution_v1(store_index)?.mode()
            != crate::mir::instruction::FieldSetModeV1::DynamicI64Initialize
        || original.field() != store.field()
    {
        return None;
    }
    Some(BirthBaseV1::DynamicI64 {
        plan: plan.clone(),
        store_index,
        store: store.clone(),
        field: store.field(),
    })
}
#[derive(Debug)]
pub(super) struct FieldCandidatesV1 {
    sources: FieldSourcesV1,
    new_source: Result<OriginalNewSourceLoanV1, String>,
    object_sources: crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::ObjectFieldClassSourcesV1,
    bases: BTreeSet<FieldKeyV1>,
    writers: BTreeMap<FieldKeyV1, Box<[FieldWriterV1]>>,
}
#[derive(Debug)]
pub(super) struct ClosedFieldComponentV1 {
    original: Rc<FieldCandidatesV1>,
    fields: BTreeSet<FieldKeyV1>,
}
impl ClosedFieldComponentV1 {
    pub(super) fn belongs_to(&self, original: &Rc<FieldCandidatesV1>) -> bool {
        Rc::ptr_eq(&self.original, original)
    }
    pub(super) fn permits(&self, expression: &ResultExpressionSourceV1) -> bool {
        expression.fields_match(&self.original.sources, &self.original.object_sources)
            && expression.fields_closed(&self.fields)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SourceDependencyRoleV1 {
    ForwardReturn,
    ExpressionOperand,
    FieldWriter,
}

fn exact_dependency_v1<'a>(
    reference: &super::super::source::CallTargetReferenceV1,
    ingress: &'a PreparedBorrowedFormalIngressV1,
) -> Result<&'a LexicalInstanceCallSourceTargetV1, String> {
    use super::super::source::CallTargetReferenceV1;
    let mut rows = ingress
        .source_incoming
        .exact_rows()
        .filter(|row| row.call == reference.call_site);
    let row = rows
        .next()
        .ok_or_else(|| freeze("borrowed-result/dependency-missing"))?;
    let target = row.source.require_instance()?;
    if rows.next().is_some()
        || row.callee != reference.callee_owner
        || CallTargetReferenceV1::from_target(target) != *reference
    {
        return Err(freeze("borrowed-result/dependency-identity"));
    }
    Ok(target)
}

fn field_writers_v1<'a>(
    component: &'a ClosedFieldComponentV1,
    field: &FieldKeyV1,
    ingress: &PreparedBorrowedFormalIngressV1,
    seen: &mut BTreeSet<FieldKeyV1>,
    expressions: &mut Vec<(&'a ResultExpressionSourceV1, SourceDependencyRoleV1)>,
) -> Result<(), String> {
    if !component.fields.contains(field) {
        return Err(freeze("borrowed-result/field-component-drift"));
    }
    if !seen.insert(field.clone()) {
        return Ok(());
    }
    let writers = component
        .original
        .writers
        .get(field)
        .ok_or_else(|| freeze("borrowed-result/field-writers-missing"))?;
    for writer in writers {
        if !component
            .original
            .target_ready(field, writer, Some(ingress))
        {
            return Err(freeze("borrowed-result/field-writer-target-drift"));
        }
        match &writer.birth {
            Some(BirthBaseV1::Parameter(basis)) if !basis.ready(ingress) => {
                return Err(freeze("borrowed-result/birth-source-drift"))
            }
            Some(BirthBaseV1::NewParameter { inputs, fields }) => {
                if !inputs.ready(ingress) {
                    return Err(freeze("borrowed-result/birth-new-source-drift"));
                }
                for (key, source) in fields {
                    if component
                        .original
                        .sources
                        .get(key)
                        .is_none_or(|current| !Rc::ptr_eq(current, source))
                    {
                        return Err(freeze("borrowed-result/birth-new-field-source-drift"));
                    }
                    field_writers_v1(component, key, ingress, seen, expressions)?;
                }
            }
            Some(_) => {}
            None => expressions.push((
                writer.expression.as_ref(),
                SourceDependencyRoleV1::FieldWriter,
            )),
        }
    }
    Ok(())
}

/// One retained-expression/writer traversal for Return and field-leaf consumers.
fn expression_dependencies_v1<'a>(
    component: Option<&'a ClosedFieldComponentV1>,
    mut expressions: Vec<(&'a ResultExpressionSourceV1, SourceDependencyRoleV1)>,
    mut seen_fields: BTreeSet<FieldKeyV1>,
    ingress: &'a PreparedBorrowedFormalIngressV1,
) -> Result<
    Vec<(
        &'a LexicalInstanceCallSourceTargetV1,
        SourceDependencyRoleV1,
    )>,
    String,
> {
    let mut dependencies = Vec::new();
    while let Some((expression, role)) = expressions.pop() {
        if !expression.calls_ready(ingress) {
            return Err(freeze("borrowed-result/expression-source-drift"));
        }
        for reference in expression.calls() {
            dependencies.push((exact_dependency_v1(reference, ingress)?, role));
        }
        for field in expression.fields() {
            let component =
                component.ok_or_else(|| freeze("borrowed-result/field-component-unsealed"))?;
            if !component.permits(expression) {
                return Err(freeze("borrowed-result/field-component-drift"));
            }
            field_writers_v1(
                component,
                field,
                ingress,
                &mut seen_fields,
                &mut expressions,
            )?;
        }
    }
    Ok(dependencies)
}

/// Direct Return forwarding retains its distinct class-equality responsibility.
pub(super) fn source_dependencies_v1<'a>(
    facts: &'a BorrowedResultSourceFactsV1,
    ingress: &'a PreparedBorrowedFormalIngressV1,
) -> Result<
    Vec<(
        &'a LexicalInstanceCallSourceTargetV1,
        SourceDependencyRoleV1,
    )>,
    String,
> {
    use super::super::source::CallTargetReferenceV1;
    let mut dependencies = Vec::new();
    for target in &facts.dependencies {
        let original = exact_dependency_v1(&CallTargetReferenceV1::from_target(target), ingress)?;
        if original != target {
            return Err(freeze("borrowed-result/dependency-identity"));
        }
        dependencies.push((original, SourceDependencyRoleV1::ForwardReturn));
    }
    dependencies.extend(expression_dependencies_v1(
        facts.field_component.as_deref(),
        facts
            .expressions
            .iter()
            .map(|row| (row.as_ref(), SourceDependencyRoleV1::ExpressionOperand))
            .collect(),
        BTreeSet::new(),
        ingress,
    )?);
    Ok(dependencies)
}

/// Re-demand one source field through the SAME closure, without a getter witness.
pub(super) fn recheck_field_source_v1(
    results: &BorrowedResultCohortV1,
    ingress: &PreparedBorrowedFormalIngressV1,
    component: &Rc<ClosedFieldComponentV1>,
    source: &Rc<crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::DeclaredFieldValueSourceV1>,
    name: &str,
) -> Result<(), String> {
    results.require_field_component_v1(component)?;
    let mut matches = component
        .original
        .sources
        .iter()
        .filter(|(_, row)| Rc::ptr_eq(row, source));
    let (field, _) = matches
        .next()
        .ok_or_else(|| freeze("integer-field/source-identity/original-row-missing"))?;
    if matches.next().is_some() || field.1.as_ref() != name {
        return Err(freeze(
            "integer-field/source-identity/original-row-ambiguous-or-name",
        ));
    }
    let (_, current) = grounded_with_fields_v1(results, Some(ingress));
    if !current.is_some_and(|current| {
        current.belongs_to(&component.original) && current.fields.contains(field)
    }) {
        return Err(freeze("integer-field/source-identity/current-closure"));
    }
    let mut seen = BTreeSet::new();
    let mut expressions = Vec::new();
    field_writers_v1(component, field, ingress, &mut seen, &mut expressions)?;
    expression_dependencies_v1(Some(component), expressions, seen, ingress)?;
    Ok(())
}

pub(super) fn grounded_source_results_with_ingress_v1(
    results: &BorrowedResultCohortV1,
    ingress: &PreparedBorrowedFormalIngressV1,
) -> BTreeSet<FunctionOwnerIdV1> {
    grounded_with_fields_v1(results, Some(ingress))
        .0
        .into_keys()
        .collect()
}

impl FieldCandidatesV1 {
    pub(super) fn sources(&self) -> &FieldSourcesV1 {
        &self.sources
    }
    pub(super) fn object_sources(&self) -> &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::ObjectFieldClassSourcesV1{
        &self.object_sources
    }
    pub(super) fn prepare(
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        claims: &OrdinaryNewFieldWriteClaimsV1,
        constructors: &VerifiedInstanceConstructorSemanticBatchV1,
        original: Result<&OriginalBorrowedFormalSourceV1, &String>,
        definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        needs: &[Result<Option<super::super::source::PreparedSourceCallNeedV1>, String>],
    ) -> Rc<Self> {
        let new_source = original
            .map_err(Clone::clone)
            .and_then(|original| original.original_new_source_v1());
        let original = original.ok();
        let mut sources = expression::field_sources_v1(claims);
        let mut targets = BTreeMap::new();
        if let Some(original) = original {
            for (class, name, source) in claims.value_sources() {
                if !matches!(source, Err(crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::FieldValueSourceIssueV1::UnattributedWriter)) { continue; }
                let qualified = claims.qualify_unattributed_value_source_v1(class, name, batch.ordinary_box_coverage(), constructors,
                    |write, field| {
                        crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
                            batch, constructors, write.write().owner(), |_, input| {
                                let receiver = original.formal_object_source_at_v1(input, write.write().receiver_site(), definitions, contracts)
                                    .ok_or_else(|| freeze("field-writer/original-formal-class"))?;
                                if receiver.class() != class || receiver.object() != field.object() {
                                    return Err(freeze("field-writer/original-target-class"));
                                }
                                Ok((receiver.binding(), FormalFieldWriterTargetV1 { original: write.clone(), receiver }))
                            })?
                    });
                if let Ok((source, qualified)) = qualified {
                    let key = (Box::<str>::from(class), Box::<str>::from(name));
                    targets.insert(
                        key.clone(),
                        qualified.into_vec().into_iter().collect::<BTreeMap<_, _>>(),
                    );
                    sources.insert(key, source);
                }
            }
        }
        let object_sources = Rc::clone(claims.object_sources());
        let mut bases = BTreeSet::new();
        let mut writers = BTreeMap::new();
        for (key, source) in &sources {
            let mut base = original.is_some_and(|source| source.field_bases_available(&key.0));
            let mut birth_seen = false;
            let mut rows = Vec::new();
            for (ordinal, write) in source.writers().enumerate() {
                if write.assignment().form() != ResolvedAssignmentFormV1::Plain {
                    base = false;
                }
                let mut birth = None;
                if write.in_birth() {
                    birth_seen = true;
                    let store = source.birth_store(constructors, ordinal).ok().flatten();
                    birth = store.and_then(|store| match store.rhs() {
                        crate::mir::normal_callable_semantic_package::instance_construction::ConstructionStoreRhsV1::LiteralI64(value) => Some(BirthBaseV1::Literal(*value)),
                        crate::mir::normal_callable_semantic_package::instance_construction::ConstructionStoreRhsV1::Parameter { site, binding, provided: None } => {
                            dynamic_i64_birth_base_v1(constructors, write.birth_source(), store)
                                .or_else(|| constructors.rows().iter().find(|row| Some(row.source_id()) == write.birth_source())
                                .and_then(|row| batch.with_normal_program_source_loan(|loan| {
                                    let input = row.lowering_input(loan.program()).ok()?;
                                    let original = original?;
                                    if let Some(basis) = original.integer_origin_at_v1(input, site, constructors, definitions, contracts)
                                        .filter(|basis| basis.binding() == *binding && basis.i64_source()) {
                                        return Some(BirthBaseV1::Parameter(basis));
                                    }
                                    let inputs = original.constructor_i64_inputs_v1(input, site, *binding, row.source_id(), constructors)?;
                                    let fields: Option<Vec<_>> = inputs.fields().map(|key| sources.get(&key)
                                        .map(|source| (key, Rc::clone(source)))).collect();
                                    Some(BirthBaseV1::NewParameter { inputs, fields: fields?.into_boxed_slice() })
                                }).ok().flatten()))
                        }
                        _ => None,
                    });
                    base &= birth.is_some();
                }
                let Ok(plan) = write.source() else {
                    base = false;
                    continue;
                };
                let prepare = |input: ResolvedFunctionLoweringInputV1<'_>| {
                    Rc::new(ResultExpressionSourceV1::prepare(
                        input,
                        Rc::clone(plan),
                        &key.0,
                        write.receiver(),
                        &sources,
                        &object_sources,
                        original,
                        constructors,
                        definitions,
                        contracts,
                        needs,
                        batch, selected, claims,
                    ))
                };
                let prepared = if let Some(birth) = write.birth_source() {
                    constructors
                        .rows()
                        .iter()
                        .find(|row| row.source_id() == birth)
                        .and_then(|row| {
                            batch
                                .with_normal_program_source_loan(|loan| {
                                    row.lowering_input_for_owner(loan.program(), write.owner())
                                        .ok()
                                        .map(prepare)
                                })
                                .ok()
                                .flatten()
                        })
                } else {
                    batch
                        .declarations()
                        .find(|row| row.owner() == write.owner())
                        .and_then(|row| {
                            batch
                                .with_owner_lowering_input(row.batch_slot(), write.owner(), prepare)
                                .ok()
                        })
                };
                match prepared {
                    Some(expression) => rows.push(FieldWriterV1 {
                        ordinal,
                        expression,
                        birth,
                        target: targets
                            .get(key)
                            .and_then(|targets| targets.get(&ordinal))
                            .cloned(),
                    }),
                    None => base = false,
                }
            }
            // A missing Birth is not an implicit zero initializer. Every
            // original store must lend genuine source-ordered initialization.
            if base && birth_seen && rows.len() == source.writers().count() {
                bases.insert(key.clone());
            }
            writers.insert(key.clone(), rows.into_boxed_slice());
        }
        Rc::new(Self {
            sources,
            new_source,
            object_sources,
            bases,
            writers,
        })
    }
    fn target_ready(
        &self,
        key: &FieldKeyV1,
        writer: &FieldWriterV1,
        source: Option<&PreparedBorrowedFormalIngressV1>,
    ) -> bool {
        let Some(field) = self.sources.get(key) else {
            return false;
        };
        match &writer.target {
            None => !field.needs_formal_target(writer.ordinal),
            Some(target) => {
                field.needs_formal_target(writer.ordinal)
                    && field.corroborates_writer(writer.ordinal, &target.original)
                    && target.receiver.matches_source_receiver_v1(
                        target.original.write().owner(),
                        target.original.write().receiver_site(),
                    )
                    && target.receiver.class() == key.0.as_ref()
                    && target.receiver.object() == field.field().object()
                    && source.is_none_or(|source| target.receiver.ready(source))
            }
        }
    }
    fn birth_ready(
        &self,
        base: &BirthBaseV1,
        fields: &BTreeSet<FieldKeyV1>,
        source: Option<&PreparedBorrowedFormalIngressV1>,
    ) -> bool {
        match base {
            BirthBaseV1::Literal(_) => true,
            BirthBaseV1::Parameter(basis) => {
                basis.i64_source() && source.is_none_or(|source| basis.ready(source))
            }
            BirthBaseV1::DynamicI64 {
                plan,
                store_index,
                store,
                field,
            } => {
                plan.object() == field.object()
                    && plan
                        .stores()
                        .get(*store_index)
                        .is_some_and(|current| current == store && current.field() == *field)
                    && plan
                        .store_execution_v1(*store_index)
                        .is_some_and(|execution| {
                            execution.mode()
                                == crate::mir::instruction::FieldSetModeV1::DynamicI64Initialize
                        })
            }
            BirthBaseV1::NewParameter {
                inputs,
                fields: dependencies,
            } => {
                source.is_none_or(|source| inputs.ready(source))
                    && dependencies.iter().all(|(key, retained)| {
                        fields.contains(key)
                            && self
                                .sources
                                .get(key)
                                .is_some_and(|current| Rc::ptr_eq(current, retained))
                    })
            }
        }
    }
    /// Birth dependencies require productive roots before the preservation
    /// fixed point. A cycle of copied fields cannot manufacture its own base.
    fn grounded_bases(
        &self,
        source: Option<&PreparedBorrowedFormalIngressV1>,
    ) -> BTreeSet<FieldKeyV1> {
        let mut grounded = BTreeSet::new();
        loop {
            let next: Vec<_> = self
                .bases
                .iter()
                .filter(|key| {
                    !grounded.contains(*key)
                        && self.writers.get(*key).is_some_and(|rows| {
                            rows.iter().all(|row| {
                                row.birth
                                    .as_ref()
                                    .is_none_or(|base| self.birth_ready(base, &grounded, source))
                            })
                        })
                })
                .cloned()
                .collect();
            if next.is_empty() {
                return grounded;
            }
            grounded.extend(next);
        }
    }
    fn preserves(
        &self,
        key: &FieldKeyV1,
        fields: &BTreeSet<FieldKeyV1>,
        results: &BTreeMap<FunctionOwnerIdV1, BorrowedResultClassV1>,
        source: Option<&PreparedBorrowedFormalIngressV1>,
    ) -> bool {
        self.writers.get(key).is_some_and(|rows| {
            rows.iter().all(|row| {
                let Some(write) = self
                    .sources
                    .get(key)
                    .and_then(|source| source.writers().nth(row.ordinal))
                else {
                    return false;
                };
                if !write
                    .source()
                    .is_ok_and(|plan| Rc::ptr_eq(plan, row.expression.plan()))
                    || !self.target_ready(key, row, source)
                {
                    return false;
                }
                match &row.birth {
                    // These bases came from SAME source-ordered construction source
                    // stores. A later update to a previously copied field does not
                    // retroactively change the value copied during Birth.
                    Some(BirthBaseV1::Literal(_)) => write.in_birth(),
                    Some(base) => write.in_birth() && self.birth_ready(base, fields, source),
                    None => {
                        !write.in_birth()
                            && row
                                .expression
                                .fields_match(&self.sources, &self.object_sources)
                            && row.expression.class(fields, results)
                                == Some(BorrowedResultClassV1::I64)
                            && source.is_none_or(|source| row.expression.calls_ready(source))
                    }
                }
            })
        })
    }
}

/// The same finite dependency fold grounds Pending requirements before promotion
/// and sealed dependencies afterwards. It never erases a pre-ingress candidate.
fn grounded_with_fields_v1(
    results: &BorrowedResultCohortV1,
    source: Option<&PreparedBorrowedFormalIngressV1>,
) -> (
    BTreeMap<FunctionOwnerIdV1, BorrowedResultClassV1>,
    Option<Rc<ClosedFieldComponentV1>>,
) {
    let context = results.field_candidates.as_ref().ok().cloned();
    let mut fields = context
        .as_ref()
        .map(|context| context.grounded_bases(source))
        .unwrap_or_default();
    loop {
        let mut grounded = BTreeMap::new();
        loop {
            let next: Vec<_> = results.iter().filter_map(|(owner, proof)| {
                let proof = proof.as_ref().ok()?;
                if grounded.contains_key(owner) || proof.returns.is_empty() || !pending::pending_ready_v1(proof) || proof.verify_original_return_v1().is_err() { return None; }
                if source.is_some_and(|source| source.source_incoming.exact_rows()
                    .filter(|row| row.callee == *owner).filter_map(|row| row.source.instance())
                    .any(|target| matches!(&target.result_requirement,
                        super::super::LexicalCallSourceResultRequirementV1::ScalarReturnSource(original)
                            if !proof.matches_original_return_v1(original)))) { return None; }
                // A recursive result is productive only when an independent
                // return expression already proves its scalar class. This lets
                // `return field + child.recurse()` use the class of a separate
                // `return field` branch without allowing a pure recursive cycle
                // to manufacture its own result type.
                let recursive_seed = if proof.fixed_class != BorrowedResultClassV1::UnresolvedScalar {
                    Some(proof.fixed_class)
                } else {
                    let mut seed = None;
                    for expression in &proof.expressions {
                        if expression.calls().any(|call| call.callee_owner == *owner)
                            || source.is_some_and(|source| !expression.calls_ready(source))
                        { continue; }
                        let Some(candidate) = expression.class(&fields, &grounded) else { continue; };
                        if seed.is_some_and(|previous| previous != candidate) { return None; }
                        seed = Some(candidate);
                    }
                    seed
                };
                let mut expression_results = grounded.clone();
                if let Some(seed) = recursive_seed {
                    expression_results.insert(*owner, seed);
                }
                let mut class = (proof.fixed_class != BorrowedResultClassV1::UnresolvedScalar).then_some(proof.fixed_class);
                for expression in &proof.expressions {
                    if expression.plan().owner() != *owner
                        || !matches!(expression.plan().root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Statement {
                            role: crate::mir::resolved_value_profile::expression_root::StatementExpressionRoleV1::ReturnValue, value, ..
                        } if proof.returns.contains(&OwnedExprSiteV1::new(*owner, value.clone())))
                        || proof.field_candidates.as_ref().is_some_and(|own| context.as_ref().is_none_or(|shared| !Rc::ptr_eq(own, shared)))
                        || context.as_ref().is_some_and(|context| !expression.fields_match(&context.sources, &context.object_sources)) { return None; }
                    if source.is_some_and(|source| !expression.calls_ready(source)) { return None; }
                    let next = expression.class(&fields, &expression_results)?;
                    if class.is_some_and(|class| class != next) { return None; }
                    class = Some(next);
                }
                // Existing direct/local forward dependencies retain exact class
                // equality. Expression operands above use their original role.
                for callee in pending::dependency_owners_v1(proof) {
                    let next = *grounded.get(&callee)?;
                    if !matches!(next, BorrowedResultClassV1::I64 | BorrowedResultClassV1::Bool)
                        || class.is_some_and(|class| class != next) { return None; }
                    class = Some(next);
                }
                class.filter(|class| !matches!(proof.phase, pending::BorrowedResultSourcePhaseV1::SourceSealed) || proof.class == *class)
                    .map(|class| (*owner, class))
            }).collect();
            if next.is_empty() {
                break;
            }
            grounded.extend(next);
        }
        let removed: Vec<_> = fields
            .iter()
            .filter(|key| {
                context
                    .as_ref()
                    .is_none_or(|context| !context.preserves(key, &fields, &grounded, source))
            })
            .cloned()
            .collect();
        if removed.is_empty() {
            let component =
                context.map(|original| Rc::new(ClosedFieldComponentV1 { original, fields }));
            return (grounded, component);
        }
        for key in removed {
            fields.remove(&key);
        }
    }
}
fn grounded_source_classes_v1(
    results: &BorrowedResultCohortV1,
) -> BTreeMap<FunctionOwnerIdV1, BorrowedResultClassV1> {
    grounded_with_fields_v1(results, None).0
}

pub(super) fn resolve_pending_source_classes_v1(results: &mut BorrowedResultCohortV1) {
    for (owner, class) in grounded_source_classes_v1(results) {
        let proof = results
            .get_mut(&owner)
            .and_then(|row| row.as_mut().ok())
            .unwrap();
        if matches!(
            proof.phase,
            pending::BorrowedResultSourcePhaseV1::Pending { .. }
        ) {
            match proof.pending_source_facts_mut_v1() {
                Ok(facts) => facts.class = class,
                Err(issue) => {
                    results.insert(owner, Err(issue));
                }
            }
        }
    }
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn grounded_source_results_v1(
    results: &BorrowedResultCohortV1,
) -> BTreeSet<FunctionOwnerIdV1> {
    grounded_source_classes_v1(results).into_keys().collect()
}

pub(super) fn ground_source_results(
    results: &mut BorrowedResultCohortV1,
    source: &PreparedBorrowedFormalIngressV1,
) {
    let (grounded, component) = grounded_with_fields_v1(results, Some(source));
    results.field_component = results
        .field_candidates
        .as_ref()
        .map(|_| component.clone())
        .map_err(Clone::clone);
    for (owner, proof) in results.iter_mut() {
        if proof.as_ref().is_ok_and(|row| {
            !matches!(
                row.phase,
                pending::BorrowedResultSourcePhaseV1::IngressVerified
            )
        }) || (proof.is_ok() && !grounded.contains_key(owner))
        {
            *proof = Err(freeze("borrowed-result/source-not-i64"));
        }
        if let Ok(row) = proof {
            let field_user = row
                .expressions
                .iter()
                .any(|expression| expression.uses_fields());
            match row.pending_source_facts_mut_v1() {
                Ok(facts) => {
                    facts.class = grounded[owner];
                    if field_user {
                        facts.field_component = component.clone();
                    }
                    row.phase = pending::BorrowedResultSourcePhaseV1::SourceSealed;
                }
                Err(issue) => *proof = Err(issue),
            }
        }
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn corroborate_borrowed_result_cohort_v1(
        &mut self,
        prepared: &[Result<Option<LexicalInstanceCallSourceTargetV1>, String>],
        results: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1,
    ) {
        for source in prepared
            .iter()
            .filter_map(|row| row.as_ref().ok().and_then(Option::as_ref))
        {
            self.corroborate_borrowed_i64_result_v1(source, results);
        }
        loop {
            let ingress = self
                .borrowed_formal_source
                .as_ref()
                .and_then(|source| source.as_ref().ok());
            let grounded = ingress
                .map(|ingress| {
                    grounded_source_results_with_ingress_v1(&self.borrowed_i64_results, ingress)
                })
                .unwrap_or_default();
            let invalid: Vec<_> = self
                .borrowed_i64_results
                .iter()
                .filter_map(|(owner, proof)| {
                    let proof = proof.as_ref().ok()?;
                    (proof.require_source_sealed_v1().is_err()
                        || !grounded.contains(owner)
                        || ingress
                            .ok_or_else(|| freeze("borrowed-result/source-missing"))
                            .and_then(|ingress| source_dependencies_v1(proof, ingress))
                            .map_or(true, |dependencies| {
                                dependencies.iter().any(|(dependency, role)| {
                                    let exact = prepared
                                        .iter()
                                        .filter_map(|row| {
                                            row.as_ref().ok().and_then(Option::as_ref)
                                        })
                                        .filter(|row| row.call_site() == dependency.call_site())
                                        .collect::<Vec<_>>();
                                    exact.len() != 1
                                        || exact[0] != *dependency
                                        || self
                                            .borrowed_i64_results
                                            .get(&dependency.callee_owner())
                                            .and_then(|row| row.as_ref().ok())
                                            .is_none_or(|callee| {
                                                (*role == SourceDependencyRoleV1::ForwardReturn
                                                    && callee.class != proof.class)
                                                    || !matches!(
                                                        callee.class,
                                                        BorrowedResultClassV1::I64
                                                            | BorrowedResultClassV1::Bool
                                                    )
                                                    || callee.returns.is_empty()
                                                    || callee.require_source_sealed_v1().is_err()
                                                    || !callee.contract_corroborated
                                            })
                                })
                            }))
                    .then_some(*owner)
                })
                .collect();
            if invalid.is_empty() {
                break;
            }
            for owner in invalid {
                self.borrowed_i64_results.insert(
                    owner,
                    Err(freeze("borrowed-result/result-contract-mismatch")),
                );
            }
        }
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_result_composition_tests.rs"]
mod tests;

impl BorrowedResultCohortV1 {
    /// Source capture is retained independently of result/transport sealing.
    pub(in crate::mir::normal_callable_semantic_package) fn original_new_argument_array_read_v1(
        &self,
        new_site: &OwnedExprSiteV1,
        ordinal: u32,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
    ) -> Result<
        Option<&Rc<crate::mir::normal_callable_semantic_package::ArgumentFieldArrayReadSourceV1>>,
        String,
    > {
        self.field_candidates
            .as_ref()
            .map_err(Clone::clone)?
            .new_source
            .as_ref()
            .map_err(Clone::clone)?
            .argument_array_read_v1(new_site, ordinal, site, binding)
    }
}
