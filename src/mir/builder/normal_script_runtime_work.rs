//! Selected-normal Script runtime descent: one Program classification, ordered existing terminals.

use super::normal_script_runtime_block_port::NormalScriptRuntimeBlockPortV1;
use super::normal_script_nonbox_statement_disposition::{
    classify_normal_script_nonbox_statement_v1, NormalScriptNonBoxStatementDispositionV1,
};
use crate::ast::ASTNode;
use crate::mir::builder::emission::constant::emit_void;
use crate::mir::builder::instance_box_constructor_batch::PreparedInstanceBoxConstructorBatchV1;
use crate::mir::builder::instance_box_declaration_lifecycle::PreparedInstanceBoxDeclarationLifecycleV1;
use crate::mir::builder::module_lifecycle::RootCallableCapturePortV1;
use crate::mir::builder::normal_instance_constructor_admission::NormalInstanceConstructorSourceBatchV1;
use crate::mir::builder::raw_expression_dispatch::{
    reject_sync_box_lowering_v1, PreparedRawNonMainStaticBoxLifecycleV1,
};
use crate::mir::builder::stmts::block_driver::drive_legacy_block_v1;
use crate::mir::{MirBuilder, ValueId};

#[derive(Debug)]
pub(super) struct PreparedNormalScriptRuntimeWorkV1 {
    statements: Box<[ASTNode]>,
    admissions: Box<[NormalScriptRuntimeStatementAdmissionV1]>,
}
#[derive(Debug)]
pub(super) struct PreparedNormalScriptRuntimeInputV1 {
    statement: ASTNode,
    kind: NormalScriptRuntimeStatementKindV1,
    constructor_sources: Option<NormalInstanceConstructorSourceBatchV1>,
    constructor_batch: Option<PreparedInstanceBoxConstructorBatchV1>,
}
impl PreparedNormalScriptRuntimeInputV1 {
    pub(super) fn preclassified(
        statement: ASTNode,
        kind: NormalScriptRuntimeStatementKindV1,
        constructor_sources: Option<NormalInstanceConstructorSourceBatchV1>,
        constructor_batch: Option<PreparedInstanceBoxConstructorBatchV1>,
    ) -> Self {
        let (constructor_sources, constructor_batch) = match kind {
            NormalScriptRuntimeStatementKindV1::InstancePrefixCompatibility
            | NormalScriptRuntimeStatementKindV1::NonPlainInstanceFullLifecycle => {
                (constructor_sources, constructor_batch)
            }
            _ => {
                debug_assert!(constructor_sources.is_none());
                debug_assert!(constructor_batch.is_none());
                (None, None)
            }
        };
        Self {
            statement,
            kind,
            constructor_sources,
            constructor_batch,
        }
    }
}
#[derive(Debug)]
pub(super) enum NormalScriptRuntimeStatementAdmissionV1 {
    DirectPrint,
    DirectIfStatement,
    DirectFastMemRegion,
    DirectPortAwareExpression,
    DirectStaticConstRuntimeCompletion,
    DirectSelectedUnsupportedStatement,
    RawCompatibility,
    CatalogedNonMainStaticBox,
    StaticMainCompatibility,
    SyncBoxRejection,
    InstancePrefixCompatibility {
        constructor_sources: Option<NormalInstanceConstructorSourceBatchV1>,
        constructor_batch: Option<PreparedInstanceBoxConstructorBatchV1>,
    },
    NonPlainInstanceFullLifecycle {
        constructor_sources: Option<NormalInstanceConstructorSourceBatchV1>,
        constructor_batch: Option<PreparedInstanceBoxConstructorBatchV1>,
    },
}
impl PreparedNormalScriptRuntimeWorkV1 {
    pub(super) fn prepare(inputs: Vec<PreparedNormalScriptRuntimeInputV1>) -> Self {
        use NormalScriptRuntimeStatementAdmissionV1 as Admission;
        use NormalScriptRuntimeStatementKindV1 as Kind;
        let mut statements = Vec::with_capacity(inputs.len());
        let mut admissions = Vec::with_capacity(inputs.len());
        for input in inputs {
            let admission = match input.kind {
                Kind::DirectPrint => Admission::DirectPrint,
                Kind::DirectIfStatement => Admission::DirectIfStatement,
                Kind::DirectFastMemRegion => Admission::DirectFastMemRegion,
                Kind::DirectPortAwareExpression => Admission::DirectPortAwareExpression,
                Kind::DirectStaticConstRuntimeCompletion => {
                    Admission::DirectStaticConstRuntimeCompletion
                }
                Kind::DirectSelectedUnsupportedStatement => {
                    Admission::DirectSelectedUnsupportedStatement
                }
                Kind::RawCompatibility => Admission::RawCompatibility,
                Kind::CatalogedNonMainStaticBox => Admission::CatalogedNonMainStaticBox,
                Kind::StaticMainCompatibility => Admission::StaticMainCompatibility,
                Kind::SyncBoxRejection => Admission::SyncBoxRejection,
                Kind::InstancePrefixCompatibility => Admission::InstancePrefixCompatibility {
                    constructor_sources: input.constructor_sources,
                    constructor_batch: input.constructor_batch,
                },
                Kind::NonPlainInstanceFullLifecycle => Admission::NonPlainInstanceFullLifecycle {
                    constructor_sources: input.constructor_sources,
                    constructor_batch: input.constructor_batch,
                },
            };
            statements.push(input.statement);
            admissions.push(admission);
        }
        Self {
            statements: statements.into_boxed_slice(),
            admissions: admissions.into_boxed_slice(),
        }
    }
    pub(super) fn len(&self) -> usize {
        self.statements.len()
    }
    pub(super) fn into_raw_statements(self) -> Vec<ASTNode> {
        self.statements.into_vec()
    }
    pub(super) fn lower_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        port: &mut Port,
    ) -> Result<ValueId, String>
    where
        Port: RootCallableCapturePortV1,
    {
        let mut block_port =
            NormalScriptRuntimeBlockPortV1::new(self.statements, self.admissions, port);
        drive_legacy_block_v1(builder, &mut block_port)
    }
    #[cfg(test)]
    pub(super) fn admission_at(&self, index: usize) -> &NormalScriptRuntimeStatementAdmissionV1 {
        &self.admissions[index]
    }
    #[cfg(test)]
    pub(super) fn statement_at(&self, index: usize) -> &ASTNode {
        &self.statements[index]
    }
    #[cfg(test)]
    pub(super) fn constructor_admission_at(
        &self,
        index: usize,
    ) -> Option<(
        &NormalInstanceConstructorSourceBatchV1,
        &PreparedInstanceBoxConstructorBatchV1,
    )> {
        match &self.admissions[index] {
            NormalScriptRuntimeStatementAdmissionV1::InstancePrefixCompatibility {
                constructor_sources: Some(sources),
                constructor_batch: Some(batch),
            }
            | NormalScriptRuntimeStatementAdmissionV1::NonPlainInstanceFullLifecycle {
                constructor_sources: Some(sources),
                constructor_batch: Some(batch),
            } => Some((sources, batch)),
            _ => None,
        }
    }
}

