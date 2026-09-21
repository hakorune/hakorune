use crate::runner::modes::common_util::resolve::MergedSourceLineageV1;

use super::super::source_authority::{ParserInvocationBrandV1, SourceBoxDeclarationPathV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::parser) struct ParserSourceDeclarationCoordinateV1 {
    pub(in crate::parser) brand: ParserInvocationBrandV1,
    pub(in crate::parser) path: SourceBoxDeclarationPathV1,
    pub(in crate::parser) global_line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::parser) struct ParserSourceAdmissionRowV1 {
    coordinate: ParserSourceDeclarationCoordinateV1,
    canonical_segment: Box<str>,
    local_line: usize,
}

impl ParserSourceAdmissionRowV1 {
    pub(in crate::parser) fn canonical_segment(&self) -> &str {
        &self.canonical_segment
    }

    pub(in crate::parser) fn local_line(&self) -> usize {
        self.local_line
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParserSourceAdmissionWitnessV1 {
    rows: Box<[ParserSourceAdmissionRowV1]>,
    _seal: ParserSourceAdmissionWitnessSealV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParserSourceAdmissionWitnessSealV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserSourceAdmissionErrorV1 {
    ForeignParserBrand,
    DuplicateDeclarationPath,
    DeclarationLineOutOfRange,
    SegmentLocalLineOutOfRange,
    MissingCoverageWitness,
}

impl ParserSourceAdmissionWitnessV1 {
    /// Issue the declaration-to-segment join for one merged-source invocation.
    ///
    /// The returned witness attests that the join ran over every declaration
    /// coordinate; an empty coordinate set still returns a witness with zero
    /// rows so a merged source-backed product always carries coverage proof.
    pub(in crate::parser) fn issue(
        lineage: &MergedSourceLineageV1,
        coordinates: Vec<ParserSourceDeclarationCoordinateV1>,
    ) -> Result<Self, ParserSourceAdmissionErrorV1> {
        let first_brand = coordinates.first().map(|first| first.brand.clone());
        let mut paths = Vec::with_capacity(coordinates.len());
        let mut rows = Vec::with_capacity(coordinates.len());
        for coordinate in coordinates {
            if let Some(first_brand) = &first_brand {
                if !coordinate.brand.same_as(first_brand) {
                    return Err(ParserSourceAdmissionErrorV1::ForeignParserBrand);
                }
            }
            if paths
                .iter()
                .any(|path: &SourceBoxDeclarationPathV1| path == &coordinate.path)
            {
                return Err(ParserSourceAdmissionErrorV1::DuplicateDeclarationPath);
            }
            paths.push(coordinate.path.clone());
            let Some((segment, offset)) = lineage.locate_global_line(coordinate.global_line) else {
                return Err(ParserSourceAdmissionErrorV1::DeclarationLineOutOfRange);
            };
            let local_line = segment.local_start_line.saturating_add(offset);
            if local_line
                >= segment
                    .local_start_line
                    .saturating_add(segment.local_line_count)
            {
                return Err(ParserSourceAdmissionErrorV1::SegmentLocalLineOutOfRange);
            }
            rows.push(ParserSourceAdmissionRowV1 {
                coordinate,
                canonical_segment: segment.canonical_path.clone(),
                local_line,
            });
        }
        Ok(Self {
            rows: rows.into_boxed_slice(),
            _seal: ParserSourceAdmissionWitnessSealV1,
        })
    }

    pub(in crate::parser) fn rows(&self) -> &[ParserSourceAdmissionRowV1] {
        &self.rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::modes::common_util::resolve::strip::MergedSourceSegmentV1;

    fn lineage() -> MergedSourceLineageV1 {
        MergedSourceLineageV1::issue(
            "root.hako",
            vec![
                MergedSourceSegmentV1 {
                    source: "lib.hako".into(),
                    canonical_path: "lib.hako".into(),
                    parent: None,
                    dfs_ordinal: 0,
                    global_start_line: 1,
                    global_line_count: 2,
                    local_start_line: 1,
                    local_line_count: 2,
                },
                MergedSourceSegmentV1 {
                    source: "root.hako".into(),
                    canonical_path: "root.hako".into(),
                    parent: None,
                    dfs_ordinal: 1,
                    global_start_line: 3,
                    global_line_count: 2,
                    local_start_line: 1,
                    local_line_count: 2,
                },
            ],
            Vec::new(),
        )
        .expect("test lineage")
    }

    #[test]
    fn source_line_is_joined_to_one_sealed_segment_and_local_line() {
        let brand = ParserInvocationBrandV1::issue();
        let path = SourceBoxDeclarationPathV1::root(brand.clone(), 0);
        let witness = ParserSourceAdmissionWitnessV1::issue(
            &lineage(),
            vec![ParserSourceDeclarationCoordinateV1 {
                brand,
                path,
                global_line: 3,
            }],
        )
        .expect("join");
        assert_eq!(witness.rows()[0].canonical_segment(), "root.hako");
        assert_eq!(witness.rows()[0].local_line(), 1);
    }

    #[test]
    fn empty_coordinates_still_issue_an_attested_empty_witness() {
        let witness = ParserSourceAdmissionWitnessV1::issue(&lineage(), Vec::new())
            .expect("empty join attests vacuous coverage");
        assert!(witness.rows().is_empty());
    }

    #[test]
    fn foreign_brand_and_out_of_range_line_are_rejected() {
        let brand = ParserInvocationBrandV1::issue();
        let foreign = ParserInvocationBrandV1::issue();
        let path = SourceBoxDeclarationPathV1::root(brand.clone(), 0);
        let error = ParserSourceAdmissionWitnessV1::issue(
            &lineage(),
            vec![
                ParserSourceDeclarationCoordinateV1 {
                    brand,
                    path: path.clone(),
                    global_line: 1,
                },
                ParserSourceDeclarationCoordinateV1 {
                    brand: foreign,
                    path: SourceBoxDeclarationPathV1::root(path.brand().clone(), 1),
                    global_line: 2,
                },
            ],
        )
        .unwrap_err();
        assert_eq!(error, ParserSourceAdmissionErrorV1::ForeignParserBrand);

        let brand = ParserInvocationBrandV1::issue();
        let error = ParserSourceAdmissionWitnessV1::issue(
            &lineage(),
            vec![ParserSourceDeclarationCoordinateV1 {
                path: SourceBoxDeclarationPathV1::root(brand.clone(), 0),
                brand,
                global_line: 0,
            }],
        )
        .unwrap_err();
        assert_eq!(
            error,
            ParserSourceAdmissionErrorV1::DeclarationLineOutOfRange
        );
    }
}
