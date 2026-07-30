//! Source-only work partition for the selected Program root.
//!
//! It consumes the normal root's once-cloned statement vector exactly once,
//! preserving source order while keeping all Builder effects in the existing
//! instance/static/Main/body lifecycle owners.

use std::collections::HashMap;

use crate::ast::{ASTNode, DeclarationAttrs, FieldDecl, ParamDecl};

use super::instance_box_declaration_lifecycle::PreparedInstanceBoxDeclarationLifecycleV1;
use super::module_lifecycle::RootCallableCapturePortV1;
use super::normal_script_runtime_work::PreparedNormalScriptRuntimeWorkV1;
use super::MirBuilder;

#[derive(Debug)]
pub(super) struct PreparedProgramRootWorkPlanV1 {
    immediate: Box<[PreparedProgramRootImmediateWorkV1]>,
    deferred_static: Box<[PreparedProgramDeferredStaticBoxWorkV1]>,
    runtime: PreparedProgramRootRuntimeWorkV1,
    terminal: ProgramRootTerminalScheduleV1,
    _seal: PreparedProgramRootWorkPlanSealV1,
}

#[derive(Debug)]
struct PreparedProgramRootWorkPlanSealV1;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ProgramRootTerminalScheduleV1 {
    ScriptRuntime,
    VerifiedAppMain,
}

#[derive(Debug)]
pub(super) struct PreparedProgramRootWorkPlanPartsV1 {
    pub(super) immediate: Box<[PreparedProgramRootImmediateWorkV1]>,
    pub(super) deferred_static: Box<[PreparedProgramDeferredStaticBoxWorkV1]>,
    pub(super) runtime: PreparedProgramRootRuntimeWorkV1,
    pub(super) terminal: ProgramRootTerminalScheduleV1,
}

#[derive(Debug)]
pub(super) enum PreparedProgramRootImmediateWorkV1 {
    InstanceBox(PreparedProgramRootInstanceBoxWorkV1),
    TopLevelFunction(PreparedProgramRootTopLevelFunctionWorkV1),
}

#[derive(Debug)]
pub(super) struct PreparedProgramRootInstanceBoxWorkV1 {
    name: String,
    methods: HashMap<String, ASTNode>,
    fields: Vec<String>,
    field_decls: Vec<FieldDecl>,
    constructors: HashMap<String, ASTNode>,
    init_fields: Vec<String>,
    weak_fields: Vec<String>,
}

#[derive(Debug)]
pub(super) struct PreparedProgramRootTopLevelFunctionWorkV1 {
    name: String,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
    body: Vec<ASTNode>,
    uses: Vec<String>,
    attrs: DeclarationAttrs,
}

#[derive(Debug)]
pub(super) struct PreparedProgramDeferredStaticBoxWorkV1 {
    name: String,
    methods: HashMap<String, ASTNode>,
}

/// Selects whether the Program work plan owns the normal-only Script runtime
/// admission receipt, or preserves the shared raw/reference statement slice.
///
/// This is an invocation route choice, not a statement-family classifier.  It
/// must be known before the work plan is prepared so raw/reference callers
/// never construct the selected-normal receipt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ProgramRootRuntimeAdmissionV1 {
    RawCompatibility,
    SelectedNormal,
}

#[derive(Debug)]
pub(super) enum PreparedProgramRootRuntimeWorkV1 {
    RawCompatibility(Box<[ASTNode]>),
    SelectedNormal(PreparedNormalScriptRuntimeWorkV1),
}

impl PreparedProgramRootRuntimeWorkV1 {
    fn prepare(statements: Vec<ASTNode>, admission: ProgramRootRuntimeAdmissionV1) -> Self {
        match admission {
            ProgramRootRuntimeAdmissionV1::RawCompatibility => {
                Self::RawCompatibility(statements.into_boxed_slice())
            }
            ProgramRootRuntimeAdmissionV1::SelectedNormal => {
                Self::SelectedNormal(PreparedNormalScriptRuntimeWorkV1::prepare(statements))
            }
        }
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        match self {
            Self::RawCompatibility(statements) => statements.len(),
            Self::SelectedNormal(work) => work.len(),
        }
    }

    #[cfg(test)]
    fn statement_at(&self, index: usize) -> &ASTNode {
        match self {
            Self::RawCompatibility(statements) => &statements[index],
            Self::SelectedNormal(work) => work.statement_at(index),
        }
    }
}

impl PreparedProgramDeferredStaticBoxWorkV1 {
    pub(super) fn into_parts(self) -> (String, HashMap<String, ASTNode>) {
        (self.name, self.methods)
    }
}

