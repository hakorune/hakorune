/*!
 * MIR-owned route plans for generic array/text read-side observers.
 *
 * This module owns the legality/provenance/consumer contract for routes such
 * as `array.get(i).indexOf(needle)`. Backends may consume this metadata to
 * select helper calls, but helper symbols and raw MIR window matching stay out
 * of the MIR contract.
 */

use std::collections::BTreeSet;

use super::{
    build_value_def_map, definitions::Callee, resolve_value_origin, BasicBlockId, BinaryOp,
    CompareOp, ConstValue, MirFunction, MirInstruction, MirModule, ValueDefMap, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverKind {
    IndexOf,
}

impl std::fmt::Display for ArrayTextObserverKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IndexOf => f.write_str("indexof"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverConsumerShape {
    DirectScalar,
    FoundPredicate,
}

impl std::fmt::Display for ArrayTextObserverConsumerShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DirectScalar => f.write_str("direct_scalar"),
            Self::FoundPredicate => f.write_str("found_predicate"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverProofRegion {
    ArrayGetReceiverIndexOf,
}

impl std::fmt::Display for ArrayTextObserverProofRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArrayGetReceiverIndexOf => f.write_str("array_get_receiver_indexof"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverPublicationBoundary {
    None,
}

impl std::fmt::Display for ArrayTextObserverPublicationBoundary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("none"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverResultRepr {
    ScalarI64,
}

impl std::fmt::Display for ArrayTextObserverResultRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ScalarI64 => f.write_str("scalar_i64"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayTextObserverArgRepr {
    Value,
    ConstUtf8 { text: String, byte_len: usize },
}

impl ArrayTextObserverArgRepr {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Value => "value",
            Self::ConstUtf8 { .. } => "const_utf8",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextObserverRoute {
    pub block: BasicBlockId,
    pub observer_instruction_index: usize,
    pub get_block: BasicBlockId,
    pub get_instruction_index: usize,
    pub array_value: ValueId,
    pub index_value: ValueId,
    pub source_value: ValueId,
    pub observer_kind: ArrayTextObserverKind,
    pub observer_arg0: ValueId,
    pub observer_arg0_repr: ArrayTextObserverArgRepr,
    pub observer_arg0_keep_live: bool,
    pub result_value: ValueId,
    pub consumer_shape: ArrayTextObserverConsumerShape,
    pub proof_region: ArrayTextObserverProofRegion,
    pub publication_boundary: ArrayTextObserverPublicationBoundary,
    pub result_repr: ArrayTextObserverResultRepr,
    pub keep_get_live: bool,
}

pub fn refresh_module_array_text_observer_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_array_text_observer_routes(function);
    }
}

pub fn refresh_function_array_text_observer_routes(function: &mut MirFunction) {
    let def_map = build_value_def_map(function);
    let mut routes = Vec::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            if let Some(route) = match_array_text_indexof_route(
                function,
                &def_map,
                block_id,
                instruction_index,
                inst,
            ) {
                routes.push(route);
            }
        }
    }

    routes.sort_by_key(|route| (route.block.as_u32(), route.observer_instruction_index));
    function.metadata.array_text_observer_routes = routes;
}

fn root(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> ValueId {
    resolve_value_origin(function, def_map, value)
}

#[allow(clippy::too_many_arguments)]
fn match_array_text_indexof_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    observer_instruction_index: usize,
    inst: &MirInstruction,
) -> Option<ArrayTextObserverRoute> {
    let (result_value, receiver_value, observer_arg0) = match_indexof_call(inst)?;
    let source_value = root(function, def_map, receiver_value);
    let (get_block, get_instruction_index, array_value, index_value) =
        match_array_get_source(function, def_map, source_value)?;
    let observer_arg0_root = root(function, def_map, observer_arg0);
    let observer_arg0_repr = const_utf8(function, def_map, observer_arg0_root)
        .map(|text| ArrayTextObserverArgRepr::ConstUtf8 {
            byte_len: text.len(),
            text,
        })
        .unwrap_or(ArrayTextObserverArgRepr::Value);

    let consumer_shape = if has_found_predicate_consumer(function, def_map, result_value) {
        ArrayTextObserverConsumerShape::FoundPredicate
    } else {
        ArrayTextObserverConsumerShape::DirectScalar
    };
    let no_covered_source_values = BTreeSet::new();

    Some(ArrayTextObserverRoute {
        block,
        observer_instruction_index,
        get_block,
        get_instruction_index,
        array_value,
        index_value,
        source_value,
        observer_kind: ArrayTextObserverKind::IndexOf,
        observer_arg0: observer_arg0_root,
        observer_arg0_repr,
        observer_arg0_keep_live: has_non_observer_value_use(
            function,
            def_map,
            observer_arg0_root,
            observer_arg0,
            block,
            observer_instruction_index,
            &no_covered_source_values,
        ),
        result_value,
        consumer_shape,
        proof_region: ArrayTextObserverProofRegion::ArrayGetReceiverIndexOf,
        publication_boundary: ArrayTextObserverPublicationBoundary::None,
        result_repr: ArrayTextObserverResultRepr::ScalarI64,
        keep_get_live: has_non_observer_source_use(
            function,
            def_map,
            source_value,
            receiver_value,
            array_value,
            index_value,
            block,
            observer_instruction_index,
        ),
    })
}

fn match_indexof_call(inst: &MirInstruction) -> Option<(ValueId, ValueId, ValueId)> {
    match inst {
        MirInstruction::Call {
            dst: Some(dst),
            callee:
                Some(Callee::Method {
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            args,
            ..
        } if method == "indexOf" && args.len() == 1 => Some((*dst, *receiver, args[0])),
        _ => None,
    }
}

fn match_array_get_source(
    function: &MirFunction,
    def_map: &ValueDefMap,
    source_value: ValueId,
) -> Option<(BasicBlockId, usize, ValueId, ValueId)> {
    let (block, index) = def_map.get(&source_value).copied()?;
    let inst = function.blocks.get(&block)?.instructions.get(index)?;
    match inst {
        MirInstruction::Call {
            callee:
                Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(array_value),
                    ..
                }),
            args,
            ..
        } if method == "get"
            && args.len() == 1
            && matches!(box_name.as_str(), "RuntimeDataBox" | "ArrayBox") =>
        {
            Some((
                block,
                index,
                root(function, def_map, *array_value),
                root(function, def_map, args[0]),
            ))
        }
        _ => None,
    }
}

fn has_found_predicate_consumer(
    function: &MirFunction,
    def_map: &ValueDefMap,
    result_value: ValueId,
) -> bool {
    let result_root = root(function, def_map, result_value);
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for inst in &block.instructions {
            if compare_is_found_predicate(function, def_map, inst, result_root) {
                return true;
            }
        }
        if let Some(term) = &block.terminator {
            if compare_is_found_predicate(function, def_map, term, result_root) {
                return true;
            }
        }
    }
    false
}

fn compare_is_found_predicate(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    result_root: ValueId,
) -> bool {
    let MirInstruction::Compare { op, lhs, rhs, .. } = inst else {
        return false;
    };
    let lhs_root = root(function, def_map, *lhs);
    let rhs_root = root(function, def_map, *rhs);
    if lhs_root == result_root {
        return match const_i64(function, def_map, *rhs) {
            Some(0) => matches!(op, CompareOp::Ge),
            Some(-1) => matches!(op, CompareOp::Gt | CompareOp::Ne),
            _ => false,
        };
    }
    if rhs_root == result_root {
        return match const_i64(function, def_map, *lhs) {
            Some(0) => matches!(op, CompareOp::Le),
            Some(-1) => matches!(op, CompareOp::Lt | CompareOp::Ne),
            _ => false,
        };
    }
    false
}

fn const_i64(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> Option<i64> {
    let value = root(function, def_map, value);
    let (block, index) = def_map.get(&value).copied()?;
    match function.blocks.get(&block)?.instructions.get(index)? {
        MirInstruction::Const {
            value: ConstValue::Integer(actual),
            ..
        } => Some(*actual),
        _ => None,
    }
}

fn const_utf8(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> Option<String> {
    let value = root(function, def_map, value);
    let (block, index) = def_map.get(&value).copied()?;
    match function.blocks.get(&block)?.instructions.get(index)? {
        MirInstruction::Const {
            value: ConstValue::String(actual),
            ..
        } => Some(actual.clone()),
        _ => None,
    }
}

fn has_non_observer_source_use(
    function: &MirFunction,
    def_map: &ValueDefMap,
    source_value: ValueId,
    observer_receiver_value: ValueId,
    array_value: ValueId,
    index_value: ValueId,
    observer_block: BasicBlockId,
    observer_instruction_index: usize,
) -> bool {
    let covered_slot_suffix_values = same_slot_const_suffix_store_source_values(
        function,
        def_map,
        source_value,
        array_value,
        index_value,
    );
    has_non_observer_value_use(
        function,
        def_map,
        source_value,
        observer_receiver_value,
        observer_block,
        observer_instruction_index,
        &covered_slot_suffix_values,
    )
}

fn has_non_observer_value_use(
    function: &MirFunction,
    def_map: &ValueDefMap,
    source_value: ValueId,
    observer_value: ValueId,
    observer_block: BasicBlockId,
    observer_instruction_index: usize,
    covered_source_values: &BTreeSet<ValueId>,
) -> bool {
    let source_root = root(function, def_map, source_value);
    let observer_chain = copy_chain_values(function, def_map, observer_value);
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            if block_id == observer_block && instruction_index == observer_instruction_index {
                continue;
            }
            if is_observer_chain_copy(function, def_map, inst, source_root, &observer_chain) {
                continue;
            }
            if is_covered_slot_consumer_source_use(
                function,
                def_map,
                inst,
                source_root,
                covered_source_values,
            ) {
                continue;
            }
            if inst
                .used_values()
                .into_iter()
                .any(|value| root(function, def_map, value) == source_root)
            {
                return true;
            }
        }
        if let Some(term) = &block.terminator {
            if term
                .used_values()
                .into_iter()
                .any(|value| root(function, def_map, value) == source_root)
            {
                return true;
            }
        }
    }
    false
}

fn same_slot_const_suffix_store_source_values(
    function: &MirFunction,
    def_map: &ValueDefMap,
    source_value: ValueId,
    array_value: ValueId,
    index_value: ValueId,
) -> BTreeSet<ValueId> {
    let source_root = root(function, def_map, source_value);
    let array_root = root(function, def_map, array_value);
    let index_root = root(function, def_map, index_value);
    let mut covered = BTreeSet::new();

    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for inst in &block.instructions {
            let Some((concat_value, source_operand)) =
                const_suffix_concat_from_source(function, def_map, inst, source_root)
            else {
                continue;
            };
            if !has_same_slot_set_consumer(function, def_map, concat_value, array_root, index_root)
            {
                continue;
            }
            covered.extend(copy_chain_values(function, def_map, source_operand));
            covered.insert(source_root);
            covered.insert(concat_value);
        }
    }

    covered
}

fn const_suffix_concat_from_source(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    source_root: ValueId,
) -> Option<(ValueId, ValueId)> {
    let MirInstruction::BinOp {
        dst,
        op: BinaryOp::Add,
        lhs,
        rhs,
        ..
    } = inst
    else {
        return None;
    };
    let lhs_root = root(function, def_map, *lhs);
    let rhs_root = root(function, def_map, *rhs);
    if lhs_root == source_root && const_utf8(function, def_map, rhs_root).is_some() {
        return Some((*dst, *lhs));
    }
    if rhs_root == source_root && const_utf8(function, def_map, lhs_root).is_some() {
        return Some((*dst, *rhs));
    }
    None
}

fn has_same_slot_set_consumer(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    array_root: ValueId,
    index_root: ValueId,
) -> bool {
    let value_root = root(function, def_map, value);
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for inst in &block.instructions {
            if is_same_slot_set_consumer(
                function, def_map, inst, value_root, array_root, index_root,
            ) {
                return true;
            }
        }
    }
    false
}

