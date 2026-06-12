use super::super::{BasicBlockId, Callee, MirFunction, MirInstruction, ValueId};
use super::route_spec::{
    classify_extern_call_route, is_hostbridge_extern_invoke_symbol, ExternCallRouteKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternCallRouteSite {
    block: BasicBlockId,
    instruction_index: usize,
}

impl ExternCallRouteSite {
    pub fn new(block: BasicBlockId, instruction_index: usize) -> Self {
        Self {
            block,
            instruction_index,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternCallRoute {
    site: ExternCallRouteSite,
    kind: ExternCallRouteKind,
    source_symbol: String,
    key_value: ValueId,
    value_value: Option<ValueId>,
    result_value: ValueId,
}

impl ExternCallRoute {
    pub fn new(
        site: ExternCallRouteSite,
        kind: ExternCallRouteKind,
        source_symbol: impl Into<String>,
        key_value: ValueId,
        value_value: Option<ValueId>,
        result_value: ValueId,
    ) -> Self {
        Self {
            site,
            kind,
            source_symbol: source_symbol.into(),
            key_value,
            value_value,
            result_value,
        }
    }

    pub fn block(&self) -> BasicBlockId {
        self.site.block
    }
    pub fn instruction_index(&self) -> usize {
        self.site.instruction_index
    }
    pub fn route_id(&self) -> &'static str {
        self.kind.route_id()
    }
    pub fn core_op(&self) -> &'static str {
        self.kind.core_op()
    }
    pub fn symbol(&self) -> &'static str {
        self.kind.symbol()
    }
    pub fn tier(&self) -> &'static str {
        self.kind.tier()
    }
    pub fn lowering_tier(&self) -> crate::mir::core_method_op::LoweringPlanTier {
        self.kind.lowering_tier()
    }
    pub fn emit_kind(&self) -> &'static str {
        self.kind.emit_kind()
    }
    pub fn lowering_emit_kind(&self) -> crate::mir::core_method_op::LoweringPlanEmitKind {
        self.kind.lowering_emit_kind()
    }
    pub fn proof(&self) -> &'static str {
        self.kind.proof()
    }
    pub fn source_symbol(&self) -> &str {
        &self.source_symbol
    }
    pub fn key_value(&self) -> ValueId {
        self.key_value
    }
    pub fn value_value(&self) -> Option<ValueId> {
        self.value_value
    }
    pub fn result_value(&self) -> ValueId {
        self.result_value
    }
    pub fn result_value_opt(&self) -> Option<ValueId> {
        if self.result_value == ValueId::INVALID {
            None
        } else {
            Some(self.result_value)
        }
    }
    pub fn arity(&self) -> usize {
        self.kind.arity()
    }
    pub fn return_shape(&self) -> &'static str {
        self.kind.return_shape()
    }
    pub fn value_demand(&self) -> &'static str {
        self.kind.value_demand()
    }
    pub fn effect_tags(&self) -> &'static [&'static str] {
        self.kind.effect_tags()
    }
}

pub(super) fn refresh_function_extern_call_routes(function: &mut MirFunction) {
    let mut routes = Vec::new();
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, instruction) in block.instructions.iter().enumerate() {
            let MirInstruction::Call {
                dst,
                callee: Some(callee),
                args,
                ..
            } = instruction
            else {
                continue;
            };
            let name = match callee {
                Callee::Extern(name) => name,
                Callee::Global(name) if is_hostbridge_extern_invoke_symbol(name, args.len()) => {
                    name
                }
                _ => continue,
            };
            let Some(kind) = classify_extern_call_route(name, args.len()) else {
                continue;
            };
            if dst.is_none() && !kind.accepts_void_result() {
                continue;
            }
            let key_value = args.first().copied().unwrap_or(ValueId::INVALID);
            let value_value = kind
                .value_arg_index()
                .and_then(|index| args.get(index).copied());
            routes.push(ExternCallRoute::new(
                ExternCallRouteSite::new(block_id, instruction_index),
                kind,
                name,
                key_value,
                value_value,
                dst.unwrap_or(ValueId::INVALID),
            ));
        }
    }

    function.metadata.extern_call_routes = routes;
}
