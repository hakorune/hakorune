pub(super) fn read_source_text(source_path: &str) -> Result<String, String> {
    std::fs::read_to_string(source_path)
        .map_err(|error| format!("emit-program-json-v0 read error: {}: {}", source_path, error))
}

#[cfg(test)]
mod tests {
    use super::read_source_text;

    #[test]
    fn read_source_text_reports_exact_missing_path() {
        let unique = format!(
            "/tmp/hakorune-stage1-bridge-missing-{}-{}.hako",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("unix epoch")
                .as_nanos()
        );
        let error = read_source_text(&unique).expect_err("missing path must fail");
        assert!(error.starts_with(&format!("emit-program-json-v0 read error: {}", unique)));
    }

    #[test]
    fn read_source_text_returns_exact_file_contents() {
        let unique = format!(
            "hakorune-stage1-bridge-read-input-{}-{}.hako",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("unix epoch")
                .as_nanos()
        );
        let path = std::env::temp_dir().join(unique);
        let path_str = path.to_string_lossy().into_owned();
        std::fs::write(&path, "static box Main { main() { return 0 } }")
            .expect("write temp source");

        let read_back = read_source_text(&path_str).expect("read source text");
        let _ = std::fs::remove_file(&path);

        assert_eq!(read_back, "static box Main { main() { return 0 } }");
    }
}
