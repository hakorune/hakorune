//! Opaque one-shot parser boundary for canonical normal source planning.
//!
//! The parser surface remains private to the parser.  MIR policy receives one
//! scoped, read-only observation loan and the affine owner remains intact
//! until a typed source-plan terminal consumes it.  No AST, name, ordinal, or
//! pointer can recreate this owner.

use super::super::callable_source_anchor::CallableDeclarationIdentityV1;
use super::normal_root_execution::ParserNormalRootExecutionSourceDispositionV1;
use super::normal_source_plan_surface::{
    ParserNormalSourcePlanCallableKindV1, ParserNormalSourcePlanSurfaceV1,
    ParserNormalSourcePlanTopLevelRowV1, ParserNormalSourcePlanUnsupportedKindV1,
};
use super::parser_invocation_witness::ParserInvocationWitnessV1;
use super::product::ParsedProgramWithCallableParameterSourceV1;
use crate::parser::NormalParserSourceLineageV1;

#[derive(Debug)]
pub(crate) struct SourcePlanBoundNormalCallableSourceV1 {
    source: ParsedProgramWithCallableParameterSourceV1,
    lineage: NormalParserSourceLineageV1,
    invocation: ParserInvocationWitnessV1,
    _seal: SourcePlanBoundNormalCallableSourceSealV1,
}

#[derive(Debug)]
struct SourcePlanBoundNormalCallableSourceSealV1;

pub(crate) struct ParserNormalRootSourcePlanConsumerV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalRootSourcePlanConsumeErrorV1 {
    CompatibilitySourceUnavailable,
    SourceAuthorityUnavailable,
    Incomplete,
    IntegrityInvalid,
}

#[derive(Debug)]
pub(crate) struct RejectedParserNormalRootSourcePlanConsumptionV1 {
    source: ParsedProgramWithCallableParameterSourceV1,
    lineage: NormalParserSourceLineageV1,
    error: ParserNormalRootSourcePlanConsumeErrorV1,
}

#[derive(Debug)]
pub(crate) struct ObservedSourcePlanBoundNormalCallableSourceV1<Observation> {
    source: SourcePlanBoundNormalCallableSourceV1,
    observation: Observation,
}

/// Parser-bound owner after the sole compiler policy callback returned `Ok`.
///
/// This is an internal affine state transition, not a second source or policy
/// receipt. Only this state can lend or extract syntax for a sealed plan.
#[derive(Debug)]
pub(crate) struct AdmittedSourcePlanBoundNormalCallableSourceV1 {
    source: SourcePlanBoundNormalCallableSourceV1,
    _seal: AdmittedSourcePlanBoundNormalCallableSourceSealV1,
}

#[derive(Debug)]
struct AdmittedSourcePlanBoundNormalCallableSourceSealV1;

#[derive(Debug)]
pub(crate) struct RejectedNormalSourcePlanSurfaceObservationV1 {
    source: SourcePlanBoundNormalCallableSourceV1,
    error: NormalSourcePlanSurfaceLoanErrorV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalSourcePlanUnsupportedKindV1 {
    NestedProgram,
    BuildGate,
    Using,
    Import,
    Enum,
    Brand,
    TypeAlias,
    Global,
    StaticConstTable,
}

#[derive(Debug)]
pub(crate) struct NormalSourcePlanMethodRefV1<'source> {
    identity: &'source CallableDeclarationIdentityV1,
    diagnostic_name: &'source str,
    arity: u32,
    is_static: bool,
}

#[derive(Debug)]
pub(crate) struct NormalSourcePlanBoxRefV1<'source> {
    statement: u32,
    is_static: bool,
    observed_member_count: u32,
    methods: Box<[NormalSourcePlanMethodRefV1<'source>]>,
}

#[derive(Debug)]
pub(crate) enum NormalSourcePlanTopLevelRowRefV1<'source> {
    Executable {
        statement: u32,
    },
    TopLevelCallable {
        statement: u32,
    },
    Box(NormalSourcePlanBoxRefV1<'source>),
    Unsupported {
        statement: u32,
        kind: NormalSourcePlanUnsupportedKindV1,
    },
}

#[derive(Debug)]
pub(crate) enum NormalSourcePlanRootRefV1<'source> {
    App {
        main_statement: u32,
        main_callable: &'source CallableDeclarationIdentityV1,
        child_callables: Box<[&'source CallableDeclarationIdentityV1]>,
    },
    ProgramRuntime,
}

