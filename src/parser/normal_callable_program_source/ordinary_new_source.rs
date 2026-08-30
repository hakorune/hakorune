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

    pub(crate) fn row_for(
        &self,
        name: &str,
    ) -> Result<Option<&ParserOrdinaryBoxSourceRowV1>, ParserOrdinaryBoxSourceLookupErrorV1> {
        let mut matches = self.rows.iter().filter(|row| row.name.as_ref() == name);
        let Some(row) = matches.next() else {
            return Ok(None);
        };
        if matches.next().is_some() {
            return Err(ParserOrdinaryBoxSourceLookupErrorV1::DuplicateName);
        }
        Ok(Some(row))
    }

    pub(crate) fn rows(&self) -> &[ParserOrdinaryBoxSourceRowV1] {
        &self.rows
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserOrdinaryBoxSourceLookupErrorV1 {
    DuplicateName,
}

impl ParserOrdinaryBoxSourceRowV1 {
    pub(crate) const fn final_box_ordinal(&self) -> usize {
        self.final_box_ordinal
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_new_source_coverage_returns_exact_box_ordinal() {
        let coverage =
            ParserOrdinaryBoxSourceCoverageV1::issue(vec![(4, "Page".into()), (9, "Other".into())]);
        let row = coverage
            .row_for("Page")
            .expect("unique ordinary Box name")
            .expect("Page row");
        assert_eq!(row.final_box_ordinal(), 4);
    }

    #[test]
    fn ordinary_new_source_coverage_rejects_duplicate_box_names() {
        let coverage =
            ParserOrdinaryBoxSourceCoverageV1::issue(vec![(4, "Page".into()), (9, "Page".into())]);
        assert_eq!(
            coverage.row_for("Page"),
            Err(ParserOrdinaryBoxSourceLookupErrorV1::DuplicateName)
        );
    }
}
