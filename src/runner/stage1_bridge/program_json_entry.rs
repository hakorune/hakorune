use super::NyashRunner;
use crate::cli::CliGroups;

pub(in crate::runner) enum ProgramJsonEntryOutcome {
    Success { out_path: String },
    Error { message: String },
}

impl NyashRunner {
    pub(in crate::runner) fn emit_program_json_v0_requested(groups: &CliGroups) -> bool {
        groups.emit.emit_program_json_v0.is_some()
    }

    /// Emit Program(JSON v0) using Stage-1 bridge glue and write to a file.
    pub(crate) fn emit_program_json_v0(groups: &CliGroups, out_path: &str) -> Result<(), String> {
        super::program_json::emit_program_json_v0(groups, out_path)
    }

    pub(in crate::runner) fn maybe_emit_program_json_v0(
        groups: &CliGroups,
    ) -> Option<ProgramJsonEntryOutcome> {
        let out_path = groups.emit.emit_program_json_v0.as_ref()?.clone();
        Some(match Self::emit_program_json_v0(groups, &out_path) {
            Ok(()) => ProgramJsonEntryOutcome::Success { out_path },
            Err(error) => ProgramJsonEntryOutcome::Error {
                message: format!("❌ emit-program-json-v0 error: {}", error),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::NyashRunner;
    use crate::cli::CliConfig;

    #[test]
    fn emit_program_json_v0_requested_reports_exact_flag_presence() {
        let groups = CliConfig::default().as_groups();
        assert!(!NyashRunner::emit_program_json_v0_requested(&groups));

        let mut groups = CliConfig::default().as_groups();
        groups.emit.emit_program_json_v0 = Some("/tmp/out.json".to_string());
        assert!(NyashRunner::emit_program_json_v0_requested(&groups));
    }
}
