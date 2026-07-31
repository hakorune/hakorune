//! Producer-backed Script semantic source for the lexical Complete closure.
//!
//! This product is intentionally narrow: the selected runtime window must be
//! empty or literal-only before a Script owner is issued.  It borrows the
//! already-owned Program while sealing one shared forest and projection; no
//! raw source carrier can manufacture the Complete loan.

use super::normal_script_lexical_binding::{
    ScriptLexicalAdmissionV1, ScriptLexicalFactsV1, ScriptSemanticLoweringState,
};
use crate::ast::ASTNode;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, ResolvedScriptSemanticDraftV1, SemanticOwnerForestDraftV1,
    SemanticOwnerRootProfileV1, SourceBindingSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
    SourcePathV1, SourceStmtSiteV1, VerifiedSemanticOwnerForestV1, VerifiedSemanticOwnerProductV1,
};

use super::normal_default_root_catalog_lifecycle::PreparedNormalDefaultProgramRootV1;

#[derive(Debug)]
pub(super) struct VerifiedScriptSemanticSourceV1<'source> {
    source: &'source PreparedNormalDefaultProgramRootV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    runtime_source_indices: Box<[usize]>,
}

impl<'source> VerifiedScriptSemanticSourceV1<'source> {
    pub(super) fn seal(
        source: &'source PreparedNormalDefaultProgramRootV1,
        owner: FunctionOwnerIdV1,
        admission: ScriptLexicalAdmissionV1,
    ) -> Result<Self, String> {
        let ASTNode::Program { statements, .. } = source.source_ast() else {
            return Err("[mir/script-semantic/source-root] expected Program".to_owned());
        };
        let ScriptLexicalAdmissionV1::Complete(ScriptLexicalFactsV1 {
            locals,
            variables,
            expression_source_indices,
        }) = admission
        else {
            return Err("[mir/script-semantic/admission] Deferred input cannot seal".to_owned());
        };
        let mut draft = ResolvedScriptSemanticDraftV1::new(owner);
        let mut local_bindings = std::collections::BTreeMap::new();
        for local in &locals {
            let site = program_statement_site(local.source_statement_index);
            if !matches!(
                statements.get(local.source_statement_index),
                Some(ASTNode::Local { .. })
            ) {
                return Err(format!(
                    "[mir/script-semantic/local-coverage] source_statement_index={}",
                    local.source_statement_index
                ));
            }
            let binding = draft.declare_local(site, local.name.clone());
            local_bindings.insert(local.source_statement_index, binding);
        }
        for variable in &variables {
            let Some(binding) = local_bindings
                .get(&variable.binding_statement_index)
                .copied()
            else {
                return Err(
                    "[mir/script-semantic/variable-binding] missing local binding".to_owned(),
                );
            };
            let mut path = SourcePathV1::from_node(
                &program_statement_site(variable.source_statement_index)
                    .node()
                    .clone(),
            );
            if variable.initializer {
                path = path.child(SourcePathSegmentV1::Initializer(0));
            }
            for segment in &variable.path {
                path = path.child(segment.clone());
            }
            let site = path.expr();
            draft.record_variable(site, binding);
        }
        let product = draft
            .seal()
            .map_err(|error| format!("[mir/script-semantic/seal] {error:?}"))?;
        let runtime_source_indices = locals
            .iter()
            .map(|local| local.source_statement_index)
            .chain(expression_source_indices.into_vec())
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect::<Box<[_]>>();
        let mut draft = SemanticOwnerForestDraftV1::new();
        draft
            .insert_product(owner, VerifiedSemanticOwnerProductV1::Script(product))
            .map_err(|error| format!("[mir/script-semantic/forest] {error:?}"))?;
        let forest = draft
            .seal()
            .map_err(|error| format!("[mir/script-semantic/forest] {error:?}"))?;
        let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
            source.source_ast(),
            &forest,
            SemanticOwnerRootProfileV1::Script,
        )
        .map_err(|error| format!("[mir/script-semantic/projection] {error}"))?;
        Ok(Self {
            source,
            forest,
            projection,
            runtime_source_indices,
        })
    }

    pub(super) fn source(&self) -> &PreparedNormalDefaultProgramRootV1 {
        &self.source
    }

    pub(super) fn forest(&self) -> &VerifiedSemanticOwnerForestV1 {
        &self.forest
    }

    pub(super) fn projection(&self) -> &VerifiedSourceProjectionV1 {
        &self.projection
    }

    pub(super) fn runtime_source_indices(&self) -> &[usize] {
        &self.runtime_source_indices
    }

    pub(super) fn local_binding_at(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        let [root] = self.forest.roots() else {
            return None;
        };
        let owner = self.forest.semantic_owner(*root)?;
        owner.declaration_binding(&SourceBindingSiteV1::Local {
            statement: SourceStmtSiteV1::from_node(site.clone()),
            ordinal: 0,
        })
    }

    pub(super) fn variable_binding_at(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        let [root] = self.forest.roots() else {
            return None;
        };
        let owner = self.forest.semantic_owner(*root)?;
        owner.variable_refs().find_map(|(candidate, reference)| {
            if candidate.node() != site {
                return None;
            }
            match reference {
                crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding) => {
                    Some(*binding)
                }
                _ => None,
            }
        })
    }

    pub(super) fn lowering_state(&self) -> ScriptSemanticLoweringState {
        let [root] = self.forest.roots() else {
            return ScriptSemanticLoweringState::default();
        };
        let Some(owner) = self.forest.semantic_owner(*root) else {
            return ScriptSemanticLoweringState::default();
        };
        let locals = owner.declaration_sites().filter_map(|site| match site {
            SourceBindingSiteV1::Local { statement, .. } => Some((
                statement.node().clone(),
                owner
                    .declaration_binding(site)
                    .expect("local declaration binding"),
            )),
            _ => None,
        });
        let variables = owner
            .variable_refs()
            .filter_map(|(site, reference)| match reference {
                crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding) => {
                    Some((site.clone(), *binding))
                }
                _ => None,
            });
        ScriptSemanticLoweringState::from_facts(locals, variables)
    }
}

