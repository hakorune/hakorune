/*!
 * Stage-1 bridge route executor.
 *
 * Keeps route-to-executor dispatch thin while delegating direct-route and
 * stage1-stub execution policy to focused helpers.
 */

mod direct;
mod stub;

use super::args::Stage1Args;
use super::plan::Stage1BridgePlan;
use super::NyashRunner;
use crate::cli::CliGroups;

pub(super) fn execute(
    runner: &NyashRunner,
    groups: &CliGroups,
    args_result: &Stage1Args,
    route_plan: Stage1BridgePlan,
) -> i32 {
    match route_plan {
        Stage1BridgePlan::BinaryOnlyEmitMirDirect { .. } => {
            direct::execute_emit_mir(runner, groups, route_plan.reason())
        }
        Stage1BridgePlan::BinaryOnlyRunDirect { .. } => {
            direct::execute_run(runner, groups, route_plan.reason())
        }
        Stage1BridgePlan::Stage1Stub { .. } => stub::execute(groups, args_result),
    }
}