#[derive(Debug)]
pub(crate) struct NormalSourcePlanSurfaceLoanV1<'source> {
    root: NormalSourcePlanRootRefV1<'source>,
    rows: Box<[NormalSourcePlanTopLevelRowRefV1<'source>]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalSourcePlanSurfaceLoanErrorV1 {
    ReadySourceMissing,
    CallableSyntaxMissing,
    StaticMemberCoverageMismatch,
}

impl ParserNormalRootSourcePlanConsumerV1 {
    pub(crate) fn consume_once(
        source: ParsedProgramWithCallableParameterSourceV1,
        lineage: NormalParserSourceLineageV1,
    ) -> Result<
        SourcePlanBoundNormalCallableSourceV1,
        RejectedParserNormalRootSourcePlanConsumptionV1,
    > {
        let invocation = match source.normal_root_execution_for_consumer() {
            ParserNormalRootExecutionSourceDispositionV1::Ready(root) => {
                root.bound().invocation().clone()
            }
            ParserNormalRootExecutionSourceDispositionV1::SourceAuthorityUnavailable(
                super::normal_source_plan_surface::ParserNormalSourcePlanSurfaceUnavailableV1::PostpassNotSourceBacked,
            ) => {
                return Err(RejectedParserNormalRootSourcePlanConsumptionV1 {
                    source,
                    lineage,
                    error: ParserNormalRootSourcePlanConsumeErrorV1::CompatibilitySourceUnavailable,
                })
            }
            ParserNormalRootExecutionSourceDispositionV1::SourceAuthorityUnavailable(_) => {
                return Err(RejectedParserNormalRootSourcePlanConsumptionV1 {
                    source,
                    lineage,
                    error: ParserNormalRootSourcePlanConsumeErrorV1::SourceAuthorityUnavailable,
                })
            }
            ParserNormalRootExecutionSourceDispositionV1::Incomplete(_) => {
                return Err(RejectedParserNormalRootSourcePlanConsumptionV1 {
                    source,
                    lineage,
                    error: ParserNormalRootSourcePlanConsumeErrorV1::Incomplete,
                })
            }
            ParserNormalRootExecutionSourceDispositionV1::IntegrityInvalid(_) => {
                return Err(RejectedParserNormalRootSourcePlanConsumptionV1 {
                    source,
                    lineage,
                    error: ParserNormalRootSourcePlanConsumeErrorV1::IntegrityInvalid,
                })
            }
        };
        Ok(SourcePlanBoundNormalCallableSourceV1 {
            source,
            lineage,
            invocation,
            _seal: SourcePlanBoundNormalCallableSourceSealV1,
        })
    }

    #[cfg(test)]
    pub(crate) fn consume_for_test(
        source: ParsedProgramWithCallableParameterSourceV1,
        lineage: NormalParserSourceLineageV1,
    ) -> Result<
        SourcePlanBoundNormalCallableSourceV1,
        RejectedParserNormalRootSourcePlanConsumptionV1,
    > {
        Self::consume_once(source, lineage)
    }
}

impl SourcePlanBoundNormalCallableSourceV1 {
    pub(crate) fn lineage(&self) -> &NormalParserSourceLineageV1 {
        &self.lineage
    }

    pub(crate) fn invocation_witness(&self) -> &ParserInvocationWitnessV1 {
        &self.invocation
    }

    /// Close a parser-bound owner after a typed source-plan terminal without
    /// extracting syntax or relying on the aggregate destructor.
    pub(crate) fn discard_after_source_plan_terminal(self) {
        let Self {
            source,
            lineage,
            invocation,
            _seal,
        } = self;
        source.discard_after_source_plan_rejection();
        drop((lineage, invocation, _seal));
    }

    pub(crate) fn observe_surface_once<R>(
        self,
        callback: impl for<'source> FnOnce(NormalSourcePlanSurfaceLoanV1<'source>) -> R,
    ) -> Result<
        ObservedSourcePlanBoundNormalCallableSourceV1<R>,
        RejectedNormalSourcePlanSurfaceObservationV1,
    > {
        let observation = match observe_surface(&self, callback) {
            Ok(observation) => observation,
            Err(error) => {
                return Err(RejectedNormalSourcePlanSurfaceObservationV1 {
                    source: self,
                    error,
                })
            }
        };
        Ok(ObservedSourcePlanBoundNormalCallableSourceV1 {
            source: self,
            observation,
        })
    }
}

impl AdmittedSourcePlanBoundNormalCallableSourceV1 {
    pub(crate) fn lineage(&self) -> &NormalParserSourceLineageV1 {
        self.source.lineage()
    }

    pub(crate) fn invocation_witness(&self) -> &ParserInvocationWitnessV1 {
        self.source.invocation_witness()
    }

