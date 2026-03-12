use crate::cli::CliGroups;

pub(in crate::runner) fn emit_program_json_v0_requested(groups: &CliGroups) -> bool {
    groups.emit.emit_program_json_v0.is_some()
}

/// Emit Program(JSON v0) using Stage-1 bridge glue and write to a file.
fn emit_program_json_v0(groups: &CliGroups, out_path: &str) -> Result<(), String> {
    super::program_json::emit_program_json_v0(groups, out_path)
}

pub(in crate::runner) fn emit_program_json_v0_and_exit(groups: &CliGroups) -> ! {
    let out_path = groups
        .emit
        .emit_program_json_v0
        .as_ref()
        .expect("emit-program-json-v0 flag should be present")
        .clone();
    match emit_program_json_v0(groups, &out_path) {
        Ok(()) => {
            println!("Program JSON written: {}", out_path);
            std::process::exit(0);
        }
        Err(error) => {
            eprintln!("❌ emit-program-json-v0 error: {}", error);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::CliConfig;
    use crate::runner::stage1_bridge::program_json_entry::emit_program_json_v0_requested;

    #[test]
    fn emit_program_json_v0_requested_reports_exact_flag_presence() {
        let groups = CliConfig::default().as_groups();
        assert!(!emit_program_json_v0_requested(&groups));

        let mut groups = CliConfig::default().as_groups();
        groups.emit.emit_program_json_v0 = Some("/tmp/out.json".to_string());
        assert!(emit_program_json_v0_requested(&groups));
    }
}
