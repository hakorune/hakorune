//! Typed ownership for one text-merged source invocation.
//!
//! The merge owner issues these rows once.  Runtime alias lowering and
//! diagnostic line spans remain separate compatibility projections.

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ImportLineageEdgeV1 {
    pub(crate) origin: Box<str>,
    pub(crate) source_line: usize,
    pub(crate) requested: Box<str>,
    pub(crate) resolved: Box<str>,
    pub(crate) alias: Option<Box<str>>,
    pub(crate) binding: Option<Box<str>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MergedSourceSegmentV1 {
    pub(crate) source: Box<str>,
    pub(crate) canonical_path: Box<str>,
    pub(crate) parent: Option<Box<str>>,
    pub(crate) dfs_ordinal: u32,
    pub(crate) global_start_line: usize,
    pub(crate) global_line_count: usize,
    pub(crate) local_start_line: usize,
    pub(crate) local_line_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MergedSourceLineageV1 {
    pub(crate) root: Box<str>,
    pub(crate) segments: Box<[MergedSourceSegmentV1]>,
    pub(crate) edges: Box<[ImportLineageEdgeV1]>,
    _seal: MergedSourceLineageSealV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MergedSourceLineageSealV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MergedSourceLineageErrorV1 {
    EmptyRoot,
    EmptySegment,
    DuplicateCanonicalPath,
    SegmentGapOrOverlap,
    InvalidDfsOrder,
    EdgeOriginMissing,
    EdgeTargetMissing,
}

impl MergedSourceLineageV1 {
    pub(crate) fn root_only(
        source: &str,
        filename: &str,
    ) -> Result<Self, MergedSourceLineageErrorV1> {
        let canonical_path = std::fs::canonicalize(filename)
            .ok()
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or_else(|| filename.to_owned());
        let line_count = source.lines().count();
        Self::issue(
            canonical_path.clone().into_boxed_str(),
            vec![MergedSourceSegmentV1 {
                source: filename.to_owned().into_boxed_str(),
                canonical_path: canonical_path.into_boxed_str(),
                parent: None,
                dfs_ordinal: 0,
                global_start_line: 1,
                global_line_count: line_count,
                local_start_line: 1,
                local_line_count: line_count,
            }],
            Vec::new(),
        )
    }

    pub(crate) fn issue(
        root: impl Into<Box<str>>,
        segments: Vec<MergedSourceSegmentV1>,
        edges: Vec<ImportLineageEdgeV1>,
    ) -> Result<Self, MergedSourceLineageErrorV1> {
        let root = root.into();
        if root.is_empty() {
            return Err(MergedSourceLineageErrorV1::EmptyRoot);
        }
        let mut paths = std::collections::HashSet::new();
        let mut previous_end = 1usize;
        for (index, segment) in segments.iter().enumerate() {
            if segment.canonical_path.is_empty() || segment.source.is_empty() {
                return Err(MergedSourceLineageErrorV1::EmptySegment);
            }
            if !paths.insert(segment.canonical_path.as_ref()) {
                return Err(MergedSourceLineageErrorV1::DuplicateCanonicalPath);
            }
            if segment.dfs_ordinal as usize != index {
                return Err(MergedSourceLineageErrorV1::InvalidDfsOrder);
            }
            if segment.global_start_line != previous_end {
                return Err(MergedSourceLineageErrorV1::SegmentGapOrOverlap);
            }
            previous_end = segment
                .global_start_line
                .saturating_add(segment.global_line_count);
        }
        // A diamond import legitimately reaches one segment through two
        // different origins. Segment identity is checked above; edge
        // uniqueness only rejects the same origin repeating the same target.
        let mut edge_targets = std::collections::HashSet::new();
        for edge in &edges {
            if edge.origin.is_empty() || !paths.contains(edge.origin.as_ref()) {
                return Err(MergedSourceLineageErrorV1::EdgeOriginMissing);
            }
            if edge.resolved.is_empty() || !paths.contains(edge.resolved.as_ref()) {
                return Err(MergedSourceLineageErrorV1::EdgeTargetMissing);
            }
            if !edge_targets.insert((edge.origin.as_ref(), edge.resolved.as_ref())) {
                return Err(MergedSourceLineageErrorV1::DuplicateCanonicalPath);
            }
        }
        Ok(Self {
            root,
            segments: segments.into_boxed_slice(),
            edges: edges.into_boxed_slice(),
            _seal: MergedSourceLineageSealV1,
        })
    }

    pub(crate) fn locate_global_line(
        &self,
        line: usize,
    ) -> Option<(&MergedSourceSegmentV1, usize)> {
        self.segments.iter().find_map(|segment| {
            let offset = line.checked_sub(segment.global_start_line)?;
            (offset < segment.global_line_count).then_some((segment, offset))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn segment(path: &str, ordinal: u32, start: usize, count: usize) -> MergedSourceSegmentV1 {
        MergedSourceSegmentV1 {
            source: path.into(),
            canonical_path: path.into(),
            parent: None,
            dfs_ordinal: ordinal,
            global_start_line: start,
            global_line_count: count,
            local_start_line: 1,
            local_line_count: count,
        }
    }

    #[test]
    fn lineage_accepts_exact_root_and_nested_coverage() {
        let lineage = MergedSourceLineageV1::issue(
            "root.hako",
            vec![
                segment("nested.hako", 0, 1, 2),
                segment("root.hako", 1, 3, 2),
            ],
            vec![ImportLineageEdgeV1 {
                origin: "root.hako".into(),
                source_line: 1,
                requested: "nested.hako".into(),
                resolved: "nested.hako".into(),
                alias: Some("Nested".into()),
                binding: Some("Nested".into()),
            }],
        )
        .expect("exact coverage");
        assert_eq!(lineage.segments.len(), 2);
        assert_eq!(lineage.edges[0].source_line, 1);
    }

    #[test]
    fn lineage_rejects_duplicate_and_gap() {
        let duplicate = MergedSourceLineageV1::issue(
            "root.hako",
            vec![segment("root.hako", 0, 1, 1), segment("root.hako", 1, 2, 1)],
            Vec::new(),
        )
        .unwrap_err();
        assert_eq!(
            duplicate,
            MergedSourceLineageErrorV1::DuplicateCanonicalPath
        );

        let gap = MergedSourceLineageV1::issue(
            "root.hako",
            vec![
                segment("nested.hako", 0, 1, 1),
                segment("root.hako", 1, 3, 1),
            ],
            Vec::new(),
        )
        .unwrap_err();
        assert_eq!(gap, MergedSourceLineageErrorV1::SegmentGapOrOverlap);
    }

    #[test]
    fn lineage_accepts_diamond_edges_to_one_shared_segment() {
        let lineage = MergedSourceLineageV1::issue(
            "root.hako",
            vec![
                segment("root.hako", 0, 1, 1),
                segment("a.hako", 1, 2, 1),
                segment("b.hako", 2, 3, 1),
                segment("c.hako", 3, 4, 1),
            ],
            vec![
                ImportLineageEdgeV1 {
                    origin: "root.hako".into(),
                    source_line: 1,
                    requested: "a.hako".into(),
                    resolved: "a.hako".into(),
                    alias: None,
                    binding: None,
                },
                ImportLineageEdgeV1 {
                    origin: "root.hako".into(),
                    source_line: 2,
                    requested: "b.hako".into(),
                    resolved: "b.hako".into(),
                    alias: None,
                    binding: None,
                },
                ImportLineageEdgeV1 {
                    origin: "a.hako".into(),
                    source_line: 1,
                    requested: "c.hako".into(),
                    resolved: "c.hako".into(),
                    alias: None,
                    binding: None,
                },
                ImportLineageEdgeV1 {
                    origin: "b.hako".into(),
                    source_line: 1,
                    requested: "c.hako".into(),
                    resolved: "c.hako".into(),
                    alias: None,
                    binding: None,
                },
            ],
        )
        .expect("diamond imports may share one resolved segment");
        assert_eq!(lineage.edges.len(), 4);
    }

    #[test]
    fn lineage_rejects_duplicate_edge_from_same_origin() {
        let duplicate = MergedSourceLineageV1::issue(
            "root.hako",
            vec![
                segment("root.hako", 0, 1, 1),
                segment("child.hako", 1, 2, 1),
            ],
            vec![
                ImportLineageEdgeV1 {
                    origin: "root.hako".into(),
                    source_line: 1,
                    requested: "child.hako".into(),
                    resolved: "child.hako".into(),
                    alias: None,
                    binding: None,
                },
                ImportLineageEdgeV1 {
                    origin: "root.hako".into(),
                    source_line: 2,
                    requested: "child.hako".into(),
                    resolved: "child.hako".into(),
                    alias: None,
                    binding: None,
                },
            ],
        )
        .expect_err("same-origin duplicate edges remain invalid");
        assert_eq!(
            duplicate,
            MergedSourceLineageErrorV1::DuplicateCanonicalPath
        );
    }
}