fn program_statement_site(index: usize) -> SourceStmtSiteV1 {
    SourceStmtSiteV1::from_node(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(index as u32))
            .node(),
    )
}

#[cfg(test)]
mod tests {
    use super::VerifiedScriptSemanticSourceV1;
    use crate::ast::{ASTNode, LiteralValue, Span, UnaryOperator};
    use crate::mir::builder::normal_script_lexical_binding::{
        ScriptLexicalAdmissionV1, ScriptLexicalFactsV1,
    };
    use crate::mir::builder::PreparedNormalDefaultProgramRootV1;
    use crate::mir::resolved_semantics::{FunctionOwnerIssuerV1, SemanticOwnerRootProfileV1};
    use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};
    use crate::parser::NyashParser;

    fn owner() -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        FunctionOwnerIssuerV1::new_for_compilation()
            .expect("owner issuer")
            .issue()
            .expect("root owner")
    }

    #[test]
    fn literal_program_seals_one_script_owner_and_program_projection() {
        let ast = NyashParser::parse_from_string("0").expect("literal source");
        let source = PreparedNormalDefaultProgramRootV1::seal(ast).expect("Program source");
        let product = VerifiedScriptSemanticSourceV1::seal(
            &source,
            owner(),
            ScriptLexicalAdmissionV1::Complete(ScriptLexicalFactsV1 {
                locals: Box::new([]),
                variables: Box::new([]),
                expression_source_indices: vec![0].into_boxed_slice(),
            }),
        )
        .expect("literal Script product");

        assert_eq!(product.forest().owner_count(), 1);
        assert_eq!(product.forest().roots().len(), 1);
        assert_eq!(
            product
                .forest()
                .semantic_owner(product.forest().roots()[0])
                .expect("Script owner")
                .root_profile(),
            SemanticOwnerRootProfileV1::Script
        );
        assert_eq!(product.runtime_source_indices(), &[0]);
        assert!(product
            .projection()
            .owner_root(source.source_ast(), product.forest().roots()[0])
            .is_ok());
    }

    #[test]
    fn lexical_product_rejects_a_missing_local_source_ordinal() {
        let ast = NyashParser::parse_from_string("0").expect("literal source");
        let source = PreparedNormalDefaultProgramRootV1::seal(ast).expect("Program source");
        let error = VerifiedScriptSemanticSourceV1::seal(
            &source,
            owner(),
            ScriptLexicalAdmissionV1::Complete(ScriptLexicalFactsV1 {
                locals: vec![
                    super::super::normal_script_lexical_binding::ScriptLocalFactV1 {
                        source_statement_index: 1,
                        name: "x".to_owned(),
                    },
                ]
                .into_boxed_slice(),
                variables: Box::new([]),
                expression_source_indices: Box::new([]),
            }),
        )
        .expect_err("out-of-range literal coverage must reject");

        assert!(error.contains("local-coverage"));
    }

    #[test]
    fn selected_normal_lexical_local_and_read_use_one_ledger() {
        let ast = ASTNode::Program {
            statements: vec![
                ASTNode::Local {
                    variables: vec!["x".to_owned()],
                    initial_values: vec![Some(Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span: Span::unknown(),
                    }))],
                    declared_type_names: vec![None],
                    span: Span::unknown(),
                },
                ASTNode::Local {
                    variables: vec!["y".to_owned()],
                    initial_values: vec![Some(Box::new(ASTNode::Variable {
                        name: "x".to_owned(),
                        span: Span::unknown(),
                    }))],
                    declared_type_names: vec![None],
                    span: Span::unknown(),
                },
                ASTNode::Variable {
                    name: "y".to_owned(),
                    span: Span::unknown(),
                },
                ASTNode::UnaryOp {
                    operator: UnaryOperator::Minus,
                    operand: Box::new(ASTNode::Variable {
                        name: "y".to_owned(),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                },
            ],
            span: Span::unknown(),
        };
        let mut legacy_compiler = MirCompiler::with_options(false);
        let legacy = legacy_compiler
            .compile_with_source(ast.clone(), Some("lexical-local.hako"))
            .expect("legacy lexical local/read compiles");
        let request = NormalCompileRequestV1::for_mir_mode(
            ast,
            Some("lexical-local.hako"),
            std::collections::HashMap::new(),
        )
        .expect("Program request");
        let result = MirCompiler::with_options(false)
            .compile_normal(request)
            .expect("lexical local/read should be Complete");
        assert_eq!(
            MirPrinter::new().print_module(&result.module),
            MirPrinter::new().print_module(&legacy.module)
        );
        assert_eq!(result.verification_result, legacy.verification_result);
        assert!(result.verification_result.is_ok());
    }

    #[test]
    fn real_local_fixture_reaches_the_lexical_complete_boundary() {
        let source =
            include_str!("../../../tools/checks/fixtures/raw_vm_reference_conformance/local.hako");
        let ast = NyashParser::parse_from_string(source).expect("local fixture parses");
        let request = NormalCompileRequestV1::for_mir_mode(
            ast,
            Some("local.hako"),
            std::collections::HashMap::new(),
        )
        .expect("Program request");
        let result = MirCompiler::with_options(false)
            .compile_normal(request)
            .expect("local fixture should remain Complete");
        assert!(result.verification_result.is_ok());
    }

    #[test]
    fn selected_normal_print_lexical_closure_matches_legacy() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let source = r#"
