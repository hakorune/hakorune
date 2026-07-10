use crate::mir::function::{
    ExactNumericRuntimeCheckContract, ExactNumericRuntimeCheckContractKind,
};
use crate::mir::numeric_substrate::{
    exact_numeric_mir_type_from_declared_name,
    exact_numeric_type_requires_dynamic_integer_range_check,
    exact_numeric_value_from_dynamic_integer, ExactNumericMirType, NumericTarget,
};
use crate::mir::type_contracts::guarantee_matrix::exact_numeric_box_field_contract_is_active;
use crate::mir::type_contracts::proof::{
    TypeContractProof, TypeContractProofKind, TypeContractSiteKind,
};
use crate::mir::{
    BasicBlockId, ConstValue, MirFunction, MirInstruction, MirModule, MirType, ValueId,
};
use std::collections::{BTreeMap, HashMap};

pub(crate) const EXACT_NUMERIC_RUNTIME_CHECK_UNSUPPORTED_BACKEND_TAG: &str =
    "[freeze:contract][exact-numeric/runtime-check-unsupported-backend]";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExactNumericFieldAssignmentFinding {
    RangeViolation(ExactNumericRangeViolationSite),
    DynamicCheckRequired(ExactNumericDynamicCheckSite),
    VerifierProofMissing(ExactNumericVerifierProofMissingSite),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExactNumericRangeViolationSite {
    pub function: String,
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub box_name: String,
    pub field: String,
    pub declared_type_name: String,
    pub value: i128,
    pub min: i128,
    pub max: i128,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExactNumericDynamicCheckSite {
    pub function: String,
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub box_name: String,
    pub field: String,
    pub declared_type_name: String,
    pub value: ValueId,
    pub producer: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExactNumericVerifierProofMissingSite {
    pub proof: TypeContractProof,
    pub box_name: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExactNumericFieldContractDisposition {
    VerifierProven(TypeContractProof),
    VerifierProofRequired(ExactNumericVerifierProofMissingSite),
    RuntimeChecked(ExactNumericDynamicCheckSite),
    RuntimeCheckRequired(ExactNumericDynamicCheckSite),
    SemanticReject(ExactNumericRangeViolationSite),
}

#[derive(Debug, Clone)]
enum ObjectDef {
    Box(String),
    Copy(ValueId),
}

#[derive(Debug, Clone, Copy)]
enum IntegerDef {
    Const(i64),
    Copy(ValueId),
}

#[derive(Debug, Clone)]
enum ValueProducer {
    Param,
    ConstInteger,
    ConstNonInteger,
    Copy(ValueId),
    Dynamic(&'static str),
}

pub(crate) fn collect_exact_numeric_field_assignment_findings(
    module: &MirModule,
) -> Vec<ExactNumericFieldAssignmentFinding> {
    collect_exact_numeric_field_contract_dispositions(module)
        .into_iter()
        .filter_map(|disposition| match disposition {
            ExactNumericFieldContractDisposition::VerifierProofRequired(site) => Some(
                ExactNumericFieldAssignmentFinding::VerifierProofMissing(site),
            ),
            ExactNumericFieldContractDisposition::RuntimeCheckRequired(site) => Some(
                ExactNumericFieldAssignmentFinding::DynamicCheckRequired(site),
            ),
            ExactNumericFieldContractDisposition::SemanticReject(site) => {
                Some(ExactNumericFieldAssignmentFinding::RangeViolation(site))
            }
            ExactNumericFieldContractDisposition::VerifierProven(_)
            | ExactNumericFieldContractDisposition::RuntimeChecked(_) => None,
        })
        .collect()
}

pub(crate) fn collect_exact_numeric_field_contract_dispositions(
    module: &MirModule,
) -> Vec<ExactNumericFieldContractDisposition> {
    let fields = exact_numeric_field_decls(module);
    if fields.is_empty() {
        return Vec::new();
    }

    let mut dispositions = Vec::new();
    for function in module.functions.values() {
        collect_function_dispositions(function, &fields, &mut dispositions);
    }
    dispositions
}

pub(crate) fn refresh_module_exact_numeric_runtime_check_contracts(
    module: &mut MirModule,
) -> usize {
    for function in module.functions.values_mut() {
        function
            .metadata
            .exact_numeric_runtime_check_contracts
            .clear();
        function
            .metadata
            .exact_numeric_field_contract_proofs
            .clear();
    }

    let dispositions = collect_exact_numeric_field_contract_dispositions(module);

    let mut inserted = 0usize;
    for disposition in dispositions {
        match disposition {
            ExactNumericFieldContractDisposition::VerifierProofRequired(site) => {
                let Some(function) = module.functions.get_mut(&site.proof.function) else {
                    continue;
                };
                function
                    .metadata
                    .exact_numeric_field_contract_proofs
                    .push(site.proof);
            }
            ExactNumericFieldContractDisposition::RuntimeCheckRequired(site) => {
                let Some(function) = module.functions.get_mut(&site.function) else {
                    continue;
                };
                function
                    .metadata
                    .exact_numeric_runtime_check_contracts
                    .push(ExactNumericRuntimeCheckContract {
                        block: site.block,
                        instruction_index: site.instruction_index,
                        field: site.field,
                        value: site.value,
                        declared_type_name: site.declared_type_name,
                        kind: ExactNumericRuntimeCheckContractKind::DynamicIntegerRange,
                    });
                inserted += 1;
            }
            ExactNumericFieldContractDisposition::VerifierProven(_)
            | ExactNumericFieldContractDisposition::RuntimeChecked(_)
            | ExactNumericFieldContractDisposition::SemanticReject(_) => {}
        }
    }
    inserted
}

pub(crate) fn exact_numeric_runtime_check_contract_count(module: &MirModule) -> usize {
    module
        .functions
        .values()
        .map(|function| {
            function
                .metadata
                .exact_numeric_runtime_check_contracts
                .iter()
                .filter(|contract| {
                    contract.kind == ExactNumericRuntimeCheckContractKind::DynamicIntegerRange
                })
                .count()
        })
        .sum()
}

pub(crate) fn enforce_exact_numeric_runtime_checks_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    let contracts = exact_numeric_runtime_check_contract_count(module);
    if contracts == 0 {
        return Ok(());
    }
    if backend_supports_exact_numeric_runtime_checks(backend) {
        return Ok(());
    }

    Err(format!(
        "{} backend={} contracts={} require=exact-numeric-dynamic-integer-range-check-lowering",
        EXACT_NUMERIC_RUNTIME_CHECK_UNSUPPORTED_BACKEND_TAG, backend, contracts
    ))
}

fn backend_supports_exact_numeric_runtime_checks(backend: &str) -> bool {
    matches!(backend, "ny-llvmc-exe")
}

fn exact_numeric_field_decls(
    module: &MirModule,
) -> BTreeMap<(String, String), ExactNumericMirType> {
    let mut fields = BTreeMap::new();
    if !exact_numeric_box_field_contract_is_active() {
        return fields;
    }
    let target = NumericTarget::host();

    for (box_name, decls) in &module.metadata.user_box_field_decls {
        for decl in decls {
            if let Some(ty) = exact_numeric_mir_type_from_declared_name(
                decl.declared_type_name.as_deref(),
                target,
            ) {
                fields.insert((box_name.clone(), decl.name.clone()), ty);
            }
        }
    }

    fields
}

fn collect_function_dispositions(
    function: &MirFunction,
    fields: &BTreeMap<(String, String), ExactNumericMirType>,
    dispositions: &mut Vec<ExactNumericFieldContractDisposition>,
) {
    let object_defs = collect_object_defs(function);
    let integer_defs = collect_integer_defs(function);
    let value_producers = collect_value_producers(function);

    for (block, basic_block) in &function.blocks {
        for (instruction_index, spanned) in basic_block.all_spanned_instructions_enumerated() {
            let MirInstruction::FieldSet {
                base, field, value, ..
            } = spanned.inst
            else {
                continue;
            };

            let Some(box_name) = resolve_object_box(*base, &object_defs) else {
                continue;
            };
            let Some(ty) = fields.get(&(box_name.clone(), field.clone())) else {
                continue;
            };

            match resolve_integer_const(*value, &integer_defs) {
                Some(integer_value) => {
                    if let Err(error) = exact_numeric_value_from_dynamic_integer(integer_value, ty)
                    {
                        let range = ty.kind.value_range();
                        dispositions.push(ExactNumericFieldContractDisposition::SemanticReject(
                            ExactNumericRangeViolationSite {
                                function: function.signature.name.clone(),
                                block: *block,
                                instruction_index,
                                box_name,
                                field: field.clone(),
                                declared_type_name: ty.source_name.clone(),
                                value: i128::from(integer_value),
                                min: range.min,
                                max: range.max,
                                reason: exact_numeric_conversion_reason(error),
                            },
                        ));
                    } else {
                        let proof = exact_numeric_constant_proof(
                            function,
                            *block,
                            instruction_index,
                            field,
                            *value,
                            &ty.source_name,
                        );
                        if has_verifier_proof(function, &proof) {
                            dispositions
                                .push(ExactNumericFieldContractDisposition::VerifierProven(proof));
                        } else {
                            dispositions.push(
                                ExactNumericFieldContractDisposition::VerifierProofRequired(
                                    ExactNumericVerifierProofMissingSite {
                                        proof,
                                        box_name,
                                        reason: "verifier-proof-missing".to_string(),
                                    },
                                ),
                            );
                        }
                    }
                }
                None => {
                    let site = ExactNumericDynamicCheckSite {
                        function: function.signature.name.clone(),
                        block: *block,
                        instruction_index,
                        box_name,
                        field: field.clone(),
                        declared_type_name: ty.source_name.clone(),
                        value: *value,
                        producer: resolve_value_producer_label(*value, &value_producers),
                        reason: if exact_numeric_type_requires_dynamic_integer_range_check(ty) {
                            "dynamic-type-and-range-check-required".to_string()
                        } else {
                            "dynamic-type-check-required".to_string()
                        },
                    };
                    if has_runtime_check_contract(
                        function,
                        *block,
                        instruction_index,
                        field,
                        *value,
                        &ty.source_name,
                    ) {
                        dispositions
                            .push(ExactNumericFieldContractDisposition::RuntimeChecked(site));
                    } else {
                        dispositions.push(
                            ExactNumericFieldContractDisposition::RuntimeCheckRequired(site),
                        );
                    }
                }
            }
        }
    }
}

fn exact_numeric_constant_proof(
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    field: &str,
    value: ValueId,
    expected_type: &str,
) -> TypeContractProof {
    TypeContractProof {
        site_kind: TypeContractSiteKind::BoxFieldWrite,
        function: function.signature.name.clone(),
        block,
        instruction_index,
        field: field.to_string(),
        value,
        expected_type: expected_type.to_string(),
        proof_kind: TypeContractProofKind::ExactNumericConstantInRange,
    }
}

fn has_verifier_proof(function: &MirFunction, expected: &TypeContractProof) -> bool {
    function
        .metadata
        .exact_numeric_field_contract_proofs
        .iter()
        .any(|proof| {
            proof.proof_kind == expected.proof_kind
                && proof.matches_box_field_site(
                    &expected.function,
                    expected.block,
                    expected.instruction_index,
                    &expected.field,
                    expected.value,
                    &expected.expected_type,
                )
        })
}

fn has_runtime_check_contract(
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    field: &str,
    value: ValueId,
    declared_type_name: &str,
) -> bool {
    function
        .metadata
        .exact_numeric_runtime_check_contracts
        .iter()
        .any(|contract| {
            contract.kind == ExactNumericRuntimeCheckContractKind::DynamicIntegerRange
                && contract.block == block
                && contract.instruction_index == instruction_index
                && contract.field == field
                && contract.value == value
                && contract.declared_type_name == declared_type_name
        })
}

fn collect_object_defs(function: &MirFunction) -> HashMap<ValueId, ObjectDef> {
    let mut defs = HashMap::new();

    for (idx, param) in function.params.iter().enumerate() {
        if let Some(MirType::Box(box_name)) = function.signature.params.get(idx) {
            defs.insert(*param, ObjectDef::Box(box_name.clone()));
        }
    }

    for block in function.blocks.values() {
        for spanned in block.all_spanned_instructions() {
            match spanned.inst {
                MirInstruction::NewBox { dst, box_type, .. } => {
                    defs.insert(*dst, ObjectDef::Box(box_type.clone()));
                }
                MirInstruction::Copy { dst, src } => {
                    defs.insert(*dst, ObjectDef::Copy(*src));
                }
                _ => {}
            }
        }
    }

    defs
}

fn collect_integer_defs(function: &MirFunction) -> HashMap<ValueId, IntegerDef> {
    let mut defs = HashMap::new();

    for block in function.blocks.values() {
        for spanned in block.all_spanned_instructions() {
            match spanned.inst {
                MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(value),
                } => {
                    defs.insert(*dst, IntegerDef::Const(*value));
                }
                MirInstruction::Copy { dst, src } => {
                    defs.insert(*dst, IntegerDef::Copy(*src));
                }
                _ => {}
            }
        }
    }

    defs
}

fn collect_value_producers(function: &MirFunction) -> HashMap<ValueId, ValueProducer> {
    let mut producers = HashMap::new();

    for param in &function.params {
        producers.insert(*param, ValueProducer::Param);
    }

    for block in function.blocks.values() {
        for spanned in block.all_spanned_instructions() {
            match spanned.inst {
                MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(_),
                } => {
                    producers.insert(*dst, ValueProducer::ConstInteger);
                }
                MirInstruction::Const { dst, .. } => {
                    producers.insert(*dst, ValueProducer::ConstNonInteger);
                }
                MirInstruction::Copy { dst, src } => {
                    producers.insert(*dst, ValueProducer::Copy(*src));
                }
                MirInstruction::BinOp { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("binop"));
                }
                MirInstruction::UnaryOp { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("unaryop"));
                }
                MirInstruction::Compare { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("compare"));
                }
                MirInstruction::Load { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("load"));
                }
                MirInstruction::StaticDataLoad { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("static_data_load"));
                }
                MirInstruction::FieldGet { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("field_get"));
                }
                MirInstruction::VariantMake { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("variant_make"));
                }
                MirInstruction::VariantTag { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("variant_tag"));
                }
                MirInstruction::VariantProject { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("variant_project"));
                }
                MirInstruction::Call { dst: Some(dst), .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("call"));
                }
                MirInstruction::NewClosure { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("new_closure"));
                }
                MirInstruction::Phi { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("phi"));
                }
                MirInstruction::NewBox { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("new_box"));
                }
                MirInstruction::TypeOp { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("typeop"));
                }
                MirInstruction::Catch {
                    exception_value, ..
                } => {
                    producers.insert(*exception_value, ValueProducer::Dynamic("catch"));
                }
                MirInstruction::RefNew { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("ref_new"));
                }
                MirInstruction::WeakRef { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("weakref"));
                }
                MirInstruction::FutureNew { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("future_new"));
                }
                MirInstruction::Await { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("await"));
                }
                MirInstruction::Select { dst, .. } => {
                    producers.insert(*dst, ValueProducer::Dynamic("select"));
                }
                _ => {}
            }
        }
    }

    producers
}

