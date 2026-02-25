//! Common diagnostics helpers (concise, centralized)

use crate::parser::ParseError;
use crate::tokenizer::TokenizeError;

/// Whether provider logs should be emitted under current policy.
/// quiet_pipe usually reflects NYASH_JSON_ONLY; allowing override with HAKO_PROVIDER_TRACE=1.
pub fn provider_log_enabled(quiet_pipe: bool) -> bool {
    // Explicit trace override always wins (even in quiet JSON pipelines).
    if crate::config::env::env_string("HAKO_PROVIDER_TRACE").as_deref() == Some("1") {
        return true;
    }
    // Otherwise, keep provider selection logs dev-only to avoid gate output noise.
    if crate::config::env::cli_verbose_enabled() || crate::config::env::debug_plugin() {
        return true;
    }
    // Default: OFF (even when not quiet).
    let _ = quiet_pipe;
    false
}

/// Emit a consistent provider-registry info line.
pub fn provider_log_info(msg: &str) {
    crate::runtime::get_global_ring0()
        .log
        .info(&format!("[provider-registry] {}", msg));
}

/// Emit the provider selection tag in a stable shape.
pub fn provider_log_select(box_name: &str, ring: &str, source: &str, caps: Option<&str>) {
    match caps {
        Some(c) if !c.is_empty() => {
            crate::runtime::get_global_ring0().log.info(&format!(
                "[provider/select:{} ring={} src={} caps={}]",
                box_name, ring, source, c
            ));
        }
        _ => {
            crate::runtime::get_global_ring0().log.info(&format!(
                "[provider/select:{} ring={} src={}]",
                box_name, ring, source
            ));
        }
    }
}

/// Emit a Fail-Fast tag for provider fallback/selection errors.
pub fn failfast_provider(reason: &str) {
    crate::runtime::get_global_ring0()
        .log
        .error(&format!("[failfast/provider/{}]", reason));
}

/// Print a parse error with enriched context (source excerpt + caret + origin mapping).
pub fn print_parse_error_with_context(filename: &str, src: &str, err: &ParseError) {
    eprintln!("❌ Parse error in {}: {}", filename, err);

    let (line_opt, col_opt) = extract_line_col(err);
    if let Some(line) = line_opt {
        print_source_snippet(filename, src, line, col_opt);

        if let Some((of, ol)) =
            crate::runner::modes::common_util::resolve::map_merged_line_to_origin(line)
        {
            if of != filename {
                eprintln!(
                    "[parse/context] merged origin: {}:{} (from merged line {})",
                    of, ol, line
                );
            }
        }
    }
}

fn extract_line_col(err: &ParseError) -> (Option<usize>, Option<usize>) {
    match err {
        ParseError::UnexpectedToken { line, .. } => (Some(*line), None),
        ParseError::UnexpectedEOF => (None, None),
        ParseError::InvalidExpression { line } => (Some(*line), None),
        ParseError::InvalidStatement { line } => (Some(*line), None),
        ParseError::UnsupportedIdentifier { line, .. } => (Some(*line), None),
        ParseError::CircularDependency { .. } => (None, None),
        ParseError::InfiniteLoop { line, .. } => (Some(*line), None),
        ParseError::TransparencySystemRemoved { line, .. } => (Some(*line), None),
        ParseError::UnsupportedNamespace { line, .. } => (Some(*line), None),
        ParseError::ExpectedIdentifier { line } => (Some(*line), None),
        ParseError::TokenizeError(te) => match te {
            TokenizeError::UnexpectedCharacter { line, column, .. } => (Some(*line), Some(*column)),
            TokenizeError::UnterminatedString { line }
            | TokenizeError::InvalidNumber { line }
            | TokenizeError::UnterminatedComment { line } => (Some(*line), None),
        },
    }
}

fn print_source_snippet(filename: &str, src: &str, line: usize, col: Option<usize>) {
    if src.is_empty() {
        return;
    }
    let lines: Vec<&str> = src.lines().collect();
    if line == 0 || line > lines.len() {
        return;
    }
    let start = line.saturating_sub(2).max(1);
    let end = (line + 1).min(lines.len());

    eprintln!("[parse/context] in {}", filename);
    for ln in start..=end {
        let text = lines[ln - 1];
        let marker = if ln == line { ">" } else { " " };
        eprintln!("{} {:6} | {}", marker, ln, text);
    }

    if let Some(col) = col {
        if line <= lines.len() {
            let text = lines[line - 1];
            let mut underline = String::new();
            for (i, ch) in text.chars().enumerate() {
                if i + 1 >= col {
                    break;
                }
                // Preserve tabs visually; spaces elsewhere
                underline.push(if ch == '\t' { '\t' } else { ' ' });
            }
            let pad = "        "; // align under "  LNNNNN |"
            eprintln!("          {}{}^", pad, underline);
        }
    }
}
