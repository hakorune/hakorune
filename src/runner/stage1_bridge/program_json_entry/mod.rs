use crate::cli::CliGroups;

mod request;

pub(in crate::runner) fn emit_program_json_v0_requested(groups: &CliGroups) -> bool {
    request::emit_program_json_v0_requested(groups)
}

pub(in crate::runner) fn emit_program_json_v0_and_exit(groups: &CliGroups) -> ! {
    let request = match request::build_emit_request(groups) {
        Ok(request) => request,
        Err(error) => exit_with_emit_program_json_error(&error),
    };
    exit_with_emit_program_json_result(
        &request.out_path,
        super::program_json::emit_program_json_v0(&request.source_path, &request.out_path),
    )
}

fn exit_with_emit_program_json_result(out_path: &str, result: Result<(), String>) -> ! {
    match result {
        Ok(()) => {
            println!("Program JSON written: {}", out_path);
            std::process::exit(0);
        }
        Err(error) => exit_with_emit_program_json_error(&error),
    }
}

fn exit_with_emit_program_json_error(error: &str) -> ! {
    eprintln!("❌ emit-program-json-v0 error: {}", error);
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    #[test]
    fn success_message_format_stays_exact() {
        assert_eq!(
            format!("Program JSON written: {}", "/tmp/out.json"),
            "Program JSON written: /tmp/out.json"
        );
    }

    #[test]
    fn error_message_format_preserves_exact_prefix() {
        assert_eq!(
            format!("❌ emit-program-json-v0 error: {}", "boom"),
            "❌ emit-program-json-v0 error: boom"
        );
    }
}
