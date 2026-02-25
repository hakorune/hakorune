use crate::mir::{MirModule, MirPrinter};

pub(super) fn maybe_dump_mir(module: &MirModule) {
    // New: file dump path for offline analysis (Stage‑1/Stage‑B selfhost, ParserBox 等)
    // Use env RUST_MIR_DUMP_PATH to write the MIR printer output to a file.
    if let Some(path) = crate::config::env::rust_mir_dump_path() {
        if let Ok(mut file) = std::fs::File::create(&path) {
            let printer = MirPrinter::new();
            let _ = std::io::Write::write_all(&mut file, printer.print_module(module).as_bytes());
        }
    }
    // Existing: verbose flag dumps to stdout
    if crate::config::env::cli_verbose() {
        let printer = MirPrinter::new();
        println!("{}", printer.print_module(module));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::{Mutex, OnceLock};

    fn temp_path(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("{}_{}", name, std::process::id()));
        path
    }

    fn env_guard() -> &'static Mutex<()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD.get_or_init(|| Mutex::new(()))
    }

    fn dummy_module() -> MirModule {
        MirModule::new("test-module".to_string())
    }

    #[test]
    fn writes_file_when_dump_path_is_set() {
        let _lock = env_guard().lock().unwrap();
        let path = temp_path("mir_dump_path");
        let _ = fs::remove_file(&path);
        std::env::set_var("RUST_MIR_DUMP_PATH", path.to_string_lossy().to_string());

        maybe_dump_mir(&dummy_module());

        assert!(
            path.exists(),
            "maybe_dump_mir should write when RUST_MIR_DUMP_PATH is set"
        );
        let contents = fs::read_to_string(&path).unwrap_or_default();
        assert!(
            contents.contains("test-module"),
            "dump should contain module name"
        );

        let _ = fs::remove_file(&path);
        std::env::remove_var("RUST_MIR_DUMP_PATH");
    }

    #[test]
    fn does_not_write_when_dump_path_is_unset() {
        let _lock = env_guard().lock().unwrap();
        std::env::remove_var("RUST_MIR_DUMP_PATH");
        let path = temp_path("mir_dump_path_unset");
        let _ = fs::remove_file(&path);

        maybe_dump_mir(&dummy_module());

        assert!(
            !path.exists(),
            "maybe_dump_mir should not write when RUST_MIR_DUMP_PATH is unset"
        );
    }
}
