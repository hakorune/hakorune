//! generic_loop v0/v1 overlap guard (SSOT).
//!
//! Centralizes the overlap decisions so that v0/v1 reject/accept logic
//! stays consistent across Facts and shape detection.

use crate::mir::policies::GenericLoopV1ShapeId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum V1ShapeMatch {
    None,
    Single(GenericLoopV1ShapeId),
    Overlap(Vec<GenericLoopV1ShapeId>),
}

pub(crate) fn v1_shape_blocks_v0(shape: Option<GenericLoopV1ShapeId>) -> bool {
    shape.is_some()
}

pub(crate) fn is_v1_shape_overlap(a: GenericLoopV1ShapeId, b: GenericLoopV1ShapeId) -> bool {
    // Current policy: any distinct shapes are treated as an overlap (fail-fast).
    a != b
}

pub(crate) fn classify_v1_shape_matches(matches: &[GenericLoopV1ShapeId]) -> V1ShapeMatch {
    match matches.len() {
        0 => V1ShapeMatch::None,
        1 => V1ShapeMatch::Single(matches[0]),
        _ => {
            let mut overlaps = Vec::new();
            for (idx, a) in matches.iter().enumerate() {
                for b in matches.iter().skip(idx + 1) {
                    if is_v1_shape_overlap(*a, *b) {
                        if !overlaps.contains(a) {
                            overlaps.push(*a);
                        }
                        if !overlaps.contains(b) {
                            overlaps.push(*b);
                        }
                    }
                }
            }
            if overlaps.is_empty() {
                V1ShapeMatch::Single(matches[0])
            } else {
                V1ShapeMatch::Overlap(overlaps)
            }
        }
    }
}
