use std::collections::{BTreeMap, BTreeSet, HashMap};

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
type ValueOriginMemo = HashMap<(String, ValueId), ValueId>;
type FunctionCopyOriginMemo = HashMap<String, DenseCopyOrigins>;

struct DenseCopyOrigins {
    parents: Vec<Option<ValueId>>,
    memo: Vec<Option<ValueId>>,
}

impl DenseCopyOrigins {
    fn new(function: &MirFunction) -> Self {
        let mut max_value = None;
        for block in function.blocks.values() {
            for inst in &block.instructions {
                if let MirInstruction::Copy { dst, src } = inst {
                    max_value = max_value.max(Some(dst.to_usize())).max(Some(src.to_usize()));
                }
            }
        }

        let len = max_value.map_or(0, |value| value.saturating_add(1));
        let mut parents = vec![None; len];
        for block in function.blocks.values() {
            for inst in &block.instructions {
                if let MirInstruction::Copy { dst, src } = inst {
                    if let Some(slot) = parents.get_mut(dst.to_usize()) {
                        *slot = Some(*src);
                    }
                }
            }
        }

        Self {
            parents,
            memo: vec![None; len],
        }
    }

    fn origin(&mut self, mut value: ValueId) -> ValueId {
        let start = value;
        let mut path = Vec::new();
        let mut visited = BTreeSet::new();

        while visited.insert(value) {
            if let Some(origin) = self.memo_for(value) {
                self.memo_path(path, origin);
                self.memo_value(start, origin);
                return origin;
            }

            path.push(value);
            let Some(parent) = self.parent_for(value) else {
                break;
            };
            value = parent;
        }

        self.memo_path(path, value);
        self.memo_value(start, value);
        value
    }

    fn parent_for(&self, value: ValueId) -> Option<ValueId> {
        self.parents.get(value.to_usize()).copied().flatten()
    }

    fn memo_for(&self, value: ValueId) -> Option<ValueId> {
        self.memo.get(value.to_usize()).copied().flatten()
    }

    fn memo_value(&mut self, value: ValueId, origin: ValueId) {
        if let Some(slot) = self.memo.get_mut(value.to_usize()) {
            *slot = Some(origin);
        }
    }

    fn memo_path(&mut self, path: Vec<ValueId>, origin: ValueId) {
        for value in path {
            self.memo_value(value, origin);
        }
    }
}

pub(super) struct BoxOriginQueryContext<'a> {
    module: &'a MirModule,
    field_box_origins: &'a FieldBoxOriginMap,
    param_box_origins: &'a ParamBoxOriginMap,
    visiting_functions: BTreeSet<String>,
    visiting_values: BTreeSet<(String, ValueId)>,
    memo: BoxOriginMemo,
    value_origin_memo: ValueOriginMemo,
    copy_origin_memo: FunctionCopyOriginMemo,
}

impl<'a> BoxOriginQueryContext<'a> {
    pub(super) fn new(
        module: &'a MirModule,
        field_box_origins: &'a FieldBoxOriginMap,
        param_box_origins: &'a ParamBoxOriginMap,
    ) -> Self {
        Self {
            module,
            field_box_origins,
            param_box_origins,
            visiting_functions: BTreeSet::new(),
            visiting_values: BTreeSet::new(),
            memo: BTreeMap::new(),
            value_origin_memo: HashMap::new(),
            copy_origin_memo: HashMap::new(),
        }
    }

    pub(super) fn box_origin_for_value(
        &mut self,
        function: &MirFunction,
        def_map: &ValueDefMap,
        value: ValueId,
    ) -> Option<String> {
        box_origin_for_value_inner(
            self.module,
            function,
            def_map,
            value,
            self.field_box_origins,
            self.param_box_origins,
            &mut self.visiting_functions,
            &mut self.visiting_values,
            &mut self.memo,
            &mut self.value_origin_memo,
            &mut self.copy_origin_memo,
        )
    }

    pub(super) fn same_module_method_target(
        &mut self,
        function: &MirFunction,
        def_map: &ValueDefMap,
        box_name: &str,
        method: &str,
        receiver: Option<ValueId>,
        arity: usize,
    ) -> Option<(String, String)> {
        same_module_method_target_inner(
            self.module,
            function,
            def_map,
            box_name,
            method,
            receiver,
            arity,
            self.field_box_origins,
            self.param_box_origins,
            &mut self.visiting_functions,
            &mut self.visiting_values,
            &mut self.memo,
            &mut self.value_origin_memo,
            &mut self.copy_origin_memo,
        )
    }
}

