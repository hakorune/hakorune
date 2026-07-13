//! Shadow-only resolver output and closed failure vocabulary.

use std::collections::BTreeMap;

use super::ids::{ShadowBindingOrdinalV0, ShadowRegionIdV0, ShadowScopeIdV0};
use crate::mir::resolved_semantics::source_site::{
    FunctionOriginV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1, SourceStmtSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShadowBindingKindV0 {
    Receiver,
    Parameter { index: u32 },
    Local { ordinal: u32 },
    Outbox { ordinal: u32 },
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
    FieldWrite { receiver: SourceExprSiteV1 },
    IndexWrite { receiver: SourceExprSiteV1 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShadowControlExitV0 {
    Continue { target_loop: ShadowRegionIdV0 },
    Break { target_loop: ShadowRegionIdV0 },
    Return { target_function: ShadowRegionIdV0 },
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
}

#[derive(Debug, Clone)]
pub(crate) struct ShadowResolvedFunctionV0 {
    pub(crate) function_origin: FunctionOriginV1,
    pub(crate) function_scope: ShadowScopeIdV0,
    pub(crate) function_region: ShadowRegionIdV0,
    pub(crate) bindings: BTreeMap<ShadowBindingOrdinalV0, ShadowBindingRecordV0>,
    pub(crate) scopes: BTreeMap<ShadowScopeIdV0, ShadowScopeRecordV0>,
    pub(crate) regions: BTreeMap<ShadowRegionIdV0, ShadowRegionRecordV0>,
    pub(crate) declarations: BTreeMap<SourceBindingSiteV1, ShadowBindingOrdinalV0>,
    pub(crate) variable_uses: BTreeMap<SourceExprSiteV1, ShadowBindingOrdinalV0>,
    pub(crate) assignment_targets: BTreeMap<SourceExprSiteV1, ShadowAssignmentTargetV0>,
    pub(crate) control_exits: BTreeMap<SourceStmtSiteV1, ShadowControlExitV0>,
}
