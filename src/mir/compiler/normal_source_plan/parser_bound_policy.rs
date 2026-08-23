//! Pure source-plan policy over the one parser-issued SourceSurface loan.
//!
//! This module never reads the AST, reissues a parser relation, or decides
//! App versus ProgramRuntime. It consumes the already-issued root relation
//! and maps that closed observation to the existing `SealedNormal*` family.

use crate::parser::{
    NormalSourcePlanBoxRefV1, NormalSourcePlanRootRefV1, NormalSourcePlanSurfaceLoanV1,
    NormalSourcePlanTopLevelRowRefV1, NormalSourcePlanUnsupportedKindV1,
    SourcePlanBoundNormalCallableSourceV1,
};

use super::product::{
    NormalAdditionalCallableSiteV1, NormalMainMethodSiteV1, NormalSourcePlanOwnerV1,
    NormalTopLevelSiteV1, SealedNormalCallableModuleSourceV1, SealedNormalMainSourceV1,
    SealedNormalScalarRootV1, SealedNormalScriptSourceV1, SealedNormalSourcePlanV1,
};
use super::rejection::{
    NormalSourcePlanErrorV1, NormalUnsupportedTopLevelKindV1, RejectedNormalSourcePlanV1,
};
use super::NormalSourcePlanClassifierV1;

enum ParserBoundNormalSourcePlanDecisionV1 {
    Script {
        statements: Box<[NormalTopLevelSiteV1]>,
    },
    Main0 {
        main_box: NormalTopLevelSiteV1,
        main_method: NormalMainMethodSiteV1,
    },
    CallableModule {
        main_box: NormalTopLevelSiteV1,
        main_method: NormalMainMethodSiteV1,
        additional_callables: Box<[NormalAdditionalCallableSiteV1]>,
    },
}

impl NormalSourcePlanClassifierV1 {
    pub(crate) fn reject_parser_bound(
        source: SourcePlanBoundNormalCallableSourceV1,
        error: NormalSourcePlanErrorV1,
    ) -> RejectedNormalSourcePlanV1 {
        RejectedNormalSourcePlanV1::from_owner(
            NormalSourcePlanOwnerV1::from_parser_bound_rejected(source),
            error,
        )
    }

    pub(crate) fn seal_parser_bound(
        source: SourcePlanBoundNormalCallableSourceV1,
    ) -> Result<SealedNormalSourcePlanV1, RejectedNormalSourcePlanV1> {
        let observed = match source.observe_surface_once(classify_surface) {
            Ok(observed) => observed,
            Err(rejected) => {
                return Err(rejected.commit(|source, error| {
                    RejectedNormalSourcePlanV1::from_owner(
                        NormalSourcePlanOwnerV1::from_parser_bound_rejected(source),
                        NormalSourcePlanErrorV1::ParserSurfaceObservation(error),
                    )
                }))
            }
        };
        observed.resolve_policy_once(
            |source, decision| {
                let owner = NormalSourcePlanOwnerV1::from_parser_bound_admitted(source);
                match decision {
                    ParserBoundNormalSourcePlanDecisionV1::Script { statements } => Ok(
                        SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Script(
                            SealedNormalScriptSourceV1::seal(owner, statements),
                        )),
                    ),
                    ParserBoundNormalSourcePlanDecisionV1::Main0 {
                        main_box,
                        main_method,
                    } => Ok(SealedNormalSourcePlanV1::ScalarRoot(
                        SealedNormalScalarRootV1::Main0(SealedNormalMainSourceV1::seal(
                            owner,
                            main_box,
                            main_method,
                        )),
                    )),
                    ParserBoundNormalSourcePlanDecisionV1::CallableModule {
                        main_box,
                        main_method,
                        additional_callables,
                    } => Ok(SealedNormalSourcePlanV1::CallableModule(
                        SealedNormalCallableModuleSourceV1::seal(
                            owner,
                            main_box,
                            main_method,
                            additional_callables,
                        ),
                    )),
                }
            },
            |source, error| {
                Err(RejectedNormalSourcePlanV1::from_owner(
                    NormalSourcePlanOwnerV1::from_parser_bound_rejected(source),
                    error,
                ))
            },
        )
    }
}

fn classify_surface(
    loan: NormalSourcePlanSurfaceLoanV1<'_>,
) -> Result<ParserBoundNormalSourcePlanDecisionV1, NormalSourcePlanErrorV1> {
    match loan.root() {
        NormalSourcePlanRootRefV1::ProgramRuntime => classify_program_runtime(loan.rows()),
        NormalSourcePlanRootRefV1::App {
            main_statement,
            main_callable,
            child_callables,
        } => classify_app(loan.rows(), *main_statement, main_callable, child_callables),
    }
}