local x = 1
print(-x)
"#;
        let legacy_ast = NyashParser::parse_from_string(source).expect("legacy Print source");
        let normal_ast = NyashParser::parse_from_string(source).expect("normal Print source");
        let mut legacy_compiler = MirCompiler::with_options(false);
        let legacy = legacy_compiler
            .compile_with_source(legacy_ast, Some("script-print-lexical.hako"))
            .expect("legacy Print module");
        let mut normal_compiler = MirCompiler::with_options(false);
        let request = NormalCompileRequestV1::for_mir_mode(
            normal_ast,
            Some("script-print-lexical.hako"),
            std::collections::HashMap::new(),
        )
        .expect("normal Print request");
        let normal = normal_compiler
            .compile_normal(request)
            .expect("normal Print module");
        assert_eq!(
            MirPrinter::new().print_module(&normal.module),
            MirPrinter::new().print_module(&legacy.module)
        );
        assert_eq!(normal.verification_result, legacy.verification_result);
    }

    #[test]
    fn real_print_fixture_uses_the_selected_normal_request() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let source =
            include_str!("../../../tools/checks/fixtures/raw_vm_reference_conformance/print.hako");
        let legacy_ast = NyashParser::parse_from_string(source).expect("legacy print fixture");
        let normal_ast = NyashParser::parse_from_string(source).expect("normal print fixture");
        let mut legacy_compiler = MirCompiler::with_options(false);
        let legacy = legacy_compiler
            .compile_with_source(legacy_ast, Some("print.hako"))
            .expect("legacy print fixture compile");
        let mut normal_compiler = MirCompiler::with_options(false);
        let request = NormalCompileRequestV1::for_mir_mode(
            normal_ast,
            Some("print.hako"),
            std::collections::HashMap::new(),
        )
        .expect("normal print fixture request");
        let normal = normal_compiler
            .compile_normal(request)
            .expect("normal print fixture compile");
        assert_eq!(
            MirPrinter::new().print_module(&normal.module),
            MirPrinter::new().print_module(&legacy.module)
        );
        assert_eq!(normal.verification_result, legacy.verification_result);
    }

    #[test]
    fn selected_normal_binary_lexical_closure_matches_legacy() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let source = r#"
local x = 1
print((x * 2) + 3)
"#;
        let legacy_ast = NyashParser::parse_from_string(source).expect("legacy Binary source");
        let normal_ast = NyashParser::parse_from_string(source).expect("normal Binary source");
        let mut legacy_compiler = MirCompiler::with_options(false);
        let legacy = legacy_compiler
            .compile_with_source(legacy_ast, Some("script-binary-lexical.hako"))
            .expect("legacy Binary module");
        let mut normal_compiler = MirCompiler::with_options(false);
        let request = NormalCompileRequestV1::for_mir_mode(
            normal_ast,
            Some("script-binary-lexical.hako"),
            std::collections::HashMap::new(),
        )
        .expect("normal Binary request");
        let normal = normal_compiler
            .compile_normal(request)
            .expect("normal Binary module");
        assert_eq!(
            MirPrinter::new().print_module(&normal.module),
            MirPrinter::new().print_module(&legacy.module)
        );
        assert_eq!(normal.verification_result, legacy.verification_result);
    }

    #[test]
    fn selected_normal_await_lexical_closure_matches_legacy() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let source = r#"
