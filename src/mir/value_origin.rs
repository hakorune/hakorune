/*!
 * Generic value-origin queries.
 *
 * This module owns copy-chain normalization for MIR values. Domain recognizers
 * may consume these queries, but they must not re-own the traversal rules.
 */

use super::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub type ValueDefMap = HashMap<ValueId, (BasicBlockId, usize)>;
pub type ParentMap = HashMap<ValueId, ValueId>;
pub type CopyParentMap = ParentMap;

pub fn build_value_def_map(function: &MirFunction) -> ValueDefMap {
    let mut defs = ValueDefMap::new();
    for (bbid, block) in &function.blocks {
        for (idx, inst) in block.instructions.iter().enumerate() {
            if let Some(dst) = inst.dst_value() {
                defs.insert(dst, (*bbid, idx));
            }
        }
    }
    defs
}

pub struct ValueOriginQueryContext<'a> {
    function: &'a MirFunction,
    def_map: &'a ValueDefMap,
    memo: HashMap<ValueId, ValueId>,
}

impl<'a> ValueOriginQueryContext<'a> {
    pub fn new(function: &'a MirFunction, def_map: &'a ValueDefMap) -> Self {
        Self {
            function,
            def_map,
            memo: HashMap::new(),
        }
    }

    pub fn origin(&mut self, value: ValueId) -> ValueId {
        if let Some(origin) = self.memo.get(&value).copied() {
            return origin;
        }
        let (origin, path) =
            walk_value_origin(self.function, self.def_map, value, Some(&self.memo));
        self.memo_path(path, origin);
        self.memo.insert(value, origin);
        origin
    }

    fn memo_path(&mut self, path: Vec<ValueId>, origin: ValueId) {
        for value in path {
            self.memo.insert(value, origin);
        }
    }
}

