//! Selected normal/default compilation through one published candidate.
//!
//! This is the live in-place migration seam.  The request fixes source
//! identity and admission before the compiler opens a candidate.  The one
//! compatibility owner below temporarily owns general raw root/module
//! lowering; later AST-node cells shrink that surface until R3 removes it.

use std::{collections::HashMap, time::Instant};

use crate::ast::ASTNode;
use crate::mir::builder::{BuilderInvocationConfigV1, ModuleBuilderInvocationSessionV1};

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
    ast: ASTNode,
    source: NormalSourceIdentityV1,
    imports: HashMap<String, String>,
    admission: NormalCompileAdmissionV1,
    result_contract: CurrentNormalCompileResultContractV1,
}

impl NormalCompileRequestV1 {
    fn new(
        ast: ASTNode,
        source_file: Option<&str>,
        imports: HashMap<String, String>,
        admission: NormalCompileAdmissionV1,
    ) -> Self {
        Self {
            ast,
            source: NormalSourceIdentityV1::from_hint(source_file, admission),
            imports,
            admission,
            result_contract: CurrentNormalCompileResultContractV1::ReportPreTransformVerification,
        }
    }

    pub fn for_mir_mode(
        ast: ASTNode,
        source_file: Option<&str>,
        imports: HashMap<String, String>,
    ) -> Self {
        Self::new(
            ast,
            source_file,
            imports,
            NormalCompileAdmissionV1::PreparedSourceWithImports(
                NormalPreparedSourceCallerV1::MirMode,
            ),
        )
    }

    pub fn for_minimal_mir_json(ast: ASTNode, source_file: Option<&str>) -> Self {
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
    ) -> Self {
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
    ) -> Self {
        Self::new(
            ast,
            source_file,
            imports,
            NormalCompileAdmissionV1::PreparedSourceWithImports(
                NormalPreparedSourceCallerV1::WasmSourceCompiler,
            ),
        )
    }

    fn into_parts(
        self,
    ) -> (
        ASTNode,
        NormalSourceIdentityV1,
        HashMap<String, String>,
        NormalCompileAdmissionV1,
        CurrentNormalCompileResultContractV1,
    ) {
        (
            self.ast,
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
        let (ast, source, imports, _admission, result_contract) = request.into_parts();
        let token = compiler
            .invocation_identity
            .issue_raw()
            .map_err(|error| error.to_string())?;
        let config = BuilderInvocationConfigV1::snapshot_for_raw_with_imports(
            &compiler.builder,
            source.source_file(),
            imports,
        );
        let mut session =
            ModuleBuilderInvocationSessionV1::open_for_token(&token, &compiler.builder, config);

        let stage_start = Instant::now();
        let module = ExistingGeneralModuleCompatibilityV1::lower(&mut session, ast)?;
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

struct ExistingGeneralModuleCompatibilityV1;

impl ExistingGeneralModuleCompatibilityV1 {
    fn lower(
        session: &mut ModuleBuilderInvocationSessionV1,
        ast: ASTNode,
    ) -> Result<crate::mir::MirModule, String> {
        session.builder_mut().build_module(ast)
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
