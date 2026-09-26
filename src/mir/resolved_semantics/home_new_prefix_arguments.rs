//! Observation-preserving sibling of `issue_new_home_prefixes_v1`.
//!
//! Same single source walk, but the caller keeps the selected-New argument
//! observations the walk already computes and installs the caller's declared
//! parameter contracts. Declared contracts are the only installed entry
//! bindings — nothing is invented.

use super::{
    scan_new_home_flow, CallerNewHomePrefixV1, HomePrefixUnavailableV1,
    SelectedNewArgumentObservationV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1};
use std::collections::BTreeMap;

pub(crate) fn issue_new_home_prefixes_with_arguments_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    selected: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
    parameters: impl IntoIterator<
        Item = (
            u32,
            BindingRefV1,
            crate::mir::callable_parameter_contract::CallableParameterContractKindV1,
        ),
    >,
) -> (
    BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>,
    BTreeMap<OwnedExprSiteV1, SelectedNewArgumentObservationV1>,
) {
    let (prefixes, _, _, observations) = scan_new_home_flow(
        input,
        selected,
        parameters,
        None,
        &mut |_, _, _, _, _| Ok::<_, std::convert::Infallible>(false),
        &mut |_, _| Ok(false),
        &mut |_| Ok(false),
        &mut |_| Ok(false),
    )
    .unwrap_or_else(|never| match never {});
    (prefixes, observations)
}
