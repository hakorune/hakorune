//! Total parser postpass result for the broad AST API family.
//!
//! `SourceSealedOrdinary` is the only resolver-visible arm. All other arms
//! preserve the historical AST contract without pretending that a parser
//! source seal exists. Cohort classification is structural and runs once on
//! the already-pruned AST; it is not a name lookup or source re-scan.

use crate::ast::ASTNode;

use super::callable_source_anchor::PreparedCallableSourceV1;
use super::source_seal::{ParsedProgramWithSourceV1, ParserBoxSourceSealV1};
use super::{BuildGateExplainReport, ParseError, ParserMetadata};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ParserCompatibilityCohortV1 {
    InterfaceBox,
    StaticBox,
    RecordBox,
    MixedProgram,
    TopLevelBuildGate,
    NoBoxDeclarations,
    NonProgram,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ParserPostpassProgramCohortV1 {
    OrdinaryTopLevelBox,
    InterfaceBox,
    StaticBox,
    RecordBox,
    MixedProgram,
    TopLevelBuildGate,
    NoBoxDeclarations,
    NonProgram,
}

impl ParserPostpassProgramCohortV1 {
    fn is_ordinary(self) -> bool {
        matches!(self, Self::OrdinaryTopLevelBox)
    }

    fn compatibility(self) -> ParserCompatibilityCohortV1 {
        match self {
            Self::InterfaceBox => ParserCompatibilityCohortV1::InterfaceBox,
            Self::StaticBox => ParserCompatibilityCohortV1::StaticBox,
            Self::RecordBox => ParserCompatibilityCohortV1::RecordBox,
            Self::MixedProgram => ParserCompatibilityCohortV1::MixedProgram,
            Self::TopLevelBuildGate => ParserCompatibilityCohortV1::TopLevelBuildGate,
            Self::NoBoxDeclarations => ParserCompatibilityCohortV1::NoBoxDeclarations,
            Self::NonProgram => ParserCompatibilityCohortV1::NonProgram,
            Self::OrdinaryTopLevelBox => {
                unreachable!("ordinary cohort cannot become compatibility")
            }
        }
    }
}

#[derive(Debug)]
pub(super) enum ParserBoxPostpassRowV1 {
    SourceSealedOrdinary {
        final_box_ordinal: usize,
        seal: ParserBoxSourceSealV1,
    },
    AstOnlyCompatibility {
        final_box_ordinal: usize,
        cohort: ParserCompatibilityCohortV1,
    },
}

#[derive(Debug)]
pub(super) struct ParserBoxPostpassCoverageV1 {
    program_cohort: ParserPostpassProgramCohortV1,
    rows: Box<[ParserBoxPostpassRowV1]>,
}

impl ParserBoxPostpassCoverageV1 {
    pub(super) fn program_cohort(&self) -> ParserPostpassProgramCohortV1 {
        self.program_cohort
    }

    pub(super) fn rows(&self) -> &[ParserBoxPostpassRowV1] {
        &self.rows
    }
}

#[derive(Debug)]
pub(crate) struct CompletedParserPostpassV1 {
    program: CompletedParserProgramV1,
    metadata: ParserMetadata,
    explain: Option<BuildGateExplainReport>,
    box_coverage: ParserBoxPostpassCoverageV1,
}

#[derive(Debug)]
enum CompletedParserProgramV1 {
    Initial(super::initial_callable_program_source::VerifiedInitialCallableProgramSourceV1),
    Compatibility {
        ast: ASTNode,
        callable_rows: Box<[PreparedCallableSourceV1]>,
    },
}

impl CompletedParserPostpassV1 {
    pub(crate) fn ast(&self) -> &ASTNode {
        match &self.program {
            CompletedParserProgramV1::Initial(program) => program.ast(),
            CompletedParserProgramV1::Compatibility { ast, .. } => ast,
        }
    }

    pub(super) fn from_source_product(
        product: ParsedProgramWithSourceV1,
        explain: Option<BuildGateExplainReport>,
    ) -> Result<Self, ParserPostpassEnvelopeErrorV1> {
        let (initial_callable_source, seals, final_box_ordinals, metadata) =
            product.into_postpass_parts();
        let ast = initial_callable_source.ast();
        if seals.len() != final_box_ordinals.len() {
            return Err(ParserPostpassEnvelopeErrorV1::SourceCoverageMismatch {
                seals: seals.len(),
                final_box_ordinals: final_box_ordinals.len(),
            });
        }
        let program_cohort = classify_program(&ast);
        if !program_cohort.is_ordinary() {
            return Err(ParserPostpassEnvelopeErrorV1::SourceSealForCompatibility {
                cohort: program_cohort,
            });
        }
        let rows = seals
            .into_vec()
            .into_iter()
            .zip(final_box_ordinals.into_vec())
            .map(
                |(seal, final_box_ordinal)| ParserBoxPostpassRowV1::SourceSealedOrdinary {
                    final_box_ordinal,
                    seal,
                },
            )
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Ok(Self {
            program: CompletedParserProgramV1::Initial(initial_callable_source),
            metadata,
            explain,
            box_coverage: ParserBoxPostpassCoverageV1 {
                program_cohort,
                rows,
            },
        })
    }

    pub(super) fn from_compatibility(
        ast: ASTNode,
        metadata: ParserMetadata,
        explain: Option<BuildGateExplainReport>,
        callable_rows: Box<[PreparedCallableSourceV1]>,
    ) -> Result<Self, ParserPostpassEnvelopeErrorV1> {
        let program_cohort = classify_program(&ast);
        if program_cohort.is_ordinary() {
            return Err(ParserPostpassEnvelopeErrorV1::CompatibilityForOrdinary);
        }
        let cohort = program_cohort.compatibility();
        let rows = compatibility_rows(&ast, cohort);
        Ok(Self {
            program: CompletedParserProgramV1::Compatibility { ast, callable_rows },
            metadata,
            explain,
            box_coverage: ParserBoxPostpassCoverageV1 {
                program_cohort,
                rows,
            },
        })
    }

    pub(super) fn from_initial_compatibility(
        program: super::initial_callable_program_source::VerifiedInitialCallableProgramSourceV1,
        metadata: ParserMetadata,
        explain: Option<BuildGateExplainReport>,
    ) -> Result<Self, ParserPostpassEnvelopeErrorV1> {
        let program_cohort = classify_program(program.ast());
        if program_cohort.is_ordinary() {
            return Err(ParserPostpassEnvelopeErrorV1::CompatibilityForOrdinary);
        }
        let cohort = program_cohort.compatibility();
        let rows = compatibility_rows(program.ast(), cohort);
        Ok(Self {
            program: CompletedParserProgramV1::Initial(program),
            metadata,
            explain,
            box_coverage: ParserBoxPostpassCoverageV1 {
                program_cohort,
                rows,
            },
        })
    }

    pub(crate) fn into_ast(self) -> ASTNode {
        match self.program {
            CompletedParserProgramV1::Initial(program) => program.into_ast(),
            CompletedParserProgramV1::Compatibility { ast, .. } => ast,
        }
    }

    pub(super) fn into_normal_callable_program(
        self,
        parameter_source: super::callable_parameter_source::ParserCallableParameterSourceDispositionV1,
    ) -> Result<
        super::normal_callable_program_source::ParsedNormalCallableProgramV1,
        super::normal_callable_program_source::NormalCallableParameterSourceRejectV1,
    > {
        use super::normal_callable_program_source::{
            NormalCallableParserCompatibilityV1 as Compatibility,
            ParsedNormalCallableProgramV1 as Program, PreparedNormalCallableProgramSourceV1,
        };

        match self.program {
            CompletedParserProgramV1::Initial(program) => {
                PreparedNormalCallableProgramSourceV1::issue(program, parameter_source)
                    .map(Program::SourceBacked)
            }
            CompletedParserProgramV1::Compatibility { ast, .. } => {
                let cohort = match self.box_coverage.program_cohort {
                    ParserPostpassProgramCohortV1::InterfaceBox => Compatibility::InterfaceBox,
                    ParserPostpassProgramCohortV1::RecordBox => Compatibility::RecordBox,
                    ParserPostpassProgramCohortV1::MixedProgram
                    | ParserPostpassProgramCohortV1::StaticBox => Compatibility::MixedProgram,
                    ParserPostpassProgramCohortV1::TopLevelBuildGate => {
                        Compatibility::TopLevelBuildGate
                    }
                    ParserPostpassProgramCohortV1::NoBoxDeclarations => {
                        Compatibility::NoBoxDeclarations
                    }
                    ParserPostpassProgramCohortV1::NonProgram => Compatibility::NonProgram,
                    ParserPostpassProgramCohortV1::OrdinaryTopLevelBox => {
                        Compatibility::UnsupportedCallableSource
                    }
                };
                Ok(Program::Compatibility { ast, cohort })
            }
        }
    }

    pub(super) fn into_ast_and_explain(
        self,
    ) -> Result<(ASTNode, BuildGateExplainReport), ParserPostpassEnvelopeErrorV1> {
        let Self {
            program, explain, ..
        } = self;
        let explain = explain.ok_or(ParserPostpassEnvelopeErrorV1::ExplainDecisionSetNotReady)?;
        let ast = match program {
            CompletedParserProgramV1::Initial(program) => program.into_ast(),
            CompletedParserProgramV1::Compatibility { ast, .. } => ast,
        };
        Ok((ast, explain))
    }

    pub(super) fn into_ast_and_metadata(self) -> (ASTNode, ParserMetadata) {
        // This is the sole consuming pair projection. Metadata is moved from
        // the completed product; it is never reconstructed from AST nodes.
        let Self {
            program, metadata, ..
        } = self;
        let ast = match program {
            CompletedParserProgramV1::Initial(program) => program.into_ast(),
            CompletedParserProgramV1::Compatibility { ast, .. } => ast,
        };
        (ast, metadata)
    }

    pub(super) fn metadata(&self) -> &ParserMetadata {
        &self.metadata
    }

    pub(super) fn explain(&self) -> Option<&BuildGateExplainReport> {
        self.explain.as_ref()
    }

    pub(super) fn box_coverage(&self) -> &ParserBoxPostpassCoverageV1 {
        &self.box_coverage
    }

    pub(super) fn callable_rows(&self) -> &[PreparedCallableSourceV1] {
        match &self.program {
            CompletedParserProgramV1::Initial(program) => program.callable_rows(),
            CompletedParserProgramV1::Compatibility { callable_rows, .. } => callable_rows,
        }
    }

    pub(super) fn initial_callable_source(
        &self,
    ) -> Option<&super::initial_callable_program_source::VerifiedInitialCallableProgramSourceV1>
    {
        match &self.program {
            CompletedParserProgramV1::Initial(program) => Some(program),
            CompletedParserProgramV1::Compatibility { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ExplainDemandV1 {
    None,
    Capture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PostpassDemandV1 {
    pub(super) explain: ExplainDemandV1,
}

impl Default for PostpassDemandV1 {
    fn default() -> Self {
        Self {
            explain: ExplainDemandV1::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ParserPostpassEnvelopeErrorV1 {
    SourceCoverageMismatch {
        seals: usize,
        final_box_ordinals: usize,
    },
    SourceSealForCompatibility {
        cohort: ParserPostpassProgramCohortV1,
    },
    CompatibilityForOrdinary,
    ExplainDecisionSetNotReady,
}

impl ParserPostpassEnvelopeErrorV1 {
    pub(super) fn into_parse_error(self) -> ParseError {
        let message = match self {
            Self::SourceCoverageMismatch {
                seals,
                final_box_ordinals,
            } => format!(
                "total parser postpass source coverage mismatch: seals={}, final_box_ordinals={}",
                seals, final_box_ordinals
            ),
            Self::SourceSealForCompatibility { cohort } => {
                format!("source seal issued for compatibility cohort: {cohort:?}")
            }
            Self::CompatibilityForOrdinary => {
                "compatibility postpass selected for an ordinary cohort".to_owned()
            }
            Self::ExplainDecisionSetNotReady => {
                "S0 explain capture requires the full BuildGate decision set".to_owned()
            }
        };
        ParseError::BuildCfg { message, line: 0 }
    }
}

pub(super) fn classify_program(ast: &ASTNode) -> ParserPostpassProgramCohortV1 {
    let ASTNode::Program { statements, .. } = ast else {
        return ParserPostpassProgramCohortV1::NonProgram;
    };
    let mut box_cohorts = Vec::new();
    for statement in statements {
        match statement {
            ASTNode::BuildGate { .. } => {
                return ParserPostpassProgramCohortV1::TopLevelBuildGate;
            }
            ASTNode::BoxDeclaration {
                is_interface,
                is_record,
                is_static,
                ..
            } => box_cohorts.push(if *is_interface {
                ParserPostpassProgramCohortV1::InterfaceBox
            } else if *is_record {
                ParserPostpassProgramCohortV1::RecordBox
            } else if *is_static {
                ParserPostpassProgramCohortV1::StaticBox
            } else {
                ParserPostpassProgramCohortV1::OrdinaryTopLevelBox
            }),
            _ => {}
        }
    }
    let Some(first) = box_cohorts.first().copied() else {
        return ParserPostpassProgramCohortV1::NoBoxDeclarations;
    };
    if box_cohorts.iter().all(|cohort| *cohort == first) {
        first
    } else {
        ParserPostpassProgramCohortV1::MixedProgram
    }
}

fn compatibility_rows(
    ast: &ASTNode,
    program_cohort: ParserCompatibilityCohortV1,
) -> Box<[ParserBoxPostpassRowV1]> {
    let ASTNode::Program { statements, .. } = ast else {
        return Box::new([]);
    };
    statements
        .iter()
        .enumerate()
        .filter_map(|(final_box_ordinal, statement)| {
            matches!(statement, ASTNode::BoxDeclaration { .. }).then_some(
                ParserBoxPostpassRowV1::AstOnlyCompatibility {
                    final_box_ordinal,
                    cohort: program_cohort,
                },
            )
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{NyashParser, ParserBuildConfig};
    use crate::tokenizer::NyashTokenizer;

    fn finish_s0(
        source: &str,
        demand: PostpassDemandV1,
    ) -> Result<CompletedParserPostpassV1, ParseError> {
        let tokens = NyashTokenizer::new(source).tokenize().unwrap();
        let mut parser = NyashParser::new(tokens);
        let ast = parser.parse_program()?;
        let product = parser.open_postpass_product(ast)?;
        product.finish_total_s0(&parser, demand)
    }

    #[test]
    fn s0_classifies_ordinary_and_compatibility_cohorts_without_names() {
        let ordinary = NyashParser::parse_from_string("box Plain {}\n").unwrap();
        assert_eq!(
            classify_program(&ordinary),
            ParserPostpassProgramCohortV1::OrdinaryTopLevelBox
        );

        let static_box = NyashParser::parse_from_string("static box StaticOnly {}\n").unwrap();
        assert_eq!(
            classify_program(&static_box),
            ParserPostpassProgramCohortV1::StaticBox
        );
    }

    #[test]
    fn s0_ordinary_rich_product_maps_to_source_sealed_row() {
        let product = NyashParser::parse_from_string_with_source_seal(
            "box Plain { run() { return 1 } }\n",
            ParserBuildConfig::default(),
        )
        .unwrap();
        let envelope = CompletedParserPostpassV1::from_source_product(product, None).unwrap();

        assert_eq!(
            envelope.box_coverage().program_cohort(),
            ParserPostpassProgramCohortV1::OrdinaryTopLevelBox
        );
        assert!(matches!(
            envelope.box_coverage().rows(),
            [ParserBoxPostpassRowV1::SourceSealedOrdinary { .. }]
        ));
        assert!(envelope.explain().is_none());
        assert!(envelope.metadata().runes.is_empty());
    }

    #[test]
    fn s0_compatibility_envelope_has_no_source_seal_row() {
        let ast = NyashParser::parse_from_string("static box StaticOnly {}\n").unwrap();
        let envelope = CompletedParserPostpassV1::from_compatibility(
            ast,
            ParserMetadata::default(),
            None,
            Box::new([]),
        )
        .unwrap();

        assert_eq!(
            envelope.box_coverage().program_cohort(),
            ParserPostpassProgramCohortV1::StaticBox
        );
        assert!(matches!(
            envelope.box_coverage().rows(),
            [ParserBoxPostpassRowV1::AstOnlyCompatibility {
                cohort: ParserCompatibilityCohortV1::StaticBox,
                ..
            }]
        ));
    }

    #[test]
    fn s0_compatibility_constructor_rejects_ordinary_ast() {
        let ast = NyashParser::parse_from_string("box Plain {}\n").unwrap();
        let error = CompletedParserPostpassV1::from_compatibility(
            ast,
            ParserMetadata::default(),
            None,
            Box::new([]),
        )
        .unwrap_err();
        assert_eq!(
            error,
            ParserPostpassEnvelopeErrorV1::CompatibilityForOrdinary
        );
    }

    #[test]
    fn s0_coordinator_selects_ordinary_source_arm_once() {
        let envelope = finish_s0(
            "box Plain { run() { return 1 } }\n",
            PostpassDemandV1::default(),
        )
        .unwrap();
        assert!(matches!(
            envelope.box_coverage().rows(),
            [ParserBoxPostpassRowV1::SourceSealedOrdinary { .. }]
        ));
    }

    #[test]
    fn s0_coordinator_selects_explicit_compatibility_arm() {
        let envelope = finish_s0(
            "box Plain {}\nstatic box StaticOnly {}\n",
            PostpassDemandV1::default(),
        )
        .unwrap();
        assert_eq!(
            envelope.box_coverage().program_cohort(),
            ParserPostpassProgramCohortV1::MixedProgram
        );
        assert!(envelope
            .box_coverage()
            .rows()
            .iter()
            .all(|row| matches!(row, ParserBoxPostpassRowV1::AstOnlyCompatibility { .. })));
        assert!(envelope.explain().is_none());
    }

    #[test]
    fn s0_explain_capture_uses_shared_projection() {
        let envelope = finish_s0(
            "box Plain {}\n",
            PostpassDemandV1 {
                explain: ExplainDemandV1::Capture,
            },
        )
        .unwrap();
        let report = envelope.explain().expect("shared projection report");
        assert_eq!(report.conditional_group_count, 0);
    }
}
