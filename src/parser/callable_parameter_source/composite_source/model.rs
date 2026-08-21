use super::super::super::callable_source_anchor::CallableDeclarationIdentityV1;
use super::super::super::source_authority::SourceBoxMethodSiteV1;
use super::super::parser_invocation_witness::ParserInvocationWitnessV1;

#[derive(Debug)]
pub(crate) enum ParserCompositeSourceDispositionV1 {
    Ready(ParserCompositeSourcePreservationV1),
    OutsideBoundedCohort(ParserCompositeOutsideReasonV1),
    SourceAuthorityUnavailable(ParserCompositeSourceUnavailableV1),
    Incomplete(ParserCompositeIncompleteV1),
    IntegrityInvalid(ParserCompositeIntegrityIssueV1),
}

impl ParserCompositeSourceDispositionV1 {
    pub(crate) fn is_ready(&self) -> bool {
        matches!(self, Self::Ready(_))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserCompositeOutsideReasonV1 {
    NoStaticProvider,
    MultipleCallableProviders,
    ProviderOutsideBoundedCohort,
    ProviderMethodCountOutsideBoundedCohort,
    TerminalOutsideBoundedCohort,
    ReceiverOutsideBoundedCohort,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserCompositeSourceUnavailableV1 {
    PostpassNotSourceBacked,
    ParameterSourceUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserCompositeIncompleteV1 {
    ProgramBodyMissing,
    ProgramStatementOrdinalOverflow,
    ProviderDeclarationMissing,
    RootTerminalValueMissing,
    ReceiverMissing,
    ArgumentCoverageMissing,
    ArgumentOrdinalOverflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserCompositeIntegrityIssueV1 {
    ForeignParser,
    ProviderPlacementMismatch,
    ProviderAnchorMismatch,
    DuplicateProvider,
    CallTreeContradiction,
}

#[derive(Debug)]
pub(crate) struct ParserCompositeSourcePreservationV1 {
    invocation: ParserInvocationWitnessV1,
    provider: ParserCompositeStaticProviderV1,
    terminal: ParserCompositeRootTerminalV1,
    _seal: ParserCompositeSourcePreservationSealV1,
}

#[derive(Debug)]
pub(super) struct ParserCompositeSourcePreservationSealV1;

#[derive(Debug)]
pub(super) struct ParserCompositeStaticProviderV1 {
    statement: u32,
    method_inventory: u32,
    identity: CallableDeclarationIdentityV1,
    source_site: SourceBoxMethodSiteV1,
    diagnostic_name: Box<str>,
    result_syntax: ParserCompositeResultSyntaxV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ParserCompositeResultSyntaxV1 {
    Implicit,
    Explicit(Box<str>),
}

#[derive(Debug)]
pub(super) enum ParserCompositeRootTerminalV1 {
    FinalSequence {
        statement: u32,
        call: ParserCompositeRootMethodCallV1,
    },
    RootReturn {
        statement: u32,
        call: ParserCompositeRootMethodCallV1,
    },
}

#[derive(Debug)]
pub(super) struct ParserCompositeRootMethodCallV1 {
    method: Box<str>,
    receiver: ParserCompositeReceiverV1,
    arguments: Box<[ParserCompositeArgumentV1]>,
    result: ParserCompositeCallResultV1,
}

#[derive(Debug)]
pub(super) struct ParserCompositeReceiverV1 {
    diagnostic_name: Box<str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ParserCompositeCallResultV1 {
    ThisMethodCall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ParserCompositeArgumentV1 {
    ordinal: u32,
}

impl ParserCompositeSourcePreservationV1 {
    pub(super) fn issue(
        invocation: ParserInvocationWitnessV1,
        provider: ParserCompositeStaticProviderV1,
        terminal: ParserCompositeRootTerminalV1,
    ) -> Self {
        Self {
            invocation,
            provider,
            terminal,
            _seal: ParserCompositeSourcePreservationSealV1,
        }
    }

    pub(super) fn provider(&self) -> &ParserCompositeStaticProviderV1 {
        &self.provider
    }

    pub(super) fn terminal(&self) -> &ParserCompositeRootTerminalV1 {
        &self.terminal
    }

    pub(super) fn invocation(&self) -> &ParserInvocationWitnessV1 {
        &self.invocation
    }
}

impl ParserCompositeStaticProviderV1 {
    pub(super) fn new(
        statement: u32,
        method_inventory: u32,
        identity: CallableDeclarationIdentityV1,
        source_site: SourceBoxMethodSiteV1,
        diagnostic_name: impl Into<Box<str>>,
        result_syntax: ParserCompositeResultSyntaxV1,
    ) -> Self {
        Self {
            statement,
            method_inventory,
            identity,
            source_site,
            diagnostic_name: diagnostic_name.into(),
            result_syntax,
        }
    }

    pub(super) const fn statement(&self) -> u32 {
        self.statement
    }

    pub(super) const fn method_inventory(&self) -> u32 {
        self.method_inventory
    }

    pub(super) fn identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.identity
    }

    pub(super) fn source_site(&self) -> &SourceBoxMethodSiteV1 {
        &self.source_site
    }

    pub(super) fn diagnostic_name(&self) -> &str {
        &self.diagnostic_name
    }

    pub(super) fn result_syntax(&self) -> &ParserCompositeResultSyntaxV1 {
        &self.result_syntax
    }
}

impl ParserCompositeRootTerminalV1 {
    pub(super) const fn statement(&self) -> u32 {
        match self {
            Self::FinalSequence { statement, .. } | Self::RootReturn { statement, .. } => {
                *statement
            }
        }
    }

    pub(super) fn call(&self) -> &ParserCompositeRootMethodCallV1 {
        match self {
            Self::FinalSequence { call, .. } | Self::RootReturn { call, .. } => call,
        }
    }

    pub(super) const fn is_root_return(&self) -> bool {
        matches!(self, Self::RootReturn { .. })
    }
}

impl ParserCompositeRootMethodCallV1 {
    pub(super) fn new(
        method: impl Into<Box<str>>,
        receiver: ParserCompositeReceiverV1,
        arguments: Box<[ParserCompositeArgumentV1]>,
    ) -> Self {
        Self {
            method: method.into(),
            receiver,
            arguments,
            result: ParserCompositeCallResultV1::ThisMethodCall,
        }
    }

    pub(super) fn method(&self) -> &str {
        &self.method
    }

    pub(super) fn receiver(&self) -> &ParserCompositeReceiverV1 {
        &self.receiver
    }

    pub(super) fn arguments(&self) -> &[ParserCompositeArgumentV1] {
        &self.arguments
    }

    pub(super) const fn result(&self) -> ParserCompositeCallResultV1 {
        self.result
    }
}

impl ParserCompositeReceiverV1 {
    pub(super) fn new(diagnostic_name: impl Into<Box<str>>) -> Self {
        Self {
            diagnostic_name: diagnostic_name.into(),
        }
    }

    pub(super) fn diagnostic_name(&self) -> &str {
        &self.diagnostic_name
    }
}

impl ParserCompositeArgumentV1 {
    pub(super) const fn new(ordinal: u32) -> Self {
        Self { ordinal }
    }

    pub(super) const fn ordinal(self) -> u32 {
        self.ordinal
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParserCompositeTransformRejectV1 {
    WitnessChanged,
    ProviderChanged,
    ProviderResultChanged,
    RootCallChanged,
    ReceiverChanged,
    ArgumentCardinalityChanged { expected: u32, actual: u32 },
    ArgumentOrderChanged { ordinal: u32 },
    ArgumentChanged { ordinal: u32 },
    ResultChanged,
    TerminalChanged,
    CompositeDropped,
    CompatibilityLoss,
}
