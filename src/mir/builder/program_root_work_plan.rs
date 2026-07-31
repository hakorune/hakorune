use crate::ast::{ASTNode, DeclarationAttrs, FieldDecl, ParamDecl};
use std::collections::HashMap;
use super::instance_box_constructor_batch::PreparedInstanceBoxConstructorBatchV1;
use super::instance_box_declaration_lifecycle::PreparedInstanceBoxDeclarationLifecycleV1;
use super::module_lifecycle::RootCallableCapturePortV1;
use super::normal_instance_constructor_admission::NormalInstanceConstructorSourceBatchV1;
use super::normal_script_program_item_admission::{
    classify_normal_script_program_item_v1, NormalScriptProgramItemAdmissionV1,
};
#[cfg(test)]
use super::normal_script_runtime_work::NormalScriptRuntimeStatementAdmissionV1;
use super::normal_script_runtime_work::{
    PreparedNormalScriptRuntimeInputV1, PreparedNormalScriptRuntimeWorkV1,
};
use super::normal_script_root_demand_window::ScriptRootDemandWindowBuilderV1;
use crate::mir::resolved_semantics::VerifiedScriptRootDemandWindowV1;
use super::normal_top_level_function_admission::{
    NormalTopLevelFunctionDraftAdmissionV1, NormalTopLevelFunctionSourceKeyV1,
};
use super::MirBuilder;
#[derive(Debug)]
pub(super) struct PreparedProgramRootWorkPlanV1 {
    immediate: Box<[PreparedProgramRootImmediateWorkV1]>,
    deferred_static: Box<[PreparedProgramDeferredStaticBoxWorkV1]>,
    runtime: PreparedProgramRootRuntimeWorkV1,
    terminal: ProgramRootTerminalScheduleV1,
    script_semantic_window: Option<VerifiedScriptRootDemandWindowV1>,
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
    pub(super) script_semantic_window: Option<VerifiedScriptRootDemandWindowV1>,
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
    constructors: PreparedInstanceBoxConstructorBatchV1,
    normal_constructor_sources: Option<NormalInstanceConstructorSourceBatchV1>,
    init_fields: Vec<String>,
    weak_fields: Vec<String>,
}
#[derive(Debug)]
pub(super) enum PreparedProgramRootTopLevelFunctionWorkV1 {
    RawCompatibility(PreparedProgramRootTopLevelFunctionPartsV1),
    SelectedNormal {
        admission: NormalTopLevelFunctionDraftAdmissionV1,
        parts: PreparedProgramRootTopLevelFunctionPartsV1,
    },
}
#[derive(Debug)]
pub(super) struct PreparedProgramRootTopLevelFunctionPartsV1 {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ProgramRootWorkPlanAdmissionV1 {
    RawCompatibility,
    SelectedNormal,
}
#[derive(Debug)]
pub(super) enum PreparedProgramRootRuntimeWorkV1 {
    RawCompatibility(Box<[ASTNode]>),
    SelectedNormal(PreparedNormalScriptRuntimeWorkV1),
}
#[derive(Debug)]
struct PreparedProgramRootRuntimeStatementV1 {
    source_statement_index: usize,
    statement: ASTNode,
    normal_script_kind: Option<NormalScriptProgramItemAdmissionV1>,
    constructor_sources: Option<NormalInstanceConstructorSourceBatchV1>,
    constructor_batch: Option<PreparedInstanceBoxConstructorBatchV1>,
}
impl PreparedProgramRootRuntimeWorkV1 {
    fn prepare(
        statements: Vec<PreparedProgramRootRuntimeStatementV1>,
        admission: ProgramRootWorkPlanAdmissionV1,
    ) -> Self {
        match admission {
            ProgramRootWorkPlanAdmissionV1::RawCompatibility => Self::RawCompatibility(
                statements
                    .into_iter()
                    .map(|statement| statement.statement)
                    .collect(),
            ),
            ProgramRootWorkPlanAdmissionV1::SelectedNormal => {
                Self::SelectedNormal(PreparedNormalScriptRuntimeWorkV1::prepare(
                    statements
                        .into_iter()
                        .map(|statement| {
                            PreparedNormalScriptRuntimeInputV1::preclassified_at(
                                statement.source_statement_index,
                                statement.statement,
                                statement
                                    .normal_script_kind
                                    .expect("selected Script runtime classifier"),
                                statement.constructor_sources,
                                statement.constructor_batch,
                            )
                        })
                        .collect(),
                ))
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
        runtime: PreparedProgramRootRuntimeStatementV1,
    },
    ImmediateOnly(PreparedProgramRootImmediateWorkV1),
    DeferredAndRuntime {
        work: PreparedProgramDeferredStaticBoxWorkV1,
        runtime: PreparedProgramRootRuntimeStatementV1,
    },
    RuntimeOnly(PreparedProgramRootRuntimeStatementV1),
}
impl PreparedProgramRootWorkPlanV1 {
    pub(super) fn prepare(
        statements: Vec<ASTNode>,
        is_app_mode: bool,
        work_plan_admission: ProgramRootWorkPlanAdmissionV1,
    ) -> Self {
        let mut immediate = Vec::new();
        let mut deferred_static = Vec::new();
        let mut runtime_statements = Vec::new();
        let mut script_window = (!is_app_mode
            && work_plan_admission == ProgramRootWorkPlanAdmissionV1::SelectedNormal)
            .then(|| ScriptRootDemandWindowBuilderV1::for_program_statement_count(statements.len()));
        for (statement_index, statement) in statements.into_iter().enumerate() {
            let normal_script_kind = (work_plan_admission
                == ProgramRootWorkPlanAdmissionV1::SelectedNormal)
                .then(|| classify_normal_script_program_item_v1(&statement));
            if let Some(window) = &mut script_window {
                window
                    .record_selected_work_item(
                        statement_index,
                        &statement,
                        normal_script_kind,
                        matches!(statement, ASTNode::FunctionDeclaration { .. }),
                    )
                    .expect("selected Script demand-window source contract");
            }
            let disposition = classify_statement(
                statement,
                is_app_mode,
                statement_index,
                work_plan_admission,
                normal_script_kind,
            );
            match disposition {
                ProgramRootStatementDispositionV1::ImmediateAndRuntime { work, runtime } => {
                    immediate.push(work);
                    runtime_statements.push(runtime);
                }
                ProgramRootStatementDispositionV1::ImmediateOnly(work) => {
                    immediate.push(work);
                }
                ProgramRootStatementDispositionV1::DeferredAndRuntime { work, runtime } => {
                    runtime_statements.push(runtime);
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
                work_plan_admission,
            ),
            terminal: if is_app_mode {
                ProgramRootTerminalScheduleV1::VerifiedAppMain
            } else {
                ProgramRootTerminalScheduleV1::ScriptRuntime
            },
            script_semantic_window: script_window
                .map(|window| window.seal().expect("selected Script demand window")),
            _seal: PreparedProgramRootWorkPlanSealV1,
        }
    }
    pub(super) fn into_parts(self) -> PreparedProgramRootWorkPlanPartsV1 {
        PreparedProgramRootWorkPlanPartsV1 {
            immediate: self.immediate,
            deferred_static: self.deferred_static,
            runtime: self.runtime,
            terminal: self.terminal,
            script_semantic_window: self.script_semantic_window,
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
        let lifecycle =
            PreparedInstanceBoxDeclarationLifecycleV1::prepare_with_constructor_batch_v1(
                &self.name,
                &self.methods,
                &self.fields,
                &self.field_decls,
                &self.init_fields,
                &self.weak_fields,
                self.constructors,
            );
        match self.normal_constructor_sources {
            Some(sources) => lifecycle.lower_normal_root_with_port_v1(builder, callables, &sources),
            None => lifecycle.lower_root_with_port_v1(builder, callables),
        }
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
        match self {
            Self::RawCompatibility(parts) => parts.lower_raw_with_port_v1(builder, callables),
            Self::SelectedNormal { admission, parts } => {
                parts.lower_normal_with_port_v1(builder, callables, admission)
            }
        }
    }
    #[cfg(test)]
    fn name(&self) -> &str {
        match self {
            Self::RawCompatibility(parts) | Self::SelectedNormal { parts, .. } => &parts.name,
        }
    }
}
impl PreparedProgramRootTopLevelFunctionPartsV1 {
    #[allow(clippy::too_many_arguments)]
    fn from_source(
        name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Self {
        Self {
            name,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        }
    }
    fn lower_raw_with_port_v1<Port>(
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
    fn lower_normal_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        callables: &mut Port,
        admission: NormalTopLevelFunctionDraftAdmissionV1,
    ) -> Result<(), String>
    where
        Port: RootCallableCapturePortV1,
    {
        callables.lower_normal_top_level_function(
            builder,
            admission,
            self.params,
            self.param_decls,
            self.return_type_name,
            self.body,
            self.uses,
            self.attrs,
        )
    }
}
fn classify_statement(
    statement: ASTNode,
    is_app_mode: bool,
    statement_index: usize,
    work_plan_admission: ProgramRootWorkPlanAdmissionV1,
    normal_script_kind: Option<NormalScriptProgramItemAdmissionV1>,
) -> ProgramRootStatementDispositionV1 {
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
        } if !is_static => {
            let constructors = PreparedInstanceBoxConstructorBatchV1::prepare(name, constructors);
            let normal_constructor_sources = match work_plan_admission {
                ProgramRootWorkPlanAdmissionV1::RawCompatibility => None,
                ProgramRootWorkPlanAdmissionV1::SelectedNormal => {
                    Some(constructors.normal_sources(statement_index))
                }
            };
            let runtime_constructor_batch = if is_app_mode {
                None
            } else {
                Some(constructors.clone())
            };
            let selected_runtime_instance_demand = !is_app_mode
                && matches!(
                    normal_script_kind,
                    Some(
                        NormalScriptProgramItemAdmissionV1::InstancePrefixCompatibility
                            | NormalScriptProgramItemAdmissionV1::NonPlainInstanceFullLifecycle
                    )
                );
            ProgramRootStatementDispositionV1::ImmediateAndRuntime {
                work: PreparedProgramRootImmediateWorkV1::InstanceBox(
                    PreparedProgramRootInstanceBoxWorkV1 {
                        name: name.clone(),
                        methods: methods.clone(),
                        fields: fields.clone(),
                        field_decls: field_decls.clone(),
                        constructors,
                        normal_constructor_sources: normal_constructor_sources.clone(),
                        init_fields: init_fields.clone(),
                        weak_fields: weak_fields.clone(),
                    },
                ),
                runtime: PreparedProgramRootRuntimeStatementV1 {
                    source_statement_index: statement_index,
                    statement,
                    normal_script_kind,
                    constructor_sources: if selected_runtime_instance_demand {
                        normal_constructor_sources
                    } else {
                        None
                    },
                    constructor_batch: if selected_runtime_instance_demand {
                        runtime_constructor_batch
                    } else {
                        None
                    },
                },
            }
        }
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
                runtime: PreparedProgramRootRuntimeStatementV1 {
                    source_statement_index: statement_index,
                    statement,
                    normal_script_kind,
                    constructor_sources: None,
                    constructor_batch: None,
                },
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
        } => {
            let parts = PreparedProgramRootTopLevelFunctionPartsV1::from_source(
                name.clone(),
                params.clone(),
                param_decls.clone(),
                return_type_name.clone(),
                body.clone(),
                uses.clone(),
                attrs.clone(),
            );
            let work = match work_plan_admission {
                ProgramRootWorkPlanAdmissionV1::RawCompatibility => {
                    PreparedProgramRootTopLevelFunctionWorkV1::RawCompatibility(parts)
                }
                ProgramRootWorkPlanAdmissionV1::SelectedNormal => {
                    let source_key = NormalTopLevelFunctionSourceKeyV1::new(
                        statement_index,
                        name.clone(),
                        params.len(),
                    );
                    PreparedProgramRootTopLevelFunctionWorkV1::SelectedNormal {
                        admission: NormalTopLevelFunctionDraftAdmissionV1::seal(source_key),
                        parts,
                    }
                }
            };
            ProgramRootStatementDispositionV1::ImmediateOnly(
                PreparedProgramRootImmediateWorkV1::TopLevelFunction(work),
            )
        }
        _ => {
            ProgramRootStatementDispositionV1::RuntimeOnly(PreparedProgramRootRuntimeStatementV1 {
                source_statement_index: statement_index,
                statement,
                normal_script_kind,
                constructor_sources: None,
                constructor_batch: None,
            })
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{DeclarationAttrs, LiteralValue, Span};
    use crate::parser::NyashParser;
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
    fn instance_box_with_birth(name: &str) -> ASTNode {
        let mut declaration = box_declaration(name, false);
        let ASTNode::BoxDeclaration { constructors, .. } = &mut declaration else {
            unreachable!()
        };
        constructors.insert("birth/0".to_owned(), function("birth"));
        declaration
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
            ProgramRootWorkPlanAdmissionV1::SelectedNormal,
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
            PreparedProgramRootImmediateWorkV1::TopLevelFunction(work) if work.name() == "helper"
        ));
        assert_eq!(parts.deferred_static.len(), 1);
        assert_eq!(parts.deferred_static[0].name, "Helpers");
        assert_eq!(parts.runtime.len(), 4);
        assert!(
            matches!(parts.runtime.statement_at(0), ASTNode::BoxDeclaration { name, .. } if name == "Page")
        );
        assert!(
            matches!(parts.runtime.statement_at(3), ASTNode::BoxDeclaration { name, .. } if name == "Main")
        );
    }
    #[test]
    fn script_partition_keeps_static_boxes_out_of_deferred_work() {
        let ASTNode::Program { statements, .. } = NyashParser::parse_from_string(
            "static box Helpers { value() { return 1 } }\nfunction helper() { return 2 }\nprint(3)",
        )
        .expect("parsed Script partition fixture") else {
            panic!("expected Program root")
        };
        let plan = PreparedProgramRootWorkPlanV1::prepare(
            statements,
            false,
            ProgramRootWorkPlanAdmissionV1::SelectedNormal,
        );
        let parts = plan.into_parts();
        assert_eq!(parts.terminal, ProgramRootTerminalScheduleV1::ScriptRuntime);
        assert_eq!(parts.deferred_static.len(), 0);
        assert_eq!(parts.immediate.len(), 1);
        assert_eq!(parts.runtime.len(), 2);
        assert!(
            matches!(parts.runtime.statement_at(0), ASTNode::BoxDeclaration { name, .. } if name == "Helpers")
        );
        assert!(matches!(
            parts.runtime.statement_at(1),
            ASTNode::Print { .. }
        ));
        let PreparedProgramRootRuntimeWorkV1::SelectedNormal(runtime) = &parts.runtime else {
            panic!("expected selected Script runtime work")
        };
        assert_eq!(
            (
                runtime.source_statement_index_at(0),
                runtime.source_statement_index_at(1)
            ),
            (0, 2)
        );
    }
    #[test]
    fn selected_script_transports_one_constructor_source_to_its_second_demand() {
        let plan = PreparedProgramRootWorkPlanV1::prepare(
            vec![instance_box_with_birth("Page")],
            false,
            ProgramRootWorkPlanAdmissionV1::SelectedNormal,
        );
        let parts = plan.into_parts();
        let PreparedProgramRootImmediateWorkV1::InstanceBox(immediate) = &parts.immediate[0] else {
            panic!("expected immediate instance Box")
        };
        let immediate_sources = immediate
            .normal_constructor_sources
            .as_ref()
            .expect("selected immediate source");
        assert_eq!(immediate_sources.sources()[0].statement_index(), 0);
        assert_eq!(
            immediate_sources.sources()[0].parser_constructor_key(),
            "birth/0"
        );
        let PreparedProgramRootRuntimeWorkV1::SelectedNormal(runtime) = &parts.runtime else {
            panic!("expected selected Script runtime work")
        };
        let (runtime_sources, _) = runtime
            .constructor_admission_at(0)
            .expect("selected Script second demand source");
        assert_eq!(runtime_sources.sources(), immediate_sources.sources());
    }
    #[test]
    fn selected_nonplain_script_retains_constructor_source_for_full_runtime_lifecycle() {
        let mut nonplain = instance_box_with_birth("RecordPage");
        let ASTNode::BoxDeclaration { is_record, .. } = &mut nonplain else {
            unreachable!()
        };
        *is_record = true;
        let plan = PreparedProgramRootWorkPlanV1::prepare(
            vec![nonplain],
            false,
            ProgramRootWorkPlanAdmissionV1::SelectedNormal,
        );
        let parts = plan.into_parts();
        let PreparedProgramRootImmediateWorkV1::InstanceBox(immediate) = &parts.immediate[0] else {
            panic!("expected immediate instance Box")
        };
        assert!(immediate.normal_constructor_sources.is_some());
        let PreparedProgramRootRuntimeWorkV1::SelectedNormal(runtime) = &parts.runtime else {
            panic!("expected selected Script runtime work")
        };
        assert!(matches!(
            runtime.admission_at(0),
            NormalScriptRuntimeStatementAdmissionV1::NonPlainInstanceFullLifecycle { .. }
        ));
        assert!(runtime.constructor_admission_at(0).is_some());
    }
    #[test]
    fn selected_constructor_sources_keep_parser_key_order_and_skip_nonfunctions() {
        let mut declaration = box_declaration("Page", false);
        let ASTNode::BoxDeclaration { constructors, .. } = &mut declaration else {
            unreachable!()
        };
        constructors.insert("init/0".to_owned(), function("init"));
        constructors.insert("birth/1".to_owned(), function("birth"));
        constructors.insert("not-a-function".to_owned(), literal(0));
        let plan = PreparedProgramRootWorkPlanV1::prepare(
            vec![declaration],
            true,
            ProgramRootWorkPlanAdmissionV1::SelectedNormal,
        );
        let parts = plan.into_parts();
        let PreparedProgramRootImmediateWorkV1::InstanceBox(immediate) = &parts.immediate[0] else {
            panic!("expected immediate instance Box")
        };
        let keys = immediate
            .normal_constructor_sources
            .as_ref()
            .expect("selected source batch")
            .sources()
            .iter()
            .map(|source| source.parser_constructor_key())
            .collect::<Vec<_>>();
        assert_eq!(keys, ["birth/1", "init/0"]);
    }
    #[test]
    fn raw_runtime_keeps_the_neutral_statement_carrier() {
        let plan = PreparedProgramRootWorkPlanV1::prepare(
            vec![box_declaration("Helpers", true), literal(7)],
            false,
            ProgramRootWorkPlanAdmissionV1::RawCompatibility,
        );
        let parts = plan.into_parts();
        assert!(matches!(
            parts.runtime,
            PreparedProgramRootRuntimeWorkV1::RawCompatibility(_)
        ));
    }
    #[test]
    fn selected_top_level_functions_keep_distinct_source_occurrences() {
        let plan = PreparedProgramRootWorkPlanV1::prepare(
            vec![function("same"), function("same")],
            false,
            ProgramRootWorkPlanAdmissionV1::SelectedNormal,
        );
        let parts = plan.into_parts();
        let admissions = parts
            .immediate
            .iter()
            .map(|work| match work {
                PreparedProgramRootImmediateWorkV1::TopLevelFunction(
                    PreparedProgramRootTopLevelFunctionWorkV1::SelectedNormal { admission, .. },
                ) => admission,
                other => panic!("expected selected top-level work, got {other:?}"),
            })
            .collect::<Vec<_>>();
        assert_eq!(admissions.len(), 2);
        assert_eq!(admissions[0].source_key().statement_index(), 0);
        assert_eq!(admissions[1].source_key().statement_index(), 1);
        assert_eq!(admissions[0].physical_symbol(), "same/0");
        assert_eq!(admissions[1].physical_symbol(), "same/0");
    }
    #[test]
    fn raw_top_level_functions_do_not_issue_selected_receipts() {
        let plan = PreparedProgramRootWorkPlanV1::prepare(
            vec![function("same")],
            false,
            ProgramRootWorkPlanAdmissionV1::RawCompatibility,
        );
        let parts = plan.into_parts();
        assert!(matches!(
            &parts.immediate[0],
            PreparedProgramRootImmediateWorkV1::TopLevelFunction(
                PreparedProgramRootTopLevelFunctionWorkV1::RawCompatibility(_)
            )
        ));
    }
}