fn cached_value_origin(
    function: &MirFunction,
    value: ValueId,
    value_origin_memo: &mut ValueOriginMemo,
    copy_origin_memo: &mut FunctionCopyOriginMemo,
) -> ValueId {
    let key = (function.signature.name.clone(), value);
    if let Some(origin) = value_origin_memo.get(&key).copied() {
        return origin;
    }
    let origin = copy_origin_memo
        .entry(function.signature.name.clone())
        .or_insert_with(|| DenseCopyOrigins::new(function))
        .origin(value);
    value_origin_memo.insert(key, origin);
    origin
}

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
    let mut visiting_functions = BTreeSet::new();
    let mut visiting_values = BTreeSet::new();
    let mut memo = BTreeMap::new();
    let mut value_origin_memo = HashMap::new();
    let mut copy_origin_memo = HashMap::new();
    same_module_method_target_inner(
        module,
        function,
        def_map,
        box_name,
        method,
        receiver,
        arity,
        field_box_origins,
        param_box_origins,
        &mut visiting_functions,
        &mut visiting_values,
        &mut memo,
        &mut value_origin_memo,
        &mut copy_origin_memo,
    )
}

fn same_module_method_target_inner(
    module: &MirModule,
    function: &MirFunction,
    def_map: &ValueDefMap,
    box_name: &str,
    method: &str,
    receiver: Option<ValueId>,
    arity: usize,
    field_box_origins: &FieldBoxOriginMap,
    param_box_origins: &ParamBoxOriginMap,
    visiting_functions: &mut BTreeSet<String>,
    visiting_values: &mut BTreeSet<(String, ValueId)>,
    memo: &mut BoxOriginMemo,
    value_origin_memo: &mut ValueOriginMemo,
    copy_origin_memo: &mut FunctionCopyOriginMemo,
) -> Option<(String, String)> {
    if let Some(receiver) = receiver {
        if let Some(receiver_box) = box_origin_for_value_inner(
            module,
            function,
            def_map,
            receiver,
            field_box_origins,
            param_box_origins,
            visiting_functions,
            visiting_values,
            memo,
            value_origin_memo,
            copy_origin_memo,
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
    let mut value_origin_memo = HashMap::new();
    let mut copy_origin_memo = HashMap::new();
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
        &mut value_origin_memo,
        &mut copy_origin_memo,
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
    value_origin_memo: &mut ValueOriginMemo,
    copy_origin_memo: &mut FunctionCopyOriginMemo,
) -> Option<String> {
    let origin = cached_value_origin(function, value, value_origin_memo, copy_origin_memo);
    let value_key = (function.signature.name.clone(), origin);
    if let Some(cached) = memo.get(&value_key) {
        return cached.clone();
    }
    if !visiting_values.insert(value_key.clone()) {
        memo.insert(value_key, None);
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
                            value_origin_memo,
                            copy_origin_memo,
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
                            value_origin_memo,
                            copy_origin_memo,
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
                    value_origin_memo,
                    copy_origin_memo,
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
    value_origin_memo: &mut ValueOriginMemo,
    copy_origin_memo: &mut FunctionCopyOriginMemo,
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
            value_origin_memo,
            copy_origin_memo,
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
    value_origin_memo: &mut ValueOriginMemo,
    copy_origin_memo: &mut FunctionCopyOriginMemo,
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
            value_origin_memo,
            copy_origin_memo,
        ),
        Callee::Method {
            box_name,
            method,
            receiver,
            ..
        } => {
            let (_, symbol) = same_module_method_target_inner(
                module,
                function,
                def_map,
                box_name,
                method,
                *receiver,
                arity,
                field_box_origins,
                param_box_origins,
                visiting_functions,
                visiting_values,
                memo,
                value_origin_memo,
                copy_origin_memo,
            )?;
            box_origin_for_global_return(
                module,
                &symbol,
                field_box_origins,
                param_box_origins,
                visiting_functions,
                visiting_values,
                memo,
                value_origin_memo,
                copy_origin_memo,
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
    value_origin_memo: &mut ValueOriginMemo,
    copy_origin_memo: &mut FunctionCopyOriginMemo,
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
            value_origin_memo,
            copy_origin_memo,
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
    value_origin_memo: &mut ValueOriginMemo,
    copy_origin_memo: &mut FunctionCopyOriginMemo,
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
                value_origin_memo,
                copy_origin_memo,
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

pub(crate) use self::storage::{storage_for_value, StorageQueryContext};