pub(super) fn lower_cataloged_nonmain_static_box_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    statement: &ASTNode,
) -> Result<ValueId, String>
where
    Port: RootCallableCapturePortV1,
{
    let ASTNode::BoxDeclaration {
        name,
        methods,
        is_static: true,
        ..
    } = statement
    else {
        return Err("[freeze:contract][mir/script-runtime/static-source-drift]".to_owned());
    };
    PreparedRawNonMainStaticBoxLifecycleV1::prepare(name.clone(), methods.clone())
        .lower_normal_with_port_v1(builder, port)
}

pub(super) fn lower_static_main_compatibility_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    statement: &ASTNode,
) -> Result<ValueId, String>
where
    Port: RootCallableCapturePortV1,
{
    let ASTNode::BoxDeclaration {
        name,
        methods,
        is_static: true,
        ..
    } = statement
    else {
        return Err("[freeze:contract][mir/script-runtime/main-source-drift]".to_owned());
    };
    if name != "Main" {
        return Err("[freeze:contract][mir/script-runtime/main-name-drift]".to_owned());
    }
    port.lower_static_main_box(builder, name.clone(), methods.clone())
}

pub(super) fn reject_sync_box_at_runtime_v1(statement: &ASTNode) -> Result<ValueId, String> {
    let ASTNode::BoxDeclaration {
        name,
        is_sync: true,
        ..
    } = statement
    else {
        return Err("[freeze:contract][mir/script-runtime/sync-source-drift]".to_owned());
    };
    Err(reject_sync_box_lowering_v1(name))
}

