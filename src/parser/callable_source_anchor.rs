//! Parser-private callable declaration anchors and direct-source staging.
//!
//! The declaration parser issues the anchor.  Names, spans, arity, AST
//! addresses, and numeric path components are diagnostics or placement only;
//! none can recreate anchor identity.

#![allow(dead_code)]

use std::sync::Arc;

use super::common::ParserUtils;
use super::declarations::box_def::members::pending_method::CommittedDirectExplicitMethodV1;
use super::delegate_source_relation::GeneratedDelegateSourceRelationV1;
use super::source_authority::ParserInvocationBrandV1;
use super::source_path::SourceProgramCallablePathV1;
use super::{NyashParser, ParseError};
use crate::ast::{
    BoxMethodGeneratedProvenanceV1, BoxMethodInventoryOrdinalV1,
    BoxMethodInventoryPlacementReceiptV1,
};

#[derive(Debug)]
pub(super) struct CallableDeclarationAnchorV1(Arc<()>);

/// Cloneable comparison-only view of one parser-issued declaration anchor.
///
/// This value proves only same-anchor origin. It carries no syntax, source
/// coordinate, callable key, resolver owner, or lowering authority.
#[derive(Debug, Clone)]
pub(crate) struct CallableDeclarationIdentityV1(Arc<()>);

impl CallableDeclarationAnchorV1 {
    pub(super) fn issue() -> Self {
        Self(Arc::new(()))
    }

    pub(super) fn same_as(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }

    pub(super) fn identity(&self) -> CallableDeclarationIdentityV1 {
        CallableDeclarationIdentityV1(Arc::clone(&self.0))
    }
}

