pub(super) fn write_program_json_output(out_path: &str, out: &str) -> Result<(), String> {
    std::fs::write(out_path, out)
        .map_err(|error| format!("emit-program-json-v0 write {} failed: {}", out_path, error))
}

#[cfg(test)]
mod tests {
    use super::write_program_json_output;

    #[test]
    fn write_program_json_output_writes_exact_payload() {
        let unique = format!(
            "hakorune-stage1-bridge-program-json-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("unix epoch")
                .as_nanos()
        );
        let out_path = std::env::temp_dir().join(unique);
        let out_path_str = out_path.to_string_lossy().into_owned();

        write_program_json_output(&out_path_str, "{\"kind\":\"Program\"}")
            .expect("write program json");
        let written = std::fs::read_to_string(&out_path).expect("read written file");
        let _ = std::fs::remove_file(&out_path);

        assert_eq!(written, "{\"kind\":\"Program\"}");
    }
}
