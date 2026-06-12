use std::collections::{BTreeMap, BTreeSet};

use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, MirModule, MirType, ValueId};

use crate::mir::Callee;
use crate::mir::ConstValue;

use super::state::{BoxOriginInference, FieldBoxOriginMap, ParamBoxOriginMap};
use super::type_facts::{
    box_name_from_mir_type, box_origin_from_mir_type, is_null_or_void_value,
    method_receiver_box_from_param, method_receiver_box_from_param_index, value_box_origin_for,
    value_type_for,
};

type BoxOriginMemo = BTreeMap<(String, ValueId), Option<String>>;

pub(super) fn box_name_for_value(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<String> {
    let origin = resolve_value_origin(function, def_map, value);
    value_type_for(function, value)
        .or_else(|| value_type_for(function, origin))
        .and_then(box_name_from_mir_type)
        .map(str::to_string)
        .or_else(|| box_name_from_origin_instruction(function, def_map, origin))
        .or_else(|| method_receiver_box_from_param(function, origin))
}

fn box_name_from_origin_instruction(
    function: &MirFunction,
    def_map: &ValueDefMap,
    origin: ValueId,
) -> Option<String> {
    let (block_id, instruction_index) = def_map.get(&origin).copied()?;
    let block = function.blocks.get(&block_id)?;
    match block.instructions.get(instruction_index)? {
        MirInstruction::NewBox { box_type, .. } => Some(box_type.clone()),
        MirInstruction::Phi { type_hint, .. } => type_hint
            .as_ref()
            .and_then(box_name_from_mir_type)
            .map(str::to_string),
        _ => None,
    }
}

pub(super) fn same_module_method_target(
    module: &MirModule,
    function: &MirFunction,
    def_map: &ValueDefMap,
    box_name: &str,
    method: &str,
    receiver: Option<ValueId>,
    arity: usize,
    field_box_origins: &FieldBoxOriginMap,
    param_box_origins: &ParamBoxOriginMap,
) -> Option<(String, String)> {
    if let Some(receiver) = receiver {
        if let Some(receiver_box) = box_origin_for_value(
            module,
            function,
            def_map,
            receiver,
            field_box_origins,
            param_box_origins,
        ) {
            let symbol = format!("{receiver_box}.{method}/{arity}");
            if module.functions.contains_key(&symbol) {
                return Some((receiver_box, symbol));
            }
        }
    }

    let symbol = format!("{box_name}.{method}/{arity}");
    module
        .functions
        .contains_key(&symbol)
        .then(|| (box_name.to_string(), symbol))
}

pub(super) fn box_origin_for_value(
    module: &MirModule,
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    field_box_origins: &FieldBoxOriginMap,
    param_box_origins: &ParamBoxOriginMap,
) -> Option<String> {
    let mut visiting_functions = BTreeSet::new();
    let mut visiting_values = BTreeSet::new();
    let mut memo = BTreeMap::new();
    box_origin_for_value_inner(
        module,
        function,
        def_map,
        value,
        field_box_origins,
        param_box_origins,
        &mut visiting_functions,
        &mut visiting_values,
        &mut memo,
    )
}

fn box_origin_for_value_inner(
    module: &MirModule,
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    field_box_origins: &FieldBoxOriginMap,
    param_box_origins: &ParamBoxOriginMap,
    visiting_functions: &mut BTreeSet<String>,
    visiting_values: &mut BTreeSet<(String, ValueId)>,
    memo: &mut BoxOriginMemo,
) -> Option<String> {
    let origin = resolve_value_origin(function, def_map, value);
    let value_key = (function.signature.name.clone(), origin);
    if let Some(cached) = memo.get(&value_key) {
        return cached.clone();
    }
    if !visiting_values.insert(value_key.clone()) {
        return None;
    }

    let result = box_origin_for_value_type_facts(function, value, origin)
        .or_else(|| {
            let (block_id, instruction_index) = def_map.get(&origin).copied()?;
            let block = function.blocks.get(&block_id)?;
            match block.instructions.get(instruction_index)? {
                MirInstruction::Const {
                    value: ConstValue::String(_),
                    ..
                } => Some("StringBox".to_string()),
                MirInstruction::NewBox { box_type, .. } => Some(box_type.clone()),
                MirInstruction::Phi {
                    inputs, type_hint, ..
                } => box_origin_for_phi_type_facts(function, origin, type_hint.as_ref()).or_else(
                    || {
                        box_origin_for_phi_inputs(
                            module,
                            function,
                            def_map,
                            inputs,
                            field_box_origins,
                            param_box_origins,
                            visiting_functions,
                            visiting_values,
                            memo,
                        )
                    },
                ),
                MirInstruction::FieldGet { base, field, .. } => {
                    let base_box = box_name_for_value(function, def_map, *base).or_else(|| {
                        box_origin_for_value_inner(
                            module,
                            function,
                            def_map,
                            *base,
                            field_box_origins,
                            param_box_origins,
                            visiting_functions,
                            visiting_values,
                            memo,
                        )
                    })?;
                    match field_box_origins.get(&(base_box, field.clone())) {
                        Some(BoxOriginInference::Known(box_name)) => Some(box_name.clone()),
                        Some(BoxOriginInference::Conflict) | None => None,
                    }
                }
                MirInstruction::Call { callee, args, .. } => box_origin_for_call_return(
                    module,
                    function,
                    def_map,
                    callee.as_ref()?,
                    args.len(),
                    field_box_origins,
                    param_box_origins,
                    visiting_functions,
                    visiting_values,
                    memo,
                ),
                _ => None,
            }
        })
        .or_else(|| box_origin_from_param(function, origin, param_box_origins));

    visiting_values.remove(&value_key);
    memo.insert(value_key, result.clone());
    result
}

fn box_origin_for_phi_type_facts(
    function: &MirFunction,
    origin: ValueId,
    type_hint: Option<&MirType>,
) -> Option<String> {
    type_hint
        .and_then(box_origin_from_mir_type)
        .or_else(|| box_origin_for_value_type_facts(function, origin, origin))
}

fn box_origin_for_value_type_facts(
    function: &MirFunction,
    value: ValueId,
    origin: ValueId,
) -> Option<String> {
    value_box_origin_for(function, value).or_else(|| value_box_origin_for(function, origin))
}

fn box_origin_for_phi_inputs(
    module: &MirModule,
    function: &MirFunction,
    def_map: &ValueDefMap,
    inputs: &[(BasicBlockId, ValueId)],
    field_box_origins: &FieldBoxOriginMap,
    param_box_origins: &ParamBoxOriginMap,
    visiting_functions: &mut BTreeSet<String>,
    visiting_values: &mut BTreeSet<(String, ValueId)>,
    memo: &mut BoxOriginMemo,
) -> Option<String> {
    let mut observed = None;
    for (_, input) in inputs {
        let next = box_origin_for_value_inner(
            module,
            function,
            def_map,
            *input,
            field_box_origins,
            param_box_origins,
            visiting_functions,
            visiting_values,
            memo,
        );
        let Some(next) = next else {
            if is_null_or_void_value(function, def_map, *input) {
                continue;
            }
            return None;
        };
        observed = match observed {
            None => Some(next),
            Some(existing) if existing == next => Some(existing),
            _ => return None,
        };
    }
    observed
}

fn box_origin_for_call_return(
    module: &MirModule,
    function: &MirFunction,
    def_map: &ValueDefMap,
    callee: &Callee,
    arity: usize,
    field_box_origins: &FieldBoxOriginMap,
    param_box_origins: &ParamBoxOriginMap,
    visiting_functions: &mut BTreeSet<String>,
    visiting_values: &mut BTreeSet<(String, ValueId)>,
    memo: &mut BoxOriginMemo,
) -> Option<String> {
    match callee {
        Callee::Global(symbol) => box_origin_for_global_return(
            module,
            symbol,
            field_box_origins,
            param_box_origins,
            visiting_functions,
            visiting_values,
            memo,
        ),
        Callee::Method {
            box_name,
            method,
            receiver,
            ..
        } => {
            let (_, symbol) = same_module_method_target(
                module,
                function,
                def_map,
                box_name,
                method,
                *receiver,
                arity,
                field_box_origins,
                param_box_origins,
            )?;
            box_origin_for_global_return(
                module,
                &symbol,
                field_box_origins,
                param_box_origins,
                visiting_functions,
                visiting_values,
                memo,
            )
        }
        _ => None,
    }
}

fn box_origin_for_global_return(
    module: &MirModule,
    name: &str,
    field_box_origins: &FieldBoxOriginMap,
    param_box_origins: &ParamBoxOriginMap,
    visiting_functions: &mut BTreeSet<String>,
    visiting_values: &mut BTreeSet<(String, ValueId)>,
    memo: &mut BoxOriginMemo,
) -> Option<String> {
    if !visiting_functions.insert(name.to_string()) {
        return None;
    }
    let origin = module.functions.get(name).and_then(|target| {
        box_origin_for_function_returns(
            module,
            target,
            field_box_origins,
            param_box_origins,
            visiting_functions,
            visiting_values,
            memo,
        )
    });
    visiting_functions.remove(name);
    origin
}

fn box_origin_for_function_returns(
    module: &MirModule,
    function: &MirFunction,
    field_box_origins: &FieldBoxOriginMap,
    param_box_origins: &ParamBoxOriginMap,
    visiting_functions: &mut BTreeSet<String>,
    visiting_values: &mut BTreeSet<(String, ValueId)>,
    memo: &mut BoxOriginMemo,
) -> Option<String> {
    let def_map = build_value_def_map(function);
    let mut observed = None;
    for block in function.blocks.values() {
        for inst in block.instructions.iter().chain(block.terminator.iter()) {
            let MirInstruction::Return { value } = inst else {
                continue;
            };
            let Some(value) = *value else {
                continue;
            };
            let next = box_origin_for_value_inner(
                module,
                function,
                &def_map,
                value,
                field_box_origins,
                param_box_origins,
                visiting_functions,
                visiting_values,
                memo,
            );
            let Some(next) = next else {
                if is_null_or_void_value(function, &def_map, value) {
                    continue;
                }
                return None;
            };
            observed = match observed {
                None => Some(next),
                Some(existing) if existing == next => Some(existing),
                _ => return None,
            };
        }
    }
    observed.or_else(|| box_origin_from_mir_type(&function.signature.return_type))
}

fn box_origin_from_param(
    function: &MirFunction,
    value: ValueId,
    param_box_origins: &ParamBoxOriginMap,
) -> Option<String> {
    let param_index = function.params.iter().position(|param| *param == value)?;
    match param_box_origins.get(&(function.signature.name.clone(), param_index)) {
        Some(BoxOriginInference::Known(box_name)) => Some(box_name.clone()),
        Some(BoxOriginInference::Conflict) | None => function
            .signature
            .params
            .get(param_index)
            .and_then(box_origin_from_mir_type)
            .or_else(|| method_receiver_box_from_param_index(function, param_index)),
    }
}

#[path = "value_analysis_storage.rs"]
mod storage;

pub(crate) use self::storage::storage_for_value;
