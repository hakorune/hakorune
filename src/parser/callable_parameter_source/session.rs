use crate::ast::BoxMethodInventoryOrdinalV1;
use crate::parser::callable_source_anchor::CallableDeclarationIdentityV1;
use crate::parser::source_authority::{ParserInvocationBrandV1, SourceBoxMethodSiteV1};
use crate::parser::{NyashParser, ParseError};

use super::catalog::{
    ParserCallableParameterSourceCatalogV1, ParserCallableParameterSourceDispositionV1,
};
use super::model::{ParserCallableDeclarationKindV1, ParserCallableParameterDeclarationSourceV1};
use super::parse_product::ParsedCallableParameterListV1;

#[derive(Debug)]
pub(in crate::parser) struct ParserCallableParameterSourceSessionV1 {
    brand: ParserInvocationBrandV1,
    declarations: Vec<ParserCallableParameterDeclarationSourceV1>,
    seen_sites: Vec<SourceBoxMethodSiteV1>,
    unsupported_member_gate: bool,
}

impl ParserCallableParameterSourceSessionV1 {
    pub(in crate::parser) fn open(brand: ParserInvocationBrandV1) -> Self {
        Self {
            brand,
            declarations: Vec::new(),
            seen_sites: Vec::new(),
            unsupported_member_gate: false,
        }
    }

    pub(super) fn commit(
        &mut self,
        source_site: SourceBoxMethodSiteV1,
        inventory_ordinal: BoxMethodInventoryOrdinalV1,
        callable_identity: CallableDeclarationIdentityV1,
        kind: ParserCallableDeclarationKindV1,
        diagnostic_name: String,
        parameters: ParsedCallableParameterListV1,
    ) -> Result<(), CallableParameterSourceIssueV1> {
        if !source_site.is_direct() || !source_site.box_site().path().brand().same_as(&self.brand) {
            return Err(CallableParameterSourceIssueV1::ForeignOrNonDirectMethod);
        }
        let statement = source_site.box_site().statement_ordinal();
        let member = source_site.source_member_ordinal();
        if self.seen_sites.iter().any(|seen| seen == &source_site) {
            return Err(CallableParameterSourceIssueV1::DuplicateMethodSite { statement, member });
        }
        self.seen_sites.push(source_site.clone());
        let (neutral, rows, borrowed_result) = parameters.into_parts();
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
                callable_identity,
                kind,
                diagnostic_name,
                rows,
                borrowed_result,
            ));
        Ok(())
    }

    fn has_from(&self) -> bool {
        self.declarations.iter().any(|declaration| declaration.borrowed_result().is_some())
    }
    fn requires_retained_source(&self) -> bool {
        self.declarations.iter().any(|declaration| declaration.requires_retained_source())
    }

    fn finish_for_normal(self) -> Result<ParserCallableParameterSourceDispositionV1, ParseError> {
        let unsupported = self.unsupported_member_gate
            || self.declarations.iter().any(|declaration| {
                declaration.source_site().box_site().path().segments().len() != 1
            });
        if unsupported && self.requires_retained_source() {
            return Err(super::source_contract_required(self.has_from(),
                "ownership signatures require a complete retained callable source catalog", 0));
        }
        Ok(if unsupported {
            ParserCallableParameterSourceDispositionV1::SelectedBuildGateUnsupported
        } else {
            ParserCallableParameterSourceDispositionV1::Complete(
                ParserCallableParameterSourceCatalogV1::new(self.brand, self.declarations.into_boxed_slice()),
            )
        })
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
        callable_identity: CallableDeclarationIdentityV1,
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
                callable_identity,
                kind,
                diagnostic_name,
                parameters,
            )
            .map_err(parameter_source_error)
    }

    /// AST-only projections must not erase an explicit ownership declaration.
    pub(in crate::parser) fn reject_discarded_parameter_transfers(&self) -> Result<(), ParseError> {
        if let Some(session) = self.callable_parameter_source_session.as_ref() {
            if session.requires_retained_source() {
                return Err(super::source_contract_required(session.has_from(),
                    "ownership signatures cannot be projected to an AST-only callable", 0));
            }
        }
        Ok(())
    }

    pub(in crate::parser) fn finish_callable_parameter_source_for_normal(
        &mut self,
    ) -> Result<ParserCallableParameterSourceDispositionV1, ParseError> {
        self.callable_parameter_source_session
            .take()
            .ok_or_else(|| parameter_source_error(CallableParameterSourceIssueV1::SessionClosed))
            .and_then(ParserCallableParameterSourceSessionV1::finish_for_normal)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum CallableParameterSourceIssueV1 {
    SessionClosed,
    ForeignOrNonDirectMethod,
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
