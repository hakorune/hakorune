/*!
 * MIR-owned route plans for generic method policy.
 *
 * This module owns narrow generic method route-policy decisions so `.inc`
 * codegen can consume pre-decided route tags instead of classifying method
 * surfaces from backend-local strings.
 */

use super::{BasicBlockId, Callee, MirFunction, MirInstruction, MirModule, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericMethodRouteKind {
    RuntimeDataContainsAny,
    MapContainsAny,
}

impl GenericMethodRouteKind {
    pub fn helper_symbol(self) -> &'static str {
        match self {
            Self::RuntimeDataContainsAny => "nyash.runtime_data.has_hh",
            Self::MapContainsAny => "nyash.map.probe_hh",
        }
    }
}

impl std::fmt::Display for GenericMethodRouteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RuntimeDataContainsAny => f.write_str("runtime_data_contains_any"),
            Self::MapContainsAny => f.write_str("map_contains_any"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericMethodRouteProof {
    HasSurfacePolicy,
}

impl std::fmt::Display for GenericMethodRouteProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HasSurfacePolicy => f.write_str("has_surface_policy"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericMethodRoute {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub box_name: String,
    pub method: String,
    pub receiver_value: ValueId,
    pub key_value: ValueId,
    pub result_value: Option<ValueId>,
    pub route_kind: GenericMethodRouteKind,
    pub proof: GenericMethodRouteProof,
}

pub fn refresh_module_generic_method_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_generic_method_routes(function);
    }
}

pub fn refresh_function_generic_method_routes(function: &mut MirFunction) {
    let mut routes = Vec::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            if let Some(route) = match_generic_has_route(block_id, instruction_index, inst) {
                routes.push(route);
            }
        }
    }

    routes.sort_by_key(|route| (route.block.as_u32(), route.instruction_index));
    function.metadata.generic_method_routes = routes;
}

fn match_generic_has_route(
    block: BasicBlockId,
    instruction_index: usize,
    inst: &MirInstruction,
) -> Option<GenericMethodRoute> {
    let MirInstruction::Call {
        dst,
        callee:
            Some(Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                ..
            }),
        args,
        ..
    } = inst
    else {
        return None;
    };
    if method != "has" || args.len() != 1 {
        return None;
    }

    let route_kind = match box_name.as_str() {
        "MapBox" => GenericMethodRouteKind::MapContainsAny,
        "ArrayBox" | "RuntimeDataBox" => GenericMethodRouteKind::RuntimeDataContainsAny,
        _ => return None,
    };

    Some(GenericMethodRoute {
        block,
        instruction_index,
        box_name: box_name.clone(),
        method: method.clone(),
        receiver_value: *receiver,
        key_value: args[0],
        result_value: *dst,
        route_kind,
        proof: GenericMethodRouteProof::HasSurfacePolicy,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirType};

    fn method_call(
        dst: Option<u32>,
        box_name: &str,
        method: &str,
        receiver: u32,
        args: Vec<u32>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: dst.map(ValueId::new),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(ValueId::new(receiver)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: args.into_iter().map(ValueId::new).collect(),
            effects: EffectMask::PURE,
        }
    }

    fn make_function() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn detects_mapbox_has_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block
            .instructions
            .push(method_call(Some(3), "MapBox", "has", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.block, BasicBlockId::new(0));
        assert_eq!(route.instruction_index, 0);
        assert_eq!(route.box_name, "MapBox");
        assert_eq!(route.method, "has");
        assert_eq!(route.receiver_value, ValueId::new(1));
        assert_eq!(route.key_value, ValueId::new(2));
        assert_eq!(route.result_value, Some(ValueId::new(3)));
        assert_eq!(route.route_kind, GenericMethodRouteKind::MapContainsAny);
        assert_eq!(route.proof, GenericMethodRouteProof::HasSurfacePolicy);
    }

    #[test]
    fn detects_runtime_data_has_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block
            .instructions
            .push(method_call(Some(3), "RuntimeDataBox", "has", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        assert_eq!(
            function.metadata.generic_method_routes[0].route_kind,
            GenericMethodRouteKind::RuntimeDataContainsAny
        );
    }

    #[test]
    fn rejects_non_has_methods() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block
            .instructions
            .push(method_call(Some(3), "MapBox", "get", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert!(function.metadata.generic_method_routes.is_empty());
    }
}
