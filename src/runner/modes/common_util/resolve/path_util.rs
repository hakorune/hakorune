//! path_util — helpers for using target path detection (SSOT)

/// Determine if an original using target string should be treated as a file path.
/// Original means it may still contain surrounding quotes.
pub fn is_using_target_path_original(target: &str) -> bool {
    if target.starts_with('"') {
        return true;
    }
    let t = target.trim_matches('"');
    is_using_target_path_unquoted(t)
}

/// Determine if an unquoted using target string is a file path.
pub fn is_using_target_path_unquoted(target_unquoted: &str) -> bool {
    target_unquoted.starts_with("./")
        || target_unquoted.starts_with('/')
        || target_unquoted.ends_with(".hako")
        || target_unquoted.ends_with(".nyash")
}
