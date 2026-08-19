use super::callable_declaration_catalog::VerifiedSelectedNormalCallableSourceInventoryV1;
use super::instance_box_constructor_batch::PreparedInstanceBoxConstructorBatchV1;
use super::instance_box_declaration_lifecycle::PreparedInstanceBoxDeclarationLifecycleV1;
use super::module_lifecycle::RootCallableCapturePortV1;
use super::normal_instance_constructor_admission::NormalInstanceConstructorSourceBatchV1;
use super::normal_script_instance_box_transfer::VerifiedScriptInstanceBoxTransferCohortV1;
use super::normal_script_program_item_admission::{
    classify_normal_script_program_item_v1, NormalScriptProgramItemAdmissionV1,
};
use super::normal_script_root_demand_window::PreparedScriptRootAdmissionV1;
use super::normal_script_root_demand_window::ScriptRootDemandWindowBuilderV1;
#[cfg(test)]
use super::normal_script_runtime_work::NormalScriptRuntimeStatementAdmissionV1;
use super::normal_script_runtime_work::{
    PreparedNormalScriptRuntimeInputV1, PreparedNormalScriptRuntimeWorkV1,
};
use super::normal_script_selected_occurrence::SelectedScriptProgramOccurrenceV1;
use super::normal_top_level_function_admission::NormalTopLevelFunctionDraftAdmissionV1;
use super::MirBuilder;
use crate::ast::{ASTNode, DeclarationAttrs, FieldDecl, ParamDecl};
#[derive(Debug)]
pub(super) struct PreparedProgramRootWorkPlanV1 {
    immediate: Box<[PreparedProgramRootImmediateWorkV1]>,
    deferred_static: Box<[PreparedProgramDeferredStaticBoxWorkV1]>,
    runtime: PreparedProgramRootRuntimeWorkV1,
    terminal: ProgramRootTerminalScheduleV1,
    script_root_admission: Option<PreparedScriptRootAdmissionV1>,
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
    pub(super) script_root_admission: Option<PreparedScriptRootAdmissionV1>,
}
#[derive(Debug)]
pub(super) enum PreparedProgramRootImmediateWorkV1 {
    InstanceBox(PreparedProgramRootInstanceBoxWorkV1),
    TopLevelFunction(PreparedProgramRootTopLevelFunctionWorkV1),
}
#[derive(Debug)]
pub(super) struct PreparedProgramRootInstanceBoxWorkV1 {
    name: String,
    methods: crate::ast::BoxMethodInventoryV1,
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
    methods: crate::ast::BoxMethodInventoryV1,
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
    pub(super) fn into_parts(self) -> (String, crate::ast::BoxMethodInventoryV1) {
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
        selected_callable_sources: Option<&VerifiedSelectedNormalCallableSourceInventoryV1>,
    ) -> Self {
        Self::prepare_with_instance_box_transfers(
            statements,
            is_app_mode,
            work_plan_admission,
            selected_callable_sources,
            None,
        )
    }

    pub(super) fn prepare_with_instance_box_transfers(
        statements: Vec<ASTNode>,
        is_app_mode: bool,
        work_plan_admission: ProgramRootWorkPlanAdmissionV1,
        selected_callable_sources: Option<&VerifiedSelectedNormalCallableSourceInventoryV1>,
        instance_box_transfers: Option<&VerifiedScriptInstanceBoxTransferCohortV1>,
    ) -> Self {
        assert_eq!(
            selected_callable_sources.is_some(),
            work_plan_admission == ProgramRootWorkPlanAdmissionV1::SelectedNormal,
            "selected callable inventory must match work-plan admission",
        );
        let mut immediate = Vec::new();
        let mut deferred_static = Vec::new();
        let mut runtime_statements = Vec::new();
        let mut script_window = (!is_app_mode
            && work_plan_admission == ProgramRootWorkPlanAdmissionV1::SelectedNormal)
            .then(|| {
                ScriptRootDemandWindowBuilderV1::for_program_statement_count(statements.len())
            });
        for (statement_index, statement) in statements.into_iter().enumerate() {
            let normal_script_kind = (work_plan_admission
                == ProgramRootWorkPlanAdmissionV1::SelectedNormal)
                .then(|| classify_normal_script_program_item_v1(&statement));
            if let Some(window) = &mut script_window {
                let mut occurrence = SelectedScriptProgramOccurrenceV1::new(
                    statement_index,
                    &statement,
                    normal_script_kind.expect("selected Script runtime classifier"),
                );
                if instance_box_transfers
                    .is_some_and(|transfers| transfers.contains(statement_index))
                {
                    occurrence = occurrence.with_instance_box_transfer();
                }
                window
                    .record_selected_work_item(&statement, occurrence)
                    .expect("selected Script demand-window source contract");
            }
            let disposition = classify_statement(
                statement,
                is_app_mode,
                statement_index,
                work_plan_admission,
                normal_script_kind,
                selected_callable_sources,
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
            script_root_admission: script_window
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
            script_root_admission: self.script_root_admission,
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
    selected_callable_sources: Option<&VerifiedSelectedNormalCallableSourceInventoryV1>,
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
                    let source_key = selected_callable_sources
                        .and_then(|sources| sources.top_level_function(statement_index))
                        .filter(|key| {
                            key.declared_name() == name && key.declared_arity() == params.len()
                        })
                        .cloned()
                        .expect("selected callable catalog/work-plan source contract");
                    PreparedProgramRootTopLevelFunctionWorkV1::SelectedNormal {
                        admission: NormalTopLevelFunctionDraftAdmissionV1::from_catalog_key(
                            source_key,
                        ),
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
#[path = "program_root_work_plan_tests.rs"]
mod tests;
