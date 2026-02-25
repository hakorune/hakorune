/*!
 * Stage-1 bridge route plan selector.
 *
 * Centralizes "direct route vs stage1 stub route" decision in one place.
 */

use super::args::Stage1Args;
use crate::config::env::stage1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Stage1BridgeRoute {
    BinaryOnlyEmitMirDirect,
    BinaryOnlyRunDirect,
    Stage1Stub,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Stage1BridgePlan {
    pub(super) route: Stage1BridgeRoute,
    pub(super) reason: &'static str,
}

fn has_run_backend_hint(args_result: &Stage1Args) -> bool {
    if args_result.args.len() < 3 {
        return false;
    }
    args_result.args[0] == "run" && args_result.args[1] == "--backend"
}

fn decide_with_flags(
    args_result: &Stage1Args,
    emit_direct_enabled: bool,
    run_direct_enabled: bool,
) -> Stage1BridgePlan {
    if args_result.emit_mir && emit_direct_enabled {
        return Stage1BridgePlan {
            route: Stage1BridgeRoute::BinaryOnlyEmitMirDirect,
            reason: "explicit:NYASH_STAGE1_BINARY_ONLY_DIRECT=1",
        };
    }

    if !args_result.emit_mir && has_run_backend_hint(args_result) && run_direct_enabled {
        return Stage1BridgePlan {
            route: Stage1BridgeRoute::BinaryOnlyRunDirect,
            reason: "explicit:NYASH_STAGE1_BINARY_ONLY_RUN_DIRECT=1",
        };
    }

    Stage1BridgePlan {
        route: Stage1BridgeRoute::Stage1Stub,
        reason: "default:stage1-stub",
    }
}

pub(super) fn decide(args_result: &Stage1Args) -> Stage1BridgePlan {
    decide_with_flags(
        args_result,
        stage1::binary_only_emit_direct_enabled(),
        stage1::binary_only_run_direct_enabled(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_args(args: &[&str], emit_mir: bool) -> Stage1Args {
        Stage1Args {
            args: args.iter().map(|s| s.to_string()).collect(),
            env_script_args: None,
            source_env: None,
            progjson_env: None,
            emit_mir,
        }
    }

    #[test]
    fn emit_direct_is_selected_only_when_explicitly_enabled() {
        let args = make_args(&["emit", "mir-json", "foo.hako"], true);
        let plan = decide_with_flags(&args, true, false);
        assert_eq!(plan.route, Stage1BridgeRoute::BinaryOnlyEmitMirDirect);
    }

    #[test]
    fn run_direct_requires_run_shape_and_explicit_flag() {
        let args = make_args(&["run", "--backend", "vm", "foo.hako"], false);
        let plan = decide_with_flags(&args, false, true);
        assert_eq!(plan.route, Stage1BridgeRoute::BinaryOnlyRunDirect);
    }

    #[test]
    fn fallback_route_is_stage1_stub_by_default() {
        let args = make_args(&["run", "--backend", "vm", "foo.hako"], false);
        let plan = decide_with_flags(&args, false, false);
        assert_eq!(plan.route, Stage1BridgeRoute::Stage1Stub);
    }
}
