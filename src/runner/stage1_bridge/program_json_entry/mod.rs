use crate::cli::CliGroups;

mod request;

pub(in crate::runner) fn emit_program_json_v0_requested(groups: &CliGroups) -> bool {
    groups.emit.emit_program_json_v0.is_some()
}

pub(in crate::runner) fn emit_program_json_v0_and_exit(groups: &CliGroups) -> ! {
    let request = match request::build_emit_request(groups) {
        Ok(request) => request,
        Err(error) => {
            eprintln!("❌ emit-program-json-v0 error: {}", error);
            std::process::exit(1);
        }
    };
    match super::program_json::emit_program_json_v0(&request.source_path, &request.out_path) {
        Ok(()) => {
            println!("Program JSON written: {}", request.out_path);
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
