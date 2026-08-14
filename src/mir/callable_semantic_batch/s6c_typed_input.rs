//! Explicitly typed source/frame co-seal for forward ScanWithInit.
//!
//! Parser declaration rows own type spelling. Resolver rows own bindings,
//! expression relations, and Loop placement. This issuer combines those
//! existing authorities without opening source-bound calls, Facts, Recipe,
//! Builder, or physical lowering.

use crate::mir::resolved_semantics::{
    assignment_value_sibling_v1, BindingRefV1, CallableSourceLedgerRejectV1,
    ResolvedAssignmentTargetV1, ResolvedBinaryExpressionSourceV1, ResolvedBinaryOperatorV1,
    ResolvedInitializerRelationV1, ResolvedLexicalRefV1, ResolvedLiteralSourceV1,
    ResolvedLoopPlacementV1, ResolvedLoopRegionLookupErrorV1, ResolvedMethodCallReceiverSourceV1,
    SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1, VerifiedCallableLoopMembershipV1,
};

use super::{
    ResolvedCallableDeclarationModeV1, VerifiedResolvedCallableParameterSourceRefV1,
    VerifiedResolvedCallableSemanticRowRefV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CLogicalValueClassV1 {
    Text,
    I64,
    Bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CTypedInputRoleV1 {
    Subject,
    Needle,
    Index,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CBinaryRoleV1 {
    LoopConditionLess,
    SliceEndAdd,
    TextEqual,
    StepAdd,
}

/// Borrow-only projection of the two call sites already verified by S6C.
///
/// The fixed fields are role identity; consumers cannot construct, reorder,
/// or retain this view independently of the non-Clone typed-input product.
#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CCallSitePairRefV1<'a> {
    length_site: &'a SourceExprSiteV1,
    length_placement: ResolvedLoopPlacementV1,
    substring_site: &'a SourceExprSiteV1,
    substring_placement: ResolvedLoopPlacementV1,
}

impl S6CCallSitePairRefV1<'_> {
    pub(crate) const fn length_site(&self) -> &SourceExprSiteV1 {
        self.length_site
    }

    pub(crate) const fn length_placement(&self) -> ResolvedLoopPlacementV1 {
        self.length_placement
    }

    pub(crate) const fn substring_site(&self) -> &SourceExprSiteV1 {
        self.substring_site
    }

    pub(crate) const fn substring_placement(&self) -> ResolvedLoopPlacementV1 {
        self.substring_placement
    }
}

#[derive(Debug)]
struct VerifiedS6CCallSitePairV1 {
    length_site: SourceExprSiteV1,
    length_placement: ResolvedLoopPlacementV1,
    substring_site: SourceExprSiteV1,
    substring_placement: ResolvedLoopPlacementV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct S6CTypedBindingV1 {
    role: S6CTypedInputRoleV1,
    binding: BindingRefV1,
    class: S6CLogicalValueClassV1,
}

impl S6CTypedBindingV1 {
    pub(crate) const fn role(&self) -> S6CTypedInputRoleV1 {
        self.role
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn class(&self) -> S6CLogicalValueClassV1 {
        self.class
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct S6CBinaryRelationV1 {
    role: S6CBinaryRoleV1,
    source: ResolvedBinaryExpressionSourceV1,
    placement: ResolvedLoopPlacementV1,
    lhs_class: S6CLogicalValueClassV1,
    rhs_class: S6CLogicalValueClassV1,
    result_class: S6CLogicalValueClassV1,
}

impl S6CBinaryRelationV1 {
    pub(crate) const fn role(&self) -> S6CBinaryRoleV1 {
        self.role
    }

    pub(crate) const fn source(&self) -> &ResolvedBinaryExpressionSourceV1 {
        &self.source
    }

    pub(crate) const fn placement(&self) -> ResolvedLoopPlacementV1 {
        self.placement
    }

    pub(crate) const fn result_class(&self) -> S6CLogicalValueClassV1 {
        self.result_class
    }
}

/// Non-Clone typed-input/source-frame authority for the first S6C cohort.
#[derive(Debug)]
pub(crate) struct VerifiedS6CTypedInputRelationV1 {
    membership: VerifiedCallableLoopMembershipV1,
    inputs: [S6CTypedBindingV1; 3],
    initializer: ResolvedInitializerRelationV1,
    binaries: [S6CBinaryRelationV1; 4],
    calls: VerifiedS6CCallSitePairV1,
}

impl VerifiedS6CTypedInputRelationV1 {
    pub(crate) fn membership(&self) -> &VerifiedCallableLoopMembershipV1 {
        &self.membership
    }

    pub(crate) const fn inputs(&self) -> &[S6CTypedBindingV1; 3] {
        &self.inputs
    }

    pub(crate) const fn initializer(&self) -> &ResolvedInitializerRelationV1 {
        &self.initializer
    }

    pub(crate) const fn binaries(&self) -> &[S6CBinaryRelationV1; 4] {
        &self.binaries
    }

    pub(crate) fn with_call_sites<R>(
        &self,
        callback: impl for<'call> FnOnce(S6CCallSitePairRefV1<'call>) -> R,
    ) -> R {
        callback(S6CCallSitePairRefV1 {
            length_site: &self.calls.length_site,
            length_placement: self.calls.length_placement,
            substring_site: &self.calls.substring_site,
            substring_placement: self.calls.substring_placement,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum S6CTypedInputRelationRejectV1 {
    UnsupportedDeclarationMode(ResolvedCallableDeclarationModeV1),
    SourceLedger(CallableSourceLedgerRejectV1),
    Loop(ResolvedLoopRegionLookupErrorV1),
    ParameterCoverage {
        actual: usize,
    },
    ParameterOrdinal {
        expected: u32,
        actual: u32,
    },
    NonOrdinaryParameter(u32),
    MissingTypeEvidence(S6CTypedInputRoleV1),
    WrongTypeSpelling {
        role: S6CTypedInputRoleV1,
        actual: Box<str>,
    },
    MissingBinding(S6CTypedInputRoleV1),
    InitializerCoverage {
        actual: usize,
    },
    WrongInitializerType(Option<Box<str>>),
    MissingInitializer,
    WrongInitializerLiteral,
    MethodCallCoverage {
        actual: usize,
    },
    MissingMethodCall(&'static str),
    DuplicateMethodCall(&'static str),
    MethodCallShape(&'static str),
    BinaryCoverage {
        actual: usize,
    },
    BinaryRoleCoverage(S6CBinaryRoleV1),
    BinaryShape(S6CBinaryRoleV1),
    AssignmentCoverage {
        actual: usize,
    },
    AssignmentShape,
}

pub(crate) fn issue_s6c_typed_input_relation_v1(
    row: &VerifiedResolvedCallableSemanticRowRefV1<'_>,
    loop_site: &SourceStmtSiteV1,
) -> Result<VerifiedS6CTypedInputRelationV1, S6CTypedInputRelationRejectV1> {
    if row.mode() != ResolvedCallableDeclarationModeV1::StaticBoxMethod {
        return Err(S6CTypedInputRelationRejectV1::UnsupportedDeclarationMode(
            row.mode(),
        ));
    }
    let parameters = row
        .parameters()
        .ok_or(S6CTypedInputRelationRejectV1::ParameterCoverage { actual: 0 })?;
    if parameters.len() != 2 {
        return Err(S6CTypedInputRelationRejectV1::ParameterCoverage {
            actual: parameters.len(),
        });
    }
    require_parameter(parameters[0], 0, S6CTypedInputRoleV1::Subject)?;
    require_parameter(parameters[1], 1, S6CTypedInputRoleV1::Needle)?;

    let ledger = row
        .source_ledger()
        .map_err(S6CTypedInputRelationRejectV1::SourceLedger)?;
    let membership = ledger
        .resolved_loop_source(loop_site)
        .map_err(S6CTypedInputRelationRejectV1::Loop)?;
    let subject = ledger
        .declaration_binding(&SourceBindingSiteV1::Parameter { index: 0 })
        .ok_or(S6CTypedInputRelationRejectV1::MissingBinding(
            S6CTypedInputRoleV1::Subject,
        ))?;
    let needle = ledger
        .declaration_binding(&SourceBindingSiteV1::Parameter { index: 1 })
        .ok_or(S6CTypedInputRelationRejectV1::MissingBinding(
            S6CTypedInputRoleV1::Needle,
        ))?;

    let initializers = ledger.initializer_relations().collect::<Vec<_>>();
    if initializers.len() != 1 {
        return Err(S6CTypedInputRelationRejectV1::InitializerCoverage {
            actual: initializers.len(),
        });
    }
    let initializer = initializers[0];
    if initializer.declared_type_name() != Some("i64") {
        return Err(S6CTypedInputRelationRejectV1::WrongInitializerType(
            initializer.declared_type_name().map(Into::into),
        ));
    }
    let initializer_site = initializer
        .initializer_site()
        .ok_or(S6CTypedInputRelationRejectV1::MissingInitializer)?;
    if ledger.literal_source(initializer_site) != Some(&ResolvedLiteralSourceV1::Integer(0)) {
        return Err(S6CTypedInputRelationRejectV1::WrongInitializerLiteral);
    }
    let index = initializer.binding();

    let calls = ledger
        .method_calls()
        .filter_map(|(_, call)| {
            ledger
                .resolved_loop_placement(loop_site, call.site())
                .transpose()
                .map(|placement| placement.map(|placement| (call, placement)))
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(S6CTypedInputRelationRejectV1::Loop)?;
    if calls.len() != 2 {
        return Err(S6CTypedInputRelationRejectV1::MethodCallCoverage {
            actual: calls.len(),
        });
    }
    let length = exact_call(&calls, "length")?;
    let substring = exact_call(&calls, "substring")?;
    require_call_receiver(length.0, subject, "length")?;
    require_call_receiver(substring.0, subject, "substring")?;
    if length.0.arity() != 0
        || !length.0.arguments().is_empty()
        || length.1 != ResolvedLoopPlacementV1::Condition
    {
        return Err(S6CTypedInputRelationRejectV1::MethodCallShape("length"));
    }
    if substring.0.arity() != 2
        || substring.0.arguments().len() != 2
        || substring.1 != ResolvedLoopPlacementV1::Body
        || !is_local_binding(&ledger, substring.0.arguments()[0].site(), index)
    {
        return Err(S6CTypedInputRelationRejectV1::MethodCallShape("substring"));
    }

    let binary_rows = ledger
        .binary_expression_sources()
        .filter_map(|binary| {
            ledger
                .resolved_loop_placement(loop_site, binary.site())
                .transpose()
                .map(|placement| placement.map(|placement| (binary, placement)))
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(S6CTypedInputRelationRejectV1::Loop)?;
    if binary_rows.len() != 4 {
        return Err(S6CTypedInputRelationRejectV1::BinaryCoverage {
            actual: binary_rows.len(),
        });
    }

    let less = exact_binary(
        &binary_rows,
        ResolvedBinaryOperatorV1::Less,
        S6CBinaryRoleV1::LoopConditionLess,
    )?;
    if less.1 != ResolvedLoopPlacementV1::Condition
        || !is_local_binding(&ledger, less.0.lhs(), index)
        || less.0.rhs() != length.0.result_site()
    {
        return Err(S6CTypedInputRelationRejectV1::BinaryShape(
            S6CBinaryRoleV1::LoopConditionLess,
        ));
    }

    let equal = exact_binary(
        &binary_rows,
        ResolvedBinaryOperatorV1::Equal,
        S6CBinaryRoleV1::TextEqual,
    )?;
    if equal.1 != ResolvedLoopPlacementV1::Body
        || equal.0.lhs() != substring.0.result_site()
        || !is_local_binding(&ledger, equal.0.rhs(), needle)
    {
        return Err(S6CTypedInputRelationRejectV1::BinaryShape(
            S6CBinaryRoleV1::TextEqual,
        ));
    }

    let adds = binary_rows
        .iter()
        .copied()
        .filter(|(binary, _)| binary.operator() == ResolvedBinaryOperatorV1::Add)
        .collect::<Vec<_>>();
    if adds.len() != 2 {
        return Err(S6CTypedInputRelationRejectV1::BinaryRoleCoverage(
            S6CBinaryRoleV1::SliceEndAdd,
        ));
    }
    let slice_end_site = substring.0.arguments()[1].site();
    let slice_end = adds
        .iter()
        .copied()
        .find(|(binary, _)| binary.site() == slice_end_site)
        .ok_or(S6CTypedInputRelationRejectV1::BinaryRoleCoverage(
            S6CBinaryRoleV1::SliceEndAdd,
        ))?;
    require_index_add(&ledger, slice_end, index, S6CBinaryRoleV1::SliceEndAdd)?;

    let assignments = ledger
        .assignment_targets()
        .filter_map(|(target_site, target)| {
            let placement = ledger
                .resolved_loop_placement(loop_site, target_site)
                .transpose();
            placement.map(|placement| placement.map(|placement| (target_site, target, placement)))
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(S6CTypedInputRelationRejectV1::Loop)?;
    if assignments.len() != 1 {
        return Err(S6CTypedInputRelationRejectV1::AssignmentCoverage {
            actual: assignments.len(),
        });
    }
    let (target_site, target, placement) = assignments[0];
    if placement != ResolvedLoopPlacementV1::Body
        || target != &ResolvedAssignmentTargetV1::BindingRebind(index)
    {
        return Err(S6CTypedInputRelationRejectV1::AssignmentShape);
    }
    let step_site = assignment_value_sibling_v1(target_site)
        .ok_or(S6CTypedInputRelationRejectV1::AssignmentShape)?;
    let step = adds
        .iter()
        .copied()
        .find(|(binary, _)| binary.site() == &step_site)
        .ok_or(S6CTypedInputRelationRejectV1::BinaryRoleCoverage(
            S6CBinaryRoleV1::StepAdd,
        ))?;
    if step.0.site() == slice_end.0.site() {
        return Err(S6CTypedInputRelationRejectV1::BinaryRoleCoverage(
            S6CBinaryRoleV1::StepAdd,
        ));
    }
    require_index_add(&ledger, step, index, S6CBinaryRoleV1::StepAdd)?;

    Ok(VerifiedS6CTypedInputRelationV1 {
        membership,
        inputs: [
            S6CTypedBindingV1 {
                role: S6CTypedInputRoleV1::Subject,
                binding: subject,
                class: S6CLogicalValueClassV1::Text,
            },
            S6CTypedBindingV1 {
                role: S6CTypedInputRoleV1::Needle,
                binding: needle,
                class: S6CLogicalValueClassV1::Text,
            },
            S6CTypedBindingV1 {
                role: S6CTypedInputRoleV1::Index,
                binding: index,
                class: S6CLogicalValueClassV1::I64,
            },
        ],
        initializer: initializer.clone(),
        binaries: [
            binary_relation(
                S6CBinaryRoleV1::LoopConditionLess,
                less,
                S6CLogicalValueClassV1::I64,
                S6CLogicalValueClassV1::I64,
                S6CLogicalValueClassV1::Bool,
            ),
            binary_relation(
                S6CBinaryRoleV1::SliceEndAdd,
                slice_end,
                S6CLogicalValueClassV1::I64,
                S6CLogicalValueClassV1::I64,
                S6CLogicalValueClassV1::I64,
            ),
            binary_relation(
                S6CBinaryRoleV1::TextEqual,
                equal,
                S6CLogicalValueClassV1::Text,
                S6CLogicalValueClassV1::Text,
                S6CLogicalValueClassV1::Bool,
            ),
            binary_relation(
                S6CBinaryRoleV1::StepAdd,
                step,
                S6CLogicalValueClassV1::I64,
                S6CLogicalValueClassV1::I64,
                S6CLogicalValueClassV1::I64,
            ),
        ],
        calls: VerifiedS6CCallSitePairV1 {
            length_site: length.0.site().clone(),
            length_placement: length.1,
            substring_site: substring.0.site().clone(),
            substring_placement: substring.1,
        },
    })
}

fn require_parameter(
    parameter: VerifiedResolvedCallableParameterSourceRefV1<'_>,
    ordinal: u32,
    role: S6CTypedInputRoleV1,
) -> Result<(), S6CTypedInputRelationRejectV1> {
    if parameter.ordinal() != ordinal {
        return Err(S6CTypedInputRelationRejectV1::ParameterOrdinal {
            expected: ordinal,
            actual: parameter.ordinal(),
        });
    }
    if !parameter.is_ordinary() {
        return Err(S6CTypedInputRelationRejectV1::NonOrdinaryParameter(ordinal));
    }
    match parameter.declared_type_name() {
        Some("StringBox") => Ok(()),
        Some(actual) => Err(S6CTypedInputRelationRejectV1::WrongTypeSpelling {
            role,
            actual: actual.into(),
        }),
        None => Err(S6CTypedInputRelationRejectV1::MissingTypeEvidence(role)),
    }
}

fn exact_call<'a>(
    calls: &'a [(
        &'a crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
        ResolvedLoopPlacementV1,
    )],
    selector: &'static str,
) -> Result<
    (
        &'a crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
        ResolvedLoopPlacementV1,
    ),
    S6CTypedInputRelationRejectV1,
> {
    let matching = calls
        .iter()
        .copied()
        .filter(|(call, _)| call.selector() == selector)
        .collect::<Vec<_>>();
    match matching.as_slice() {
        [] => Err(S6CTypedInputRelationRejectV1::MissingMethodCall(selector)),
        [row] => Ok(*row),
        _ => Err(S6CTypedInputRelationRejectV1::DuplicateMethodCall(selector)),
    }
}

fn require_call_receiver(
    call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    binding: BindingRefV1,
    selector: &'static str,
) -> Result<(), S6CTypedInputRelationRejectV1> {
    (call.receiver()
        == ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)))
    .then_some(())
    .ok_or(S6CTypedInputRelationRejectV1::MethodCallShape(selector))
}

fn exact_binary<'a>(
    binaries: &'a [(
        &'a ResolvedBinaryExpressionSourceV1,
        ResolvedLoopPlacementV1,
    )],
    operator: ResolvedBinaryOperatorV1,
    role: S6CBinaryRoleV1,
) -> Result<
    (
        &'a ResolvedBinaryExpressionSourceV1,
        ResolvedLoopPlacementV1,
    ),
    S6CTypedInputRelationRejectV1,
> {
    let matching = binaries
        .iter()
        .copied()
        .filter(|(binary, _)| binary.operator() == operator)
        .collect::<Vec<_>>();
    match matching.as_slice() {
        [row] => Ok(*row),
        _ => Err(S6CTypedInputRelationRejectV1::BinaryRoleCoverage(role)),
    }
}

fn require_index_add(
    ledger: &crate::mir::resolved_semantics::CallableSemanticSourceLedgerView<'_>,
    row: (&ResolvedBinaryExpressionSourceV1, ResolvedLoopPlacementV1),
    index: BindingRefV1,
    role: S6CBinaryRoleV1,
) -> Result<(), S6CTypedInputRelationRejectV1> {
    if row.1 != ResolvedLoopPlacementV1::Body
        || !is_local_binding(ledger, row.0.lhs(), index)
        || ledger.literal_source(row.0.rhs()) != Some(&ResolvedLiteralSourceV1::Integer(1))
    {
        return Err(S6CTypedInputRelationRejectV1::BinaryShape(role));
    }
    Ok(())
}

fn is_local_binding(
    ledger: &crate::mir::resolved_semantics::CallableSemanticSourceLedgerView<'_>,
    site: &SourceExprSiteV1,
    binding: BindingRefV1,
) -> bool {
    ledger.variable_ref(site) == Some(ResolvedLexicalRefV1::Local(binding))
}

fn binary_relation(
    role: S6CBinaryRoleV1,
    row: (&ResolvedBinaryExpressionSourceV1, ResolvedLoopPlacementV1),
    lhs_class: S6CLogicalValueClassV1,
    rhs_class: S6CLogicalValueClassV1,
    result_class: S6CLogicalValueClassV1,
) -> S6CBinaryRelationV1 {
    S6CBinaryRelationV1 {
        role,
        source: row.0.clone(),
        placement: row.1,
        lhs_class,
        rhs_class,
        result_class,
    }
}
