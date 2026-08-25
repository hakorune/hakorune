//! Final-source transport for the ordinary user-Box cohort.
//!
//! This is parser coverage only. It does not classify a `New` expression or
//! issue a birth target; the semantic package may only borrow this exact
//! source-sealed Box inventory when it co-seals a New-site claim.

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParserOrdinaryBoxSourceRowV1 {
    final_box_ordinal: usize,
    name: Box<str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParserOrdinaryBoxSourceCoverageV1 {
    rows: Box<[ParserOrdinaryBoxSourceRowV1]>,
}

impl ParserOrdinaryBoxSourceCoverageV1 {
    pub(crate) fn issue(rows: Vec<(usize, Box<str>)>) -> Self {
        Self {
            rows: rows
                .into_iter()
                .map(|(final_box_ordinal, name)| ParserOrdinaryBoxSourceRowV1 {
                    final_box_ordinal,
                    name,
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    pub(crate) fn contains_box(&self, name: &str) -> bool {
        self.rows.iter().any(|row| row.name.as_ref() == name)
    }

    pub(crate) fn rows(&self) -> &[ParserOrdinaryBoxSourceRowV1] {
        &self.rows
    }
}

impl ParserOrdinaryBoxSourceRowV1 {
    pub(crate) const fn final_box_ordinal(&self) -> usize {
        self.final_box_ordinal
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}
