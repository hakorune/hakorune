/*!
 * Function-level backend route tags for temporary exact seed bridges.
 *
 * Exact seed payload routes still own their detailed proof fields. This layer
 * only chooses which already-proven exact backend route the C boundary should
 * try first, so the function entry does not have to rediscover the ladder.
 */

use super::{MirFunction, MirModule};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExactSeedBackendRouteKind {
    ArrayStringStoreMicro,
}

impl ExactSeedBackendRouteKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ArrayStringStoreMicro => "array_string_store_micro",
        }
    }

    pub fn source_route_field(self) -> &'static str {
        match self {
            Self::ArrayStringStoreMicro => "array_string_store_micro_seed_route",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactSeedBackendRoute {
    pub tag: ExactSeedBackendRouteKind,
    pub source_route: String,
    pub proof: String,
}

pub fn refresh_module_exact_seed_backend_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_exact_seed_backend_route(function);
    }
}

pub fn refresh_function_exact_seed_backend_route(function: &mut MirFunction) {
    function.metadata.exact_seed_backend_route = match_exact_seed_backend_route(function);
}

fn match_exact_seed_backend_route(function: &MirFunction) -> Option<ExactSeedBackendRoute> {
    function
        .metadata
        .array_string_store_micro_seed_route
        .as_ref()
        .map(|route| ExactSeedBackendRoute {
            tag: ExactSeedBackendRouteKind::ArrayStringStoreMicro,
            source_route: ExactSeedBackendRouteKind::ArrayStringStoreMicro
                .source_route_field()
                .to_string(),
            proof: route.proof.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        ArrayStringStoreMicroSeedProof, ArrayStringStoreMicroSeedRoute, EffectMask,
        FunctionSignature, MirType,
    };
    use hakorune_mir_core::BasicBlockId;

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
    fn exact_seed_backend_route_selects_array_string_store_metadata() {
        let mut function = make_function();
        function.metadata.array_string_store_micro_seed_route =
            Some(ArrayStringStoreMicroSeedRoute {
                seed: "line-seed-abcdef".to_string(),
                seed_len: 16,
                size: 128,
                ops: 800000,
                suffix: "xy".to_string(),
                store_len: 18,
                next_text_window_start: 2,
                next_text_window_len: 16,
                proof: ArrayStringStoreMicroSeedProof::KiloMicroArrayStringStore8Block,
            });

        refresh_function_exact_seed_backend_route(&mut function);

        let route = function
            .metadata
            .exact_seed_backend_route
            .expect("exact seed backend route");
        assert_eq!(route.tag.as_str(), "array_string_store_micro");
        assert_eq!(route.source_route, "array_string_store_micro_seed_route");
        assert_eq!(route.proof, "kilo_micro_array_string_store_8block");
    }

    #[test]
    fn exact_seed_backend_route_stays_absent_without_seed_route() {
        let mut function = make_function();

        refresh_function_exact_seed_backend_route(&mut function);

        assert!(function.metadata.exact_seed_backend_route.is_none());
    }
}