    /// Borrow syntax only from the policy-admitted affine state.
    pub(crate) fn source_ast_after_policy(&self) -> &crate::ast::ASTNode {
        self.source.source.source_ast_for_bound_terminal()
    }

    /// Extract syntax once after policy admission and close every parser
    /// sibling at the same named terminal.
    pub(crate) fn into_ast_after_policy(self) -> crate::ast::ASTNode {
        let Self { source, _seal } = self;
        let SourcePlanBoundNormalCallableSourceV1 {
            source,
            lineage,
            invocation,
            _seal: bound_seal,
        } = source;
        let ast = source.into_ast_after_source_plan_terminal();
        drop((lineage, invocation, bound_seal, _seal));
        ast
    }

    pub(crate) fn discard_after_source_plan_terminal(self) {
        let Self { source, _seal } = self;
        source.discard_after_source_plan_terminal();
        drop(_seal);
    }
}

fn observe_surface<R>(
    source: &SourcePlanBoundNormalCallableSourceV1,
    callback: impl for<'source> FnOnce(NormalSourcePlanSurfaceLoanV1<'source>) -> R,
) -> Result<R, NormalSourcePlanSurfaceLoanErrorV1> {
    let root = source
        .source
        .normal_root_execution_for_consumer()
        .ready()
        .ok_or(NormalSourcePlanSurfaceLoanErrorV1::ReadySourceMissing)?;
    let root_relation = match root.app_relation() {
        Some(relation) => NormalSourcePlanRootRefV1::App {
            main_statement: relation.main_statement(),
            main_callable: relation.main_callable(),
            child_callables: relation
                .static_children()
                .iter()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        },
        None => NormalSourcePlanRootRefV1::ProgramRuntime,
    };
    let bound = root.bound();
    let surface_rows = match bound.surface() {
        ParserNormalSourcePlanSurfaceV1::CompleteEmpty => &[][..],
        ParserNormalSourcePlanSurfaceV1::CompleteRows(rows) => rows.rows(),
    };
    let mut rows = Vec::with_capacity(surface_rows.len());
    for row in surface_rows {
        let statement = row.slot().final_statement_slot();
        rows.push(match row {
            ParserNormalSourcePlanTopLevelRowV1::Executable { .. } => {
                NormalSourcePlanTopLevelRowRefV1::Executable { statement }
            }
            ParserNormalSourcePlanTopLevelRowV1::TopLevelCallable {
                callable_identity, ..
            } => {
                let _top_level_callable = method_ref(bound, callable_identity)?;
                NormalSourcePlanTopLevelRowRefV1::TopLevelCallable { statement }
            }
            ParserNormalSourcePlanTopLevelRowV1::StaticBox { source, .. } => {
                if usize::try_from(source.member_count()).ok() != Some(source.member_kinds().len())
                {
                    return Err(NormalSourcePlanSurfaceLoanErrorV1::StaticMemberCoverageMismatch);
                }
                let methods = source
                    .direct_method_relations()
                    .map(|(_, identity)| method_ref(bound, identity))
                    .collect::<Result<Vec<_>, _>>()?;
                NormalSourcePlanTopLevelRowRefV1::Box(NormalSourcePlanBoxRefV1 {
                    statement,
                    is_static: true,
                    observed_member_count: source.member_count(),
                    methods: methods.into_boxed_slice(),
                })
            }
            ParserNormalSourcePlanTopLevelRowV1::OrdinaryBox { source, .. } => {
                let methods = source
                    .direct_method_relations()
                    .iter()
                    .map(|relation| {
                        let _source_site = relation.source_site();
                        method_ref(bound, relation.callable_identity())
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                NormalSourcePlanTopLevelRowRefV1::Box(NormalSourcePlanBoxRefV1 {
                    statement,
                    is_static: false,
                    observed_member_count: source.observed_member_count(),
                    methods: methods.into_boxed_slice(),
                })
            }
            ParserNormalSourcePlanTopLevelRowV1::Unsupported { kind, .. } => {
                NormalSourcePlanTopLevelRowRefV1::Unsupported {
                    statement,
                    kind: map_unsupported_kind(*kind),
                }
            }
        });
    }
    Ok(callback(NormalSourcePlanSurfaceLoanV1 {
        root: root_relation,
        rows: rows.into_boxed_slice(),
    }))
}

impl RejectedParserNormalRootSourcePlanConsumptionV1 {
    pub(crate) const fn error(&self) -> ParserNormalRootSourcePlanConsumeErrorV1 {
        self.error
    }

    pub(crate) fn discard(self) {
        let Self {
            source,
            lineage,
            error,
        } = self;
        source.discard_after_source_plan_rejection();
        drop((lineage, error));
    }
}

impl<Decision, Error> ObservedSourcePlanBoundNormalCallableSourceV1<Result<Decision, Error>> {
    pub(crate) fn resolve_policy_once<T>(
        self,
        admitted: impl FnOnce(AdmittedSourcePlanBoundNormalCallableSourceV1, Decision) -> T,
        rejected: impl FnOnce(SourcePlanBoundNormalCallableSourceV1, Error) -> T,
    ) -> T {
        match self.observation {
            Ok(decision) => admitted(
                AdmittedSourcePlanBoundNormalCallableSourceV1 {
                    source: self.source,
                    _seal: AdmittedSourcePlanBoundNormalCallableSourceSealV1,
                },
                decision,
            ),
            Err(error) => rejected(self.source, error),
        }
    }
}

impl RejectedNormalSourcePlanSurfaceObservationV1 {
    pub(crate) fn commit<T>(
        self,
        consume: impl FnOnce(
            SourcePlanBoundNormalCallableSourceV1,
            NormalSourcePlanSurfaceLoanErrorV1,
        ) -> T,
    ) -> T {
        consume(self.source, self.error)
    }
}

impl NormalSourcePlanSurfaceLoanV1<'_> {
    pub(crate) fn root(&self) -> &NormalSourcePlanRootRefV1<'_> {
        &self.root
    }

    pub(crate) fn rows(&self) -> &[NormalSourcePlanTopLevelRowRefV1<'_>] {
        &self.rows
    }
}