fn is_same_slot_set_consumer(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    value_root: ValueId,
    array_root: ValueId,
    index_root: ValueId,
) -> bool {
    match inst {
        MirInstruction::Call {
            callee:
                Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            args,
            ..
        } if method == "set"
            && args.len() == 2
            && matches!(box_name.as_str(), "RuntimeDataBox" | "ArrayBox") =>
        {
            root(function, def_map, *receiver) == array_root
                && root(function, def_map, args[0]) == index_root
                && root(function, def_map, args[1]) == value_root
        }
        _ => false,
    }
}

fn is_covered_slot_consumer_source_use(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    source_root: ValueId,
    covered_source_values: &BTreeSet<ValueId>,
) -> bool {
    match inst {
        MirInstruction::Copy { dst, src } => {
            covered_source_values.contains(dst) && root(function, def_map, *src) == source_root
        }
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
            ..
        } => {
            covered_source_values.contains(dst)
                && (root(function, def_map, *lhs) == source_root
                    || root(function, def_map, *rhs) == source_root)
        }
        _ => false,
    }
}

fn copy_chain_values(
    function: &MirFunction,
    def_map: &ValueDefMap,
    mut value: ValueId,
) -> BTreeSet<ValueId> {
    let mut chain = BTreeSet::new();
    while chain.insert(value) {
        let Some((block, index)) = def_map.get(&value).copied() else {
            break;
        };
        let Some(inst) = function
            .blocks
            .get(&block)
            .and_then(|block| block.instructions.get(index))
        else {
            break;
        };
        match inst {
            MirInstruction::Copy { src, .. } => value = *src,
            _ => break,
        }
    }
    chain
}

