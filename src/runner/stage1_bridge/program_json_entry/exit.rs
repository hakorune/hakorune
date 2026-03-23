use super::execute::ProgramJsonEmitResponse;

pub(super) fn exit_with_emit_program_json_response(response: ProgramJsonEmitResponse) -> ! {
    match response.result {
        Ok(()) => {
            println!("Program JSON written: {}", response.out_path);
            std::process::exit(0);
        }
        Err(error) => exit_with_emit_program_json_error(&error),
    }
}

pub(super) fn exit_with_emit_program_json_error(error: &str) -> ! {
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
