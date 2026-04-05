/*!
 * Stage-1 bridge route plan selector.
 *
 * Centralizes "direct route vs stage1 stub route" decision in one place.
 */

use super::args::{Stage1Args, Stage1ArgsMode};
use crate::config::env::stage1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Stage1BridgePlan {
    BinaryOnlyEmitMirDirect { reason: &'static str },
    BinaryOnlyRunDirect { reason: &'static str },
    Stage1Stub { reason: &'static str },
}

impl Stage1BridgePlan {
    pub(super) fn reason(self) -> &'static str {
        match self {
            Self::BinaryOnlyEmitMirDirect { reason }
            | Self::BinaryOnlyRunDirect { reason }
            | Self::Stage1Stub { reason } => reason,
        }
    }
}

fn decide_with_flags(
    args_result: &Stage1Args,
    emit_direct_enabled: bool,
    run_direct_enabled: bool,
) -> Stage1BridgePlan {
    if args_result.mode == Stage1ArgsMode::EmitMirJson && emit_direct_enabled {
        return Stage1BridgePlan::BinaryOnlyEmitMirDirect {
            reason: "explicit:NYASH_STAGE1_BINARY_ONLY_DIRECT=1",
        };
    }

    if args_result.mode == Stage1ArgsMode::Run && run_direct_enabled {
        return Stage1BridgePlan::BinaryOnlyRunDirect {
            reason: "explicit:NYASH_STAGE1_BINARY_ONLY_RUN_DIRECT=1",
        };
    }

    Stage1BridgePlan::Stage1Stub {
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

    fn make_args(args: &[&str], mode: Stage1ArgsMode) -> Stage1Args {
        Stage1Args {
            mode,
            args: args.iter().map(|s| s.to_string()).collect(),
            env_script_args: None,
            source_env: None,
            progjson_env: None,
        }
    }

    #[test]
    fn emit_direct_is_selected_only_when_explicitly_enabled() {
        let args = make_args(
            &["emit", "mir-json", "foo.hako"],
            Stage1ArgsMode::EmitMirJson,
        );
        let plan = decide_with_flags(&args, true, false);
        assert!(matches!(
            plan,
            Stage1BridgePlan::BinaryOnlyEmitMirDirect { .. }
        ));
        assert_eq!(plan.reason(), "explicit:NYASH_STAGE1_BINARY_ONLY_DIRECT=1");
    }

    #[test]
    fn run_direct_requires_run_shape_and_explicit_flag() {
        let args = make_args(&["run", "--backend", "vm", "foo.hako"], Stage1ArgsMode::Run);
        let plan = decide_with_flags(&args, false, true);
        assert!(matches!(plan, Stage1BridgePlan::BinaryOnlyRunDirect { .. }));
        assert_eq!(
            plan.reason(),
            "explicit:NYASH_STAGE1_BINARY_ONLY_RUN_DIRECT=1"
        );
    }

    #[test]
    fn run_direct_is_not_selected_for_emit_shapes() {
        let args = make_args(
            &["emit", "mir-json", "foo.hako"],
            Stage1ArgsMode::EmitMirJson,
        );
        let plan = decide_with_flags(&args, false, true);
        assert!(matches!(plan, Stage1BridgePlan::Stage1Stub { .. }));
        assert_eq!(plan.reason(), "default:stage1-stub");
    }

    #[test]
    fn fallback_route_is_stage1_stub_by_default() {
        let args = make_args(&["run", "--backend", "vm", "foo.hako"], Stage1ArgsMode::Run);
        let plan = decide_with_flags(&args, false, false);
        assert!(matches!(plan, Stage1BridgePlan::Stage1Stub { .. }));
        assert_eq!(plan.reason(), "default:stage1-stub");
    }
}
