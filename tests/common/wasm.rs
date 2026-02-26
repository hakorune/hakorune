use std::path::PathBuf;

pub fn fixture_path(rel: &str) -> PathBuf {
    let mut fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fixture.push(rel);
    fixture
}

pub fn hakorune_bin_path() -> PathBuf {
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_hakorune") {
        return PathBuf::from(path);
    }
    let mut fallback = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fallback.push("target/debug/hakorune");
    fallback
}

pub fn target_temp_wat_path(prefix: &str) -> PathBuf {
    let mut out = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    out.push(format!("target/{}_{}.wat", prefix, std::process::id()));
    out
}