/// The same copy-chain query, retaining every visited definition for consumers
/// that must corroborate a proof after MIR finishing. Memo shortcuts are omitted
/// so the returned chain always includes the complete original dependency path.
pub(in crate::mir) fn trace_value_origin(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> (ValueId, Vec<ValueId>) {
    walk_value_origin(function, def_map, value, None)
}

fn walk_value_origin(
    function: &MirFunction,
    def_map: &ValueDefMap,
    mut value: ValueId,
    memo: Option<&HashMap<ValueId, ValueId>>,
) -> (ValueId, Vec<ValueId>) {
    let mut path = Vec::new();
    let mut visited = BTreeSet::new();
    while visited.insert(value) {
        if let Some(origin) = memo.and_then(|memo| memo.get(&value)).copied() {
            return (origin, path);
        }
        path.push(value);
        let Some((bbid, idx)) = def_map.get(&value).copied() else {
            break;
        };
        let Some(inst) = function
            .blocks
            .get(&bbid)
            .and_then(|block| block.instructions.get(idx))
        else {
            break;
        };
        match inst {
            MirInstruction::Copy { src, .. } => value = *src,
            _ => break,
        }
    }
    (value, path)
}

pub fn resolve_value_origin(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> ValueId {
    ValueOriginQueryContext::new(function, def_map).origin(value)
}

/// Opt-in identity transport through Copies and acyclic unanimous Phis.
/// This does not infer a resource role or grant any ownership permission.
/// Unlike the compatibility Copy query, missing/ambiguous definitions fail.
pub(in crate::mir) struct AcyclicUnanimousValueOrigins {
    roots: HashMap<ValueId, Option<ValueId>>,
}

impl AcyclicUnanimousValueOrigins {
    pub(in crate::mir) fn new(function: &MirFunction) -> Self {
        let mut definitions = HashMap::new();
        let mut ambiguous = BTreeSet::new();
        let mut predecessors: BTreeMap<BasicBlockId, BTreeSet<BasicBlockId>> = BTreeMap::new();
        let mut parameters = BTreeSet::new();
        for value in &function.params {
            if !parameters.insert(*value) { ambiguous.insert(*value); }
        }
        for (id, block) in &function.blocks {
            for instruction in &block.instructions {
                if let Some(dst) = instruction.dst_value() {
                    if definitions.insert(dst, (*id, instruction)).is_some()
                        || parameters.contains(&dst)
                    { ambiguous.insert(dst); }
                }
            }
            for target in block.successors_from_terminator() {
                predecessors.entry(target).or_default().insert(*id);
            }
        }
        fn resolve(
            value: ValueId,
            definitions: &HashMap<ValueId, (BasicBlockId, &MirInstruction)>,
            parameters: &BTreeSet<ValueId>,
            ambiguous: &BTreeSet<ValueId>,
            predecessors: &BTreeMap<BasicBlockId, BTreeSet<BasicBlockId>>,
            active: &mut BTreeSet<ValueId>,
            roots: &mut HashMap<ValueId, Option<ValueId>>,
        ) -> Option<ValueId> {
            if let Some(root) = roots.get(&value) { return *root; }
            if ambiguous.contains(&value) || !active.insert(value) { return None; }
            let result = (|| {
                let Some((block, instruction)) = definitions.get(&value) else {
                    return parameters.contains(&value).then_some(value);
                };
                let mut child = |value| resolve(value, definitions, parameters, ambiguous,
                    predecessors, active, roots);
                match instruction {
                    MirInstruction::Copy { src, .. } => child(*src),
                    MirInstruction::Phi { inputs, .. } => {
                        let incoming: BTreeSet<_> = inputs.iter().map(|(bb, _)| *bb).collect();
                        if inputs.is_empty() || incoming.len() != inputs.len()
                            || predecessors.get(block) != Some(&incoming)
                        { return None; }
                        let root = child(inputs[0].1)?;
                        for (_, input) in inputs.iter().skip(1) {
                            if child(*input) != Some(root) { return None; }
                        }
                        Some(root)
                    }
                    _ => Some(value),
                }
            })();
            active.remove(&value);
            roots.insert(value, result);
            result
        }
        let mut roots = HashMap::new();
        let mut active = BTreeSet::new();
        for value in definitions.keys().chain(parameters.iter()).copied() {
            resolve(value, &definitions, &parameters, &ambiguous, &predecessors,
                &mut active, &mut roots);
        }
        Self { roots }
    }

    pub(in crate::mir) fn origin(&self, value: ValueId) -> Option<ValueId> {
        self.roots.get(&value).copied().flatten()
    }
}

pub fn resolve_value_origin_from_parent_map(mut value: ValueId, parents: &ParentMap) -> ValueId {
    let mut seen = BTreeSet::new();
    while let Some(parent) = parents.get(&value) {
        if !seen.insert(value) {
            break;
        }
        value = *parent;
    }
    value
}

pub fn resolve_value_origin_from_copy_parents(
    value: ValueId,
    copy_parents: &CopyParentMap,
) -> ValueId {
    resolve_value_origin_from_parent_map(value, copy_parents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlock, EffectMask, FunctionSignature, MirModule, MirType};

    #[test]
    fn resolve_value_origin_follows_copy_chain_in_function() {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            target: crate::mir::ConstructionTarget::Named("Point".to_string()),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(3),
            src: ValueId::new(2),
        });
        function.add_block(block);

        let def_map = build_value_def_map(&function);
        assert_eq!(
            resolve_value_origin(&function, &def_map, ValueId::new(3)),
            ValueId::new(1)
        );
        assert_eq!(
            trace_value_origin(&function, &def_map, ValueId::new(3)),
            (
                ValueId::new(1),
                vec![ValueId::new(3), ValueId::new(2), ValueId::new(1)]
            )
        );
    }

    #[test]
    fn value_origin_query_context_reuses_copy_chain_origin() {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            target: crate::mir::ConstructionTarget::Named("Point".to_string()),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(3),
            src: ValueId::new(2),
        });
        function.add_block(block);

        let def_map = build_value_def_map(&function);
        let mut context = ValueOriginQueryContext::new(&function, &def_map);

        assert_eq!(context.origin(ValueId::new(3)), ValueId::new(1));
        assert_eq!(context.origin(ValueId::new(2)), ValueId::new(1));
    }

    #[test]
    fn value_origin_query_context_breaks_copy_cycles() {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(1),
            src: ValueId::new(2),
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        function.add_block(block);

        let def_map = build_value_def_map(&function);
        let mut context = ValueOriginQueryContext::new(&function, &def_map);
        let resolved = context.origin(ValueId::new(1));

        assert!(matches!(resolved, ValueId(1) | ValueId(2)));
    }

    #[test]
    fn resolve_value_origin_from_copy_parents_breaks_cycles() {
        let mut copy_parents = CopyParentMap::new();
        copy_parents.insert(ValueId::new(2), ValueId::new(1));
        copy_parents.insert(ValueId::new(1), ValueId::new(2));

        let resolved = resolve_value_origin_from_copy_parents(ValueId::new(2), &copy_parents);
        assert!(matches!(resolved, ValueId(1) | ValueId(2)));
    }

    #[test]
    fn resolve_value_origin_from_parent_map_follows_generic_parent_chain() {
        let mut parents = ParentMap::new();
        parents.insert(ValueId::new(3), ValueId::new(2));
        parents.insert(ValueId::new(2), ValueId::new(1));

        assert_eq!(
            resolve_value_origin_from_parent_map(ValueId::new(3), &parents),
            ValueId::new(1)
        );
    }

    #[test]
    fn build_value_def_map_records_defined_values() {
        let mut module = MirModule::new("value_origin_test".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block")
            .add_instruction(MirInstruction::Const {
                dst: ValueId::new(1),
                value: crate::mir::ConstValue::Integer(1),
            });
        module.add_function(function);

        let function = module.get_function("main").expect("main");
        let def_map = build_value_def_map(function);
        assert!(def_map.contains_key(&ValueId::new(1)));
    }
}
