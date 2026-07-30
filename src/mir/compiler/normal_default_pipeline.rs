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
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};
    use crate::mir::MirPrinter;
    use crate::parser::NyashParser;

    fn program() -> ASTNode {
        ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        }
    }

    fn non_program() -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }
    }

    #[test]
    fn normal_ingress_materializes_required_callable_main_without_changing_script() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        crate::test_support::with_env_var("NYASH_BUILD_STATIC_MAIN_ENTRY", "1", || {
            let app = NyashParser::parse_from_string(
                "static box Main { helper() { return 1 } main(p0) { return p0 } }",
            )
            .expect("App source");
            let script = program();
            let mut compiler = MirCompiler::with_options(false);

            let app_result = compiler
                .compile_normal(
                    NormalCompileRequestV1::for_mir_mode(app, None, HashMap::new())
                        .expect("App request"),
                )
                .expect("App compile");
            assert!(app_result.module.functions.contains_key("Main.helper/0"));
            assert!(app_result.module.functions.contains_key("Main.main/1"));
            assert!(app_result.module.functions.contains_key("main"));

            let script_result = compiler
                .compile_normal(
                    NormalCompileRequestV1::for_mir_mode(script, None, HashMap::new())
                        .expect("Script request"),
                )
                .expect("Script compile");
            assert!(script_result.module.functions.contains_key("main"));
            assert!(!script_result.module.functions.contains_key("Main.main/0"));
        });
    }

    #[test]
    fn normal_ingress_snapshots_runtime_inputs_permissively_at_compile_time() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let source = "static box Main { main(args) { return 0 } }";
        let request = crate::test_support::with_env_vars(
            &[
                ("NYASH_SCRIPT_ARGS_JSON", Some(r#"["request-time"]"#)),
                ("HAKO_SCRIPT_ARGS_JSON", None),
                ("NYASH_BUILDER_SAFEPOINT_ENTRY", Some("0")),
            ],
            || {
                let ast = NyashParser::parse_from_string(source).expect("App source");
                NormalCompileRequestV1::for_mir_mode(ast, None, HashMap::new())
                    .expect("App request")
            },
        );
        let selected = crate::test_support::with_env_vars(
            &[
                ("NYASH_SCRIPT_ARGS_JSON", Some(r#"["ingress-time"]"#)),
                ("HAKO_SCRIPT_ARGS_JSON", Some(r#"["must-not-win"]"#)),
                ("NYASH_BUILDER_SAFEPOINT_ENTRY", Some("On")),
            ],
            || {
                MirCompiler::with_options(false)
                    .compile_normal(request)
                    .expect("normal compile must snapshot at compile ingress")
            },
        );
        let selected_dump = MirPrinter::new().print_module(&selected.module);
        assert!(selected_dump.contains("safepoint"), "{selected_dump}");
        assert!(selected_dump.contains("ingress-time"), "{selected_dump}");
        assert!(!selected_dump.contains("request-time"), "{selected_dump}");
        assert!(!selected_dump.contains("must-not-win"), "{selected_dump}");

        let malformed = crate::test_support::with_env_vars(
            &[
                ("NYASH_SCRIPT_ARGS_JSON", Some("{malformed}")),
                ("HAKO_SCRIPT_ARGS_JSON", Some(r#"["must-stay-masked"]"#)),
                ("NYASH_BUILDER_SAFEPOINT_ENTRY", Some(" maybe ")),
            ],
            || {
                let ast = NyashParser::parse_from_string(source).expect("App source");
                MirCompiler::with_options(false)
                    .compile_normal(
                        NormalCompileRequestV1::for_mir_mode(ast, None, HashMap::new())
                            .expect("App request"),
                    )
                    .expect("malformed normal inputs remain permissive")
            },
        );
        let malformed_dump = MirPrinter::new().print_module(&malformed.module);
        assert!(!malformed_dump.contains("safepoint"), "{malformed_dump}");
        assert!(
            !malformed_dump.contains("must-stay-masked"),
            "{malformed_dump}"
        );

        let hako_fallback = crate::test_support::with_env_vars(
            &[
                ("NYASH_SCRIPT_ARGS_JSON", None),
                ("HAKO_SCRIPT_ARGS_JSON", Some(r#"["hako-fallback"]"#)),
                ("NYASH_BUILDER_SAFEPOINT_ENTRY", Some("off")),
            ],
            || {
                let ast = NyashParser::parse_from_string(source).expect("App source");
                MirCompiler::with_options(false)
                    .compile_normal(
                        NormalCompileRequestV1::for_mir_mode(ast, None, HashMap::new())
                            .expect("App request"),
                    )
                    .expect("HAKO fallback must remain normal-compatible")
            },
        );
        let hako_dump = MirPrinter::new().print_module(&hako_fallback.module);
        assert!(hako_dump.contains("hako-fallback"), "{hako_dump}");
        assert!(!hako_dump.contains("safepoint"), "{hako_dump}");
    }

    #[test]
    fn all_typed_program_constructors_share_one_program_admission() {
        let imports = HashMap::from([("Alias".to_owned(), "Target".to_owned())]);
        assert!(
            NormalCompileRequestV1::for_mir_mode(program(), Some("mir.hako"), imports.clone(),)
                .is_ok()
        );
        assert!(
            NormalCompileRequestV1::for_minimal_mir_json(program(), Some("minimal.hako")).is_ok()
        );
        assert!(NormalCompileRequestV1::for_llvm_source(
            program(),
            Some("llvm.hako"),
            imports.clone(),
        )
        .is_ok());
        assert!(
            NormalCompileRequestV1::for_wasm_source(program(), Some("wasm.hako"), imports).is_ok()
        );
        assert!(NormalCompileRequestV1::for_program_json_v0_import_bundle(program()).is_ok());
        assert!(NormalCompileRequestV1::for_repl_program(program()).is_ok());

        for rejected in [
            NormalCompileRequestV1::for_mir_mode(non_program(), Some("mir.hako"), HashMap::new()),
            NormalCompileRequestV1::for_minimal_mir_json(non_program(), Some("minimal.hako")),
            NormalCompileRequestV1::for_llvm_source(
                non_program(),
                Some("llvm.hako"),
                HashMap::new(),
            ),
            NormalCompileRequestV1::for_wasm_source(
                non_program(),
                Some("wasm.hako"),
                HashMap::new(),
            ),
            NormalCompileRequestV1::for_program_json_v0_import_bundle(non_program()),
            NormalCompileRequestV1::for_repl_program(non_program()),
        ] {
            let rejected = rejected.expect_err("non-Program root must reject at request admission");
            assert_eq!(
                rejected.error(),
                NormalProgramCompileRequestErrorV1::ExpectedProgramRoot
            );
            rejected.discard();
        }
    }

    #[test]
    fn program_json_v0_import_bundle_fixes_source_and_empty_builder_imports() {
        let request = NormalCompileRequestV1::for_program_json_v0_import_bundle(program())
            .expect("Program-v0 import bundle must use typed Program admission");
        let (_, source, imports, admission, _) = request.into_parts();

        assert_eq!(source.source_file(), Some("<json_v0/imports>"));
        assert!(imports.is_empty());
        assert_eq!(
            admission,
            NormalCompileAdmissionV1::ProgramJsonV0ImportBundleNoBuilderImports
        );
    }

    #[test]
    fn repl_program_fixes_source_and_empty_builder_imports() {
        let request = NormalCompileRequestV1::for_repl_program(program())
            .expect("REPL must use typed Program admission");
        let (_, source, imports, admission, _) = request.into_parts();

        assert_eq!(source.source_file(), Some("<repl>"));
        assert!(imports.is_empty());
        assert_eq!(
            admission,
            NormalCompileAdmissionV1::ReplProgramNoBuilderImports
        );
    }

    #[test]
    fn post_macro_whole_file_seal_accepts_program_and_rejects_non_program() {
        let program =
            VerifiedPostMacroWholeFileProgramV1::seal(program()).expect("Program must seal once");
        let request =
            NormalCompileRequestV1::for_stage1_direct_post_macro(program, Some("stage1.hako"));
        let (_, source, imports, admission, _) = request.into_parts();
        assert_eq!(source.source_file(), Some("stage1.hako"));
        assert!(imports.is_empty());
        assert_eq!(
            admission,
            NormalCompileAdmissionV1::Stage1DirectPostMacroProgramNoImports
        );

        let rejected = VerifiedPostMacroWholeFileProgramV1::seal(non_program())
            .expect_err("Literal must fail the whole-file Program contract");
        assert_eq!(
            rejected.error(),
            PostMacroWholeFileProgramErrorV1::ExpectedProgram
        );
        rejected.discard();
    }

    #[test]
    fn selfhost_macro_preexpand_fixes_anonymous_source_and_empty_imports() {
        let program =
            VerifiedPostMacroWholeFileProgramV1::seal(program()).expect("Program must seal once");
        let request = NormalCompileRequestV1::for_selfhost_macro_preexpand(program);
        let (_, source, imports, admission, _) = request.into_parts();

        assert_eq!(source.source_file(), None);
        assert!(imports.is_empty());
        assert_eq!(
            admission,
            NormalCompileAdmissionV1::SelfhostMacroPreexpandProgramNoImports
        );
    }

    #[test]
    fn vm_hako_post_macro_preserves_named_source_and_exact_imports() {
        let program =
            VerifiedPostMacroWholeFileProgramV1::seal(program()).expect("Program must seal once");
        let imports = HashMap::from([("Alias".to_owned(), "Target".to_owned())]);
        let request = NormalCompileRequestV1::for_vm_hako_post_macro(
            program,
            "vm-hako.hako",
            imports.clone(),
        );
        let (_, source, actual_imports, admission, _) = request.into_parts();

        assert_eq!(source.source_file(), Some("vm-hako.hako"));
        assert_eq!(actual_imports, imports);
        assert_eq!(
            admission,
            NormalCompileAdmissionV1::VmHakoPostMacroProgramWithImports
        );
    }

    #[test]
    fn vm_fallback_post_macro_preserves_named_source_and_empty_imports() {
        let program =
            VerifiedPostMacroWholeFileProgramV1::seal(program()).expect("Program must seal once");
        let request = NormalCompileRequestV1::for_vm_fallback_post_macro(program, "fallback.hako");
        let (_, source, imports, admission, _) = request.into_parts();

        assert_eq!(source.source_file(), Some("fallback.hako"));
        assert!(imports.is_empty());
        assert_eq!(
            admission,
            NormalCompileAdmissionV1::VmFallbackPostMacroProgramNoImports
        );
    }

    #[test]
    fn vm_keep_post_macro_preserves_named_source_and_exact_imports() {
        let program =
            VerifiedPostMacroWholeFileProgramV1::seal(program()).expect("Program must seal once");
        let imports = HashMap::from([("KeepAlias".to_owned(), "KeepTarget".to_owned())]);
        let request = NormalCompileRequestV1::for_vm_keep_post_macro(
            program,
            "vm-keep.hako",
            imports.clone(),
        );
        let (_, source, actual_imports, admission, _) = request.into_parts();

        assert_eq!(source.source_file(), Some("vm-keep.hako"));
        assert_eq!(actual_imports, imports);
        assert_eq!(
            admission,
            NormalCompileAdmissionV1::VmKeepPostMacroProgramWithImports
        );
    }
}
