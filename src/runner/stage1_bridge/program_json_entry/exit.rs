fn success_message(out_path: &str) -> String {
    format!("Program JSON written: {}", out_path)
}

fn error_message(error: &str) -> String {
    format!("❌ emit-program-json-v0 error: {}", error)
}

pub(super) fn exit_with_emit_program_json_result(out_path: &str, result: Result<(), String>) -> ! {
    match result {
        Ok(()) => {
            println!("{}", success_message(out_path));
            std::process::exit(0);
        }
        Err(error) => exit_with_emit_program_json_error(&error),
    }
}

pub(super) fn exit_with_emit_program_json_error(error: &str) -> ! {
    eprintln!("{}", error_message(error));
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::{error_message, success_message};

    #[test]
    fn success_message_reports_exact_out_path() {
        assert_eq!(
            success_message("/tmp/out.json"),
            "Program JSON written: /tmp/out.json"
        );
    }

    #[test]
    fn error_message_preserves_exact_prefix() {
        assert_eq!(
            error_message("boom"),
            "❌ emit-program-json-v0 error: boom"
        );
    }
}
