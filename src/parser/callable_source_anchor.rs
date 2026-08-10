//! Parser-private callable declaration anchors and direct-source staging.
//!
//! The declaration parser issues the anchor.  Names, spans, arity, AST
//! addresses, and numeric path components are diagnostics or placement only;
//! none can recreate anchor identity.

#![allow(dead_code)]

use std::sync::Arc;

use super::common::ParserUtils;
use super::declarations::box_def::members::pending_method::CommittedDirectExplicitMethodV1;
use super::source_authority::ParserInvocationBrandV1;
use super::source_path::SourceProgramCallablePathV1;
use super::{NyashParser, ParseError};
use crate::ast::BoxMethodInventoryOrdinalV1;

#[derive(Debug, Clone)]
pub(super) struct CallableDeclarationAnchorV1(Arc<()>);

impl CallableDeclarationAnchorV1 {
    fn issue() -> Self {
        Self(Arc::new(()))
    }

    pub(super) fn same_as(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DirectCallableDeclarationKindV1 {
    FreeFunction,
    FreeStaticFunction,
    StaticBoxMethod,
    InstanceBoxMethod,
}

#[derive(Debug, Clone)]
pub(super) struct PreparedDirectCallableSourceV1 {
    anchor: CallableDeclarationAnchorV1,
    parser_brand: ParserInvocationBrandV1,
    path: SourceProgramCallablePathV1,
    kind: DirectCallableDeclarationKindV1,
    diagnostic_name: Box<str>,
}

impl PreparedDirectCallableSourceV1 {
    pub(super) fn anchor(&self) -> &CallableDeclarationAnchorV1 {
        &self.anchor
    }

    pub(super) fn parser_brand(&self) -> &ParserInvocationBrandV1 {
        &self.parser_brand
    }

    pub(super) fn path(&self) -> &SourceProgramCallablePathV1 {
        &self.path
    }

    pub(super) fn kind(&self) -> DirectCallableDeclarationKindV1 {
        self.kind
    }

    pub(super) fn diagnostic_name(&self) -> &str {
        &self.diagnostic_name
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum DirectCallableSourceIssueV1 {
    ForeignParser,
    DuplicateAnchor,
    DuplicatePath,
}

/// The sole parser-invocation owner for direct callable anchors.
#[derive(Debug)]
pub(super) struct ParserCallableSourceSessionV1 {
    brand: ParserInvocationBrandV1,
    rows: Vec<PreparedDirectCallableSourceV1>,
}

impl ParserCallableSourceSessionV1 {
    pub(super) fn open(brand: ParserInvocationBrandV1) -> Self {
        Self {
            brand,
            rows: Vec::new(),
        }
    }

    fn prepare_direct(
        &self,
        path: SourceProgramCallablePathV1,
        kind: DirectCallableDeclarationKindV1,
        diagnostic_name: impl Into<Box<str>>,
    ) -> Result<PreparedDirectCallableSourceV1, DirectCallableSourceIssueV1> {
        if !path.declaration().brand().same_as(&self.brand) {
            return Err(DirectCallableSourceIssueV1::ForeignParser);
        }
        Ok(PreparedDirectCallableSourceV1 {
            anchor: CallableDeclarationAnchorV1::issue(),
            parser_brand: self.brand.clone(),
            path,
            kind,
            diagnostic_name: diagnostic_name.into(),
        })
    }

    fn commit_direct(
        &mut self,
        prepared: PreparedDirectCallableSourceV1,
    ) -> Result<(), DirectCallableSourceIssueV1> {
        if !prepared.parser_brand.same_as(&self.brand)
            || !prepared.path.declaration().brand().same_as(&self.brand)
        {
            return Err(DirectCallableSourceIssueV1::ForeignParser);
        }
        if self
            .rows
            .iter()
            .any(|row| row.anchor.same_as(&prepared.anchor))
        {
            return Err(DirectCallableSourceIssueV1::DuplicateAnchor);
        }
        if self.rows.iter().any(|row| row.path == prepared.path) {
            return Err(DirectCallableSourceIssueV1::DuplicatePath);
        }
        self.rows.push(prepared);
        Ok(())
    }

    fn issue_direct(
        &mut self,
        path: SourceProgramCallablePathV1,
        kind: DirectCallableDeclarationKindV1,
        diagnostic_name: impl Into<Box<str>>,
    ) -> Result<(), DirectCallableSourceIssueV1> {
        let prepared = self.prepare_direct(path, kind, diagnostic_name)?;
        self.commit_direct(prepared)
    }

    pub(super) fn rows(&self) -> &[PreparedDirectCallableSourceV1] {
        &self.rows
    }
}

pub(super) fn map_issue(error: DirectCallableSourceIssueV1, line: usize) -> ParseError {
    let detail = match error {
        DirectCallableSourceIssueV1::ForeignParser => {
            "direct callable source belongs to another parser invocation"
        }
        DirectCallableSourceIssueV1::DuplicateAnchor => {
            "direct callable anchor was committed more than once"
        }
        DirectCallableSourceIssueV1::DuplicatePath => {
            "direct callable source path was committed more than once"
        }
    };
    ParseError::GrammarContract {
        stable_reject_tag: "parser/direct-callable-source",
        detail: detail.to_owned(),
        line,
    }
}

impl NyashParser {
    fn issue_direct_top_level_callable(
        &mut self,
        kind: DirectCallableDeclarationKindV1,
        diagnostic_name: impl Into<Box<str>>,
    ) -> Result<(), ParseError> {
        let line = self.current_token().line;
        let path = self
            .active_source_declaration_path()
            .cloned()
            .ok_or_else(|| ParseError::GrammarContract {
                stable_reject_tag: "parser/direct-callable-source",
                detail: "top-level callable requires an active parser source path".to_owned(),
                line,
            })?;
        self.callable_source_session
            .issue_direct(
                SourceProgramCallablePathV1::top_level(path),
                kind,
                diagnostic_name,
            )
            .map_err(|error| map_issue(error, line))
    }

    pub(super) fn issue_direct_free_function(
        &mut self,
        diagnostic_name: impl Into<Box<str>>,
    ) -> Result<(), ParseError> {
        self.issue_direct_top_level_callable(
            DirectCallableDeclarationKindV1::FreeFunction,
            diagnostic_name,
        )
    }

    pub(super) fn issue_direct_free_static_function(
        &mut self,
        diagnostic_name: impl Into<Box<str>>,
    ) -> Result<(), ParseError> {
        self.issue_direct_top_level_callable(
            DirectCallableDeclarationKindV1::FreeStaticFunction,
            diagnostic_name,
        )
    }

    fn issue_committed_explicit_box_method(
        &mut self,
        kind: DirectCallableDeclarationKindV1,
        committed: CommittedDirectExplicitMethodV1,
    ) -> Result<
        (
            BoxMethodInventoryOrdinalV1,
            String,
            Option<super::callable_parameter_source::ParsedCallableParameterListV1>,
        ),
        ParseError,
    > {
        let line = self.current_token().line;
        let (path, inventory_ordinal, diagnostic_name, parameter_source) = committed.into_parts();
        self.callable_source_session
            .issue_direct(path, kind, diagnostic_name.clone())
            .map_err(|error| map_issue(error, line))?;
        Ok((inventory_ordinal, diagnostic_name, parameter_source))
    }

    pub(super) fn issue_committed_instance_box_method(
        &mut self,
        committed: CommittedDirectExplicitMethodV1,
    ) -> Result<
        (
            BoxMethodInventoryOrdinalV1,
            String,
            Option<super::callable_parameter_source::ParsedCallableParameterListV1>,
        ),
        ParseError,
    > {
        self.issue_committed_explicit_box_method(
            DirectCallableDeclarationKindV1::InstanceBoxMethod,
            committed,
        )
    }

    pub(super) fn issue_committed_static_box_method(
        &mut self,
        committed: CommittedDirectExplicitMethodV1,
    ) -> Result<
        (
            BoxMethodInventoryOrdinalV1,
            String,
            Option<super::callable_parameter_source::ParsedCallableParameterListV1>,
        ),
        ParseError,
    > {
        self.issue_committed_explicit_box_method(
            DirectCallableDeclarationKindV1::StaticBoxMethod,
            committed,
        )
    }
}

#[cfg(test)]
#[path = "callable_source_anchor_tests.rs"]
mod tests;
