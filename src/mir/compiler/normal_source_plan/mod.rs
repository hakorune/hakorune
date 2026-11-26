//! Normal source policy and compatibility origin.

mod compatibility_origin;
#[cfg(test)]
mod main_function_plan;
#[cfg(test)]
mod main_resolved_source;
#[cfg(test)]
mod main_source;
#[cfg(test)]
mod main_thunk_plan;
mod parser_bound_policy;
#[cfg(test)]
mod product;
mod rejection;
#[cfg(test)]
mod test_support;

pub(crate) use compatibility_origin::{
    NormalCallableCompatibilityOriginErrorV1, NormalCallableCompatibilityOriginV1,
};
#[cfg(test)]
pub(crate) use main_function_plan::{NormalMainFunctionPlanErrorV1, NormalMainFunctionPreflightV1};
#[cfg(test)]
pub(crate) use main_resolved_source::VerifiedNormalMainResolvedSourceUnitV1;
#[cfg(test)]
pub(crate) use main_source::VerifiedNormalMainFunctionSourceUnitV1;
#[cfg(test)]
pub(crate) use main_thunk_plan::{
    NormalMainThunkPlanErrorV1,
    VerifiedNormalMainEntryRelationV1, VerifiedNormalMainPhysicalRelationV1,
    VerifiedNormalMainThunkPlanV1, VerifiedNormalMainThunkResultV1,
};
pub(in crate::mir) use parser_bound_policy::require_static_main_v1;
pub(crate) use rejection::NormalSourcePlanErrorV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VerifiedNormalMainRoleV1 {
    _seal: VerifiedNormalMainRoleSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VerifiedNormalMainRoleSealV1;

#[cfg(test)]
impl VerifiedNormalMainRoleV1 {
    fn seal() -> Self {
        Self {
            _seal: VerifiedNormalMainRoleSealV1,
        }
    }
}
