/*!
 * MIR-owned route plans for extern call policy.
 *
 * Extern calls are not CoreMethodContract rows. This module keeps the narrow
 * extern-call backend contract in MIR metadata so ny-llvmc can consume an
 * explicit plan instead of classifying raw `env.*` strings in the C shim.
 */

use super::{BasicBlockId, Callee, MirFunction, MirInstruction, MirModule, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternCallRouteKind {
    EnvGet,
    EnvSet,
    HostBridgeExternInvoke,
}

impl ExternCallRouteKind {
    pub fn route_id(self) -> &'static str {
        match self {
            Self::EnvGet => "extern.env.get",
            Self::EnvSet => "extern.env.set",
            Self::HostBridgeExternInvoke => "extern.hostbridge.extern_invoke",
        }
    }

    pub fn core_op(self) -> &'static str {
        match self {
            Self::EnvGet => "EnvGet",
            Self::EnvSet => "EnvSet",
            Self::HostBridgeExternInvoke => "HostBridgeExternInvoke",
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            Self::EnvGet => "nyash.env.get",
            Self::EnvSet => "nyash.env.set",
            Self::HostBridgeExternInvoke => "nyash.hostbridge.extern_invoke",
        }
    }

    pub fn tier(self) -> &'static str {
        match self {
            Self::EnvGet => "ColdRuntime",
            Self::EnvSet => "ColdRuntime",
            Self::HostBridgeExternInvoke => "ColdRuntime",
        }
    }

    pub fn emit_kind(self) -> &'static str {
        match self {
            Self::EnvGet => "runtime_call",
            Self::EnvSet => "runtime_call",
            Self::HostBridgeExternInvoke => "runtime_call",
        }
    }

    pub fn proof(self) -> &'static str {
        match self {
            Self::EnvGet => "extern_registry",
            Self::EnvSet => "extern_registry",
            Self::HostBridgeExternInvoke => "extern_registry",
        }
    }

    pub fn return_shape(self) -> &'static str {
        match self {
            Self::EnvGet => "string_handle_or_null",
            Self::EnvSet => "scalar_i64",
            Self::HostBridgeExternInvoke => "string_handle_or_null",
        }
    }

    pub fn value_demand(self) -> &'static str {
        match self {
            Self::EnvGet => "runtime_i64_or_handle",
            Self::EnvSet => "runtime_i64",
            Self::HostBridgeExternInvoke => "runtime_i64_or_handle",
        }
    }

    pub fn effect_tags(self) -> &'static [&'static str] {
        match self {
            Self::EnvGet => &["read.env"],
            Self::EnvSet => &["write.env"],
            Self::HostBridgeExternInvoke => &["hostbridge.extern"],
        }
    }
}

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

    pub fn emit_kind(&self) -> &'static str {
        self.kind.emit_kind()
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

    pub fn arity(&self) -> usize {
        match self.kind {
            ExternCallRouteKind::EnvGet => 1,
            ExternCallRouteKind::EnvSet => 2,
            ExternCallRouteKind::HostBridgeExternInvoke => 3,
        }
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

pub fn normalize_extern_symbol(name: &str) -> &str {
    name.strip_suffix("/1")
        .or_else(|| name.strip_suffix("/2"))
        .or_else(|| name.strip_suffix("/3"))
        .unwrap_or(name)
}

pub fn classify_extern_call_route(name: &str, argc: usize) -> Option<ExternCallRouteKind> {
    match (normalize_extern_symbol(name), argc) {
        ("env.get", 1) | ("nyash.env.get", 1) => Some(ExternCallRouteKind::EnvGet),
        ("env.set", 2) | ("nyash.env.set", 2) => Some(ExternCallRouteKind::EnvSet),
        ("hostbridge.extern_invoke", 3) => Some(ExternCallRouteKind::HostBridgeExternInvoke),
        _ => None,
    }
}

pub fn is_hostbridge_extern_invoke_symbol(name: &str, argc: usize) -> bool {
    classify_extern_call_route(name, argc) == Some(ExternCallRouteKind::HostBridgeExternInvoke)
}

pub fn refresh_module_extern_call_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_extern_call_routes(function);
    }
}