impl NormalSourcePlanMethodRefV1<'_> {
    pub(crate) fn identity(&self) -> &CallableDeclarationIdentityV1 {
        self.identity
    }

    pub(crate) fn diagnostic_name(&self) -> &str {
        self.diagnostic_name
    }

    pub(crate) const fn arity(&self) -> u32 {
        self.arity
    }

    pub(crate) const fn is_static(&self) -> bool {
        self.is_static
    }
}

impl NormalSourcePlanBoxRefV1<'_> {
    pub(crate) const fn statement(&self) -> u32 {
        self.statement
    }

    pub(crate) const fn is_static(&self) -> bool {
        self.is_static
    }

    pub(crate) const fn observed_member_count(&self) -> u32 {
        self.observed_member_count
    }

    pub(crate) fn methods(&self) -> &[NormalSourcePlanMethodRefV1<'_>] {
        &self.methods
    }
}

fn method_ref<'source>(
    bound: &'source super::normal_source_plan_surface::ParserBackedNormalSourcePlanBoundV1,
    identity: &'source CallableDeclarationIdentityV1,
) -> Result<NormalSourcePlanMethodRefV1<'source>, NormalSourcePlanSurfaceLoanErrorV1> {
    let syntax = bound
        .callable_syntax(identity)
        .ok_or(NormalSourcePlanSurfaceLoanErrorV1::CallableSyntaxMissing)?;
    Ok(NormalSourcePlanMethodRefV1 {
        identity,
        diagnostic_name: syntax.diagnostic_name(),
        arity: syntax.arity(),
        is_static: matches!(
            syntax.kind(),
            ParserNormalSourcePlanCallableKindV1::StaticBoxMethod
        ),
    })
}

fn map_unsupported_kind(
    kind: ParserNormalSourcePlanUnsupportedKindV1,
) -> NormalSourcePlanUnsupportedKindV1 {
    match kind {
        ParserNormalSourcePlanUnsupportedKindV1::NestedProgram => {
            NormalSourcePlanUnsupportedKindV1::NestedProgram
        }
        ParserNormalSourcePlanUnsupportedKindV1::BuildGate => {
            NormalSourcePlanUnsupportedKindV1::BuildGate
        }
        ParserNormalSourcePlanUnsupportedKindV1::Using => NormalSourcePlanUnsupportedKindV1::Using,
        ParserNormalSourcePlanUnsupportedKindV1::Import => {
            NormalSourcePlanUnsupportedKindV1::Import
        }
        ParserNormalSourcePlanUnsupportedKindV1::Enum => NormalSourcePlanUnsupportedKindV1::Enum,
        ParserNormalSourcePlanUnsupportedKindV1::Brand => NormalSourcePlanUnsupportedKindV1::Brand,
        ParserNormalSourcePlanUnsupportedKindV1::TypeAlias => {
            NormalSourcePlanUnsupportedKindV1::TypeAlias
        }
        ParserNormalSourcePlanUnsupportedKindV1::Global => {
            NormalSourcePlanUnsupportedKindV1::Global
        }
        ParserNormalSourcePlanUnsupportedKindV1::StaticConstTable => {
            NormalSourcePlanUnsupportedKindV1::StaticConstTable
        }
    }
}