fn classify_program_runtime(
    rows: &[NormalSourcePlanTopLevelRowRefV1<'_>],
) -> Result<ParserBoundNormalSourcePlanDecisionV1, NormalSourcePlanErrorV1> {
    let mut script = Vec::new();
    let mut callables = Vec::new();
    let mut unsupported = None;
    for row in rows {
        match row {
            NormalSourcePlanTopLevelRowRefV1::Executable { statement } => {
                script.push(site(*statement)?);
            }
            NormalSourcePlanTopLevelRowRefV1::TopLevelCallable { statement, .. } => {
                callables.push(site(*statement)?);
            }
            NormalSourcePlanTopLevelRowRefV1::Box(source) => {
                unsupported
                    .get_or_insert((source.statement(), NormalUnsupportedTopLevelKindV1::Box));
            }
            NormalSourcePlanTopLevelRowRefV1::Unsupported { statement, kind } => {
                unsupported.get_or_insert((*statement, map_unsupported(*kind)));
            }
        }
    }
    if let Some((statement, kind)) = unsupported {
        return Err(unsupported_error(statement, kind)?);
    }
    if !script.is_empty() && !callables.is_empty() {
        return Err(NormalSourcePlanErrorV1::MixedSourceFamilies);
    }
    if !callables.is_empty() {
        return Err(NormalSourcePlanErrorV1::MissingSourceEntry);
    }
    Ok(ParserBoundNormalSourcePlanDecisionV1::Script {
        statements: script.into_boxed_slice(),
    })
}

fn classify_app(
    rows: &[NormalSourcePlanTopLevelRowRefV1<'_>],
    main_statement: u32,
    main_callable: &crate::parser::CallableDeclarationIdentityV1,
    child_callables: &[&crate::parser::CallableDeclarationIdentityV1],
) -> Result<ParserBoundNormalSourcePlanDecisionV1, NormalSourcePlanErrorV1> {
    let mut script = Vec::new();
    let mut top_level = Vec::new();
    let mut main = None;
    let mut unsupported = None;
    for row in rows {
        match row {
            NormalSourcePlanTopLevelRowRefV1::Executable { statement } => {
                script.push(site(*statement)?);
            }
            NormalSourcePlanTopLevelRowRefV1::TopLevelCallable { statement, .. } => {
                top_level.push(NormalAdditionalCallableSiteV1::TopLevel(site(*statement)?));
            }
            NormalSourcePlanTopLevelRowRefV1::Box(source)
                if source.statement() == main_statement =>
            {
                if main.replace(source).is_some() {
                    return Err(NormalSourcePlanErrorV1::RootExecutionRelationMismatch);
                }
            }
            NormalSourcePlanTopLevelRowRefV1::Box(source) => {
                unsupported
                    .get_or_insert((source.statement(), NormalUnsupportedTopLevelKindV1::Box));
            }
            NormalSourcePlanTopLevelRowRefV1::Unsupported { statement, kind } => {
                unsupported.get_or_insert((*statement, map_unsupported(*kind)));
            }
        }
    }
    if let Some((statement, kind)) = unsupported {
        return Err(unsupported_error(statement, kind)?);
    }
    if !script.is_empty() {
        return Err(NormalSourcePlanErrorV1::MixedSourceFamilies);
    }
    let main = main.ok_or(NormalSourcePlanErrorV1::RootExecutionRelationMismatch)?;
    validate_app(main, main_callable, child_callables, top_level)
}

fn validate_app(
    main: &NormalSourcePlanBoxRefV1<'_>,
    main_callable: &crate::parser::CallableDeclarationIdentityV1,
    child_callables: &[&crate::parser::CallableDeclarationIdentityV1],
    mut additional: Vec<NormalAdditionalCallableSiteV1>,
) -> Result<ParserBoundNormalSourcePlanDecisionV1, NormalSourcePlanErrorV1> {
    require_static_main_v1(main.is_static())?;
    if usize::try_from(main.observed_member_count()).ok() != Some(main.methods().len()) {
        return Err(NormalSourcePlanErrorV1::MainMemberCoverageMismatch {
            observed: main.observed_member_count(),
            callable: main.methods().len(),
        });
    }
    let statement = site(main.statement())?;
    let mut main_method = None;
    let mut matched_children = 0usize;
    for method in main.methods() {
        let method_site = NormalMainMethodSiteV1::new(
            statement.statement_index(),
            method.diagnostic_name().into(),
            usize::try_from(method.arity())
                .map_err(|_| NormalSourcePlanErrorV1::RootExecutionRelationMismatch)?,
            method.is_static(),
        );
        if method.identity().same_as(main_callable) {
            if main_method.replace(method_site).is_some() {
                return Err(NormalSourcePlanErrorV1::RootExecutionRelationMismatch);
            }
            continue;
        }
        if !child_callables
            .iter()
            .any(|identity| method.identity().same_as(identity))
        {
            return Err(NormalSourcePlanErrorV1::RootExecutionRelationMismatch);
        }
        matched_children += 1;
        additional.push(NormalAdditionalCallableSiteV1::MainMethod(method_site));
    }
    if matched_children != child_callables.len() {
        return Err(NormalSourcePlanErrorV1::RootExecutionRelationMismatch);
    }
    let main_method = main_method.ok_or(NormalSourcePlanErrorV1::MainMethodMissing)?;
    if main_method.method_key() != "main" {
        return Err(NormalSourcePlanErrorV1::MainMethodNameMismatch {
            method_key: main_method.method_key().into(),
            declaration_name: main_method.method_key().into(),
        });
    }
    if !main_method.is_static() {
        return Err(NormalSourcePlanErrorV1::MainMethodMustBeStatic);
    }
    if main_method.arity() != 0 {
        return Err(NormalSourcePlanErrorV1::MainArityMismatch {
            actual: main_method.arity(),
        });
    }
    if additional.is_empty() {
        Ok(ParserBoundNormalSourcePlanDecisionV1::Main0 {
            main_box: statement,
            main_method,
        })
    } else {
        Ok(ParserBoundNormalSourcePlanDecisionV1::CallableModule {
            main_box: statement,
            main_method,
            additional_callables: additional.into_boxed_slice(),
        })
    }
}

/// Shared source-policy boundary for the exact App root relation.
///
/// Both canonical source-plan admission and the normal/default root consumer
/// call this helper before any Builder projection. It observes the parser-
/// issued fact only; it does not reopen syntax or issue a second policy.
pub(in crate::mir) fn require_static_main_v1(
    main_box_is_static: bool,
) -> Result<(), NormalSourcePlanErrorV1> {
    if main_box_is_static {
        Ok(())
    } else {
        Err(NormalSourcePlanErrorV1::MainMustBeStatic)
    }
}

fn site(statement: u32) -> Result<NormalTopLevelSiteV1, NormalSourcePlanErrorV1> {
    usize::try_from(statement)
        .map(NormalTopLevelSiteV1::new)
        .map_err(|_| NormalSourcePlanErrorV1::RootExecutionRelationMismatch)
}

fn unsupported_error(
    statement: u32,
    kind: NormalUnsupportedTopLevelKindV1,
) -> Result<NormalSourcePlanErrorV1, NormalSourcePlanErrorV1> {
    Ok(NormalSourcePlanErrorV1::UnsupportedTopLevelSurface {
        statement_index: usize::try_from(statement)
            .map_err(|_| NormalSourcePlanErrorV1::RootExecutionRelationMismatch)?,
        kind,
    })
}

fn map_unsupported(kind: NormalSourcePlanUnsupportedKindV1) -> NormalUnsupportedTopLevelKindV1 {
    match kind {
        NormalSourcePlanUnsupportedKindV1::NestedProgram => {
            NormalUnsupportedTopLevelKindV1::NestedProgram
        }
        NormalSourcePlanUnsupportedKindV1::BuildGate => NormalUnsupportedTopLevelKindV1::BuildGate,
        NormalSourcePlanUnsupportedKindV1::Using => NormalUnsupportedTopLevelKindV1::Using,
        NormalSourcePlanUnsupportedKindV1::Import => NormalUnsupportedTopLevelKindV1::Import,
        NormalSourcePlanUnsupportedKindV1::Enum => NormalUnsupportedTopLevelKindV1::Enum,
        NormalSourcePlanUnsupportedKindV1::Brand => NormalUnsupportedTopLevelKindV1::Brand,
        NormalSourcePlanUnsupportedKindV1::TypeAlias => NormalUnsupportedTopLevelKindV1::TypeAlias,
        NormalSourcePlanUnsupportedKindV1::Global => NormalUnsupportedTopLevelKindV1::Global,
        NormalSourcePlanUnsupportedKindV1::StaticConstTable => {
            NormalUnsupportedTopLevelKindV1::StaticConstTable
        }
    }
}