fn is_observer_chain_copy(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    source_root: ValueId,
    observer_chain: &BTreeSet<ValueId>,
) -> bool {
    match inst {
        MirInstruction::Copy { dst, src } => {
            observer_chain.contains(dst) && root(function, def_map, *src) == source_root
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{BasicBlock, EffectMask, FunctionSignature, MirType};

    #[test]
    fn detects_array_get_indexof_found_predicate_route() {
        let mut function = test_function(MirType::Bool);
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, 1, 2));
        block.add_instruction(const_s(11, "line"));
        block.add_instruction(indexof_call(12, 10, 11));
        block.add_instruction(const_i(13, 0));
        block.add_instruction(compare(14, CompareOp::Ge, 12, 13));
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(14)),
        });

        refresh_function_array_text_observer_routes(&mut function);

        assert_eq!(function.metadata.array_text_observer_routes.len(), 1);
        let route = &function.metadata.array_text_observer_routes[0];
        assert_eq!(route.array_value, ValueId::new(1));
        assert_eq!(route.index_value, ValueId::new(2));
        assert_eq!(route.source_value, ValueId::new(10));
        assert_eq!(route.observer_arg0, ValueId::new(11));
        assert_eq!(
            route.observer_arg0_repr,
            ArrayTextObserverArgRepr::ConstUtf8 {
                text: "line".to_string(),
                byte_len: 4,
            }
        );
        assert!(!route.observer_arg0_keep_live);
        assert_eq!(route.result_value, ValueId::new(12));
        assert_eq!(
            route.consumer_shape,
            ArrayTextObserverConsumerShape::FoundPredicate
        );
        assert_eq!(
            route.publication_boundary,
            ArrayTextObserverPublicationBoundary::None
        );
        assert_eq!(route.result_repr, ArrayTextObserverResultRepr::ScalarI64);
        assert!(!route.keep_get_live);
    }

    #[test]
    fn detects_array_get_indexof_direct_scalar_route() {
        let mut function = test_function(MirType::Integer);
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, 1, 2));
        block.add_instruction(const_s(11, "needle"));
        block.add_instruction(indexof_call(12, 10, 11));
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(12)),
        });

        refresh_function_array_text_observer_routes(&mut function);

        assert_eq!(function.metadata.array_text_observer_routes.len(), 1);
        let route = &function.metadata.array_text_observer_routes[0];
        assert_eq!(
            route.consumer_shape,
            ArrayTextObserverConsumerShape::DirectScalar
        );
        assert_eq!(route.observer_kind, ArrayTextObserverKind::IndexOf);
        assert_eq!(
            route.proof_region,
            ArrayTextObserverProofRegion::ArrayGetReceiverIndexOf
        );
    }

    #[test]
    fn marks_get_live_when_source_has_non_observer_use() {
        let mut function = test_function(MirType::Integer);
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, 1, 2));
        block.add_instruction(const_s(11, "needle"));
        block.add_instruction(indexof_call(12, 10, 11));
        block.add_instruction(len_call(13, 10));
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(12)),
        });

        refresh_function_array_text_observer_routes(&mut function);

        let route = &function.metadata.array_text_observer_routes[0];
        assert!(route.keep_get_live);
    }

    #[test]
    fn keeps_get_dead_when_source_only_feeds_same_slot_const_suffix_store() {
        let mut function = test_function(MirType::Integer);
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, 1, 2));
        block.add_instruction(const_s(11, "line"));
        block.add_instruction(indexof_call(12, 10, 11));
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(13),
            src: ValueId::new(10),
        });
        block.add_instruction(const_s(14, "ln"));
        block.add_instruction(add(15, 13, 14));
        block.add_instruction(array_set(16, 1, 2, 15));
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(12)),
        });

        refresh_function_array_text_observer_routes(&mut function);

        let route = &function.metadata.array_text_observer_routes[0];
        assert!(!route.keep_get_live);
    }

    #[test]
    fn marks_observer_arg_live_when_const_is_used_elsewhere() {
        let mut function = test_function(MirType::Integer);
        let block = entry_block(&mut function);
        block.add_instruction(array_get(10, 1, 2));
        block.add_instruction(const_s(11, "needle"));
        block.add_instruction(indexof_call(12, 10, 11));
        block.add_instruction(len_call(13, 11));
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(12)),
        });

        refresh_function_array_text_observer_routes(&mut function);

        let route = &function.metadata.array_text_observer_routes[0];
        assert!(route.observer_arg0_keep_live);
    }

    fn test_function(return_type: MirType) -> MirFunction {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("ArrayBox".to_string()), MirType::Integer],
            return_type,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function.params = vec![ValueId::new(1), ValueId::new(2)];
        function
    }

    fn entry_block(function: &mut MirFunction) -> &mut BasicBlock {
        function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block")
    }

    fn const_i(dst: u32, value: i64) -> MirInstruction {
        MirInstruction::Const {
            dst: ValueId::new(dst),
            value: ConstValue::Integer(value),
        }
    }

    fn const_s(dst: u32, value: &str) -> MirInstruction {
        MirInstruction::Const {
            dst: ValueId::new(dst),
            value: ConstValue::String(value.to_string()),
        }
    }

    fn compare(dst: u32, op: CompareOp, lhs: u32, rhs: u32) -> MirInstruction {
        MirInstruction::Compare {
            dst: ValueId::new(dst),
            op,
            lhs: ValueId::new(lhs),
            rhs: ValueId::new(rhs),
        }
    }

    fn array_get(dst: u32, array: u32, index: u32) -> MirInstruction {
        method_call(
            dst,
            "RuntimeDataBox",
            "get",
            array,
            vec![ValueId::new(index)],
        )
    }

    fn indexof_call(dst: u32, receiver: u32, needle: u32) -> MirInstruction {
        method_call(
            dst,
            "RuntimeDataBox",
            "indexOf",
            receiver,
            vec![ValueId::new(needle)],
        )
    }

    fn len_call(dst: u32, receiver: u32) -> MirInstruction {
        method_call(dst, "RuntimeDataBox", "length", receiver, vec![])
    }

    fn add(dst: u32, lhs: u32, rhs: u32) -> MirInstruction {
        MirInstruction::BinOp {
            dst: ValueId::new(dst),
            op: BinaryOp::Add,
            lhs: ValueId::new(lhs),
            rhs: ValueId::new(rhs),
        }
    }

    fn array_set(dst: u32, array: u32, index: u32, value: u32) -> MirInstruction {
        method_call(
            dst,
            "RuntimeDataBox",
            "set",
            array,
            vec![ValueId::new(index), ValueId::new(value)],
        )
    }

    fn method_call(
        dst: u32,
        box_name: &str,
        method: &str,
        receiver: u32,
        args: Vec<ValueId>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: Some(ValueId::new(dst)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(ValueId::new(receiver)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args,
            effects: EffectMask::PURE,
        }
    }
}
