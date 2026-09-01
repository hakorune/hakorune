use crate::mir::core_method_op::{LoweringPlanEmitKind, LoweringPlanTier};

use super::{ExternCallRouteKind, ExternCallRouteSpec, EXTERN_CALL_ROUTE_SPECS};

impl ExternCallRouteKind {
    pub fn spec(self) -> &'static ExternCallRouteSpec {
        EXTERN_CALL_ROUTE_SPECS
            .iter()
            .find(|spec| spec.kind == self)
            .expect("extern call route kind must have a spec")
    }

    pub fn route_id(self) -> &'static str {
        self.spec().route_id()
    }
    pub fn core_op(self) -> &'static str {
        self.spec().core_op()
    }
    pub fn symbol(self) -> &'static str {
        self.spec().symbol()
    }
    pub fn lowering_tier(self) -> LoweringPlanTier {
        LoweringPlanTier::ColdRuntime
    }
    pub fn tier(self) -> &'static str {
        self.lowering_tier().as_json_name()
    }
    pub fn lowering_emit_kind(self) -> LoweringPlanEmitKind {
        LoweringPlanEmitKind::RuntimeCall
    }
    pub fn emit_kind(self) -> &'static str {
        self.lowering_emit_kind().as_json_name()
    }
    pub fn proof(self) -> &'static str {
        self.spec().proof()
    }
    pub fn return_shape(self) -> &'static str {
        self.spec().return_shape()
    }
    pub fn value_demand(self) -> &'static str {
        self.spec().value_demand()
    }
    pub fn effect_tags(self) -> &'static [&'static str] {
        self.spec().effect_tags()
    }
    pub fn arity(self) -> usize {
        self.spec().arity()
    }
    pub fn value_arg_index(self) -> Option<usize> {
        self.spec().value_arg_index()
    }
    pub fn accepts_void_result(self) -> bool {
        self.spec().accepts_void_result()
    }
}

pub fn normalize_extern_symbol(name: &str) -> &str {
    name.strip_suffix("/0")
        .or_else(|| name.strip_suffix("/1"))
        .or_else(|| name.strip_suffix("/2"))
        .or_else(|| name.strip_suffix("/3"))
        .or_else(|| name.strip_suffix("/4"))
        .or_else(|| name.strip_suffix("/5"))
        .unwrap_or(name)
}

pub fn classify_extern_call_route(name: &str, argc: usize) -> Option<ExternCallRouteKind> {
    let normalized = normalize_extern_symbol(name);
    EXTERN_CALL_ROUTE_SPECS
        .iter()
        .find(|spec| spec.arity() == argc && spec.accepts_symbol(normalized))
        .map(ExternCallRouteSpec::kind)
}

pub fn is_hostbridge_extern_invoke_symbol(name: &str, argc: usize) -> bool {
    classify_extern_call_route(name, argc) == Some(ExternCallRouteKind::HostBridgeExternInvoke)
}
