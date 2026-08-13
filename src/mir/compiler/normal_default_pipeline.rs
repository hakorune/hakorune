//! Selected normal/default compilation through one published candidate.
//!
//! This is the live in-place migration seam.  The request fixes source
//! identity and admission before the compiler opens a candidate.  The one
//! root/catalog lifecycle below owns selected-normal orchestration while later
//! AST-node cells retire the remaining raw lowering responsibilities.

use std::{collections::HashMap, time::Instant};

use crate::ast::ASTNode;
use crate::mir::builder::{
    BuilderInvocationConfigV1, CallableMainMaterializationPolicyV1,
    ModuleBuilderInvocationSessionV1, NormalRuntimeInputSnapshotV1,
    PreparedNormalDefaultProgramRootV1,
};
use crate::parser::VerifiedFinalCallableProgramSourceV1;

use super::{MirCompileResult, MirCompiler, MirFinishScheduleV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NormalPreparedSourceCallerV1 {
    MirMode,
    LlvmSourceCompiler,
    WasmSourceCompiler,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NormalCompileAdmissionV1 {
    PreparedSourceWithImports(NormalPreparedSourceCallerV1),
    MinimalMirJsonNoImports,
    ProgramJsonV0ImportBundleNoBuilderImports,
    ReplProgramNoBuilderImports,
    Stage1DirectPostMacroProgramNoImports,
    SelfhostMacroPreexpandProgramNoImports,
    VmHakoPostMacroProgramWithImports,
    VmFallbackPostMacroProgramNoImports,
    VmKeepPostMacroProgramWithImports,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CurrentNormalCompileResultContractV1 {
    ReportPreTransformVerification,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NormalSourceIdentityV1 {
    Named(Box<str>),
    Anonymous(NormalCompileAdmissionV1),
}

impl NormalSourceIdentityV1 {
    fn from_hint(source_file: Option<&str>, admission: NormalCompileAdmissionV1) -> Self {
        match source_file.filter(|source| !source.is_empty()) {
            Some(source) => Self::Named(source.into()),
            None => Self::Anonymous(admission),
        }
    }

    fn source_file(&self) -> Option<&str> {
        match self {
            Self::Named(source) => Some(source),
            Self::Anonymous(_) => None,
        }
    }
}

#[derive(Debug)]
pub struct NormalCompileRequestV1 {
    program: PreparedNormalDefaultProgramRootV1,
    source: NormalSourceIdentityV1,
    imports: HashMap<String, String>,
    admission: NormalCompileAdmissionV1,
    result_contract: CurrentNormalCompileResultContractV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalProgramCompileRequestErrorV1 {
    ExpectedProgramRoot,
}

#[derive(Debug)]
pub(crate) struct VerifiedPostMacroWholeFileProgramV1 {
    program: PreparedNormalDefaultProgramRootV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PostMacroWholeFileProgramErrorV1 {
    ExpectedProgram,
}

#[derive(Debug)]
pub(crate) struct RejectedPostMacroWholeFileProgramV1 {
    _output: ASTNode,
    error: PostMacroWholeFileProgramErrorV1,
}

impl VerifiedPostMacroWholeFileProgramV1 {
    pub(crate) fn seal(output: ASTNode) -> Result<Self, RejectedPostMacroWholeFileProgramV1> {
        match prepare_normal_program_root(output) {
            Ok(program) => Ok(Self { program }),
            Err(output) => Err(RejectedPostMacroWholeFileProgramV1 {
                _output: output,
                error: PostMacroWholeFileProgramErrorV1::ExpectedProgram,
            }),
        }
    }
}

impl std::fmt::Display for PostMacroWholeFileProgramErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectedProgram => formatter.write_str(
                "[macro/whole-file-root] expected Program output from whole-file macro expansion",
            ),
        }
    }
}

fn prepare_normal_program_root(
    ast: ASTNode,
) -> Result<PreparedNormalDefaultProgramRootV1, ASTNode> {
    PreparedNormalDefaultProgramRootV1::seal(ast)
}

impl RejectedPostMacroWholeFileProgramV1 {
    pub(crate) fn error(&self) -> PostMacroWholeFileProgramErrorV1 {
        self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }
}

impl std::fmt::Display for NormalProgramCompileRequestErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectedProgramRoot => formatter.write_str(
                "[mir/normal-program-admission] selected normal/default source must produce Program",
            ),
        }
    }
}

impl std::error::Error for NormalProgramCompileRequestErrorV1 {}

#[derive(Debug)]
pub struct RejectedNormalProgramCompileRequestV1 {
    _ast: ASTNode,
    _source: NormalSourceIdentityV1,
    _imports: HashMap<String, String>,
    _admission: NormalCompileAdmissionV1,
    _result_contract: CurrentNormalCompileResultContractV1,
    error: NormalProgramCompileRequestErrorV1,
}

