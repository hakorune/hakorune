use crate::mir::exact_seed_backend_route::ExactSeedBackendRoute;
use serde_json::json;

pub(super) fn build_exact_seed_backend_route_json(
    route: &ExactSeedBackendRoute,
) -> serde_json::Value {
    json!({
        "tag": route.tag(),
        "source_route": route.source_route(),
        "proof": route.proof(),
        "selected_value": route.selected_value().map(|value| value.as_u32()),
    })
}