fn resolve_object_box(value: ValueId, defs: &HashMap<ValueId, ObjectDef>) -> Option<String> {
    let mut current = value;
    for _ in 0..16 {
        match defs.get(&current)? {
            ObjectDef::Box(box_name) => return Some(box_name.clone()),
            ObjectDef::Copy(src) => current = *src,
        }
    }
    None
}

fn resolve_integer_const(value: ValueId, defs: &HashMap<ValueId, IntegerDef>) -> Option<i64> {
    let mut current = value;
    for _ in 0..16 {
        match defs.get(&current)? {
            IntegerDef::Const(value) => return Some(*value),
            IntegerDef::Copy(src) => current = *src,
        }
    }
    None
}

fn resolve_value_producer_label(
    value: ValueId,
    producers: &HashMap<ValueId, ValueProducer>,
) -> String {
    let mut current = value;
    for _ in 0..16 {
        match producers.get(&current) {
            Some(ValueProducer::Param) => return "param".to_string(),
            Some(ValueProducer::ConstInteger) => return "const_integer_unresolved".to_string(),
            Some(ValueProducer::ConstNonInteger) => return "const_non_integer".to_string(),
            Some(ValueProducer::Dynamic(label)) => return (*label).to_string(),
            Some(ValueProducer::Copy(src)) => current = *src,
            None => return "unknown".to_string(),
        }
    }
    "copy_chain_too_deep".to_string()
}

fn exact_numeric_conversion_reason(
    error: crate::mir::numeric_substrate::ExactNumericConversionError,
) -> String {
    match error {
        crate::mir::numeric_substrate::ExactNumericConversionError::NegativeToUnsigned {
            ..
        } => "negative-to-unsigned".to_string(),
        crate::mir::numeric_substrate::ExactNumericConversionError::OutOfRange { .. } => {
            "out-of-range".to_string()
        }
    }
}

#[cfg(test)]
mod tests;
