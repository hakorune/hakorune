//! Requests through the same source-flow argument callback, with explicit roles.
use super::{
    BorrowedCallActualCandidateV1, LocalCallArgumentV1, QualifiedStaticCallClaimV1,
    StaticI64CallClaimV1,
};
#[derive(Debug, Clone, Copy)]
pub(crate) enum BorrowedCallActualRequestV1<'a> {
    NamedArrayHolderInitializer(super::BindingRefV1),
    NamedArrayAppendStatement(super::BindingRefV1),
    Observe(crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'a>, &'a [BorrowedCallActualCandidateV1]),
    /// Original argument plan and optional Core loan; no evaluation permission.
    ArgumentExpressionSource(crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'a>, u32),
    CompleteArgumentExpression(&'a std::rc::Rc<super::CheckedArgumentExpressionFrameV1>),
    ArgumentTextEntryRead { source: &'a crate::mir::normal_callable_semantic_package::ArgumentExpressionSourceLoanV1,
        read: &'a super::SourceExprSiteV1, binding: super::BindingRefV1, root: super::BindingRefV1 },
    InitializerTextEntryRead {
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'a>,
        plan: &'a std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
        read: &'a super::SourceExprSiteV1, binding: super::BindingRefV1, root: super::BindingRefV1,
    },
    ExpressionCoreSource {
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'a>,
        plan: &'a std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    },
    ScalarArguments,
    I64ResultArguments,
    SourceScalarResultArguments,
    SourceUnitDiscardArguments,
    ReceivedObjectArguments(
        crate::mir::resolved_semantics::BindingRefV1,
        super::LocalCallResultClassV1,
    ),
    ReceivedHandleArguments(crate::mir::resolved_semantics::BindingRefV1),
    BorrowedNullableLocalArguments(crate::mir::resolved_semantics::BindingRefV1),
    MixedLocalArguments(crate::mir::resolved_semantics::BindingRefV1),
    ObjectArguments(
        &'a crate::mir::normal_callable_semantic_package::ObjectReturnCallQualificationV1,
        Option<crate::mir::resolved_semantics::BindingRefV1>,
    ),
    QualifiedStaticSourceArguments(&'a QualifiedStaticCallClaimV1),
    CurrentOwnerStaticSourceArguments(&'a StaticI64CallClaimV1),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BorrowedCallArgumentsV1 {
    ExpressionCoreSource(crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1),
    InitializerTextEntryRead(crate::mir::normal_callable_semantic_package::InitializerTextEntryReadV1),
    TextEntryRead(crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1),
    ArgumentExpressionSource(crate::mir::normal_callable_semantic_package::ArgumentExpressionSourceLoanV1),
    ArgumentExpressionFrame(std::rc::Rc<super::CheckedArgumentExpressionFrameV1>),
    MixedLocalSource {
        loan: crate::mir::normal_callable_semantic_package::MixedLocalCallSourceLoanV1,
        arguments: Box<[LocalCallArgumentV1]>,
    },
    NamedArrayHolderSource(
        crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1,
    ),
    /// Original nullable borrow and argument observation only, never a Home.
    BorrowedNullableLocalSource {
        loan: crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1,
        arguments: Box<[LocalCallArgumentV1]>,
    },
    /// SAME sealed result source and original arguments; no execution permission.
    SourceScalarResult {
        loan: crate::mir::normal_callable_semantic_package::BorrowedScalarResultSourceLoanV1,
        arguments: Box<[LocalCallArgumentV1]>,
    },
    /// Original Unit statement demand and ordered source arguments, never executable actuals.
    SourceUnitDiscard { source: crate::mir::normal_callable_semantic_package::LexicalInstanceCallSourceTargetV1, arguments: Box<[LocalCallArgumentV1]> },
    Scalar(Box<[LocalCallArgumentV1]>),
    StaticSource(Box<[LocalCallArgumentV1]>),
    /// Original Instance arguments only; cannot seal an executable call.
    SourceInstance(Box<[LocalCallArgumentV1]>),
    /// Source observation only; no result, entry or executable permission.
    HandleSource(Box<[LocalCallArgumentV1]>),
    /// Exact producer result class; input support retains its original phase.
    SourceObject {
        result: super::LocalCallResultClassV1,
        arguments: ObjectCallSourceSupportV1,
    },
    Object {
        qualification:
            crate::mir::normal_callable_semantic_package::ObjectReturnCallQualificationV1,
        arguments: ObjectCallSourceSupportV1,
    },
}

/// Retained source availability only; never a Normal or physical permission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ObjectCallSourceSupportV1 {
    Observed(Box<[LocalCallArgumentV1]>),
    SourceOnly(Box<[LocalCallArgumentV1]>),
    Unavailable,
}