pub fn refresh_function_extern_call_routes(function: &mut MirFunction) {
    let mut routes = Vec::new();
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, instruction) in block.instructions.iter().enumerate() {
            let MirInstruction::Call {
                dst: Some(dst),
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
            let Some(key_value) = args.first().copied() else {
                continue;
            };
            let value_value = match kind {
                ExternCallRouteKind::EnvGet => None,
                ExternCallRouteKind::EnvSet => args.get(1).copied(),
                ExternCallRouteKind::HostBridgeExternInvoke => args.get(2).copied(),
            };
            routes.push(ExternCallRoute::new(
                ExternCallRouteSite::new(block_id, instruction_index),
                kind,
                name,
                key_value,
                value_value,
                *dst,
            ));
        }
    }

    function.metadata.extern_call_routes = routes;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirType, ValueId};

    fn make_function_with_call(
        callee: &str,
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
            value: ConstValue::String("STAGE1_SOURCE_TEXT".to_string()),
        });
        block.instructions.push(MirInstruction::Call {
            dst,
            func: ValueId::INVALID,
            callee: Some(Callee::Extern(callee.to_string())),
            args,
            effects: EffectMask::PURE,
        });
        function.blocks.insert(BasicBlockId::new(0), block);
        function
    }

    #[test]
    fn refresh_function_extern_call_routes_records_env_get_plan_source() {
        let mut function =
            make_function_with_call("env.get/1", vec![ValueId::new(1)], Some(ValueId::new(2)));

        refresh_function_extern_call_routes(&mut function);

        assert_eq!(function.metadata.extern_call_routes.len(), 1);
        let route = &function.metadata.extern_call_routes[0];
        assert_eq!(route.route_id(), "extern.env.get");
        assert_eq!(route.core_op(), "EnvGet");
        assert_eq!(route.symbol(), "nyash.env.get");
        assert_eq!(route.tier(), "ColdRuntime");
        assert_eq!(route.emit_kind(), "runtime_call");
        assert_eq!(route.proof(), "extern_registry");
        assert_eq!(route.source_symbol(), "env.get/1");
        assert_eq!(route.key_value(), ValueId::new(1));
        assert_eq!(route.value_value(), None);
        assert_eq!(route.result_value(), ValueId::new(2));
        assert_eq!(route.arity(), 1);
        assert_eq!(route.return_shape(), "string_handle_or_null");
        assert_eq!(route.value_demand(), "runtime_i64_or_handle");
        assert_eq!(route.effect_tags(), &["read.env"]);
    }

    #[test]
    fn refresh_function_extern_call_routes_records_env_set_plan_source() {
        let mut function = make_function_with_call(
            "env.set/2",
            vec![ValueId::new(1), ValueId::new(2)],
            Some(ValueId::new(3)),
        );

        refresh_function_extern_call_routes(&mut function);

        assert_eq!(function.metadata.extern_call_routes.len(), 1);
        let route = &function.metadata.extern_call_routes[0];
        assert_eq!(route.route_id(), "extern.env.set");
        assert_eq!(route.core_op(), "EnvSet");
        assert_eq!(route.symbol(), "nyash.env.set");
        assert_eq!(route.tier(), "ColdRuntime");
        assert_eq!(route.emit_kind(), "runtime_call");
        assert_eq!(route.proof(), "extern_registry");
        assert_eq!(route.source_symbol(), "env.set/2");
        assert_eq!(route.key_value(), ValueId::new(1));
        assert_eq!(route.value_value(), Some(ValueId::new(2)));
        assert_eq!(route.result_value(), ValueId::new(3));
        assert_eq!(route.arity(), 2);
        assert_eq!(route.return_shape(), "scalar_i64");
        assert_eq!(route.value_demand(), "runtime_i64");
        assert_eq!(route.effect_tags(), &["write.env"]);
    }

    #[test]
    fn refresh_function_extern_call_routes_records_hostbridge_extern_invoke_global_source() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Unknown,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("hostbridge.extern_invoke/3".to_string())),
            args: vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)],
            effects: EffectMask::IO,
        });
        function.blocks.insert(BasicBlockId::new(0), block);

        refresh_function_extern_call_routes(&mut function);

        assert_eq!(function.metadata.extern_call_routes.len(), 1);
        let route = &function.metadata.extern_call_routes[0];
        assert_eq!(route.route_id(), "extern.hostbridge.extern_invoke");
        assert_eq!(route.core_op(), "HostBridgeExternInvoke");
        assert_eq!(route.symbol(), "nyash.hostbridge.extern_invoke");
        assert_eq!(route.tier(), "ColdRuntime");
        assert_eq!(route.emit_kind(), "runtime_call");
        assert_eq!(route.proof(), "extern_registry");
        assert_eq!(route.source_symbol(), "hostbridge.extern_invoke/3");
        assert_eq!(route.key_value(), ValueId::new(1));
        assert_eq!(route.value_value(), Some(ValueId::new(3)));
        assert_eq!(route.result_value(), ValueId::new(10));
        assert_eq!(route.arity(), 3);
        assert_eq!(route.return_shape(), "string_handle_or_null");
        assert_eq!(route.value_demand(), "runtime_i64_or_handle");
        assert_eq!(route.effect_tags(), &["hostbridge.extern"]);
    }

    #[test]
    fn refresh_function_extern_call_routes_requires_dst_and_matching_arity() {
        let mut missing_dst = make_function_with_call("env.get/1", vec![ValueId::new(1)], None);
        refresh_function_extern_call_routes(&mut missing_dst);
        assert!(missing_dst.metadata.extern_call_routes.is_empty());

        let mut missing_arg = make_function_with_call("env.get/1", vec![], Some(ValueId::new(2)));
        refresh_function_extern_call_routes(&mut missing_arg);
        assert!(missing_arg.metadata.extern_call_routes.is_empty());

        let mut missing_value =
            make_function_with_call("env.set/2", vec![ValueId::new(1)], Some(ValueId::new(2)));
        refresh_function_extern_call_routes(&mut missing_value);
        assert!(missing_value.metadata.extern_call_routes.is_empty());
    }
}
