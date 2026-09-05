//! Final-source transport for the ordinary user-Box cohort.
//!
//! This is parser coverage only. It does not classify a `New` expression or
//! issue a birth target; the semantic package may only borrow this exact
//! source-sealed Box inventory when it co-seals a New-site claim.

use crate::ast::ASTNode;
use crate::parser::source_authority::SourceBoxDeclarationSiteV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParserOrdinaryBoxSourceRowV1 {
    site: SourceBoxDeclarationSiteV1,
    final_box_ordinal: usize,
    name: Box<str>,
    has_stored_field_initializer: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParserOrdinaryBoxSourceCoverageV1 {
    rows: Box<[ParserOrdinaryBoxSourceRowV1]>,
}

impl ParserOrdinaryBoxSourceCoverageV1 {
    pub(in crate::parser) fn issue(
        rows: Vec<(usize, Box<str>, SourceBoxDeclarationSiteV1, bool)>,
    ) -> Self {
        Self {
            rows: rows
                .into_iter()
                .map(|(final_box_ordinal, name, site, has_stored_field_initializer)| ParserOrdinaryBoxSourceRowV1 {
                    site,
                    final_box_ordinal,
                    name,
                    has_stored_field_initializer,
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

    pub(super) fn declaration<'a>(
        &self,
        row: &ParserOrdinaryBoxSourceRowV1,
        ast: &'a ASTNode,
    ) -> Option<&'a ASTNode> {
        if !self.rows.iter().any(|own| own == row) {
            return None;
        }
        let ASTNode::Program { statements, .. } = ast else { return None };
        let declaration = statements.get(row.final_box_ordinal)?;
        match declaration {
            ASTNode::BoxDeclaration { name, .. } if name.as_str() == row.name() =>
                Some(declaration),
            _ => None,
        }
    }

    pub(super) fn preserves_declarations(&self, initial: &ASTNode, final_ast: &ASTNode) -> bool {
        self.rows.iter().all(|row| {
            let Some(before) = self.declaration(row, initial) else { return false };
            self.declaration(row, final_ast) == Some(before)
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserOrdinaryBoxSourceLookupErrorV1 {
    DuplicateName,
}

impl ParserOrdinaryBoxSourceRowV1 {
    pub(crate) const fn has_stored_field_initializer(&self) -> bool {
        self.has_stored_field_initializer
    }

    pub(in crate::parser) fn has_site(&self, site: &SourceBoxDeclarationSiteV1) -> bool {
        &self.site == site
    }

    pub(crate) fn same_source_as(&self, other: &Self) -> bool {
        self == other
    }
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

    fn coverage() -> ParserOrdinaryBoxSourceCoverageV1 {
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            "box Page {} box Other {}", crate::parser::ParserBuildConfig::default(),
        ).unwrap();
        let super::super::ParsedNormalCallableProgramV1::SourceBacked(initial) = parsed else {
            panic!("source-backed boxes");
        };
        let source = initial.begin_transform().finish_exact().unwrap();
        let coverage = source.ordinary_box_coverage().clone();
        source.discard_at_named_root_execution_terminal();
        coverage
    }

    #[test]
    fn ordinary_new_source_coverage_returns_exact_box_ordinal() {
        let coverage = coverage();
        let row = coverage
            .row_for("Page")
            .expect("unique ordinary Box name")
            .expect("Page row");
        assert_eq!(row.final_box_ordinal(), 0);
    }

    #[test]
    fn ordinary_source_retains_initializer_provenance_not_generated_store_shape() {
        for (source, expected) in [
            ("box Page { value: i64 = 1 }", true),
            ("box Page { value: i64 = 1\nbirth() {} }", true),
            ("box Page { value: i64\nbirth() { me.value = 1 } }", false),
        ] {
            let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
                source, crate::parser::ParserBuildConfig {
                    mode: crate::parser::BuildMode::Test,
                    ..crate::parser::ParserBuildConfig::default()
                },
            ).unwrap();
            let super::super::ParsedNormalCallableProgramV1::SourceBacked(initial) = parsed else {
                panic!("source-backed Box");
            };
            let final_source = initial.begin_transform().finish_exact().unwrap();
            let row = final_source.ordinary_box_coverage().row_for("Page").unwrap().unwrap();
            assert_eq!(row.has_stored_field_initializer(), expected, "{source}");
            final_source.discard_at_named_root_execution_terminal();
        }
        // Initializers themselves belong to the existing gate signature. These
        // sources stop upstream; they cannot prove downstream branch transport.
        for source in [
            "box Page { gate Build.test { value: i64 = 1\nbirth() {} } else { value: i64\nbirth() {} } }",
            "box Page { gate Build.test { value: i64\nbirth() {} } else { value: i64 = 1\nbirth() {} } }",
        ] {
            let error = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
                source, crate::parser::ParserBuildConfig {
                    mode: crate::parser::BuildMode::Test,
                    ..crate::parser::ParserBuildConfig::default()
                },
            ).unwrap_err();
            assert!(matches!(error, crate::parser::ParseError::BuildCfg { message, .. }
                if message == "member-level gate branches must preserve the same public signature"));
        }
    }

    #[test]
    fn ordinary_new_source_coverage_rejects_duplicate_box_names() {
        let mut coverage = coverage();
        coverage.rows[1].name = "Page".into();
        assert_eq!(
            coverage.row_for("Page"),
            Err(ParserOrdinaryBoxSourceLookupErrorV1::DuplicateName)
        );
    }
}