impl RejectedNormalProgramCompileRequestV1 {
    pub fn error(&self) -> NormalProgramCompileRequestErrorV1 {
        self.error
    }

    pub fn discard(self) {}
}

impl std::fmt::Display for RejectedNormalProgramCompileRequestV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(formatter)
    }
}

impl std::error::Error for RejectedNormalProgramCompileRequestV1 {}

impl NormalCompileRequestV1 {
    fn from_prepared(
        program: PreparedNormalDefaultProgramRootV1,
        source_file: Option<&str>,
        imports: HashMap<String, String>,
        admission: NormalCompileAdmissionV1,
    ) -> Self {
        Self {
            program,
            source: NormalSourceIdentityV1::from_hint(source_file, admission),
            imports,
            admission,
            result_contract: CurrentNormalCompileResultContractV1::ReportPreTransformVerification,
        }
    }

    fn new(
        ast: ASTNode,
        source_file: Option<&str>,
        imports: HashMap<String, String>,
        admission: NormalCompileAdmissionV1,
    ) -> Result<Self, RejectedNormalProgramCompileRequestV1> {
        let source = NormalSourceIdentityV1::from_hint(source_file, admission);
        let result_contract = CurrentNormalCompileResultContractV1::ReportPreTransformVerification;
        match prepare_normal_program_root(ast) {
            Ok(program) => Ok(Self {
                program,
                source,
                imports,
                admission,
                result_contract,
            }),
            Err(ast) => Err(RejectedNormalProgramCompileRequestV1 {
                _ast: ast,
                _source: source,
                _imports: imports,
                _admission: admission,
                _result_contract: result_contract,
                error: NormalProgramCompileRequestErrorV1::ExpectedProgramRoot,
            }),
        }
    }

    pub fn for_mir_mode(
        ast: ASTNode,
        source_file: Option<&str>,
        imports: HashMap<String, String>,
    ) -> Result<Self, RejectedNormalProgramCompileRequestV1> {
        Self::new(
            ast,
            source_file,
            imports,
            NormalCompileAdmissionV1::PreparedSourceWithImports(
                NormalPreparedSourceCallerV1::MirMode,
            ),
        )
    }

    pub(crate) fn for_mir_mode_callable_source(
        source: VerifiedFinalCallableProgramSourceV1,
        source_file: Option<&str>,
        imports: HashMap<String, String>,
    ) -> Self {
        Self::from_prepared(
            PreparedNormalDefaultProgramRootV1::from_callable_source(source),
            source_file,
            imports,
            NormalCompileAdmissionV1::PreparedSourceWithImports(
                NormalPreparedSourceCallerV1::MirMode,
            ),
        )
    }

    pub(crate) fn for_llvm_callable_source(
        source: VerifiedFinalCallableProgramSourceV1,
        source_file: Option<&str>,
        imports: HashMap<String, String>,
    ) -> Self {
        Self::from_prepared(
            PreparedNormalDefaultProgramRootV1::from_callable_source(source),
            source_file,
            imports,
            NormalCompileAdmissionV1::PreparedSourceWithImports(
                NormalPreparedSourceCallerV1::LlvmSourceCompiler,
            ),
        )
    }

    pub fn for_minimal_mir_json(
        ast: ASTNode,
        source_file: Option<&str>,
    ) -> Result<Self, RejectedNormalProgramCompileRequestV1> {
        Self::new(
            ast,
            source_file,
            HashMap::new(),
            NormalCompileAdmissionV1::MinimalMirJsonNoImports,
        )
    }

    pub fn for_llvm_source(
        ast: ASTNode,
        source_file: Option<&str>,
        imports: HashMap<String, String>,
    ) -> Result<Self, RejectedNormalProgramCompileRequestV1> {
        Self::new(
            ast,
            source_file,
            imports,
            NormalCompileAdmissionV1::PreparedSourceWithImports(
                NormalPreparedSourceCallerV1::LlvmSourceCompiler,
            ),
        )
    }

    pub fn for_wasm_source(
        ast: ASTNode,
        source_file: Option<&str>,
        imports: HashMap<String, String>,
    ) -> Result<Self, RejectedNormalProgramCompileRequestV1> {
        Self::new(
            ast,
            source_file,
            imports,
            NormalCompileAdmissionV1::PreparedSourceWithImports(
                NormalPreparedSourceCallerV1::WasmSourceCompiler,
            ),
        )
    }

    pub(crate) fn for_program_json_v0_import_bundle(
        ast: ASTNode,
    ) -> Result<Self, RejectedNormalProgramCompileRequestV1> {
        Self::new(
            ast,
            Some("<json_v0/imports>"),
            HashMap::new(),
            NormalCompileAdmissionV1::ProgramJsonV0ImportBundleNoBuilderImports,
        )
    }

