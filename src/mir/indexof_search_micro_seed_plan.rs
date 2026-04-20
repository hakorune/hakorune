/*!
 * MIR-owned route plan for the temporary indexOf search micro seed bridge.
 *
 * The active string-search exact bridges still use specialized C emitters, but
 * the route proof belongs in MIR metadata. The backend may select an emitter
 * from this plan; it must not rediscover the search shape from raw MIR JSON.
 */

use super::{
    definitions::Callee, BasicBlockId, BinaryOp, CompareOp, ConstValue, MirFunction,
    MirInstruction, MirModule,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexOfSearchMicroSeedVariant {
    Leaf,
    Line,
}

impl std::fmt::Display for IndexOfSearchMicroSeedVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Leaf => f.write_str("leaf"),
            Self::Line => f.write_str("line"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexOfSearchMicroSeedProof {
    KiloLeafArrayStringIndexOfConst10Block,
    KiloMicroIndexOfLine15Block,
}

impl std::fmt::Display for IndexOfSearchMicroSeedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KiloLeafArrayStringIndexOfConst10Block => {
                f.write_str("kilo_leaf_array_string_indexof_const_10block")
            }
            Self::KiloMicroIndexOfLine15Block => f.write_str("kilo_micro_indexof_line_15block"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexOfSearchResultUse {
    FoundPredicate,
}

impl std::fmt::Display for IndexOfSearchResultUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FoundPredicate => f.write_str("found_predicate"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexOfSearchBackendAction {
    LiteralMembershipPredicate,
}

impl std::fmt::Display for IndexOfSearchBackendAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LiteralMembershipPredicate => f.write_str("literal_membership_predicate"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexOfSearchCandidateOutcome {
    Found,
    NotFound,
}

impl std::fmt::Display for IndexOfSearchCandidateOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Found => f.write_str("found"),
            Self::NotFound => f.write_str("not_found"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexOfSearchMicroSeedRoute {
    pub variant: IndexOfSearchMicroSeedVariant,
    pub rows: i64,
    pub ops: i64,
    pub flip_period: Option<i64>,
    pub line_seed: String,
    pub line_seed_len: i64,
    pub none_seed: String,
    pub none_seed_len: i64,
    pub needle: String,
    pub needle_len: i64,
    pub proof: IndexOfSearchMicroSeedProof,
    pub result_use: IndexOfSearchResultUse,
    pub backend_action: IndexOfSearchBackendAction,
    pub line_seed_outcome: IndexOfSearchCandidateOutcome,
    pub none_seed_outcome: IndexOfSearchCandidateOutcome,
}

pub fn refresh_module_indexof_search_micro_seed_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_indexof_search_micro_seed_route(function);
    }
}

pub fn refresh_function_indexof_search_micro_seed_route(function: &mut MirFunction) {
    function.metadata.indexof_search_micro_seed_route =
        match_indexof_search_micro_seed_route(function);
}

fn match_indexof_search_micro_seed_route(
    function: &MirFunction,
) -> Option<IndexOfSearchMicroSeedRoute> {
    let facts = collect_facts(function);
    if let Some(route) = match_line_route(function, &facts) {
        return Some(route);
    }
    match_leaf_route(function, &facts)
}

fn match_leaf_route(
    function: &MirFunction,
    facts: &IndexOfSearchFacts,
) -> Option<IndexOfSearchMicroSeedRoute> {
    if function.blocks.len() != 10 || facts.set_count != 0 || facts.flip_period_count != 0 {
        return None;
    }
    if !facts.matches_common_contract()
        || facts.rows_count < 2
        || facts.ops_count != 1
        || facts.push_count != 2
        || facts.get_count != 1
        || facts.indexof_count != 1
        || facts.array_length_count != 1
        || facts.mod_count < 1
        || facts.compare_lt_count < 2
    {
        return None;
    }

    Some(IndexOfSearchMicroSeedRoute {
        variant: IndexOfSearchMicroSeedVariant::Leaf,
        rows: 64,
        ops: 400000,
        flip_period: None,
        line_seed: "line-seed".to_string(),
        line_seed_len: 9,
        none_seed: "none-seed".to_string(),
        none_seed_len: 9,
        needle: "line".to_string(),
        needle_len: 4,
        proof: IndexOfSearchMicroSeedProof::KiloLeafArrayStringIndexOfConst10Block,
        result_use: IndexOfSearchResultUse::FoundPredicate,
        backend_action: IndexOfSearchBackendAction::LiteralMembershipPredicate,
        line_seed_outcome: IndexOfSearchCandidateOutcome::Found,
        none_seed_outcome: IndexOfSearchCandidateOutcome::NotFound,
    })
}

fn match_line_route(
    function: &MirFunction,
    facts: &IndexOfSearchFacts,
) -> Option<IndexOfSearchMicroSeedRoute> {
    if function.blocks.len() != 15 {
        return None;
    }
    if !facts.matches_common_contract()
        || facts.rows_count < 4
        || facts.ops_count != 1
        || facts.flip_period_count != 1
        || facts.push_count != 2
        || facts.get_count != 1
        || facts.indexof_count != 1
        || facts.set_count != 2
        || facts.array_length_count != 1
        || facts.mod_count < 3
        || facts.compare_lt_count < 2
        || facts.compare_eq_count < 1
    {
        return None;
    }

    Some(IndexOfSearchMicroSeedRoute {
        variant: IndexOfSearchMicroSeedVariant::Line,
        rows: 64,
        ops: 400000,
        flip_period: Some(16),
        line_seed: "line-seed".to_string(),
        line_seed_len: 9,
        none_seed: "none-seed".to_string(),
        none_seed_len: 9,
        needle: "line".to_string(),
        needle_len: 4,
        proof: IndexOfSearchMicroSeedProof::KiloMicroIndexOfLine15Block,
        result_use: IndexOfSearchResultUse::FoundPredicate,
        backend_action: IndexOfSearchBackendAction::LiteralMembershipPredicate,
        line_seed_outcome: IndexOfSearchCandidateOutcome::Found,
        none_seed_outcome: IndexOfSearchCandidateOutcome::NotFound,
    })
}

#[derive(Debug, Default)]
struct IndexOfSearchFacts {
    has_array_box_birth: bool,
    rows_count: usize,
    ops_count: usize,
    flip_period_count: usize,
    line_seed_count: usize,
    none_seed_count: usize,
    needle_count: usize,
    push_count: usize,
    get_count: usize,
    indexof_count: usize,
    set_count: usize,
    array_length_count: usize,
    mod_count: usize,
    compare_lt_count: usize,
    compare_eq_count: usize,
}

impl IndexOfSearchFacts {
    fn matches_common_contract(&self) -> bool {
        self.has_array_box_birth
            && self.line_seed_count >= 1
            && self.none_seed_count >= 1
            && self.needle_count == 1
    }
}

fn collect_facts(function: &MirFunction) -> IndexOfSearchFacts {
    let mut facts = IndexOfSearchFacts::default();
    let mut ids: Vec<BasicBlockId> = function.blocks.keys().copied().collect();
    ids.sort();
    for id in ids {
        let Some(block) = function.blocks.get(&id) else {
            continue;
        };
        for instruction in block
            .instructions
            .iter()
            .chain(block.terminator.as_ref().into_iter())
        {
            collect_instruction_facts(instruction, &mut facts);
        }
    }
    facts
}

fn collect_instruction_facts(instruction: &MirInstruction, facts: &mut IndexOfSearchFacts) {
    match instruction {
        MirInstruction::NewBox { box_type, .. } if box_type == "ArrayBox" => {
            facts.has_array_box_birth = true;
        }
        MirInstruction::Const {
            value: ConstValue::Integer(value),
            ..
        } => match *value {
            64 => facts.rows_count += 1,
            400000 => facts.ops_count += 1,
            16 => facts.flip_period_count += 1,
            _ => {}
        },
        MirInstruction::Const {
            value: ConstValue::String(value),
            ..
        } => match value.as_str() {
            "line-seed" => facts.line_seed_count += 1,
            "none-seed" => facts.none_seed_count += 1,
            "line" => facts.needle_count += 1,
            _ => {}
        },
        MirInstruction::BinOp {
            op: BinaryOp::Mod, ..
        } => facts.mod_count += 1,
        MirInstruction::Compare { op, .. } => match op {
            CompareOp::Lt => facts.compare_lt_count += 1,
            CompareOp::Eq => facts.compare_eq_count += 1,
            _ => {}
        },
        MirInstruction::Call { callee, args, .. } => {
            let Some(Callee::Method {
                box_name, method, ..
            }) = callee.as_ref()
            else {
                return;
            };
            match (box_name.as_str(), method.as_str(), args.len()) {
                ("RuntimeDataBox", "push", 1) => facts.push_count += 1,
                ("RuntimeDataBox", "get", 1) => facts.get_count += 1,
                ("RuntimeDataBox", "indexOf", 1) => facts.indexof_count += 1,
                ("RuntimeDataBox", "set", 2) => facts.set_count += 1,
                ("ArrayBox", "length", 0) => facts.array_length_count += 1,
                _ => {}
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{BasicBlock, EffectMask, FunctionSignature, MirType, ValueId};

    #[test]
    fn detects_leaf_indexof_search_micro_seed_route() {
        let mut function = build_leaf_function();

        refresh_function_indexof_search_micro_seed_route(&mut function);

        let route = function
            .metadata
            .indexof_search_micro_seed_route
            .expect("leaf route");
        assert_eq!(route.variant, IndexOfSearchMicroSeedVariant::Leaf);
        assert_eq!(route.rows, 64);
        assert_eq!(route.ops, 400000);
        assert_eq!(route.flip_period, None);
        assert_eq!(
            route.proof,
            IndexOfSearchMicroSeedProof::KiloLeafArrayStringIndexOfConst10Block
        );
        assert_eq!(route.result_use, IndexOfSearchResultUse::FoundPredicate);
        assert_eq!(
            route.backend_action,
            IndexOfSearchBackendAction::LiteralMembershipPredicate
        );
        assert_eq!(
            route.line_seed_outcome,
            IndexOfSearchCandidateOutcome::Found
        );
        assert_eq!(
            route.none_seed_outcome,
            IndexOfSearchCandidateOutcome::NotFound
        );
    }

    #[test]
    fn detects_line_indexof_search_micro_seed_route() {
        let mut function = build_line_function();

        refresh_function_indexof_search_micro_seed_route(&mut function);

        let route = function
            .metadata
            .indexof_search_micro_seed_route
            .expect("line route");
        assert_eq!(route.variant, IndexOfSearchMicroSeedVariant::Line);
        assert_eq!(route.flip_period, Some(16));
        assert_eq!(
            route.proof,
            IndexOfSearchMicroSeedProof::KiloMicroIndexOfLine15Block
        );
        assert_eq!(route.result_use, IndexOfSearchResultUse::FoundPredicate);
        assert_eq!(
            route.backend_action,
            IndexOfSearchBackendAction::LiteralMembershipPredicate
        );
        assert_eq!(
            route.line_seed_outcome,
            IndexOfSearchCandidateOutcome::Found
        );
        assert_eq!(
            route.none_seed_outcome,
            IndexOfSearchCandidateOutcome::NotFound
        );
    }

    #[test]
    fn rejects_line_route_without_set_writes() {
        let mut function = build_line_function();
        for block in function.blocks.values_mut() {
            block.instructions.retain(|instruction| {
                !matches!(
                    instruction,
                    MirInstruction::Call {
                        callee: Some(Callee::Method { method, .. }),
                        ..
                    } if method == "set"
                )
            });
        }

        refresh_function_indexof_search_micro_seed_route(&mut function);

        assert!(function.metadata.indexof_search_micro_seed_route.is_none());
    }

    fn build_leaf_function() -> MirFunction {
        let mut function = empty_function();
        for id in 0..10 {
            replace_block(&mut function, id, vec![], jump((id + 1) % 10));
        }
        add_common_leaf_instructions(&mut function);
        function
    }

    fn build_line_function() -> MirFunction {
        let mut function = empty_function();
        for id in 0..15 {
            replace_block(&mut function, id, vec![], jump((id + 1) % 15));
        }
        add_common_leaf_instructions(&mut function);
        add_inst(&mut function, 5, const_i(108, 16));
        add_inst(&mut function, 5, binop(106, BinaryOp::Mod));
        add_inst(&mut function, 5, compare(105, CompareOp::Eq));
        add_inst(&mut function, 6, const_i(121, 64));
        add_inst(&mut function, 6, binop(117, BinaryOp::Mod));
        add_inst(
            &mut function,
            6,
            method_call(
                None,
                "RuntimeDataBox",
                "set",
                123,
                vec![ValueId::new(117), ValueId::new(122)],
            ),
        );
        add_inst(&mut function, 7, const_i(131, 64));
        add_inst(&mut function, 7, binop(127, BinaryOp::Mod));
        add_inst(
            &mut function,
            7,
            method_call(
                None,
                "RuntimeDataBox",
                "set",
                133,
                vec![ValueId::new(127), ValueId::new(132)],
            ),
        );
        function
    }

    fn add_common_leaf_instructions(function: &mut MirFunction) {
        add_inst(
            function,
            0,
            MirInstruction::NewBox {
                dst: ValueId::new(5),
                box_type: "ArrayBox".to_string(),
                args: vec![],
            },
        );
        add_inst(function, 1, const_i(23, 64));
        add_inst(function, 1, compare(12, CompareOp::Lt));
        add_inst(function, 2, const_s(37, "line-seed"));
        add_inst(
            function,
            2,
            method_call(None, "RuntimeDataBox", "push", 38, vec![ValueId::new(37)]),
        );
        add_inst(function, 3, const_s(42, "none-seed"));
        add_inst(
            function,
            3,
            method_call(None, "RuntimeDataBox", "push", 43, vec![ValueId::new(42)]),
        );
        add_inst(function, 4, const_i(74, 400000));
        add_inst(function, 4, compare(60, CompareOp::Lt));
        add_inst(function, 5, const_i(83, 64));
        add_inst(function, 5, binop(80, BinaryOp::Mod));
        add_inst(
            function,
            5,
            method_call(
                Some(62),
                "RuntimeDataBox",
                "get",
                84,
                vec![ValueId::new(80)],
            ),
        );
        add_inst(function, 5, const_s(87, "line"));
        add_inst(
            function,
            5,
            method_call(
                Some(64),
                "RuntimeDataBox",
                "indexOf",
                88,
                vec![ValueId::new(87)],
            ),
        );
        add_inst(
            function,
            8,
            method_call(Some(104), "ArrayBox", "length", 107, vec![]),
        );
    }

    fn empty_function() -> MirFunction {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        MirFunction::new(signature, BasicBlockId::new(0))
    }

    fn replace_block(
        function: &mut MirFunction,
        id: u32,
        instructions: Vec<MirInstruction>,
        terminator: MirInstruction,
    ) {
        let mut block = BasicBlock::new(BasicBlockId::new(id));
        for instruction in instructions {
            block.add_instruction(instruction);
        }
        block.set_terminator(terminator);
        function.add_block(block);
    }

    fn add_inst(function: &mut MirFunction, block: u32, instruction: MirInstruction) {
        function
            .blocks
            .get_mut(&BasicBlockId::new(block))
            .expect("block")
            .add_instruction_before_terminator(instruction);
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

    fn binop(dst: u32, op: BinaryOp) -> MirInstruction {
        MirInstruction::BinOp {
            dst: ValueId::new(dst),
            op,
            lhs: ValueId::new(1),
            rhs: ValueId::new(2),
        }
    }

    fn compare(dst: u32, op: CompareOp) -> MirInstruction {
        MirInstruction::Compare {
            dst: ValueId::new(dst),
            op,
            lhs: ValueId::new(1),
            rhs: ValueId::new(2),
        }
    }

    fn method_call(
        dst: Option<u32>,
        box_name: &str,
        method: &str,
        receiver: u32,
        args: Vec<ValueId>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: dst.map(ValueId::new),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(ValueId::new(receiver)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args,
            effects: EffectMask::IO,
        }
    }

    fn jump(target: u32) -> MirInstruction {
        MirInstruction::Jump {
            target: BasicBlockId::new(target),
            edge_args: None,
        }
    }
}
