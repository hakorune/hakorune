mod import_lineage;
mod merge;
mod preexpand;
mod prelude;
mod using;

pub(crate) use import_lineage::{MergedSourceLineageV1, MergedSourceSegmentV1};
pub(crate) use merge::merge_prelude_text_with_imports_and_lineage;
pub use merge::{
    merge_normal_prelude_text_with_imports, merge_prelude_asts_with_main, merge_prelude_text,
    merge_prelude_text_with_imports,
};
pub use preexpand::preexpand_at_local;
pub use prelude::{
    parse_preludes_to_asts, resolve_normal_prelude_paths_profiled, resolve_prelude_paths_profiled,
};
pub use using::collect_using_and_strip;
