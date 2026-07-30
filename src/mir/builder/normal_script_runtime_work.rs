//! Selected-normal Script runtime descent with cataloged Box-method admission.
//!
//! The Program work plan classifies direct runtime Box statements once.  This
//! adapter retains the original statement slice for the shared block driver,
//! while routing only the selected plain Box terminals through the existing
//! cataloged root-callable port.  Raw/reference and nested descent do not use
//! this owner.

use crate::ast::ASTNode;
use crate::mir::builder::emission::constant::emit_void;
use crate::mir::builder::instance_box_declaration_lifecycle::PreparedInstanceBoxDeclarationLifecycleV1;
use crate::mir::builder::module_lifecycle::RootCallableCapturePortV1;
use crate::mir::builder::raw_expression_dispatch::PreparedRawNonMainStaticBoxLifecycleV1;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_statement_v1, RecursiveChildLoweringPortV1,
};
use crate::mir::builder::stmts::block_driver::{drive_legacy_block_v1, LegacyBlockDescentPortV1};
use crate::mir::{MirBuilder, ValueId};

#[derive(Debug)]
pub(super) struct PreparedNormalScriptRuntimeWorkV1 {
    statements: Box<[ASTNode]>,
    admissions: Box<[NormalScriptRuntimeStatementAdmissionV1]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NormalScriptRuntimeStatementAdmissionV1 {
    RawCompatibility,
    CatalogedNonMainStaticBox,
    InstancePrefixCompatibility,
}

impl PreparedNormalScriptRuntimeWorkV1 {
    pub(super) fn prepare(statements: Vec<ASTNode>) -> Self {
        let admissions = statements.iter().map(classify_runtime_statement).collect();
        Self {
            statements: statements.into_boxed_slice(),
            admissions,
        }
    }

    pub(super) fn len(&self) -> usize {
        self.statements.len()
    }

    pub(super) fn into_raw_statements(self) -> Vec<ASTNode> {
        self.statements.into_vec()
    }

    pub(super) fn lower_with_port_v1<Port>(
        &self,
        builder: &mut MirBuilder,
        port: &mut Port,
    ) -> Result<ValueId, String>
    where
        Port: RootCallableCapturePortV1,
    {
        let mut block_port = NormalScriptRuntimeBlockPortV1 { work: self, port };
        drive_legacy_block_v1(builder, &mut block_port)
    }

    #[cfg(test)]
    fn admission_at(&self, index: usize) -> NormalScriptRuntimeStatementAdmissionV1 {
        self.admissions[index]
    }

    #[cfg(test)]
    pub(super) fn statement_at(&self, index: usize) -> &ASTNode {
        &self.statements[index]
    }
}

struct NormalScriptRuntimeBlockPortV1<'work, 'port, Port> {
    work: &'work PreparedNormalScriptRuntimeWorkV1,
    port: &'port mut Port,
}

impl<Port> LegacyBlockDescentPortV1 for NormalScriptRuntimeBlockPortV1<'_, '_, Port>
where
    Port: RootCallableCapturePortV1,
{
    type SuffixInput<'a>
        = &'a [ASTNode]
    where
        Self: 'a;

    fn len(&self) -> usize {
        self.work.statements.len()
    }

    fn suffix_route_input(&self, index: usize) -> Result<Option<Self::SuffixInput<'_>>, String> {
        Ok(Some(&self.work.statements[index..]))
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        index: usize,
    ) -> Result<ValueId, String> {
        let statement = &self.work.statements[index];
        match self.work.admissions[index] {
            NormalScriptRuntimeStatementAdmissionV1::RawCompatibility => {
                drive_legacy_statement_v1(builder, self.port, statement.clone())
            }
            NormalScriptRuntimeStatementAdmissionV1::CatalogedNonMainStaticBox => {
                lower_cataloged_nonmain_static_box_v1(builder, self.port, statement)
            }
            NormalScriptRuntimeStatementAdmissionV1::InstancePrefixCompatibility => {
                lower_instance_runtime_prefix_v1(builder, self.port, statement)
            }
        }
    }
}

fn lower_cataloged_nonmain_static_box_v1<Port>(
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

fn lower_instance_runtime_prefix_v1<Port>(
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
        fields,
        field_decls,
        constructors,
        init_fields,
        weak_fields,
        is_static: false,
        ..
    } = statement
    else {
        return Err("[freeze:contract][mir/script-runtime/instance-source-drift]".to_owned());
    };
    PreparedInstanceBoxDeclarationLifecycleV1::prepare(
        name,
        methods,
        fields,
        field_decls,
        constructors,
        init_fields,
        weak_fields,
    )
    .lower_runtime_prefix_with_port_v1(builder, port)?;
    emit_void(builder)
}

fn classify_runtime_statement(statement: &ASTNode) -> NormalScriptRuntimeStatementAdmissionV1 {
    match statement {
        ASTNode::BoxDeclaration {
            name,
            is_static: true,
            ..
        } if name != "Main" && is_plain_box(statement) => {
            NormalScriptRuntimeStatementAdmissionV1::CatalogedNonMainStaticBox
        }
        ASTNode::BoxDeclaration {
            is_static: false, ..
        } if is_plain_box(statement) => {
            NormalScriptRuntimeStatementAdmissionV1::InstancePrefixCompatibility
        }
        _ => NormalScriptRuntimeStatementAdmissionV1::RawCompatibility,
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
    use crate::ast::{DeclarationAttrs, Span};
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

    #[test]
    fn classifies_only_plain_direct_runtime_boxes() {
        let mut sync_box = plain_box("Sync", false);
        let ASTNode::BoxDeclaration { is_sync, .. } = &mut sync_box else {
            unreachable!()
        };
        *is_sync = true;

        let work = PreparedNormalScriptRuntimeWorkV1::prepare(vec![
            plain_box("Helpers", true),
            plain_box("Page", false),
            plain_box("Main", true),
            sync_box,
        ]);

        assert_eq!(work.len(), 4);
        assert_eq!(
            work.admission_at(0),
            NormalScriptRuntimeStatementAdmissionV1::CatalogedNonMainStaticBox
        );
        assert_eq!(
            work.admission_at(1),
            NormalScriptRuntimeStatementAdmissionV1::InstancePrefixCompatibility
        );
        assert_eq!(
            work.admission_at(2),
            NormalScriptRuntimeStatementAdmissionV1::RawCompatibility
        );
        assert_eq!(
            work.admission_at(3),
            NormalScriptRuntimeStatementAdmissionV1::RawCompatibility
        );
    }

    #[test]
    fn selected_script_box_methods_match_legacy_without_duplicate_functions() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let source = r#"
box Page {
  capacity: usize = 0
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
                .filter(|name| name.as_str() == "Helpers.value/0")
                .count(),
            1
        );
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
