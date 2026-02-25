//! Resolve context — capture per-thread prelude merge context for enriched diagnostics.
use std::cell::RefCell;

/// Line span mapping for merged prelude+main sources.
/// Represents that lines [start_line, start_line + line_count) in the merged
/// text originate from `file` at local lines [1, line_count].
#[derive(Clone, Debug)]
pub struct LineSpan {
    pub file: String,
    pub start_line: usize,
    pub line_count: usize,
}

thread_local! {
    static LAST_MERGED_PRELUDES: RefCell<Vec<String>> = RefCell::new(Vec::new());
    static LAST_TEXT_MERGE_LINE_SPANS: RefCell<Vec<LineSpan>> = RefCell::new(Vec::new());
}

/// Record the list of prelude file paths used for the last text merge in this thread.
pub fn set_last_merged_preludes(paths: Vec<String>) {
    LAST_MERGED_PRELUDES.with(|c| {
        *c.borrow_mut() = paths;
    });
}

/// Get a clone of the last recorded prelude file paths (if any).
pub fn clone_last_merged_preludes() -> Vec<String> {
    LAST_MERGED_PRELUDES.with(|c| c.borrow().clone())
}

/// Take and clear the last recorded prelude file paths.
#[allow(dead_code)]
pub fn take_last_merged_preludes() -> Vec<String> {
    LAST_MERGED_PRELUDES.with(|c| std::mem::take(&mut *c.borrow_mut()))
}

/// Record the line-span mapping for the last text merge in this thread.
pub fn set_last_text_merge_line_spans(spans: Vec<LineSpan>) {
    LAST_TEXT_MERGE_LINE_SPANS.with(|c| {
        *c.borrow_mut() = spans;
    });
}

/// Try to map a merged (global) line number back to its origin file and local line.
pub fn map_merged_line_to_origin(line: usize) -> Option<(String, usize)> {
    if line == 0 {
        return None;
    }
    LAST_TEXT_MERGE_LINE_SPANS.with(|c| {
        for span in c.borrow().iter() {
            if line >= span.start_line && line < span.start_line + span.line_count {
                let local = line - span.start_line + 1;
                return Some((span.file.clone(), local));
            }
        }
        None
    })
}
