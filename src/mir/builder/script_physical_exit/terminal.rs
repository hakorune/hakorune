use crate::mir::raw_root_body_recipe::{RawRootBodySourceSiteV1, RawScriptUnitOriginV1};
use crate::mir::ValueId;

/// Physical operands produced for one already source-classified Script
/// terminal. This owns no Return, signature, completion tracker, or route.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum LoweredScriptTerminalV1 {
    Value {
        value: ValueId,
    },
    Unit {
        origin: RawScriptUnitOriginV1,
        payload: LoweredScriptUnitPayloadV1,
    },
}

/// Whether a Script Unit terminal already has its exact physical Void operand
/// or requires the later physical-exit owner to synthesize one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum LoweredScriptUnitPayloadV1 {
    ExistingVoid { value: ValueId },
    SyntheticVoid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum ScriptRecipeLoweringOperationV1 {
    PreludeStatement,
    TerminalValueExpression,
    TerminalUnitExpression,
    TerminalUnitStatement,
}

/// Typed lowering failure retained by the shared Script terminal kernel.
/// Raw BODY temporarily adapts it to its legacy error surface; canonical
/// Script later retains this exact cause through its candidate owner.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::mir) struct ScriptRecipeLoweringErrorV1 {
    operation: ScriptRecipeLoweringOperationV1,
    site: RawRootBodySourceSiteV1,
    detail: Box<str>,
}

impl ScriptRecipeLoweringErrorV1 {
    pub(in crate::mir::builder) fn new(
        operation: ScriptRecipeLoweringOperationV1,
        site: RawRootBodySourceSiteV1,
        detail: impl Into<Box<str>>,
    ) -> Self {
        Self {
            operation,
            site,
            detail: detail.into(),
        }
    }

    pub(in crate::mir) const fn operation(&self) -> ScriptRecipeLoweringOperationV1 {
        self.operation
    }

    pub(in crate::mir) fn site(&self) -> &RawRootBodySourceSiteV1 {
        &self.site
    }

    pub(in crate::mir) fn detail(&self) -> &str {
        &self.detail
    }
}

impl std::fmt::Display for ScriptRecipeLoweringErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[script/recipe-lowering] operation={:?} path={:?}: {}",
            self.operation,
            self.site.path(),
            self.detail
        )
    }
}

impl std::error::Error for ScriptRecipeLoweringErrorV1 {}
