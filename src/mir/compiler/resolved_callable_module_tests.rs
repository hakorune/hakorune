use std::collections::BTreeMap;

use crate::mir::resolved_semantics::{
    CanonicalCallableKeyV1, SourceCallableDeclarationSiteV1, VerifiedCallableCatalogSourceUnitV1,
    VerifiedSemanticOwnerForestV1,
};

use super::resolved_callable_module::{
    VerifiedResolvedCallableModuleV1, VerifiedResolvedFunctionUnitV1,
};
use super::source_projection::VerifiedSourceProjectionV1;

#[test]
fn passive_module_carrier_exposes_only_the_canonical_keyed_primary_map() {
    fn schema(
        module: &VerifiedResolvedCallableModuleV1,
    ) -> (
        &VerifiedCallableCatalogSourceUnitV1,
        &BTreeMap<CanonicalCallableKeyV1, VerifiedResolvedFunctionUnitV1>,
    ) {
        (module.source(), module.functions_by_key())
    }

    let _typed_schema = schema;
}

#[test]
fn passive_function_unit_keeps_site_forest_and_projection_together() {
    fn schema(
        unit: &VerifiedResolvedFunctionUnitV1,
    ) -> (
        SourceCallableDeclarationSiteV1,
        &VerifiedSemanticOwnerForestV1,
        &VerifiedSourceProjectionV1,
    ) {
        (unit.declaration_site(), unit.forest(), unit.projection())
    }

    let _typed_schema = schema;
}
