//! Source field-writer candidates; fixed-point closure stays with composition.
use super::*;

impl FieldCandidatesV1 {
    pub(in super::super) fn prepare(
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        claims: &OrdinaryNewFieldWriteClaimsV1,
        constructors: &VerifiedInstanceConstructorSemanticBatchV1,
        original: Result<&OriginalBorrowedFormalSourceV1, &String>,
        definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        needs: &[Result<Option<super::super::super::source::PreparedSourceCallNeedV1>, String>],
        results: &super::super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
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
        let object_sources = claims.object_sources_with_sums_v1(batch.ordinary_box_coverage(), constructors,
            |field| original.and_then(|original| original.source_field_value_sum_v1(claims, field)));
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
                        batch, selected, claims, results,
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
}
