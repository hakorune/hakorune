use super::super::{BasicBlockId, Callee, MirFunction, MirInstruction, MirType, ValueId};
use super::route_spec::{
    classify_extern_call_route, is_hostbridge_extern_invoke_symbol, ExternCallRouteKind,
};
use crate::mir::route_value_type_publication::route_return_shape_value_type;

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
    publish_extern_call_route_result_value_types(function);
}

fn publish_extern_call_route_result_value_types(function: &mut MirFunction) -> bool {
    let facts = function
        .metadata
        .extern_call_routes
        .iter()
        .filter_map(|route| {
            Some((
                route.result_value_opt()?,
                route_return_shape_value_type(Some(route.return_shape()))?,
            ))
        })
        .collect::<Vec<_>>();

    let mut changed = false;
    for (value, ty) in facts {
        changed |= publish_value_type(function, value, ty);
    }
    changed
}

fn publish_value_type(function: &mut MirFunction, value: ValueId, ty: MirType) -> bool {
    match function.metadata.value_types.get(&value) {
        Some(existing) if existing == &ty => false,
        Some(MirType::Unknown) | None => {
            function.metadata.value_types.insert(value, ty);
            true
        }
        Some(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlock, ConstValue, EffectMask, FunctionSignature, MirInstruction, MirType,
    };

    fn function_with_extern_call(
        symbol: &str,
        args: Vec<ValueId>,
        dst: Option<ValueId>,
    ) -> MirFunction {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::String("KEY".to_string()),
        });
        block.instructions.push(MirInstruction::Call {
            dst,
            func: ValueId::INVALID,
            callee: Some(Callee::Extern(symbol.to_string())),
            args,
            effects: EffectMask::PURE,
        });
        function.blocks.insert(BasicBlockId::new(0), block);
        function
    }

    #[test]
    fn extern_call_return_shapes_publish_stable_value_types() {
        let mut string_function =
            function_with_extern_call("env.get/1", vec![ValueId::new(1)], Some(ValueId::new(2)));
        refresh_function_extern_call_routes(&mut string_function);
        assert_eq!(
            string_function.metadata.value_types.get(&ValueId::new(2)),
            Some(&MirType::Box("StringBox".to_string()))
        );

        let mut scalar_function = function_with_extern_call(
            "env.set/2",
            vec![ValueId::new(1), ValueId::new(2)],
            Some(ValueId::new(3)),
        );
        refresh_function_extern_call_routes(&mut scalar_function);
        assert_eq!(
            scalar_function.metadata.value_types.get(&ValueId::new(3)),
            Some(&MirType::Integer)
        );
    }

    #[test]
    fn extern_call_native_pointer_return_shape_stays_unpublished() {
        let mut function = function_with_extern_call(
            "hako_mem_alloc",
            vec![ValueId::new(1)],
            Some(ValueId::new(2)),
        );
        refresh_function_extern_call_routes(&mut function);

        assert_eq!(function.metadata.extern_call_routes.len(), 1);
        assert_eq!(
            function.metadata.extern_call_routes[0].return_shape(),
            "native_ptr_nullable"
        );
        assert_eq!(function.metadata.value_types.get(&ValueId::new(2)), None);
    }
}