local f = 1
print(await -(f + 2))
"#;
        let legacy_ast = NyashParser::parse_from_string(source).expect("legacy Await source");
        let normal_ast = NyashParser::parse_from_string(source).expect("normal Await source");
        let mut legacy_compiler = MirCompiler::with_options(false);
        let legacy = legacy_compiler
            .compile_with_source(legacy_ast, Some("script-await-lexical.hako"))
            .expect("legacy Await module");
        let mut normal_compiler = MirCompiler::with_options(false);
        let request = NormalCompileRequestV1::for_mir_mode(
            normal_ast,
            Some("script-await-lexical.hako"),
            std::collections::HashMap::new(),
        )
        .expect("normal Await request");
        let normal = normal_compiler
            .compile_normal(request)
            .expect("normal Await module");
        assert_eq!(
            MirPrinter::new().print_module(&normal.module),
            MirPrinter::new().print_module(&legacy.module)
        );
        assert_eq!(normal.verification_result, legacy.verification_result);
    }

    #[test]
    fn selected_normal_check_lexical_closure_matches_legacy() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let source = r#"
local observed = 2
local ok = check "lexical" {
    "first": observed == 2
    "second": observed == 99
}
print(ok)
"#;
        let legacy_ast = NyashParser::parse_from_string(source).expect("legacy Check source");
        let normal_ast = NyashParser::parse_from_string(source).expect("normal Check source");
        let mut legacy_compiler = MirCompiler::with_options(false);
        let legacy = legacy_compiler
            .compile_with_source(legacy_ast, Some("script-check-lexical.hako"))
            .expect("legacy Check module");
        let mut normal_compiler = MirCompiler::with_options(false);
        let request = NormalCompileRequestV1::for_mir_mode(
            normal_ast,
            Some("script-check-lexical.hako"),
            std::collections::HashMap::new(),
        )
        .expect("normal Check request");
        let normal = normal_compiler
            .compile_normal(request)
            .expect("normal Check module");
        assert_eq!(
            MirPrinter::new().print_module(&normal.module),
            MirPrinter::new().print_module(&legacy.module)
        );
        assert_eq!(normal.verification_result, legacy.verification_result);
    }

    #[test]
    fn selected_normal_and_or_lexical_closure_matches_legacy() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let source = r#"
local lhs = true
local rhs = false
print(lhs and rhs)
print(lhs or rhs)
print((lhs and rhs) or (lhs and rhs))
"#;
        let legacy_ast = NyashParser::parse_from_string(source).expect("legacy AndOr source");
        let normal_ast = NyashParser::parse_from_string(source).expect("normal AndOr source");
        let mut legacy_compiler = MirCompiler::with_options(false);
        let legacy = legacy_compiler
            .compile_with_source(legacy_ast, Some("script-andor-lexical.hako"))
            .expect("legacy AndOr module");
        let mut normal_compiler = MirCompiler::with_options(false);
        let request = NormalCompileRequestV1::for_mir_mode(
            normal_ast,
            Some("script-andor-lexical.hako"),
            std::collections::HashMap::new(),
        )
        .expect("normal AndOr request");
        let normal = normal_compiler
            .compile_normal(request)
            .expect("normal AndOr module");
        assert_eq!(
            MirPrinter::new().print_module(&normal.module),
            MirPrinter::new().print_module(&legacy.module)
        );
        assert_eq!(normal.verification_result, legacy.verification_result);
    }

    #[test]
    fn selected_and_or_failure_discards_candidate_and_reuses_compiler() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let source = r#"
local lhs = true
print(lhs and missing)
"#;
        let ast = NyashParser::parse_from_string(source).expect("failing AndOr source");
        let mut compiler = MirCompiler::with_options(false);
        let error = compiler
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    ast,
                    Some("script-andor-failure.hako"),
                    std::collections::HashMap::new(),
                )
                .expect("failing AndOr request"),
            )
            .expect_err("undefined AndOr RHS must reject");
        assert!(error.contains("Undefined variable: missing"), "{error}");

        compiler
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    NyashParser::parse_from_string("print(1)").expect("fresh source"),
                    Some("script-andor-reuse.hako"),
                    std::collections::HashMap::new(),
                )
                .expect("fresh AndOr reuse request"),
            )
            .expect("fresh request after AndOr failure");
    }
}