pub(super) fn lower_instance_runtime_prefix_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    statement: &ASTNode,
    constructor_sources: Option<&NormalInstanceConstructorSourceBatchV1>,
    constructor_batch: Option<&PreparedInstanceBoxConstructorBatchV1>,
) -> Result<ValueId, String>
where
    Port: RootCallableCapturePortV1,
{
    let ASTNode::BoxDeclaration {
        name,
        methods,
        fields,
        field_decls,
        constructors: _,
        init_fields,
        weak_fields,
        is_static: false,
        ..
    } = statement
    else {
        return Err("[freeze:contract][mir/script-runtime/instance-source-drift]".to_owned());
    };
    let constructor_sources = constructor_sources.ok_or_else(|| {
        "[freeze:contract][mir/script-runtime/instance-constructor-source]".to_owned()
    })?;
    let constructor_batch = constructor_batch.ok_or_else(|| {
        "[freeze:contract][mir/script-runtime/instance-constructor-batch]".to_owned()
    })?;
    PreparedInstanceBoxDeclarationLifecycleV1::prepare_with_constructor_batch_v1(
        name,
        methods,
        fields,
        field_decls,
        init_fields,
        weak_fields,
        constructor_batch.clone(),
    )
    .lower_normal_runtime_prefix_with_port_v1(builder, port, constructor_sources)?;
    emit_void(builder)
}

