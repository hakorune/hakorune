//! Shadow-only resolver output and closed failure vocabulary.

use std::collections::{BTreeMap, BTreeSet};

use super::ids::{ShadowBindingOrdinalV0, ShadowRegionIdV0, ShadowScopeIdV0};
use crate::mir::resolved_semantics::source_site::{
    ResolvedExitSiteV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1, SourceStmtSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShadowBindingKindV0 {
    Receiver,
    Parameter { index: u32 },
    Local { ordinal: u32 },
    Outbox { ordinal: u32 },
    Nowait,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShadowBindingRecordV0 {
    pub(crate) diagnostic_name: Box<str>,
    pub(crate) kind: ShadowBindingKindV0,
    pub(crate) owner_scope: ShadowScopeIdV0,
    pub(crate) origin: SourceBindingSiteV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShadowScopeKindV0 {
    Function,
    LexicalBlock,
    BlockExpr,
    IfThen,
    IfElse,
    LoopBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShadowScopeRecordV0 {
    pub(crate) kind: ShadowScopeKindV0,
    pub(crate) parent: Option<ShadowScopeIdV0>,
    pub(crate) declarations: Box<[ShadowBindingOrdinalV0]>,
    pub(crate) origin: Option<SourceNodeSiteV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShadowRegionKindV0 {
    Function,
    Sequence,
    LexicalScope,
    BlockExpr,
    If,
    IfThen,
    IfElse,
    Loop,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShadowRegionRecordV0 {
    pub(crate) kind: ShadowRegionKindV0,
    pub(crate) parent: Option<ShadowRegionIdV0>,
    pub(crate) lexical_scope: Option<ShadowScopeIdV0>,
    pub(crate) origin: Option<SourceNodeSiteV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ShadowAssignmentTargetV0 {
    BindingRebind(ShadowBindingOrdinalV0),
    AncestorRebind(Box<str>),
    FieldWrite { receiver: SourceExprSiteV1 },
    IndexWrite { receiver: SourceExprSiteV1 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ShadowLexicalRefV0 {
    Local(ShadowBindingOrdinalV0),
    Ancestor(Box<str>),
}

/// Construction-local first-demand order for one ancestor capture.
///
/// This intentionally keeps the ancestor name only until canonicalization;
/// the sealed forest stores BindingRef-based rows instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShadowAncestorCaptureAccessV0 {
    Read,
    Rebind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShadowAncestorCaptureEventV0 {
    pub(crate) site: SourceExprSiteV1,
    pub(crate) name: Box<str>,
    pub(crate) access: ShadowAncestorCaptureAccessV0,
}

/// Positive lexical disposition for one pre-verified qualified receiver site.
///
/// Shadow binding ordinals are intentionally erased: source-call routing needs
/// only the proven presence or proven absence of a lexical binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum ShadowQualifiedReceiverDispositionV0 {
    Bound,
    ProvenUnbound,
}

/// Read-only classification of one exact MethodCall receiver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum ShadowMethodCallReceiverV0 {
    CurrentOwner,
    Qualified(ShadowQualifiedReceiverDispositionV0),
    Dynamic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct ShadowMethodCallObservationV0 {
    receiver_site: SourceExprSiteV1,
    receiver: ShadowMethodCallReceiverV0,
}

impl ShadowMethodCallObservationV0 {
    pub(super) const fn new(
        receiver_site: SourceExprSiteV1,
        receiver: ShadowMethodCallReceiverV0,
    ) -> Self {
        Self {
            receiver_site,
            receiver,
        }
    }

    pub(in crate::mir) const fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    pub(in crate::mir) const fn receiver(&self) -> ShadowMethodCallReceiverV0 {
        self.receiver
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShadowControlExitV0 {
    Continue { target_loop: ShadowRegionIdV0 },
    Break { target_loop: ShadowRegionIdV0 },
    Return { target_function: ShadowRegionIdV0 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShadowExitOriginV0 {
    ExplicitContinue,
    ExplicitBreak,
    ExplicitReturn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ShadowExitRecordV0 {
    pub(crate) source_region: ShadowRegionIdV0,
    pub(crate) origin: ShadowExitOriginV0,
    pub(crate) transfer: ShadowControlExitV0,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShadowDirectCallUseV0 {
    pub(crate) name: Box<str>,
    pub(crate) arity: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ShadowResolveErrorV0 {
    ExpectedFunctionDeclaration,
    SameScopeRedeclaration {
        name: Box<str>,
    },
    UnresolvedName {
        name: Box<str>,
        site: SourceExprSiteV1,
    },
    QualifiedReceiverObservationCoverageMismatch {
        missing: Box<[SourceExprSiteV1]>,
        extra: Box<[SourceExprSiteV1]>,
    },
    DuplicateQualifiedReceiverObservation {
        site: SourceExprSiteV1,
    },
    DuplicateMethodCallObservation {
        site: SourceExprSiteV1,
    },
    ExitOutsideLoop {
        kind: &'static str,
        site: SourceStmtSiteV1,
    },
    UnsupportedStatement {
        kind: &'static str,
        site: SourceStmtSiteV1,
    },
    UnsupportedExpression {
        kind: &'static str,
        site: SourceExprSiteV1,
    },
    UnsupportedAssignmentTarget {
        site: SourceExprSiteV1,
    },
    DuplicateExitSite {
        site: SourceStmtSiteV1,
    },
    DuplicateDirectCallSite {
        site: SourceExprSiteV1,
    },
    DuplicateRecordLiteralDemand {
        site: SourceExprSiteV1,
    },
    DuplicateQMarkPropagation {
        site: SourceExprSiteV1,
    },
    DuplicateMatchControl {
        site: SourceExprSiteV1,
    },
    FunctionCallArityOverflow {
        site: SourceExprSiteV1,
    },
    BlockExprNonLocalExit {
        site: ResolvedExitSiteV1,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct ShadowResolvedFunctionV0 {
    pub(crate) root_profile: super::super::SemanticOwnerRootProfileV1,
    pub(crate) function_scope: ShadowScopeIdV0,
    pub(crate) function_region: ShadowRegionIdV0,
    pub(crate) bindings: BTreeMap<ShadowBindingOrdinalV0, ShadowBindingRecordV0>,
    pub(crate) scopes: BTreeMap<ShadowScopeIdV0, ShadowScopeRecordV0>,
    pub(crate) regions: BTreeMap<ShadowRegionIdV0, ShadowRegionRecordV0>,
    pub(crate) declarations: BTreeMap<SourceBindingSiteV1, ShadowBindingOrdinalV0>,
    pub(crate) variable_uses: BTreeMap<SourceExprSiteV1, ShadowLexicalRefV0>,
    pub(crate) assignment_targets: BTreeMap<SourceExprSiteV1, ShadowAssignmentTargetV0>,
    pub(crate) ancestor_capture_events: Box<[ShadowAncestorCaptureEventV0]>,
    pub(crate) direct_calls: BTreeMap<SourceExprSiteV1, ShadowDirectCallUseV0>,
    pub(crate) resolved_exits: BTreeMap<SourceStmtSiteV1, ShadowExitRecordV0>,
    pub(crate) record_literal_demands: BTreeMap<SourceExprSiteV1, u32>,
    pub(crate) qmark_propagation_sites: BTreeSet<SourceExprSiteV1>,
    pub(crate) match_control_sites: BTreeSet<SourceExprSiteV1>,
}

#[derive(Debug)]
pub(crate) struct ShadowResolvedOwnerV0<'ast> {
    pub(crate) function: ShadowResolvedFunctionV0,
    pub(crate) lambdas: Box<[super::owner_boundary::ShadowLambdaSyntaxV0<'ast>]>,
}
