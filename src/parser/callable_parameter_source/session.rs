use std::collections::BTreeSet;

use crate::ast::BoxMethodInventoryOrdinalV1;
use crate::parser::source_authority::{ParserInvocationBrandV1, SourceBoxMethodSiteV1};
use crate::parser::{NyashParser, ParseError};

use super::catalog::ParserCallableParameterSourceCatalogV1;
use super::model::{ParserCallableDeclarationKindV1, ParserCallableParameterDeclarationSourceV1};
use super::parse_product::ParsedCallableParameterListV1;

#[derive(Debug)]
pub(in crate::parser) struct ParserCallableParameterSourceSessionV1 {
    brand: ParserInvocationBrandV1,
    declarations: Vec<ParserCallableParameterDeclarationSourceV1>,
    seen_sites: BTreeSet<(u32, u32)>,
    unsupported_member_gate: bool,
}

impl ParserCallableParameterSourceSessionV1 {
    pub(in crate::parser) fn open(brand: ParserInvocationBrandV1) -> Self {
        Self {
            brand,
            declarations: Vec::new(),
            seen_sites: BTreeSet::new(),
            unsupported_member_gate: false,
        }
    }

    pub(super) fn commit(
        &mut self,
        source_site: SourceBoxMethodSiteV1,
        inventory_ordinal: BoxMethodInventoryOrdinalV1,
        kind: ParserCallableDeclarationKindV1,
        diagnostic_name: String,
        parameters: ParsedCallableParameterListV1,
    ) -> Result<(), CallableParameterSourceIssueV1> {
        if !source_site.is_direct() || !source_site.box_site().path().brand().same_as(&self.brand) {
            return Err(CallableParameterSourceIssueV1::ForeignOrNonDirectMethod);
        }
        let statement = source_site.box_site().statement_ordinal();
        let member = source_site.source_member_ordinal();
        if !self.seen_sites.insert((statement, member)) {
            return Err(CallableParameterSourceIssueV1::DuplicateMethodSite { statement, member });
        }
        let (neutral, rows) = parameters.into_parts();
        if neutral.len() != rows.len()
            || neutral
                .iter()
                .zip(rows.iter())
                .enumerate()
                .any(|(index, (declaration, row))| {
                    row.ordinal() != u32::try_from(index).unwrap_or(u32::MAX)
                        || row.name() != declaration.name
                        || row.declared_type().as_deref()
                            != declaration.declared_type_name.as_deref()
                        || !row.transfer().is_ordinary()
                })
        {
            return Err(CallableParameterSourceIssueV1::ParameterCoverageMismatch {
                statement,
                member,
            });
        }
        self.declarations
            .push(ParserCallableParameterDeclarationSourceV1::new(
                source_site,
                inventory_ordinal,
                kind,
                diagnostic_name,
                rows,
            ));
        Ok(())
    }

    fn finish(
        self,
    ) -> Result<ParserCallableParameterSourceCatalogV1, CallableParameterSourceIssueV1> {
        if self.unsupported_member_gate
            || self.declarations.iter().any(|declaration| {
                declaration.source_site().box_site().path().segments().len() != 1
            })
        {
            return Err(CallableParameterSourceIssueV1::SelectedBuildGateUnsupported);
        }
        Ok(ParserCallableParameterSourceCatalogV1::new(
            self.brand,
            self.declarations.into_boxed_slice(),
        ))
    }
}

impl NyashParser {
    pub(in crate::parser) fn mark_callable_parameter_member_gate_unsupported(&mut self) {
        if let Some(session) = self.callable_parameter_source_session.as_mut() {
            session.unsupported_member_gate = true;
        }
    }

    pub(in crate::parser) fn commit_callable_parameter_source(
        &mut self,
        source_site: SourceBoxMethodSiteV1,
        inventory_ordinal: BoxMethodInventoryOrdinalV1,
        kind: ParserCallableDeclarationKindV1,
        diagnostic_name: String,
        parameters: ParsedCallableParameterListV1,
    ) -> Result<(), ParseError> {
        self.callable_parameter_source_session
            .as_mut()
            .ok_or_else(|| parameter_source_error(CallableParameterSourceIssueV1::SessionClosed))?
            .commit(
                source_site,
                inventory_ordinal,
                kind,
                diagnostic_name,
                parameters,
            )
            .map_err(parameter_source_error)
    }

    pub(in crate::parser) fn finish_callable_parameter_source_catalog(
        &mut self,
    ) -> Result<ParserCallableParameterSourceCatalogV1, ParseError> {
        self.callable_parameter_source_session
            .take()
            .ok_or_else(|| parameter_source_error(CallableParameterSourceIssueV1::SessionClosed))
            .and_then(|session| session.finish().map_err(parameter_source_error))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum CallableParameterSourceIssueV1 {
    SessionClosed,
    ForeignOrNonDirectMethod,
    SelectedBuildGateUnsupported,
    DuplicateMethodSite { statement: u32, member: u32 },
    ParameterCoverageMismatch { statement: u32, member: u32 },
}

fn parameter_source_error(error: CallableParameterSourceIssueV1) -> ParseError {
    ParseError::GrammarContract {
        stable_reject_tag: "parser/callable-parameter-source",
        detail: format!("callable parameter source catalog rejected: {error:?}"),
        line: 0,
    }
}
