//! One affine `Ready`/`Taken` cell shared by every disposition map.
//!
//! Direct, root-instance, and lexical-instance call inventories all mark a
//! sealed row consumed exactly once with the same two-state slot; the row
//! type differs, the discipline does not. Site-keyed maps keep their own
//! names via per-family aliases.

#[derive(Debug)]
pub(crate) enum DispositionSlotV1<T> {
    Ready(T),
    Taken,
}