pub(super) fn lower_nonplain_instance_runtime_lifecycle_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    statement: &ASTNode,
    constructor_sources: Option<&NormalInstanceConstructorSourceBatchV1>,
    constructor_batch: Option<&PreparedInstanceBoxConstructorBatchV1>,
) -> Result<ValueId, String>
where
    Port: RootCallableCapturePortV1,
{
    let ASTNode::BoxDeclaration {
        name,
        methods,
        fields,
        field_decls,
        constructors: _,
        init_fields,
        weak_fields,
        is_static: false,
        ..
    } = statement
    else {
        return Err(
            "[freeze:contract][mir/script-runtime/nonplain-instance-source-drift]".to_owned(),
        );
    };
    let constructor_sources = constructor_sources.ok_or_else(|| {
        "[freeze:contract][mir/script-runtime/nonplain-instance-constructor-source]".to_owned()
    })?;
    let constructor_batch = constructor_batch.ok_or_else(|| {
        "[freeze:contract][mir/script-runtime/nonplain-instance-constructor-batch]".to_owned()
    })?;
    PreparedInstanceBoxDeclarationLifecycleV1::prepare_with_constructor_batch_v1(
        name,
        methods,
        fields,
        field_decls,
        init_fields,
        weak_fields,
        constructor_batch.clone(),
    )
    .lower_normal_root_with_port_v1(builder, port, constructor_sources)?;
    emit_void(builder)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum NormalScriptRuntimeStatementKindV1 {
    DirectPrint,
    DirectIfStatement,
    DirectFastMemRegion,
    DirectPortAwareExpression,
    DirectStaticConstRuntimeCompletion,
    DirectSelectedUnsupportedStatement,
    RawCompatibility,
    CatalogedNonMainStaticBox,
    StaticMainCompatibility,
    SyncBoxRejection,
    InstancePrefixCompatibility,
    NonPlainInstanceFullLifecycle,
}

pub(super) fn classify_normal_script_runtime_statement_v1(
    statement: &ASTNode,
) -> NormalScriptRuntimeStatementKindV1 {
    match statement {
        ASTNode::BoxDeclaration { is_sync: true, .. } => {
            NormalScriptRuntimeStatementKindV1::SyncBoxRejection
        }
        ASTNode::BoxDeclaration {
            name,
            is_static: true,
            ..
        } if name == "Main" => NormalScriptRuntimeStatementKindV1::StaticMainCompatibility,
        ASTNode::BoxDeclaration {
            name,
            is_static: true,
            ..
        } if name != "Main" => NormalScriptRuntimeStatementKindV1::CatalogedNonMainStaticBox,
        ASTNode::BoxDeclaration {
            is_static: false, ..
        } if is_plain_box(statement) => {
            NormalScriptRuntimeStatementKindV1::InstancePrefixCompatibility
        }
        ASTNode::BoxDeclaration {
            is_static: false, ..
        } => NormalScriptRuntimeStatementKindV1::NonPlainInstanceFullLifecycle,
        _ => match classify_normal_script_nonbox_statement_v1(statement) {
            NormalScriptNonBoxStatementDispositionV1::DirectPrint => {
                NormalScriptRuntimeStatementKindV1::DirectPrint
            }
            NormalScriptNonBoxStatementDispositionV1::DirectIfStatement => {
                NormalScriptRuntimeStatementKindV1::DirectIfStatement
            }
            NormalScriptNonBoxStatementDispositionV1::DirectFastMemRegion => {
                NormalScriptRuntimeStatementKindV1::DirectFastMemRegion
            }
            NormalScriptNonBoxStatementDispositionV1::DirectPortAwareExpression => {
                NormalScriptRuntimeStatementKindV1::DirectPortAwareExpression
            }
            NormalScriptNonBoxStatementDispositionV1::DirectStaticConstRuntimeCompletion => {
                NormalScriptRuntimeStatementKindV1::DirectStaticConstRuntimeCompletion
            }
            NormalScriptNonBoxStatementDispositionV1::DirectSelectedUnsupportedStatement => {
                NormalScriptRuntimeStatementKindV1::DirectSelectedUnsupportedStatement
            }
            NormalScriptNonBoxStatementDispositionV1::TopLevelFunctionImmediateOnly => {
                NormalScriptRuntimeStatementKindV1::RawCompatibility
            }
            NormalScriptNonBoxStatementDispositionV1::DirectBoxOwnedElsewhere => {
                unreachable!("direct Box statements are classified before non-Box disposition")
            }
        },
    }
}

fn is_plain_box(statement: &ASTNode) -> bool {
    let ASTNode::BoxDeclaration {
        delegates,
        invariants,
        transitions,
        is_interface,
        is_record,
        extends,
        implements,
        type_parameters,
        is_sync,
        ..
    } = statement
    else {
        return false;
    };
    delegates.is_empty()
        && invariants.is_empty()
        && transitions.is_empty()
        && !is_interface
        && !is_record
        && extends.is_empty()
        && implements.is_empty()
        && type_parameters.is_empty()
        && !is_sync
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::ast::{DeclarationAttrs, LiteralValue, Span};
    use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};
    use crate::parser::NyashParser;

    fn plain_box(name: &str, is_static: bool) -> ASTNode {
        ASTNode::BoxDeclaration {
            name: name.to_owned(),
            fields: Vec::new(),
            field_decls: Vec::new(),
            public_fields: Vec::new(),
            private_fields: Vec::new(),
            methods: HashMap::new(),
            constructors: HashMap::new(),
            init_fields: Vec::new(),
            weak_fields: Vec::new(),
            delegates: Vec::new(),
            invariants: Vec::new(),
            transitions: Vec::new(),
            is_interface: false,
            is_record: false,
            extends: Vec::new(),
            implements: Vec::new(),
            type_parameters: Vec::new(),
            is_sync: false,
            is_static,
            static_init: None,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn selected_script_executes_three_located_direct_expression_bodies() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let statements = vec![
            ASTNode::BlockExpr {
                prelude_stmts: vec![ASTNode::If {
                    condition: Box::new(integer(1)),
                    then_body: vec![integer(2)],
                    else_body: Some(vec![integer(3)]),
                    span: Span::unknown(),
                }],
                tail_expr: Box::new(integer(4)),
                span: Span::unknown(),
            },
            ASTNode::TaskScope {
                body: vec![ASTNode::FastMemRegion {
                    contract: "PageMapV0".to_owned(),
                    body: vec![integer(5)],
                    span: Span::unknown(),
                }],
                source_keyword: "co".to_owned(),
                span: Span::unknown(),
            },
            ASTNode::ScopeBox {
                body: vec![ASTNode::BlockExpr {
                    prelude_stmts: vec![integer(6)],
                    tail_expr: Box::new(integer(7)),
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            },
        ];
        let program = ASTNode::Program {
            statements,
            span: Span::unknown(),
        };
        let mut legacy = MirCompiler::with_options(false);
        let legacy = legacy
            .compile_with_source(program.clone(), Some("structured-script.hako"))
            .expect("legacy structured Script");
        let mut normal = MirCompiler::with_options(false);
        let normal = normal
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    program,
                    Some("structured-script.hako"),
                    HashMap::new(),
                )
                .expect("normal request"),
            )
            .expect("selected normal structured Script");
        assert_eq!(
            MirPrinter::new().print_module(&normal.module),
            MirPrinter::new().print_module(&legacy.module)
        );
    }

    #[test]
    fn classifies_only_plain_direct_runtime_boxes() {
        let mut sync_box = plain_box("Sync", false);
        let ASTNode::BoxDeclaration { is_sync, .. } = &mut sync_box else {
            unreachable!()
        };
        *is_sync = true;
        let mut record_box = plain_box("RecordPage", false);
        let ASTNode::BoxDeclaration { is_record, .. } = &mut record_box else {
            unreachable!()
        };
        *is_record = true;

        let work = PreparedNormalScriptRuntimeWorkV1::prepare(vec![
            PreparedNormalScriptRuntimeInputV1::preclassified(
                plain_box("Helpers", true),
                classify_normal_script_runtime_statement_v1(&plain_box("Helpers", true)),
                None,
                None,
            ),
            PreparedNormalScriptRuntimeInputV1::preclassified(
                plain_box("Page", false),
                classify_normal_script_runtime_statement_v1(&plain_box("Page", false)),
                None,
                None,
            ),
            PreparedNormalScriptRuntimeInputV1::preclassified(
                plain_box("Main", true),
                classify_normal_script_runtime_statement_v1(&plain_box("Main", true)),
                None,
                None,
            ),
            PreparedNormalScriptRuntimeInputV1::preclassified(
                sync_box.clone(),
                classify_normal_script_runtime_statement_v1(&sync_box),
                None,
                None,
            ),
            PreparedNormalScriptRuntimeInputV1::preclassified(
                record_box.clone(),
                classify_normal_script_runtime_statement_v1(&record_box),
                None,
                None,
            ),
        ]);

        assert_eq!(work.len(), 5);
        assert!(matches!(
            work.admission_at(0),
            NormalScriptRuntimeStatementAdmissionV1::CatalogedNonMainStaticBox
        ));
        assert!(matches!(
            work.admission_at(1),
            NormalScriptRuntimeStatementAdmissionV1::InstancePrefixCompatibility { .. }
        ));
        assert!(matches!(
            work.admission_at(2),
            NormalScriptRuntimeStatementAdmissionV1::StaticMainCompatibility
        ));
        assert!(matches!(
            work.admission_at(3),
            NormalScriptRuntimeStatementAdmissionV1::SyncBoxRejection
        ));
        assert!(matches!(
            work.admission_at(4),
            NormalScriptRuntimeStatementAdmissionV1::NonPlainInstanceFullLifecycle { .. }
        ));
    }

    #[test]
    fn selected_script_box_methods_match_legacy_without_duplicate_functions() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let source = r#"
box Page {
  capacity: usize = 0
  birth() { return 6 }
  answer() { return 7 }
}
static box Helpers { value() { return 8 } }
print(1)
"#;
        let legacy_ast = NyashParser::parse_from_string(source).expect("legacy Script source");
        let normal_ast = NyashParser::parse_from_string(source).expect("normal Script source");
        let mut legacy_compiler = MirCompiler::with_options(false);
        let legacy = legacy_compiler
            .compile_with_source(legacy_ast, Some("script-box-parity.hako"))
            .expect("legacy Script module");
        let mut normal_compiler = MirCompiler::with_options(false);
        let request = NormalCompileRequestV1::for_mir_mode(
            normal_ast,
            Some("script-box-parity.hako"),
            HashMap::new(),
        )
        .expect("normal Script request");
        let normal = normal_compiler
            .compile_normal(request)
            .expect("normal Script module");

        assert_eq!(
            MirPrinter::new().print_module(&normal.module),
            MirPrinter::new().print_module(&legacy.module)
        );
        assert_eq!(
            normal.module.function_names(),
            legacy.module.function_names()
        );
        assert_eq!(
            normal
                .module
                .function_names()
                .iter()
                .filter(|name| name.as_str() == "Page.answer/0")
                .count(),
            1
        );
        assert_eq!(
            normal
                .module
                .function_names()
                .iter()
                .filter(|name| name.as_str() == "Page.birth/0")
                .count(),
            1
        );
        assert_eq!(
            normal
                .module
                .function_names()
                .iter()
                .filter(|name| name.as_str() == "Helpers.value/0")
                .count(),
            1
        );
    }

    #[test]
    fn selected_generic_script_box_keeps_full_legacy_callable_parity() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let source = "box Page<T> { birth() { return 6 } answer() { return 7 } } print(1)";
        let legacy_ast = NyashParser::parse_from_string(source).expect("legacy Script source");
        let normal_ast = NyashParser::parse_from_string(source).expect("normal Script source");
        let mut legacy_compiler = MirCompiler::with_options(false);
        let legacy = legacy_compiler
            .compile_with_source(legacy_ast, Some("nonplain-script-box-parity.hako"))
            .expect("legacy nonplain Script module");
        let mut normal_compiler = MirCompiler::with_options(false);
        let normal = normal_compiler
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    normal_ast,
                    Some("nonplain-script-box-parity.hako"),
                    HashMap::new(),
                )
                .expect("normal Script request"),
            )
            .expect("normal nonplain Script module");

        assert_eq!(
            MirPrinter::new().print_module(&normal.module),
            MirPrinter::new().print_module(&legacy.module)
        );
        assert_eq!(
            normal.module.function_names(),
            legacy.module.function_names()
        );
    }

    #[test]
    fn selected_generic_static_script_box_keeps_legacy_callable_parity() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let source = "static box Helpers<T> { value() { return 8 } } print(1)";
        let legacy_ast = NyashParser::parse_from_string(source).expect("legacy Script source");
        let normal_ast = NyashParser::parse_from_string(source).expect("normal Script source");
        let mut legacy_compiler = MirCompiler::with_options(false);
        let legacy = legacy_compiler
            .compile_with_source(legacy_ast, Some("generic-static-script-box.hako"))
            .expect("legacy generic static Script module");
        let mut normal_compiler = MirCompiler::with_options(false);
        let normal = normal_compiler
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    normal_ast,
                    Some("generic-static-script-box.hako"),
                    HashMap::new(),
                )
                .expect("normal Script request"),
            )
            .expect("normal generic static Script module");
        assert_eq!(
            MirPrinter::new().print_module(&normal.module),
            MirPrinter::new().print_module(&legacy.module)
        );
        assert_eq!(
            normal.module.function_names(),
            legacy.module.function_names()
        );
    }

    #[test]
    fn selected_sync_script_box_keeps_the_runtime_rejection_and_reuses_compiler() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let source = "sync box Sync { } print(1)";
        let legacy_ast = NyashParser::parse_from_string(source).expect("legacy sync Script source");
        let normal_ast = NyashParser::parse_from_string(source).expect("normal sync Script source");
        let mut legacy_compiler = MirCompiler::with_options(false);
        let legacy_error = legacy_compiler
            .compile_with_source(legacy_ast, Some("sync-script-box.hako"))
            .expect_err("legacy sync Script must reject");
        let mut normal_compiler = MirCompiler::with_options(false);
        let normal_error = normal_compiler
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    normal_ast,
                    Some("sync-script-box.hako"),
                    HashMap::new(),
                )
                .expect("normal sync Script request"),
            )
            .expect_err("normal sync Script must reject");
        assert_eq!(normal_error, legacy_error);
        assert!(
            normal_error.contains("sync_box_lowering_missing"),
            "{normal_error}"
        );
        normal_compiler
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    NyashParser::parse_from_string("print(1)").expect("fresh Script source"),
                    Some("sync-script-reuse.hako"),
                    HashMap::new(),
                )
                .expect("fresh Script request"),
            )
            .expect("fresh Script candidate after sync rejection");
    }

    #[test]
    fn selected_script_catalog_failure_discards_candidate_and_reuses_compiler() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let failing = NyashParser::parse_from_string(
            "static box Helpers { value() { return missing } } print(1)",
        )
        .expect("failing Script source");
        let corrected =
            NyashParser::parse_from_string("static box Helpers { value() { return 1 } } print(1)")
                .expect("corrected Script source");
        let mut compiler = MirCompiler::with_options(false);
        let error = compiler
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    failing,
                    Some("script-failure.hako"),
                    HashMap::new(),
                )
                .expect("normal Script request"),
            )
            .expect_err("missing Script static method value must reject the candidate");
        assert!(error.contains("Undefined variable: missing"), "{error}");
        let result = compiler
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    corrected,
                    Some("script-reused.hako"),
                    HashMap::new(),
                )
                .expect("corrected normal Script request"),
            )
            .expect("fresh corrected Script candidate");
        assert!(result.module.functions.contains_key("Helpers.value/0"));
    }
}