impl CallableDeclarationIdentityV1 {
    pub(crate) fn same_as(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct GeneratedPropertyCallableOriginV1 {
    source_path: SourceProgramCallablePathV1,
    placement: BoxMethodInventoryPlacementReceiptV1,
    provenance: BoxMethodGeneratedProvenanceV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct GeneratedDelegateCallableOriginV1 {
    source_path: SourceProgramCallablePathV1,
    relation: GeneratedDelegateSourceRelationV1,
}

impl GeneratedDelegateCallableOriginV1 {
    pub(super) fn new(
        source_path: SourceProgramCallablePathV1,
        relation: GeneratedDelegateSourceRelationV1,
    ) -> Self {
        Self {
            source_path,
            relation,
        }
    }

    pub(super) fn source_path(&self) -> &SourceProgramCallablePathV1 {
        &self.source_path
    }

    pub(super) fn relation(&self) -> &GeneratedDelegateSourceRelationV1 {
        &self.relation
    }
}

impl GeneratedPropertyCallableOriginV1 {
    pub(super) fn new(
        source_path: SourceProgramCallablePathV1,
        placement: BoxMethodInventoryPlacementReceiptV1,
        provenance: BoxMethodGeneratedProvenanceV1,
    ) -> Self {
        Self {
            source_path,
            placement,
            provenance,
        }
    }

    pub(super) fn source_path(&self) -> &SourceProgramCallablePathV1 {
        &self.source_path
    }

    pub(super) fn placement(&self) -> &BoxMethodInventoryPlacementReceiptV1 {
        &self.placement
    }

    pub(super) fn provenance(&self) -> &BoxMethodGeneratedProvenanceV1 {
        &self.provenance
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum GeneratedCallableOriginV1 {
    Property(GeneratedPropertyCallableOriginV1),
    Delegate(GeneratedDelegateCallableOriginV1),
}

#[derive(Debug)]
pub(super) struct PreparedGeneratedCallableSourceV1 {
    anchor: CallableDeclarationAnchorV1,
    parser_brand: ParserInvocationBrandV1,
    origin: GeneratedCallableOriginV1,
    diagnostic_name: Box<str>,
}

impl PreparedGeneratedCallableSourceV1 {
    pub(super) fn issue(
        parser_brand: ParserInvocationBrandV1,
        origin: GeneratedCallableOriginV1,
        diagnostic_name: impl Into<Box<str>>,
    ) -> Self {
        Self {
            anchor: CallableDeclarationAnchorV1::issue(),
            parser_brand,
            origin,
            diagnostic_name: diagnostic_name.into(),
        }
    }

    pub(super) fn anchor(&self) -> &CallableDeclarationAnchorV1 {
        &self.anchor
    }

    pub(super) fn parser_brand(&self) -> &ParserInvocationBrandV1 {
        &self.parser_brand
    }

    pub(super) fn origin(&self) -> &GeneratedCallableOriginV1 {
        &self.origin
    }

    pub(super) fn diagnostic_name(&self) -> &str {
        &self.diagnostic_name
    }
}

#[derive(Debug)]
pub(super) enum PreparedCallableSourceV1 {
    Direct(PreparedDirectCallableSourceV1),
    Generated(PreparedGeneratedCallableSourceV1),
}

impl PreparedCallableSourceV1 {
    pub(super) fn anchor(&self) -> &CallableDeclarationAnchorV1 {
        match self {
            Self::Direct(row) => row.anchor(),
            Self::Generated(row) => row.anchor(),
        }
    }

    pub(super) fn parser_brand(&self) -> &ParserInvocationBrandV1 {
        match self {
            Self::Direct(row) => row.parser_brand(),
            Self::Generated(row) => row.parser_brand(),
        }
    }

    pub(super) fn diagnostic_name(&self) -> &str {
        match self {
            Self::Direct(row) => row.diagnostic_name(),
            Self::Generated(row) => row.diagnostic_name(),
        }
    }

    pub(super) fn direct(&self) -> Option<&PreparedDirectCallableSourceV1> {
        match self {
            Self::Direct(row) => Some(row),
            Self::Generated(_) => None,
        }
    }

    pub(super) fn generated(&self) -> Option<&PreparedGeneratedCallableSourceV1> {
        match self {
            Self::Direct(_) => None,
            Self::Generated(row) => Some(row),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DirectCallableDeclarationKindV1 {
    FreeFunction,
    FreeStaticFunction,
    StaticBoxMethod,
    InstanceBoxMethod,
}

/// Placement captured by the declaration commit that issued the anchor.
///
/// This is never callable identity. A selected member-gate method may be
/// rebased when its branch inventory is merged, so the final co-seal must use
/// the exact gate/source relation for that case instead of trusting this
/// branch-local commit ordinal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DirectCallableCommitPlacementV1 {
    TopLevel,
    BoxMethod {
        committed_inventory: BoxMethodInventoryOrdinalV1,
    },
}

#[derive(Debug)]
pub(super) struct PreparedDirectCallableSourceV1 {
    anchor: CallableDeclarationAnchorV1,
    parser_brand: ParserInvocationBrandV1,
    path: SourceProgramCallablePathV1,
    kind: DirectCallableDeclarationKindV1,
    commit_placement: DirectCallableCommitPlacementV1,
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

    pub(super) fn commit_placement(&self) -> DirectCallableCommitPlacementV1 {
        self.commit_placement
    }

    pub(super) fn diagnostic_name(&self) -> &str {
        &self.diagnostic_name
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum DirectCallableSourceIssueV1 {
    ForeignParser,
    DuplicatePath,
    SessionAlreadyMoved,
}

/// The sole parser-invocation owner for direct callable anchors.
#[derive(Debug)]
pub(super) struct ParserCallableSourceSessionV1 {
    brand: ParserInvocationBrandV1,
    rows: Vec<PreparedDirectCallableSourceV1>,
    static_box_sources: Vec<
        super::callable_parameter_source::static_box_source::PreparedParserStaticBoxParentSourceV1,
    >,
}

impl ParserCallableSourceSessionV1 {
    pub(super) fn open(brand: ParserInvocationBrandV1) -> Self {
        Self {
            brand,
            rows: Vec::new(),
            static_box_sources: Vec::new(),
        }
    }

    fn prepare_direct(
        &self,
        path: SourceProgramCallablePathV1,
        kind: DirectCallableDeclarationKindV1,
        commit_placement: DirectCallableCommitPlacementV1,
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
            commit_placement,
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
        commit_placement: DirectCallableCommitPlacementV1,
        diagnostic_name: impl Into<Box<str>>,
    ) -> Result<(), DirectCallableSourceIssueV1> {
        let prepared = self.prepare_direct(path, kind, commit_placement, diagnostic_name)?;
        self.commit_direct(prepared)
    }

    pub(super) fn rows(&self) -> &[PreparedDirectCallableSourceV1] {
        &self.rows
    }

    pub(super) fn into_rows(self) -> Vec<PreparedDirectCallableSourceV1> {
        self.rows
    }

    pub(super) fn into_postpass_parts(
        self,
    ) -> (
        Vec<PreparedDirectCallableSourceV1>,
        Vec<super::callable_parameter_source::static_box_source::PreparedParserStaticBoxParentSourceV1>,
    ) {
        (self.rows, self.static_box_sources)
    }

    pub(super) fn register_static_box_source(
        &mut self,
        prepared: super::callable_parameter_source::static_box_source::PreparedParserStaticBoxParentSourceV1,
    ) {
        self.static_box_sources.push(prepared);
    }
}

pub(super) fn map_issue(error: DirectCallableSourceIssueV1, line: usize) -> ParseError {
    let detail = match error {
        DirectCallableSourceIssueV1::ForeignParser => {
            "direct callable source belongs to another parser invocation"
        }
        DirectCallableSourceIssueV1::DuplicatePath => {
            "direct callable source path was committed more than once"
        }
        DirectCallableSourceIssueV1::SessionAlreadyMoved => {
            "direct callable source session was already moved into postpass"
        }
    };
    ParseError::GrammarContract {
        stable_reject_tag: "parser/direct-callable-source",
        detail: detail.to_owned(),
        line,
    }
}

impl NyashParser {
    pub(super) fn register_prepared_static_box_source(
        &mut self,
        prepared: super::callable_parameter_source::static_box_source::PreparedParserStaticBoxParentSourceV1,
    ) -> Result<(), ParseError> {
        let line = self.current_token().line;
        self.callable_source_session
            .as_mut()
            .ok_or(DirectCallableSourceIssueV1::SessionAlreadyMoved)
            .map_err(|error| map_issue(error, line))?
            .register_static_box_source(prepared);
        Ok(())
    }

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
            .as_mut()
            .ok_or(DirectCallableSourceIssueV1::SessionAlreadyMoved)
            .map_err(|error| map_issue(error, line))?
            .issue_direct(
                SourceProgramCallablePathV1::top_level(path),
                kind,
                DirectCallableCommitPlacementV1::TopLevel,
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
            .as_mut()
            .ok_or(DirectCallableSourceIssueV1::SessionAlreadyMoved)
            .map_err(|error| map_issue(error, line))?
            .issue_direct(
                path,
                kind,
                DirectCallableCommitPlacementV1::BoxMethod {
                    committed_inventory: inventory_ordinal,
                },
                diagnostic_name.clone(),
            )
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