enum ProgramRootStatementDispositionV1 {
    ImmediateAndRuntime {
        work: PreparedProgramRootImmediateWorkV1,
        runtime_statement: ASTNode,
    },
    ImmediateOnly(PreparedProgramRootImmediateWorkV1),
    DeferredAndRuntime {
        work: PreparedProgramDeferredStaticBoxWorkV1,
        runtime_statement: ASTNode,
    },
    RuntimeOnly(ASTNode),
}

impl PreparedProgramRootWorkPlanV1 {
    pub(super) fn prepare(
        statements: Vec<ASTNode>,
        is_app_mode: bool,
        runtime_admission: ProgramRootRuntimeAdmissionV1,
    ) -> Self {
        let mut immediate = Vec::new();
        let mut deferred_static = Vec::new();
        let mut runtime_statements = Vec::new();

        for statement in statements {
            match classify_statement(statement, is_app_mode) {
                ProgramRootStatementDispositionV1::ImmediateAndRuntime {
                    work,
                    runtime_statement,
                } => {
                    immediate.push(work);
                    runtime_statements.push(runtime_statement);
                }
                ProgramRootStatementDispositionV1::ImmediateOnly(work) => {
                    immediate.push(work);
                }
                ProgramRootStatementDispositionV1::DeferredAndRuntime {
                    work,
                    runtime_statement,
                } => {
                    runtime_statements.push(runtime_statement);
                    deferred_static.push(work);
                }
                ProgramRootStatementDispositionV1::RuntimeOnly(statement) => {
                    runtime_statements.push(statement)
                }
            }
        }

        Self {
            immediate: immediate.into_boxed_slice(),
            deferred_static: deferred_static.into_boxed_slice(),
            runtime: PreparedProgramRootRuntimeWorkV1::prepare(
                runtime_statements,
                runtime_admission,
            ),
            terminal: if is_app_mode {
                ProgramRootTerminalScheduleV1::VerifiedAppMain
            } else {
                ProgramRootTerminalScheduleV1::ScriptRuntime
            },
            _seal: PreparedProgramRootWorkPlanSealV1,
        }
    }

    pub(super) fn into_parts(self) -> PreparedProgramRootWorkPlanPartsV1 {
        PreparedProgramRootWorkPlanPartsV1 {
            immediate: self.immediate,
            deferred_static: self.deferred_static,
            runtime: self.runtime,
            terminal: self.terminal,
        }
    }
}

impl PreparedProgramRootImmediateWorkV1 {
    pub(super) fn lower_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        callables: &mut Port,
    ) -> Result<(), String>
    where
        Port: RootCallableCapturePortV1,
    {
        match self {
            Self::InstanceBox(work) => work.lower_with_port_v1(builder, callables),
            Self::TopLevelFunction(work) => work.lower_with_port_v1(builder, callables),
        }
    }
}

impl PreparedProgramRootInstanceBoxWorkV1 {
    fn lower_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        callables: &mut Port,
    ) -> Result<(), String>
    where
        Port: RootCallableCapturePortV1,
    {
        PreparedInstanceBoxDeclarationLifecycleV1::prepare(
            &self.name,
            &self.methods,
            &self.fields,
            &self.field_decls,
            &self.constructors,
            &self.init_fields,
            &self.weak_fields,
        )
        .lower_root_with_port_v1(builder, callables)
    }
}

impl PreparedProgramRootTopLevelFunctionWorkV1 {
    fn lower_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        callables: &mut Port,
    ) -> Result<(), String>
    where
        Port: RootCallableCapturePortV1,
    {
        callables.lower_static_box_method(
            builder,
            format!("{}/{}", self.name, self.params.len()),
            self.params,
            self.param_decls,
            self.return_type_name,
            self.body,
            self.uses,
            self.attrs,
        )
    }
}