    pub(crate) fn for_repl_program(
        ast: ASTNode,
    ) -> Result<Self, RejectedNormalProgramCompileRequestV1> {
        Self::new(
            ast,
            Some("<repl>"),
            HashMap::new(),
            NormalCompileAdmissionV1::ReplProgramNoBuilderImports,
        )
    }

    pub(crate) fn for_stage1_direct_post_macro(
        program: VerifiedPostMacroWholeFileProgramV1,
        source_file: Option<&str>,
    ) -> Self {
        Self::from_prepared(
            program.program,
            source_file,
            HashMap::new(),
            NormalCompileAdmissionV1::Stage1DirectPostMacroProgramNoImports,
        )
    }

    pub(crate) fn for_selfhost_macro_preexpand(
        program: VerifiedPostMacroWholeFileProgramV1,
    ) -> Self {
        Self::from_prepared(
            program.program,
            None,
            HashMap::new(),
            NormalCompileAdmissionV1::SelfhostMacroPreexpandProgramNoImports,
        )
    }

    pub(crate) fn for_vm_hako_post_macro(
        program: VerifiedPostMacroWholeFileProgramV1,
        source_file: &str,
        imports: HashMap<String, String>,
    ) -> Self {
        Self::from_prepared(
            program.program,
            Some(source_file),
            imports,
            NormalCompileAdmissionV1::VmHakoPostMacroProgramWithImports,
        )
    }

    pub(crate) fn for_vm_fallback_post_macro(
        program: VerifiedPostMacroWholeFileProgramV1,
        source_file: &str,
    ) -> Self {
        Self::from_prepared(
            program.program,
            Some(source_file),
            HashMap::new(),
            NormalCompileAdmissionV1::VmFallbackPostMacroProgramNoImports,
        )
    }

    pub(crate) fn for_vm_keep_post_macro(
        program: VerifiedPostMacroWholeFileProgramV1,
        source_file: &str,
        imports: HashMap<String, String>,
    ) -> Self {
        Self::from_prepared(
            program.program,
            Some(source_file),
            imports,
            NormalCompileAdmissionV1::VmKeepPostMacroProgramWithImports,
        )
    }

    fn into_parts(
        self,
    ) -> (
        PreparedNormalDefaultProgramRootV1,
        NormalSourceIdentityV1,
        HashMap<String, String>,
        NormalCompileAdmissionV1,
        CurrentNormalCompileResultContractV1,
    ) {
        (
            self.program,
            self.source,
            self.imports,
            self.admission,
            self.result_contract,
        )
    }
}

struct NormalDefaultPublishedPipelineV1;

impl NormalDefaultPublishedPipelineV1 {
    fn compile(
        compiler: &mut MirCompiler,
        request: NormalCompileRequestV1,
    ) -> Result<MirCompileResult, String> {
        let (program, source, imports, _admission, result_contract) = request.into_parts();
        let runtime_inputs = NormalRuntimeInputSnapshotV1::capture_from_normal_ingress();
        let token = compiler
            .invocation_identity
            .issue_raw()
            .map_err(|error| error.to_string())?;
        let materialization = CallableMainMaterializationPolicyV1::snapshot_from_normal_ingress();
        let config = BuilderInvocationConfigV1::snapshot_for_raw_with_imports(
            &compiler.builder,
            source.source_file(),
            imports,
        );
        let session =
            ModuleBuilderInvocationSessionV1::open_for_token(&token, &compiler.builder, config);

        let stage_start = Instant::now();
        let completed = session
            .complete_normal_default_program_root_catalog_lifecycle(
                program,
                materialization,
                runtime_inputs,
            )
            .map_err(|rejected| {
                let message = rejected.error().to_string();
                rejected.discard();
                message
            })?;
        let (session, module) = completed.into_parts();
        super::super::compile_timing::trace_stage("build_module", stage_start.elapsed());

        match result_contract {
            CurrentNormalCompileResultContractV1::ReportPreTransformVerification => {}
        }
        let result = compiler.finish_built_module(module, MirFinishScheduleV1::Legacy)?;
        let prepared = session
            .prepare_external_commit()
            .map_err(|error| error.to_string())?;
        let _receipt = prepared.commit(&mut compiler.builder);
        Ok(result)
    }
}

impl MirCompiler {
    pub fn compile_normal(
        &mut self,
        request: NormalCompileRequestV1,
    ) -> Result<MirCompileResult, String> {
        NormalDefaultPublishedPipelineV1::compile(self, request)
    }
}

#[cfg(test)]
#[path = "normal_default_pipeline_tests.rs"]
mod tests;
