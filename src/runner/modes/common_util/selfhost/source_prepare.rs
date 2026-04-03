/*!
 * Selfhost source materialization helper.
 *
 * Purpose:
 * - Keep `selfhost.rs` focused on route sequencing and terminal accept.
 * - Keep source extension gate / source read / using merge / preexpand / tmp staging under one thin owner.
 */

use crate::runner::NyashRunner;
use std::path::{Path, PathBuf};

const SELFHOST_TMP_INPUT_PATH: &str = "tmp/ny_parser_input.ny";

pub(crate) struct PreparedSelfhostSource {
    pub(crate) source_name: String,
    pub(crate) raw_code: String,
    pub(crate) prepared_code: String,
    pub(crate) tmp_path: PathBuf,
}

pub(crate) fn prepare_selfhost_source(
    runner: &NyashRunner,
    filename: &str,
) -> Option<PreparedSelfhostSource> {
    let path = Path::new(filename);
    let source_name = validate_selfhost_source_path(path, filename)?;
    let raw_code = read_source_text(filename)?;
    let prepared_code = merge_and_preexpand_source(runner, filename, &raw_code)?;
    let tmp_path = materialize_tmp_source(&prepared_code)?;

    Some(PreparedSelfhostSource {
        source_name,
        raw_code,
        prepared_code,
        tmp_path,
    })
}

fn validate_selfhost_source_path(path: &Path, filename: &str) -> Option<String> {
    if !is_supported_selfhost_extension(
        path.extension().and_then(|s| s.to_str()),
        crate::config::env::use_ny_compiler(),
    ) {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            crate::cli_v!(
                "[ny-compiler] skip selfhost pipeline for non-Ny source: {} (ext={})",
                filename,
                ext
            );
        } else {
            crate::cli_v!(
                "[ny-compiler] skip selfhost pipeline for source without extension: {}",
                filename
            );
        }
        return None;
    }

    Some(
        path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(filename)
            .to_string(),
    )
}

fn is_supported_selfhost_extension(ext: Option<&str>, allow_hako: bool) -> bool {
    match ext {
        Some("ny") | Some("nyash") => true,
        Some("hako") => allow_hako,
        _ => false,
    }
}

fn read_source_text(filename: &str) -> Option<String> {
    match std::fs::read_to_string(filename) {
        Ok(c) => Some(c),
        Err(e) => {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.error(&format!("[ny-compiler] read error: {}", e));
            None
        }
    }
}

fn merge_and_preexpand_source(
    runner: &NyashRunner,
    filename: &str,
    raw_code: &str,
) -> Option<String> {
    let mut code_ref: std::borrow::Cow<'_, str> = std::borrow::Cow::Borrowed(raw_code);
    if crate::config::env::enable_using() {
        let using_ast = crate::config::env::using_ast_enabled();
        if using_ast {
            match crate::runner::modes::common_util::resolve::merge_prelude_text(
                runner, raw_code, filename,
            ) {
                Ok(merged) => {
                    code_ref = std::borrow::Cow::Owned(merged);
                }
                Err(e) => {
                    let ring0 = crate::runtime::ring0::get_global_ring0();
                    ring0
                        .log
                        .error(&format!("[ny-compiler] using text merge error: {}", e));
                    return None;
                }
            }
        } else {
            match crate::runner::modes::common_util::resolve::resolve_prelude_paths_profiled(
                runner, raw_code, filename,
            ) {
                Ok((clean, paths)) => {
                    if !paths.is_empty() {
                        let ring0 = crate::runtime::ring0::get_global_ring0();
                        ring0.log.error("[ny-compiler] using: AST prelude merge is disabled in this profile. Enable NYASH_USING_AST=1 or remove 'using' lines.");
                        return None;
                    }
                    code_ref = std::borrow::Cow::Owned(clean);
                }
                Err(e) => {
                    let ring0 = crate::runtime::ring0::get_global_ring0();
                    ring0.log.error(&format!("[ny-compiler] {}", e));
                    return None;
                }
            }
        }
    }

    Some(crate::runner::modes::common_util::resolve::preexpand_at_local(
        code_ref.as_ref(),
    ))
}

fn materialize_tmp_source(prepared_code: &str) -> Option<PathBuf> {
    let tmp_dir = Path::new("tmp");
    if let Err(e) = std::fs::create_dir_all(tmp_dir) {
        let ring0 = crate::runtime::ring0::get_global_ring0();
        ring0
            .log
            .error(&format!("[ny-compiler] mkdir tmp failed: {}", e));
        return None;
    }

    let tmp_path = PathBuf::from(SELFHOST_TMP_INPUT_PATH);
    if !crate::config::env::ny_compiler_use_tmp_only() {
        if let Err(e) = std::fs::write(&tmp_path, prepared_code.as_bytes()) {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0
                .log
                .error(&format!("[ny-compiler] write tmp failed: {}", e));
            return None;
        }
    }

    Some(tmp_path)
}

#[cfg(test)]
mod tests {
    use super::{is_supported_selfhost_extension, SELFHOST_TMP_INPUT_PATH};

    #[test]
    fn selfhost_supported_extension_contract_is_stable() {
        assert!(is_supported_selfhost_extension(Some("ny"), false));
        assert!(is_supported_selfhost_extension(Some("nyash"), false));
        assert!(!is_supported_selfhost_extension(Some("hako"), false));
        assert!(is_supported_selfhost_extension(Some("hako"), true));
        assert!(!is_supported_selfhost_extension(None, false));
    }

    #[test]
    fn selfhost_tmp_input_path_is_stable() {
        assert_eq!(SELFHOST_TMP_INPUT_PATH, "tmp/ny_parser_input.ny");
    }
}
