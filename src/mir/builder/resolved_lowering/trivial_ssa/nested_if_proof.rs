//! Nested Recipe proof adapter for the existing canonical If physicalizer.

use crate::mir::compiler::located::LocatedStmtV1;

use super::lowerer::CanonicalTrivialSsaLowererV1;
use crate::mir::builder::resolved_lowering::if_recipe_adapter::NestedIfNodeDemandV1;

pub(super) fn lower<'builder, 'source>(
    lowerer: &mut CanonicalTrivialSsaLowererV1<'builder, 'source>,
    statement: &LocatedStmtV1<'source>,
    demand: NestedIfNodeDemandV1,
) -> Result<(), String> {
    lowerer.lower_nested_if_recipe_selected(statement, demand.binding())
}
