//! RAW-SOURCE0-LOWER0-ROOT0-PLAN0: source-only Raw root plan.
//!
//! This module consumes the already bound Raw package and emits only owned
//! source facts.  It deliberately does not open a Builder session, shell,
//! collector, ledger, or publication route.  Unsupported module-level facts
//! are represented explicitly so later Root rows cannot silently rediscover
//! them from `current_module`.

use std::collections::HashMap;

use crate::ast::ASTNode;

use super::raw_source_binding::SourceBoundRawPackageV1;
use crate::mir::builder::{
    OwnedRawRootProjectionV1, OwnedRawSourceV1, RawRootProjectionPartsV1, RawSourceLocatorV1,
    RawSourceOriginV1,
};
use crate::mir::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationTokenV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct RawPhysicalRootIdentityV1 {
    main_symbol: &'static str,
    main_arity: usize,
    condition_symbol: &'static str,
    condition_arity: usize,
}

impl RawPhysicalRootIdentityV1 {
    pub(in crate::mir) const fn fixed() -> Self {
        Self {
            main_symbol: "main",
            main_arity: 0,
            condition_symbol: "condition_fn",
            condition_arity: 1,
        }
    }

    pub(in crate::mir) const fn main_symbol(&self) -> &'static str {
        self.main_symbol
    }

    pub(in crate::mir) const fn main_arity(&self) -> usize {
        self.main_arity
    }

    pub(in crate::mir) const fn condition_symbol(&self) -> &'static str {
        self.condition_symbol
    }

    pub(in crate::mir) const fn condition_arity(&self) -> usize {
        self.condition_arity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawRootWorkKindV1 {
    MainRoot,
    StaticBox,
    InstanceBox,
    TopLevelFunction,
    DeclarationFact,
    StaticData,
    RuntimeStatement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct RawRootWorkItemV1 {
    statement_index: usize,
    kind: RawRootWorkKindV1,
    name: Option<Box<str>>,
}

impl RawRootWorkItemV1 {
    pub(in crate::mir) const fn statement_index(&self) -> usize {
        self.statement_index
    }

    pub(in crate::mir) const fn kind(&self) -> RawRootWorkKindV1 {
        self.kind
    }

    pub(in crate::mir) fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawDeclarationFactKindV1 {
    Box,
    Enum,
    Brand,
    TypeAlias,
    Global,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct RawDeclarationFactRowV1 {
    statement_index: usize,
    kind: RawDeclarationFactKindV1,
    name: Box<str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct RawDeclarationFactPlanV1 {
    rows: Box<[RawDeclarationFactRowV1]>,
}

impl RawDeclarationFactPlanV1 {
    pub(in crate::mir) fn rows(&self) -> &[RawDeclarationFactRowV1] {
        &self.rows
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct RawStaticDataSourceRowV1 {
    statement_index: usize,
    name: Box<str>,
    element_type: Box<str>,
    value_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct RawStaticDataSourcePlanV1 {
    rows: Box<[RawStaticDataSourceRowV1]>,
}

impl RawStaticDataSourcePlanV1 {
    pub(in crate::mir) fn rows(&self) -> &[RawStaticDataSourceRowV1] {
        &self.rows
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawClosureAccessDispositionV1 {
    UnsupportedUntilAccess0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawRuntimeInputDispositionV1 {
    NotCapturedUntilRuntimeInput0,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct RawRootRuntimeInputSnapshotV1 {
    source_file: Option<Box<str>>,
    script_args: RawRuntimeInputDispositionV1,
    entry_safepoint: RawRuntimeInputDispositionV1,
}

impl RawRootRuntimeInputSnapshotV1 {
    pub(in crate::mir) fn source_file(&self) -> Option<&str> {
        self.source_file.as_deref()
    }

    pub(in crate::mir) const fn script_args(&self) -> RawRuntimeInputDispositionV1 {
        self.script_args
    }

    pub(in crate::mir) const fn entry_safepoint(&self) -> RawRuntimeInputDispositionV1 {
        self.entry_safepoint
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct RawRootAccessRequirementsV1 {
    pub(in crate::mir) static_data: bool,
    pub(in crate::mir) closure: RawClosureAccessDispositionV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct RawCallableHeaderRowV1 {
    symbol: Box<str>,
    arity: usize,
    selected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct RawCallableHeaderPlanV1 {
    rows: Box<[RawCallableHeaderRowV1]>,
}

impl RawCallableHeaderPlanV1 {
    pub(in crate::mir) fn rows(&self) -> &[RawCallableHeaderRowV1] {
        &self.rows
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct RawRootEnvironmentPlanV1 {
    work_schedule: Box<[RawRootWorkItemV1]>,
    declarations: RawDeclarationFactPlanV1,
    callable_headers: RawCallableHeaderPlanV1,
    static_data: RawStaticDataSourcePlanV1,
    access: RawRootAccessRequirementsV1,
    runtime_inputs: RawRootRuntimeInputSnapshotV1,
}

impl RawRootEnvironmentPlanV1 {
    pub(in crate::mir) fn work_schedule(&self) -> &[RawRootWorkItemV1] {
        &self.work_schedule
    }

    pub(in crate::mir) const fn declarations(&self) -> &RawDeclarationFactPlanV1 {
        &self.declarations
    }

    pub(in crate::mir) const fn callable_headers(&self) -> &RawCallableHeaderPlanV1 {
        &self.callable_headers
    }

    pub(in crate::mir) const fn static_data(&self) -> &RawStaticDataSourcePlanV1 {
        &self.static_data
    }

    pub(in crate::mir) const fn access(&self) -> RawRootAccessRequirementsV1 {
        self.access
    }

    pub(in crate::mir) const fn runtime_inputs(&self) -> &RawRootRuntimeInputSnapshotV1 {
        &self.runtime_inputs
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct RawScriptRootPlanV1 {
    statement_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct RawAppRootPlanV1 {
    main: RawSourceLocatorV1,
    static_children: Box<[RawSourceLocatorV1]>,
    callable_main: RawSourceLocatorV1,
    callable_main_selected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawRootKindV1 {
    Script(RawScriptRootPlanV1),
    App(RawAppRootPlanV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) struct RawRootPlanV1 {
    token: ModuleInvocationTokenV1,
    origin: RawSourceOriginV1,
    physical: RawPhysicalRootIdentityV1,
    kind: RawRootKindV1,
    environment: RawRootEnvironmentPlanV1,
}

impl RawRootPlanV1 {
    pub(in crate::mir) const fn token(&self) -> &ModuleInvocationTokenV1 {
        &self.token
    }

    pub(in crate::mir) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.token.brand()
    }

    pub(in crate::mir) fn from_package(
        package: SourceBoundRawPackageV1,
    ) -> Result<Self, RawRootPlanErrorV1> {
        let (token, source, continuation, config, _module_name) = package.into_parts();
        let source_file = config.source_file().map(str::to_owned).map(Into::into);
        let (ast, origin, projection) = source.into_plan_parts();
        let ASTNode::Program { statements, .. } = &ast else {
            return Err(RawRootPlanErrorV1::RootMustBeProgram);
        };
        let schedule = build_schedule(statements);
        let declarations = build_declarations(statements);
        let static_data = build_static_data(statements);
        let callable_headers =
            build_callable_headers(&projection, statements, continuation.callable_main());
        let environment = RawRootEnvironmentPlanV1 {
            work_schedule: schedule.into_boxed_slice(),
            declarations: RawDeclarationFactPlanV1 {
                rows: declarations.into_boxed_slice(),
            },
            callable_headers: RawCallableHeaderPlanV1 {
                rows: callable_headers.into_boxed_slice(),
            },
            access: RawRootAccessRequirementsV1 {
                static_data: !static_data.is_empty(),
                closure: RawClosureAccessDispositionV1::UnsupportedUntilAccess0,
            },
            static_data: RawStaticDataSourcePlanV1 {
                rows: static_data.into_boxed_slice(),
            },
            runtime_inputs: RawRootRuntimeInputSnapshotV1 {
                source_file,
                script_args: RawRuntimeInputDispositionV1::NotCapturedUntilRuntimeInput0,
                entry_safepoint: RawRuntimeInputDispositionV1::NotCapturedUntilRuntimeInput0,
            },
        };
        let kind = match projection.into_plan_parts() {
            RawRootProjectionPartsV1::Script { statement_count } => {
                RawRootKindV1::Script(RawScriptRootPlanV1 { statement_count })
            }
            RawRootProjectionPartsV1::App {
                main,
                static_children,
                callable_main,
            } => RawRootKindV1::App(RawAppRootPlanV1 {
                main,
                static_children,
                callable_main,
                callable_main_selected: matches!(
                    continuation.callable_main(),
                    crate::mir::builder::RawCallableMainCompatibilityDispositionV1::Selected
                ),
            }),
        };
        Ok(Self {
            token,
            origin,
            physical: RawPhysicalRootIdentityV1::fixed(),
            kind,
            environment,
        })
    }

    pub(in crate::mir) const fn physical(&self) -> RawPhysicalRootIdentityV1 {
        self.physical
    }

    pub(in crate::mir) const fn kind(&self) -> &RawRootKindV1 {
        &self.kind
    }

    pub(in crate::mir) const fn environment(&self) -> &RawRootEnvironmentPlanV1 {
        &self.environment
    }

    pub(in crate::mir) const fn origin(&self) -> RawSourceOriginV1 {
        self.origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawRootPlanErrorV1 {
    RootMustBeProgram,
}

impl std::fmt::Display for RawRootPlanErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][raw_root_plan0] {self:?}")
    }
}

impl std::error::Error for RawRootPlanErrorV1 {}

fn build_schedule(statements: &[ASTNode]) -> Vec<RawRootWorkItemV1> {
    statements
        .iter()
        .enumerate()
        .map(|(statement_index, statement)| {
            let (kind, name) = match statement {
                ASTNode::BoxDeclaration {
                    name, is_static, ..
                } => (
                    if name == "Main" && *is_static {
                        RawRootWorkKindV1::MainRoot
                    } else if *is_static {
                        RawRootWorkKindV1::StaticBox
                    } else {
                        RawRootWorkKindV1::InstanceBox
                    },
                    Some(name.clone().into_boxed_str()),
                ),
                ASTNode::FunctionDeclaration { name, .. } => (
                    RawRootWorkKindV1::TopLevelFunction,
                    Some(name.clone().into_boxed_str()),
                ),
                ASTNode::EnumDeclaration { name, .. }
                | ASTNode::BrandDeclaration { name, .. }
                | ASTNode::TypeAliasDeclaration { name, .. }
                | ASTNode::GlobalVar { name, .. } => (
                    RawRootWorkKindV1::DeclarationFact,
                    Some(name.clone().into_boxed_str()),
                ),
                ASTNode::StaticConstTable { name, .. } => (
                    RawRootWorkKindV1::StaticData,
                    Some(name.clone().into_boxed_str()),
                ),
                _ => (RawRootWorkKindV1::RuntimeStatement, None),
            };
            RawRootWorkItemV1 {
                statement_index,
                kind,
                name,
            }
        })
        .collect()
}

fn build_declarations(statements: &[ASTNode]) -> Vec<RawDeclarationFactRowV1> {
    statements
        .iter()
        .enumerate()
        .filter_map(|(statement_index, statement)| {
            let (kind, name) = match statement {
                ASTNode::BoxDeclaration { name, .. } => (RawDeclarationFactKindV1::Box, name),
                ASTNode::EnumDeclaration { name, .. } => (RawDeclarationFactKindV1::Enum, name),
                ASTNode::BrandDeclaration { name, .. } => (RawDeclarationFactKindV1::Brand, name),
                ASTNode::TypeAliasDeclaration { name, .. } => {
                    (RawDeclarationFactKindV1::TypeAlias, name)
                }
                ASTNode::GlobalVar { name, .. } => (RawDeclarationFactKindV1::Global, name),
                _ => return None,
            };
            Some(RawDeclarationFactRowV1 {
                statement_index,
                kind,
                name: name.clone().into_boxed_str(),
            })
        })
        .collect()
}

fn build_static_data(statements: &[ASTNode]) -> Vec<RawStaticDataSourceRowV1> {
    statements
        .iter()
        .enumerate()
        .filter_map(|(statement_index, statement)| {
            let ASTNode::StaticConstTable {
                name,
                element_type,
                values,
                ..
            } = statement
            else {
                return None;
            };
            Some(RawStaticDataSourceRowV1 {
                statement_index,
                name: name.clone().into_boxed_str(),
                element_type: element_type.clone().into_boxed_str(),
                value_count: values.len(),
            })
        })
        .collect()
}

fn build_callable_headers(
    projection: &OwnedRawRootProjectionV1,
    statements: &[ASTNode],
    disposition: crate::mir::builder::RawCallableMainCompatibilityDispositionV1,
) -> Vec<RawCallableHeaderRowV1> {
    let mut rows = Vec::new();
    for locator in projection.static_child_locators() {
        rows.push(RawCallableHeaderRowV1 {
            symbol: locator.symbol().to_owned().into_boxed_str(),
            arity: locator.arity(),
            selected: false,
        });
    }
    if let Some(locator) = projection.callable_main_locator() {
        rows.push(RawCallableHeaderRowV1 {
            symbol: locator.symbol().to_owned().into_boxed_str(),
            arity: locator.arity(),
            selected: matches!(
                disposition,
                crate::mir::builder::RawCallableMainCompatibilityDispositionV1::Selected
            ),
        });
    }
    for statement in statements {
        if let ASTNode::FunctionDeclaration { name, params, .. } = statement {
            rows.push(RawCallableHeaderRowV1 {
                symbol: name.clone().into_boxed_str(),
                arity: params.len(),
                selected: false,
            });
        }
    }
    rows
}

impl SourceBoundRawPackageV1 {
    pub(in crate::mir) fn into_root_plan(self) -> Result<RawRootPlanV1, RawRootPlanErrorV1> {
        RawRootPlanV1::from_package(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, DeclarationAttrs, Span};
    use crate::mir::compiler::lowering_input::LegacyModuleLoweringInputV1;
    use crate::mir::compiler::raw_source_binding::RawCallableMainSelectionV1;
    use crate::mir::MirCompiler;

    fn function(name: &str, arity: usize) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: name.into(),
            params: (0..arity).map(|index| format!("p{index}")).collect(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: Vec::new(),
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    fn app_source() -> ASTNode {
        let mut methods = HashMap::new();
        methods.insert("main".into(), function("main", 2));
        methods.insert("helper".into(), function("helper", 0));
        ASTNode::Program {
            statements: vec![ASTNode::BoxDeclaration {
                name: "Main".into(),
                methods,
                is_static: true,
                fields: Vec::new(),
                field_decls: Vec::new(),
                public_fields: Vec::new(),
                private_fields: Vec::new(),
                constructors: HashMap::new(),
                init_fields: Vec::new(),
                weak_fields: Vec::new(),
                delegates: Vec::new(),
                invariants: Vec::new(),
                transitions: Vec::new(),
                is_interface: false,
                is_sync: false,
                is_record: false,
                type_parameters: Vec::new(),
                extends: Vec::new(),
                implements: Vec::new(),
                static_init: None,
                attrs: DeclarationAttrs::default(),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }
    }

    fn bind(source: ASTNode, selection: RawCallableMainSelectionV1) -> SourceBoundRawPackageV1 {
        let mut compiler = MirCompiler::new();
        compiler
            .bind_raw_source(
                LegacyModuleLoweringInputV1::bare_ast(source),
                Some("plan0.hako"),
                "plan0",
                selection,
            )
            .unwrap()
    }

    #[test]
    fn script_plan_seals_physical_identity_and_schedule() {
        let source = ASTNode::Program {
            statements: vec![
                ASTNode::Literal {
                    value: crate::ast::LiteralValue::Integer(1),
                    span: Span::unknown(),
                },
                ASTNode::Print {
                    expression: Box::new(ASTNode::Literal {
                        value: crate::ast::LiteralValue::Integer(2),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                },
            ],
            span: Span::unknown(),
        };
        let package = bind(source, RawCallableMainSelectionV1::Omitted);
        let package_brand = package.brand();
        let plan = package.into_root_plan().unwrap();
        assert_eq!(plan.brand(), package_brand);
        assert!(matches!(plan.kind(), RawRootKindV1::Script(_)));
        assert_eq!(plan.physical().main_symbol(), "main");
        assert_eq!(plan.physical().main_arity(), 0);
        assert_eq!(plan.physical().condition_symbol(), "condition_fn");
        assert_eq!(plan.physical().condition_arity(), 1);
        assert_eq!(plan.environment().work_schedule().len(), 2);
        assert_eq!(
            plan.environment().runtime_inputs().source_file(),
            Some("plan0.hako")
        );
    }

    #[test]
    fn app_plan_keeps_source_arity_separate_from_physical_root() {
        let plan = bind(app_source(), RawCallableMainSelectionV1::Omitted)
            .into_root_plan()
            .unwrap();
        let RawRootKindV1::App(app) = plan.kind() else {
            panic!("expected app plan");
        };
        assert_eq!(app.main.arity(), 2);
        assert_eq!(plan.physical().main_arity(), 0);
        assert!(!app.callable_main_selected);
        assert_eq!(plan.environment().callable_headers().rows().len(), 2);
    }

    #[test]
    fn app_plan_retains_selected_callable_main_disposition() {
        let plan = bind(app_source(), RawCallableMainSelectionV1::Required)
            .into_root_plan()
            .unwrap();
        let RawRootKindV1::App(app) = plan.kind() else {
            panic!("expected app plan");
        };
        assert!(app.callable_main_selected);
        let selected = plan
            .environment()
            .callable_headers()
            .rows()
            .iter()
            .filter(|row| row.selected)
            .count();
        assert_eq!(selected, 1);
    }
}
