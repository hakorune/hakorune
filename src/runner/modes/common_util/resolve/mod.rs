/*!
 * Using resolver utilities — static resolution line (manifest + strip) 📦
 *
 * 箱化モジュール化で綺麗綺麗になったにゃ！🎉
 *
 * Separation of concerns:
 * - Layer 1 (manifest): Resolve package/module aliases from hako.toml
 *   (with nyash.toml compatibility fallback).
 * - Layer 2 (runner strip/text-merge): strip `using` lines, collect prelude
 *   file paths, and record imported static-box alias bindings for `using ... as Alias`.
 * - Optional compatibility path: parse/merge preludes as AST when
 *   `NYASH_USING_AST=1`, but the default route is strip + text merge + imports.
 * - Dynamic (runtime): Plugin/extern dispatch only. User instance BoxCall
 *   fallback is disallowed in prod; builder must rewrite obj.method() to
 *   a function call.
 *
 * 📦 箱化モジュール構造 (Box-First Architecture):
 * - strip: Legacy functions (preserved for compatibility)
 * - using_resolution: 🎯 UsingResolutionBox - using文解析専門家！
 * - prelude_manager: 📚 PreludeManagerBox - プレリュード統合専門家！
 * - selfhost_pipeline: 🚀 SelfhostPipelineBox - パイプライン管理専門家！
 * - seam: seam logging and optional boundary markers (for diagnostics).
 */

pub mod context;
pub mod path_util;
pub mod prelude_manager;
pub mod root;
pub mod seam;
pub mod selfhost_pipeline;
pub mod strip;
pub mod using_resolution;

// 📦 箱化モジュールの公開にゃ！
pub use using_resolution::{UsingConfig, UsingResolutionBox, UsingTarget};

pub use prelude_manager::{MergeResult, MergeStrategy, PreludeManagerBox};

pub use selfhost_pipeline::{CompilationResult, PipelineConfig, SelfhostPipelineBox};

// 🔧 Legacy functions (preserved for compatibility)
pub use strip::{
    collect_using_and_strip, merge_prelude_asts_with_main, merge_prelude_text,
    merge_prelude_text_with_imports, parse_preludes_to_asts, preexpand_at_local,
    resolve_prelude_paths_profiled,
};

// Expose context helpers for enhanced diagnostics
pub use context::{
    clone_last_merged_preludes, map_merged_line_to_origin, set_last_merged_preludes,
    set_last_text_merge_line_spans, LineSpan,
};
