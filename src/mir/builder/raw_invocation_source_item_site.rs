//! Canonical item-site projection for raw body transport.
//!
//! A `child_body` receipt names the body root. A body driver then needs the
//! canonical site of one item in that body. Nested body kinds are rootless at
//! the item level; `Program` stays explicitly rootful.

use crate::mir::resolved_semantics::{SourceBodyKindV1, SourceNodeSiteV1, SourcePathV1};

fn is_rootless_item_site_kind(kind: SourceBodyKindV1) -> bool {
    matches!(
        kind,
        SourceBodyKindV1::Scope
            | SourceBodyKindV1::TaskScope
            | SourceBodyKindV1::FastMem
            | SourceBodyKindV1::IfThen
            | SourceBodyKindV1::IfElse
            | SourceBodyKindV1::Loop
            | SourceBodyKindV1::BlockExprPrelude
    )
}

pub(in crate::mir::builder) fn body_item_site(
    kind: SourceBodyKindV1,
    site: &SourceNodeSiteV1,
    index: usize,
) -> SourceNodeSiteV1 {
    if kind == SourceBodyKindV1::Function {
        return SourcePathV1::root_body(index).node();
    }
    if is_rootless_item_site_kind(kind) && site.segments().last() == kind.root_segment().as_ref() {
        let mut segments = site.segments().to_vec();
        let _ = segments.pop();
        segments.push(kind.item_segment(index as u32));
        return SourceNodeSiteV1::from_segments(segments);
    }
    SourcePathV1::from_node(site)
        .child(kind.item_segment(index as u32))
        .node()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::SourcePathSegmentV1;

    #[test]
    fn nested_body_items_drop_only_their_body_root() {
        for kind in [
            SourceBodyKindV1::Scope,
            SourceBodyKindV1::TaskScope,
            SourceBodyKindV1::FastMem,
            SourceBodyKindV1::IfThen,
            SourceBodyKindV1::IfElse,
            SourceBodyKindV1::Loop,
            SourceBodyKindV1::BlockExprPrelude,
        ] {
            let root = SourcePathV1::root_body(5).child(kind.root_segment().unwrap());
            let item = body_item_site(kind, &root.node(), 0);
            assert_eq!(
                item.segments(),
                &[SourcePathSegmentV1::Body(5), kind.item_segment(0)]
            );
        }
    }

    #[test]
    fn chained_if_items_remain_under_the_outer_body() {
        let root = SourcePathV1::root_body(5)
            .child(SourcePathSegmentV1::IfThen(0))
            .child(SourcePathSegmentV1::IfThenBody);
        let item = body_item_site(SourceBodyKindV1::IfThen, &root.node(), 1);
        assert_eq!(
            item.segments(),
            &[
                SourcePathSegmentV1::Body(5),
                SourcePathSegmentV1::IfThen(0),
                SourcePathSegmentV1::IfThen(1),
            ]
        );
    }

    #[test]
    fn program_items_keep_the_explicit_program_root() {
        let root = SourcePathV1::program_body().node();
        let item = body_item_site(SourceBodyKindV1::Program, &root, 3);
        assert_eq!(
            item.segments(),
            &[
                SourcePathSegmentV1::ProgramBodyRoot,
                SourcePathSegmentV1::ProgramBody(3),
            ]
        );
    }
}
