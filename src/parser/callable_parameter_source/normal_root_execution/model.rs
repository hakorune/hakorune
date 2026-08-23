use crate::parser::callable_source_anchor::CallableDeclarationIdentityV1;

use super::super::normal_source_plan_surface::{
    ParserBackedNormalSourcePlanBoundV1, ParserNormalSourcePlanSurfaceIncompleteV1,
    ParserNormalSourcePlanSurfaceIntegrityIssueV1, ParserNormalSourcePlanSurfaceUnavailableV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalRootExecutionRoleV1 {
    App,
    ProgramRuntime,
}

#[derive(Debug)]
pub(crate) struct ParserNormalAppExecutionRelationV1 {
    main_statement: u32,
    main_box_is_static: bool,
    main_callable: CallableDeclarationIdentityV1,
    static_children: Box<[CallableDeclarationIdentityV1]>,
    _seal: ParserNormalAppExecutionRelationSealV1,
}

#[derive(Debug)]
pub(super) struct ParserNormalAppExecutionRelationSealV1;

impl ParserNormalAppExecutionRelationV1 {
    pub(super) fn issue(
        main_statement: u32,
        main_box_is_static: bool,
        main_callable: CallableDeclarationIdentityV1,
        static_children: Box<[CallableDeclarationIdentityV1]>,
    ) -> Self {
        Self {
            main_statement,
            main_box_is_static,
            main_callable,
            static_children,
            _seal: ParserNormalAppExecutionRelationSealV1,
        }
    }

    pub(crate) const fn main_statement(&self) -> u32 {
        self.main_statement
    }

    pub(crate) const fn main_box_is_static(&self) -> bool {
        self.main_box_is_static
    }

    pub(crate) fn main_callable(&self) -> &CallableDeclarationIdentityV1 {
        &self.main_callable
    }

    pub(crate) fn static_children(&self) -> &[CallableDeclarationIdentityV1] {
        &self.static_children
    }
}

#[derive(Debug)]
enum ParserNormalRootExecutionRelationV1 {
    App(ParserNormalAppExecutionRelationV1),
    ProgramRuntime,
}

#[derive(Debug)]
pub(crate) struct ParserNormalRootExecutionSourceV1 {
    bound: ParserBackedNormalSourcePlanBoundV1,
    relation: ParserNormalRootExecutionRelationV1,
    _seal: ParserNormalRootExecutionSourceSealV1,
}

#[derive(Debug)]
pub(super) struct ParserNormalRootExecutionSourceSealV1;

impl ParserNormalRootExecutionSourceV1 {
    pub(super) fn app(
        bound: ParserBackedNormalSourcePlanBoundV1,
        relation: ParserNormalAppExecutionRelationV1,
    ) -> Self {
        Self {
            bound,
            relation: ParserNormalRootExecutionRelationV1::App(relation),
            _seal: ParserNormalRootExecutionSourceSealV1,
        }
    }

    pub(super) fn program_runtime(bound: ParserBackedNormalSourcePlanBoundV1) -> Self {
        Self {
            bound,
            relation: ParserNormalRootExecutionRelationV1::ProgramRuntime,
            _seal: ParserNormalRootExecutionSourceSealV1,
        }
    }

    pub(crate) const fn role(&self) -> ParserNormalRootExecutionRoleV1 {
        match self.relation {
            ParserNormalRootExecutionRelationV1::App(_) => ParserNormalRootExecutionRoleV1::App,
            ParserNormalRootExecutionRelationV1::ProgramRuntime => {
                ParserNormalRootExecutionRoleV1::ProgramRuntime
            }
        }
    }

    pub(in crate::parser) fn bound(&self) -> &ParserBackedNormalSourcePlanBoundV1 {
        &self.bound
    }

    pub(crate) fn app_relation(&self) -> Option<&ParserNormalAppExecutionRelationV1> {
        match &self.relation {
            ParserNormalRootExecutionRelationV1::App(relation) => Some(relation),
            ParserNormalRootExecutionRelationV1::ProgramRuntime => None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum ParserNormalRootExecutionSourceDispositionV1 {
    Ready(ParserNormalRootExecutionSourceV1),
    SourceAuthorityUnavailable(ParserNormalSourcePlanSurfaceUnavailableV1),
    Incomplete(ParserNormalRootExecutionIncompleteV1),
    IntegrityInvalid(ParserNormalRootExecutionIntegrityIssueV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalRootExecutionTerminalClassV1 {
    SourceAuthorityUnavailable,
    Incomplete,
    IntegrityInvalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalRootExecutionIncompleteV1 {
    Surface(ParserNormalSourcePlanSurfaceIncompleteV1),
    MainMethodMissing,
    MainMemberCoverage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalRootExecutionIntegrityIssueV1 {
    Surface(ParserNormalSourcePlanSurfaceIntegrityIssueV1),
    DuplicateMain,
    DuplicateMainMethod,
}

impl ParserNormalRootExecutionSourceDispositionV1 {
    pub(crate) fn ready(&self) -> Option<&ParserNormalRootExecutionSourceV1> {
        match self {
            Self::Ready(source) => Some(source),
            Self::SourceAuthorityUnavailable(_)
            | Self::Incomplete(_)
            | Self::IntegrityInvalid(_) => None,
        }
    }

    pub(crate) const fn terminal_class(&self) -> Option<ParserNormalRootExecutionTerminalClassV1> {
        match self {
            Self::Ready(_) => None,
            Self::SourceAuthorityUnavailable(_) => {
                Some(ParserNormalRootExecutionTerminalClassV1::SourceAuthorityUnavailable)
            }
            Self::Incomplete(_) => Some(ParserNormalRootExecutionTerminalClassV1::Incomplete),
            Self::IntegrityInvalid(_) => {
                Some(ParserNormalRootExecutionTerminalClassV1::IntegrityInvalid)
            }
        }
    }
}