fn classify_statement(statement: ASTNode, is_app_mode: bool) -> ProgramRootStatementDispositionV1 {
    match &statement {
        ASTNode::BoxDeclaration {
            name,
            methods,
            fields,
            field_decls,
            constructors,
            init_fields,
            weak_fields,
            is_static,
            ..
        } if !is_static => ProgramRootStatementDispositionV1::ImmediateAndRuntime {
            work: PreparedProgramRootImmediateWorkV1::InstanceBox(
                PreparedProgramRootInstanceBoxWorkV1 {
                    name: name.clone(),
                    methods: methods.clone(),
                    fields: fields.clone(),
                    field_decls: field_decls.clone(),
                    constructors: constructors.clone(),
                    init_fields: init_fields.clone(),
                    weak_fields: weak_fields.clone(),
                },
            ),
            runtime_statement: statement,
        },
        ASTNode::BoxDeclaration {
            name,
            methods,
            is_static: true,
            ..
        } if is_app_mode && name != "Main" => {
            ProgramRootStatementDispositionV1::DeferredAndRuntime {
                work: PreparedProgramDeferredStaticBoxWorkV1 {
                    name: name.clone(),
                    methods: methods.clone(),
                },
                runtime_statement: statement,
            }
        }
        ASTNode::FunctionDeclaration {
            name,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
            ..
        } => ProgramRootStatementDispositionV1::ImmediateOnly(
            PreparedProgramRootImmediateWorkV1::TopLevelFunction(
                PreparedProgramRootTopLevelFunctionWorkV1 {
                    name: name.clone(),
                    params: params.clone(),
                    param_decls: param_decls.clone(),
                    return_type_name: return_type_name.clone(),
                    body: body.clone(),
                    uses: uses.clone(),
                    attrs: attrs.clone(),
                },
            ),
        ),
        _ => ProgramRootStatementDispositionV1::RuntimeOnly(statement),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{DeclarationAttrs, LiteralValue, Span};

    fn literal(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn function(name: &str) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: name.to_owned(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: vec![ASTNode::Return {
                value: Some(Box::new(literal(0))),
                span: Span::unknown(),
            }],
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    fn box_declaration(name: &str, is_static: bool) -> ASTNode {
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
    fn app_partition_preserves_source_order_and_runtime_retention() {
        let plan = PreparedProgramRootWorkPlanV1::prepare(
            vec![
                box_declaration("Page", false),
                function("helper"),
                box_declaration("Helpers", true),
                literal(7),
                box_declaration("Main", true),
            ],
            true,
            ProgramRootRuntimeAdmissionV1::SelectedNormal,
        );
        let parts = plan.into_parts();

        assert_eq!(
            parts.terminal,
            ProgramRootTerminalScheduleV1::VerifiedAppMain
        );
        assert_eq!(parts.immediate.len(), 2);
        assert!(matches!(
            &parts.immediate[0],
            PreparedProgramRootImmediateWorkV1::InstanceBox(work) if work.name == "Page"
        ));
        assert!(matches!(
            &parts.immediate[1],
            PreparedProgramRootImmediateWorkV1::TopLevelFunction(work) if work.name == "helper"
        ));
        assert_eq!(parts.deferred_static.len(), 1);
        assert_eq!(parts.deferred_static[0].name, "Helpers");
        assert_eq!(parts.runtime.len(), 4);
        assert!(matches!(
            parts.runtime.statement_at(0),
            ASTNode::BoxDeclaration { name, .. } if name == "Page"
        ));
        assert!(matches!(
            parts.runtime.statement_at(1),
            ASTNode::BoxDeclaration { name, .. } if name == "Helpers"
        ));
        assert!(matches!(
            parts.runtime.statement_at(2),
            ASTNode::Literal {
                value: LiteralValue::Integer(7),
                ..
            }
        ));
        assert!(matches!(
            parts.runtime.statement_at(3),
            ASTNode::BoxDeclaration { name, .. } if name == "Main"
        ));
    }

    #[test]
    fn script_partition_keeps_static_boxes_out_of_deferred_work() {
        let plan = PreparedProgramRootWorkPlanV1::prepare(
            vec![box_declaration("Helpers", true), function("helper")],
            false,
            ProgramRootRuntimeAdmissionV1::SelectedNormal,
        );
        let parts = plan.into_parts();

        assert_eq!(parts.terminal, ProgramRootTerminalScheduleV1::ScriptRuntime);
        assert_eq!(parts.deferred_static.len(), 0);
        assert_eq!(parts.immediate.len(), 1);
        assert_eq!(parts.runtime.len(), 1);
        assert!(matches!(
            parts.runtime.statement_at(0),
            ASTNode::BoxDeclaration { name, .. } if name == "Helpers"
        ));
    }

    #[test]
    fn raw_runtime_keeps_the_neutral_statement_carrier() {
        let plan = PreparedProgramRootWorkPlanV1::prepare(
            vec![box_declaration("Helpers", true), literal(7)],
            false,
            ProgramRootRuntimeAdmissionV1::RawCompatibility,
        );
        let parts = plan.into_parts();

        assert!(matches!(
            parts.runtime,
            PreparedProgramRootRuntimeWorkV1::RawCompatibility(_)
        ));
    }
}
