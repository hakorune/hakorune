use crate::mir::exact_numeric_unification::{
    unify_exact_numeric_control_merge, ExactNumericMergeSite, ExactNumericUnificationError,
};
use crate::mir::numeric_substrate::{
    exact_numeric_mir_type_from_declared_name, ExactNumericMirType, NumericTarget,
};
use crate::mir::{
    BasicBlockId, BinaryOp, MirFunction, MirInstruction, MirModule, MirType, ValueId,
};
use std::collections::BTreeMap;

mod binary_op_routes;
use binary_op_routes::{
    collect_binary_op_route_facts, collect_binary_op_route_rejections,
    try_publish_binary_op_arithmetic_fact,
};
pub use binary_op_routes::{
    ExactNumericBinaryOpRouteFact, ExactNumericBinaryOpRouteRejection,
    ExactNumericBinaryOpRouteRejectionKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactNumericValueFact {
    pub declared_type_name: String,
    pub source: ExactNumericValueFactSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExactNumericValueFactSource {
    Param {
        index: usize,
        name: String,
    },
    FieldGet {
        box_name: String,
        field: String,
    },
    BinaryOp {
        op: BinaryOp,
        lhs: ValueId,
        rhs: ValueId,
    },
    Copy {
        src: ValueId,
    },
    Phi,
    Select,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactNumericReturnFact {
    pub declared_type_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExactNumericValueFactMergeSite {
    Phi,
    Select,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExactNumericValueFactRejectionKind {
    MixedExactAndDynamic {
        exact_source_name: String,
    },
    TypeMismatch {
        left_source_name: String,
        right_source_name: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactNumericValueFactRejection {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub dst: ValueId,
    pub site: ExactNumericValueFactMergeSite,
    pub kind: ExactNumericValueFactRejectionKind,
}

#[derive(Debug, Clone)]
enum ObjectDef {
    Box(String),
    Copy(ValueId),
}

struct ControlMerge {
    block: BasicBlockId,
    instruction_index: usize,
    dst: ValueId,
    site: ExactNumericValueFactMergeSite,
    inputs: Vec<ValueId>,
}

pub(crate) fn refresh_module_exact_numeric_value_facts(module: &mut MirModule) -> usize {
    let field_decls = exact_numeric_field_decls(module);
    let mut total = 0usize;
    for function in module.functions.values_mut() {
        total += refresh_function_exact_numeric_value_facts(function, &field_decls);
    }
    total
}

pub(crate) fn refresh_function_exact_numeric_value_facts(
    function: &mut MirFunction,
    field_decls: &BTreeMap<(String, String), String>,
) -> usize {
    let object_defs = collect_object_defs(function);
    let mut facts = BTreeMap::new();

    seed_param_facts(function, &mut facts);
    seed_field_get_facts(function, field_decls, &object_defs, &mut facts);
    propagate_exact_numeric_value_facts(function, &mut facts);
    let rejections = collect_control_merge_rejections(function, &facts);
    let binary_op_route_facts = collect_binary_op_route_facts(function, &facts);
    let binary_op_route_rejections = collect_binary_op_route_rejections(function, &facts);
    let return_fact = exact_numeric_return_fact(function);

    let inserted = facts.len();
    function.metadata.exact_numeric_value_facts = facts;
    function.metadata.exact_numeric_value_fact_rejections = rejections;
    function.metadata.exact_numeric_binary_op_route_facts = binary_op_route_facts;
    function.metadata.exact_numeric_binary_op_route_rejections = binary_op_route_rejections;
    function.metadata.exact_numeric_return_fact = return_fact;
    inserted
}

pub(crate) fn exact_numeric_type_for_value_fact(
    fact: &ExactNumericValueFact,
) -> Option<ExactNumericMirType> {
    exact_numeric_mir_type_from_declared_name(
        Some(fact.declared_type_name.as_str()),
        NumericTarget::host(),
    )
}

fn exact_numeric_field_decls(module: &MirModule) -> BTreeMap<(String, String), String> {
    let mut fields = BTreeMap::new();
    let target = NumericTarget::host();

    for (box_name, decls) in &module.metadata.user_box_field_decls {
        for decl in decls {
            if exact_numeric_mir_type_from_declared_name(decl.declared_type_name.as_deref(), target)
                .is_some()
            {
                fields.insert(
                    (box_name.clone(), decl.name.clone()),
                    decl.declared_type_name.clone().unwrap_or_default(),
                );
            }
        }
    }

    fields
}

fn seed_param_facts(function: &MirFunction, facts: &mut BTreeMap<ValueId, ExactNumericValueFact>) {
    let target = NumericTarget::host();
    for (index, value) in function.params.iter().enumerate() {
        let Some(decl) = function.metadata.declared_param_decls.get(index) else {
            continue;
        };
        let Some(declared_type_name) = decl.declared_type_name.as_ref() else {
            continue;
        };
        if exact_numeric_mir_type_from_declared_name(Some(declared_type_name.as_str()), target)
            .is_none()
        {
            continue;
        }
        facts.insert(
            *value,
            ExactNumericValueFact {
                declared_type_name: declared_type_name.clone(),
                source: ExactNumericValueFactSource::Param {
                    index,
                    name: decl.name.clone(),
                },
            },
        );
    }
}

fn exact_numeric_return_fact(function: &MirFunction) -> Option<ExactNumericReturnFact> {
    let declared_type_name = function.metadata.declared_return_type_name.as_ref()?;
    exact_numeric_mir_type_from_declared_name(
        Some(declared_type_name.as_str()),
        NumericTarget::host(),
    )?;
    Some(ExactNumericReturnFact {
        declared_type_name: declared_type_name.clone(),
    })
}

fn seed_field_get_facts(
    function: &MirFunction,
    field_decls: &BTreeMap<(String, String), String>,
    object_defs: &BTreeMap<ValueId, ObjectDef>,
    facts: &mut BTreeMap<ValueId, ExactNumericValueFact>,
) {
    for block_id in function.block_ids() {
        let Some(block) = function.get_block(block_id) else {
            continue;
        };
        for spanned in block.all_spanned_instructions() {
            let MirInstruction::FieldGet {
                dst, base, field, ..
            } = spanned.inst
            else {
                continue;
            };
            let Some(box_name) = resolve_object_box(*base, object_defs) else {
                continue;
            };
            let Some(declared_type_name) = field_decls.get(&(box_name.clone(), field.clone()))
            else {
                continue;
            };
            facts.insert(
                *dst,
                ExactNumericValueFact {
                    declared_type_name: declared_type_name.clone(),
                    source: ExactNumericValueFactSource::FieldGet {
                        box_name,
                        field: field.clone(),
                    },
                },
            );
        }
    }
}

fn propagate_exact_numeric_value_facts(
    function: &MirFunction,
    facts: &mut BTreeMap<ValueId, ExactNumericValueFact>,
) {
    for _ in 0..16 {
        let mut changed = false;
        for block_id in function.block_ids() {
            let Some(block) = function.get_block(block_id) else {
                continue;
            };
            for spanned in block.all_spanned_instructions() {
                match spanned.inst {
                    MirInstruction::Copy { dst, src } => {
                        if facts.contains_key(dst) {
                            continue;
                        }
                        if let Some(fact) = facts.get(src).cloned() {
                            facts.insert(
                                *dst,
                                ExactNumericValueFact {
                                    declared_type_name: fact.declared_type_name,
                                    source: ExactNumericValueFactSource::Copy { src: *src },
                                },
                            );
                            changed = true;
                        }
                    }
                    MirInstruction::Phi { dst, inputs, .. } => {
                        changed |= try_publish_control_merge_fact(
                            facts,
                            *dst,
                            ExactNumericMergeSite::Phi,
                            inputs.iter().map(|(_, value)| *value),
                            ExactNumericValueFactSource::Phi,
                        );
                    }
                    MirInstruction::Select {
                        dst,
                        then_val,
                        else_val,
                        ..
                    } => {
                        changed |= try_publish_control_merge_fact(
                            facts,
                            *dst,
                            ExactNumericMergeSite::Select,
                            [*then_val, *else_val],
                            ExactNumericValueFactSource::Select,
                        );
                    }
                    MirInstruction::BinOp { dst, op, lhs, rhs } => {
                        changed |=
                            try_publish_binary_op_arithmetic_fact(facts, *dst, *op, *lhs, *rhs);
                    }
                    _ => {}
                }
            }
        }
        if !changed {
            break;
        }
    }
}

fn try_publish_control_merge_fact<I>(
    facts: &mut BTreeMap<ValueId, ExactNumericValueFact>,
    dst: ValueId,
    site: ExactNumericMergeSite,
    inputs: I,
    source: ExactNumericValueFactSource,
) -> bool
where
    I: IntoIterator<Item = ValueId>,
{
    if facts.contains_key(&dst) {
        return false;
    }

    let incoming: Vec<_> = inputs
        .into_iter()
        .map(|value| {
            facts
                .get(&value)
                .and_then(exact_numeric_type_for_value_fact)
        })
        .collect();
    let Ok(Some(ty)) = unify_exact_numeric_control_merge(site, &incoming) else {
        return false;
    };

    facts.insert(
        dst,
        ExactNumericValueFact {
            declared_type_name: ty.source_name,
            source,
        },
    );
    true
}

fn collect_control_merge_rejections(
    function: &MirFunction,
    facts: &BTreeMap<ValueId, ExactNumericValueFact>,
) -> Vec<ExactNumericValueFactRejection> {
    let mut rejections = Vec::new();
    for merge in collect_control_merges(function) {
        let incoming: Vec<_> = merge
            .inputs
            .iter()
            .map(|value| facts.get(value).and_then(exact_numeric_type_for_value_fact))
            .collect();
        let Some(error) =
            unify_exact_numeric_control_merge(merge.site.as_unification_site(), &incoming).err()
        else {
            continue;
        };
        rejections.push(ExactNumericValueFactRejection {
            block: merge.block,
            instruction_index: merge.instruction_index,
            dst: merge.dst,
            site: merge.site,
            kind: rejection_kind(error),
        });
    }
    rejections
}

fn collect_control_merges(function: &MirFunction) -> Vec<ControlMerge> {
    let mut merges = Vec::new();
    for block_id in function.block_ids() {
        let Some(block) = function.get_block(block_id) else {
            continue;
        };
        for (instruction_index, spanned) in block.all_spanned_instructions_enumerated() {
            match spanned.inst {
                MirInstruction::Phi { dst, inputs, .. } => {
                    merges.push(ControlMerge {
                        block: block_id,
                        instruction_index,
                        dst: *dst,
                        site: ExactNumericValueFactMergeSite::Phi,
                        inputs: inputs.iter().map(|(_, value)| *value).collect(),
                    });
                }
                MirInstruction::Select {
                    dst,
                    then_val,
                    else_val,
                    ..
                } => {
                    merges.push(ControlMerge {
                        block: block_id,
                        instruction_index,
                        dst: *dst,
                        site: ExactNumericValueFactMergeSite::Select,
                        inputs: vec![*then_val, *else_val],
                    });
                }
                _ => {}
            }
        }
    }
    merges
}

fn collect_object_defs(function: &MirFunction) -> BTreeMap<ValueId, ObjectDef> {
    let mut defs = BTreeMap::new();

    for (idx, param) in function.params.iter().enumerate() {
        if let Some(MirType::Box(box_name)) = function.signature.params.get(idx) {
            defs.insert(*param, ObjectDef::Box(box_name.clone()));
        }
    }

    for block_id in function.block_ids() {
        let Some(block) = function.get_block(block_id) else {
            continue;
        };
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

fn resolve_object_box(value: ValueId, defs: &BTreeMap<ValueId, ObjectDef>) -> Option<String> {
    let mut current = value;
    for _ in 0..16 {
        match defs.get(&current)? {
            ObjectDef::Box(box_name) => return Some(box_name.clone()),
            ObjectDef::Copy(src) => current = *src,
        }
    }
    None
}

fn rejection_kind(error: ExactNumericUnificationError) -> ExactNumericValueFactRejectionKind {
    match error {
        ExactNumericUnificationError::MixedExactAndDynamic {
            exact_source_name, ..
        } => ExactNumericValueFactRejectionKind::MixedExactAndDynamic { exact_source_name },
        ExactNumericUnificationError::TypeMismatch {
            left_source_name,
            right_source_name,
            ..
        } => ExactNumericValueFactRejectionKind::TypeMismatch {
            left_source_name,
            right_source_name,
        },
    }
}

impl ExactNumericValueFactMergeSite {
    fn as_unification_site(self) -> ExactNumericMergeSite {
        match self {
            Self::Phi => ExactNumericMergeSite::Phi,
            Self::Select => ExactNumericMergeSite::Select,
        }
    }
}

#[cfg(test)]
#[path = "exact_numeric_value_facts/tests.rs"]
mod tests;
