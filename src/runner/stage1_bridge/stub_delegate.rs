/*!
 * Stage-1 bridge stub delegate-status executor.
 *
 * Owns plain child-status execution, delegation log emission, and spawn-failure
 * mapping for the future-retire stage1 stub lane.
 */

use super::stub_child::PreparedStage1StubChild;

pub(super) fn run_status(mut prepared: PreparedStage1StubChild) -> i32 {
    crate::cli_v!(
        "[stage1-cli] delegating to stub: {}",
        prepared.trace_summary
    );

    match prepared.command.status() {
        Ok(status) => status.code().unwrap_or(1),
        Err(error) => {
            crate::runtime::get_global_ring0()
                .log
                .error(&format!("[stage1-cli] failed to spawn stub: {}", error));
            97
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::stub_child::PreparedStage1StubChild;
    use super::super::test_support;
    use super::run_status;
    use std::process::Command;

    #[test]
    fn run_status_returns_child_exit_code() {
        let mut command = Command::new("sh");
        command.arg("-c").arg("exit 7");
        let prepared = PreparedStage1StubChild {
            command,
            trace_summary: "sh -c exit 7".to_string(),
        };

        assert_eq!(run_status(prepared), 7);
    }

    #[test]
    fn run_status_maps_spawn_failure_to_97() {
        test_support::ensure_ring0_initialized();
        let prepared = PreparedStage1StubChild {
            command: Command::new("/definitely-missing-stage1-stub-binary"),
            trace_summary: "missing-binary".to_string(),
        };

        assert_eq!(run_status(prepared), 97);
    }
}
