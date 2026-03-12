/*!
 * Stage-1 bridge route executor - stage1 stub helper.
 */

use super::super::args::{Stage1Args, Stage1StubExecPlan};
use crate::cli::CliGroups;

pub(super) fn execute(groups: &CliGroups, args_result: &Stage1Args) -> i32 {
    let prepared = match super::super::stub_child::prepare_or_log(groups, args_result) {
        Ok(prepared) => prepared,
        Err(code) => return code,
    };

    match args_result.stub_exec_plan() {
        Stage1StubExecPlan::EmitCapture(mode) => {
            super::super::stub_emit::run_capture(prepared.command, groups, mode)
        }
        Stage1StubExecPlan::DelegateStatus => super::super::stub_delegate::run_status(prepared),
    }
}
