use super::super::{BasicBlockId, Callee, MirFunction, MirInstruction, MirType, ValueId};
use super::route_spec::{
    classify_extern_call_route, is_hostbridge_extern_invoke_symbol, ExternCallRouteKind,
};
use crate::mir::route_value_type_publication::route_return_shape_value_type;
use std::collections::BTreeSet;

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
    pub fn kind(&self) -> ExternCallRouteKind {
        self.kind
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
    pub fn abi_return_shape(&self) -> &'static str {
        self.kind.return_shape()
    }
    pub fn bridge_encoding(&self) -> Option<&'static str> {
        (self.kind == ExternCallRouteKind::HakoMemFree).then_some("void_sentinel_i64_zero")
    }
    pub fn semantic_result_policy(&self) -> Option<&'static str> {
        crate::mir::extern_call_route_plan::extern_outcome_spec(self.kind).map(|spec| {
            match spec.result_policy {
                crate::mir::extern_call_route_plan::ExternResultPolicy::NoPayload => "NoPayload",
            }
        })
    }
    pub fn value_use_policy(&self) -> Option<&'static str> {
        crate::mir::extern_call_route_plan::extern_outcome_spec(self.kind).map(|spec| {
            match spec.value_use_policy {
                crate::mir::extern_call_route_plan::ExternValueUsePolicy::StatementOnly => {
                    "StatementOnly"
                }
            }
        })
    }
    pub fn required_capability(&self) -> Option<&'static str> {
        crate::mir::extern_call_route_plan::extern_outcome_backend_support(
            self.kind,
            crate::mir::extern_call_route_plan::ExternOutcomeConsumer::NyLlvmObject,
        )
        .map(|support| support.capability.as_str())
    }
    pub fn semantic_activation(&self) -> bool {
        crate::mir::extern_call_route_plan::extern_outcome_activation(self.kind).is_some()
    }
    pub fn backend_support_consumer(&self) -> Option<&'static str> {
        crate::mir::extern_call_route_plan::extern_outcome_backend_support(
            self.kind,
            crate::mir::extern_call_route_plan::ExternOutcomeConsumer::NyLlvmObject,
        )
        .map(|support| support.consumer.as_str())
    }
    pub fn effect_tags(&self) -> &'static [&'static str] {
        self.kind.effect_tags()
    }
}

pub(super) fn refresh_function_extern_call_routes(function: &mut MirFunction) {
    let mut routes = Vec::new();
    let used_values = function
        .blocks
        .values()
        .flat_map(|block| {
            block
                .instructions
                .iter()
                .chain(block.terminator.iter())
                .flat_map(|instruction| instruction.used_values())
        })
        .collect::<BTreeSet<_>>();
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for block_id in block_ids {
        let Some(block) = function.blocks.get_mut(&block_id) else {
            continue;
        };
        for (instruction_index, instruction) in block.instructions.iter_mut().enumerate() {
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
                Callee::Extern(name) => name.as_str(),
                Callee::Global(name) if is_hostbridge_extern_invoke_symbol(name, args.len()) => {
                    name.as_str()
                }
                _ => continue,
            };
            let Some(kind) = classify_extern_call_route(name, args.len()) else {
                continue;
            };
            if kind == ExternCallRouteKind::HakoMemFree {
                if let Some(result) = *dst {
                    if !used_values.contains(&result) {
                        *dst = None;
                    }
                }
            }
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
        BasicBlock, ConstValue, EffectMask, FunctionSignature, MirInstruction, MirModule, MirType,
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
    fn hako_mem_free_no_payload_does_not_publish_integer_type() {
        let mut function = function_with_extern_call(
            "hako_mem_free/1",
            vec![ValueId::new(1)],
            Some(ValueId::new(2)),
        );
        refresh_function_extern_call_routes(&mut function);
        let route = &function.metadata.extern_call_routes[0];
        assert_eq!(route.abi_return_shape(), "c_void");
        assert_eq!(route.bridge_encoding(), Some("void_sentinel_i64_zero"));
        assert_eq!(route.semantic_result_policy(), Some("NoPayload"));
        assert_eq!(route.value_use_policy(), Some("StatementOnly"));
        assert_eq!(
            route.required_capability(),
            Some("extern_unit_no_payload_hako_mem_free_v1")
        );
        assert!(route.semantic_activation());
        assert_eq!(route.backend_support_consumer(), Some("ny-llvmc-object"));
        assert_eq!(route.result_value_opt(), None);
        assert_eq!(function.metadata.value_types.get(&ValueId::new(2)), None);
    }

    #[test]
    fn hako_mem_free_result_use_is_rejected_at_contract_boundary() {
        let mut function = function_with_extern_call(
            "hako_mem_free/1",
            vec![ValueId::new(1)],
            Some(ValueId::new(2)),
        );
        function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("test block")
            .set_terminator(MirInstruction::Return {
                value: Some(ValueId::new(2)),
            });
        refresh_function_extern_call_routes(&mut function);
        let mut module = MirModule::new("hako_mem_free_result_use".to_string());
        module.add_function(function);
        let error = crate::mir::extern_call_route_plan::validate_semantic_outcome_routes(&module)
            .expect_err("direct hako_mem_free result use must reject");
        assert!(error.contains("[failure/outcome_unit_result_value_present]"));
        assert!(error.contains("result_value=%2"));
    }

    #[test]
    fn hako_mem_free_statement_route_passes_result_use_guard() {
        let mut function =
            function_with_extern_call("hako_mem_free/1", vec![ValueId::new(1)], None);
        refresh_function_extern_call_routes(&mut function);
        let mut module = MirModule::new("hako_mem_free_statement".to_string());
        module.add_function(function);
        assert!(
            crate::mir::extern_call_route_plan::validate_semantic_outcome_routes(&module).is_ok()
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

    #[test]
    fn decoded_utf8_byte_len_v0_route_publishes_the_internal_integer_contract() {
        let mut function = function_with_extern_call(
            "hako.analysis.decoded_utf8_byte_len_v0",
            vec![ValueId::new(1)],
            Some(ValueId::new(2)),
        );

        refresh_function_extern_call_routes(&mut function);

        assert_eq!(function.metadata.extern_call_routes.len(), 1);
        let route = &function.metadata.extern_call_routes[0];
        assert_eq!(
            route.kind(),
            ExternCallRouteKind::HakoAnalysisDecodedUtf8ByteLenV0
        );
        assert_eq!(
            route.route_id(),
            "extern.hako.analysis.decoded_utf8_byte_len_v0"
        );
        assert_eq!(route.core_op(), "HakoAnalysisDecodedUtf8ByteLenV0");
        assert_eq!(route.symbol(), "hako.analysis.decoded_utf8_byte_len_v0");
        assert_eq!(route.arity(), 1);
        assert_eq!(route.return_shape(), "scalar_i64");
        assert_eq!(route.value_demand(), "string_handle");
        assert_eq!(route.effect_tags(), &["analysis.decoded_utf8_byte_len_v0"]);
        assert_eq!(
            function.metadata.value_types.get(&ValueId::new(2)),
            Some(&MirType::Integer)
        );
    }
}
