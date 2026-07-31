//! Neutral, total source window for one Script-root semantic traversal.
//!
//! The builder produces this receipt from its already-classified Program work
//! plan.  The shadow resolver consumes it without importing builder types or
//! reconstructing the Program partition.

use crate::mir::resolved_semantics::{SourcePathSegmentV1, SourceStmtSiteV1};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ScriptRootSemanticDispositionV1 {
    Resolved,
    Deferred(ScriptDeferredBoundaryV1),
    Transparent(ScriptTransparentBoundaryV1),
    Transferred(ScriptTransferredBoundaryV1),
    Diagnostic(ScriptDiagnosticBoundaryV1),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ScriptDeferredBoundaryV1 {
    ExistingRuntimeResponsibility,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ScriptTransparentBoundaryV1 {
    UsingDirective,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ScriptTransferredBoundaryV1 {
    ProgramStaticMetadata,
    TopLevelCallable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ScriptDiagnosticBoundaryV1 {
    ExistingSelectedUnsupported,
    ExistingReceiverAbsent,
    ExistingBareThisUnsupported,
    ExistingContextScopeUnsupported,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ScriptRootRuntimeDispositionV1 {
    None,
    RetainedExistingTerminal,
}

/// One exact Program statement and its already-chosen semantic/runtime roles.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct VerifiedScriptRootDemandEntryV1 {
    site: SourceStmtSiteV1,
    semantic: ScriptRootSemanticDispositionV1,
    runtime: ScriptRootRuntimeDispositionV1,
}

impl VerifiedScriptRootDemandEntryV1 {
    pub(crate) fn new(
        site: SourceStmtSiteV1,
        semantic: ScriptRootSemanticDispositionV1,
        runtime: ScriptRootRuntimeDispositionV1,
    ) -> Self {
        Self {
            site,
            semantic,
            runtime,
        }
    }

    pub(crate) const fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(crate) const fn semantic(&self) -> ScriptRootSemanticDispositionV1 {
        self.semantic
    }

    pub(crate) const fn runtime(&self) -> ScriptRootRuntimeDispositionV1 {
        self.runtime
    }
}

/// A source-only, ordinal-complete Script root input.
#[derive(Debug)]
pub(crate) struct VerifiedScriptRootDemandWindowV1 {
    entries: Box<[VerifiedScriptRootDemandEntryV1]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ScriptRootDemandWindowSealErrorV1 {
    NonProgramStatementSite,
    NonCanonicalOrder,
    IncompleteCoverage,
}

impl VerifiedScriptRootDemandWindowV1 {
    /// Seals exactly `statement_count` ProgramBody(original ordinal) entries.
    /// The caller may not use a compact runtime index as source identity.
    pub(crate) fn seal(
        entries: Vec<VerifiedScriptRootDemandEntryV1>,
        statement_count: usize,
    ) -> Result<Self, ScriptRootDemandWindowSealErrorV1> {
        if entries.len() != statement_count {
            return Err(ScriptRootDemandWindowSealErrorV1::IncompleteCoverage);
        }
        for (expected, entry) in entries.iter().enumerate() {
            match program_statement_index(entry.site()) {
                Some(actual) if actual == expected => {}
                Some(_) => return Err(ScriptRootDemandWindowSealErrorV1::NonCanonicalOrder),
                None => return Err(ScriptRootDemandWindowSealErrorV1::NonProgramStatementSite),
            }
        }
        Ok(Self {
            entries: entries.into_boxed_slice(),
        })
    }

    pub(crate) fn entries(&self) -> &[VerifiedScriptRootDemandEntryV1] {
        &self.entries
    }

    pub(crate) fn entry_at(&self, ordinal: usize) -> Option<&VerifiedScriptRootDemandEntryV1> {
        self.entries.get(ordinal)
    }
}

fn program_statement_index(site: &SourceStmtSiteV1) -> Option<usize> {
    match site.node().segments() {
        [SourcePathSegmentV1::ProgramBodyRoot, SourcePathSegmentV1::ProgramBody(index)] => {
            Some(*index as usize)
        }
        _ => None,
    }
}
