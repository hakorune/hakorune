//! RAW-SOURCE0-LOWER0-ROOT0-OWNER0-PACKAGE0.
//!
//! This is the one-time source/plan handoff for the Raw root owner.  Planning
//! borrows the complete source-bound package first.  Only a successful plan
//! permits the original package to be consumed and co-sealed with the plan.
//! No Builder, shell, collector, ledger, tracker, or publication effect is
//! opened here.

use super::raw_root_plan0::{RawRootPlanErrorV1, RawRootPlanV1};
use super::raw_runtime_inputs::RawRuntimeInputSnapshotV1;
use super::raw_source_binding::{RawRootContinuationV1, SourceBoundRawPackageV1};
use crate::mir::builder::{BuilderInvocationConfigV1, OwnedRawSourceV1};
use crate::mir::module_invocation_identity::{
    ModuleInvocationBrandV1, ModuleInvocationFamilyV1, ModuleInvocationTokenV1,
};

#[derive(Debug)]
pub(in crate::mir) struct SourceBoundRawRootPackageV1 {
    token: ModuleInvocationTokenV1,
    source: OwnedRawSourceV1,
    continuation: RawRootContinuationV1,
    runtime_inputs: RawRuntimeInputSnapshotV1,
    config: BuilderInvocationConfigV1,
    module_name: Box<str>,
    plan: RawRootPlanV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawRootPlanningV1 {
    owner: SourceBoundRawPackageV1,
    error: RawRootPlanErrorV1,
}

impl SourceBoundRawPackageV1 {
    /// The sole PACKAGE0 handoff.  The borrow happens before any owner is
    /// moved, so a planning failure retains the exact original package.
    pub(in crate::mir) fn into_root_package(
        self,
    ) -> Result<SourceBoundRawRootPackageV1, RejectedRawRootPlanningV1> {
        let plan = match RawRootPlanV1::from_bound_package(&self) {
            Ok(plan) => plan,
            Err(error) => return Err(RejectedRawRootPlanningV1 { owner: self, error }),
        };
        let (token, source, continuation, runtime_inputs, config, module_name) = self.into_parts();
        Ok(SourceBoundRawRootPackageV1 {
            token,
            source,
            continuation: continuation.into_root_continuation(),
            runtime_inputs,
            config,
            module_name,
            plan,
        })
    }
}

impl SourceBoundRawRootPackageV1 {
    pub(in crate::mir) const fn token(&self) -> &ModuleInvocationTokenV1 {
        &self.token
    }

    pub(in crate::mir) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.token.brand()
    }

    pub(in crate::mir) const fn family(&self) -> ModuleInvocationFamilyV1 {
        self.token.family()
    }

    pub(in crate::mir) const fn plan(&self) -> &RawRootPlanV1 {
        &self.plan
    }

    pub(in crate::mir) const fn source(&self) -> &OwnedRawSourceV1 {
        &self.source
    }

    pub(in crate::mir) const fn continuation(&self) -> &RawRootContinuationV1 {
        &self.continuation
    }

    pub(in crate::mir) const fn runtime_inputs(&self) -> &RawRuntimeInputSnapshotV1 {
        &self.runtime_inputs
    }

    pub(in crate::mir) const fn config(&self) -> &BuilderInvocationConfigV1 {
        &self.config
    }

    pub(in crate::mir) fn module_name(&self) -> &str {
        &self.module_name
    }

    pub(in crate::mir::compiler) fn into_manifest_parts(
        self,
    ) -> (
        ModuleInvocationTokenV1,
        OwnedRawSourceV1,
        RawRootContinuationV1,
        RawRuntimeInputSnapshotV1,
        BuilderInvocationConfigV1,
        Box<str>,
        RawRootPlanV1,
    ) {
        (
            self.token,
            self.source,
            self.continuation,
            self.runtime_inputs,
            self.config,
            self.module_name,
            self.plan,
        )
    }
}

impl RejectedRawRootPlanningV1 {
    pub(in crate::mir) const fn error(&self) -> &RawRootPlanErrorV1 {
        &self.error
    }

    /// Rejected planning has no recovery or retry terminal.  Consuming this
    /// owner is the only supported disposition after inspection.
    pub(in crate::mir) fn discard(self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, DeclarationAttrs, Span};
    use crate::mir::builder::RawCallableMainCompatibilityDispositionV1;
    use crate::mir::compiler::lowering_input::LegacyModuleLoweringInputV1;
    use crate::mir::compiler::raw_source_binding::RawCallableMainSelectionV1;
    use crate::mir::MirCompiler;
    use std::collections::HashMap;

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

    fn app() -> ASTNode {
        let mut methods = HashMap::new();
        methods.insert("main".into(), function("main", 2));
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
                Some("owner0.hako"),
                "owner0",
                selection,
            )
            .unwrap()
    }

    #[test]
    fn package_retains_exact_bound_owners_and_fixed_physical_root() {
        let package = bind(
            ASTNode::Program {
                statements: Vec::new(),
                span: Span::unknown(),
            },
            RawCallableMainSelectionV1::Omitted,
        );
        let brand = package.brand();
        let root = package.into_root_package().unwrap();
        assert_eq!(root.brand(), brand);
        assert_eq!(root.family(), ModuleInvocationFamilyV1::Raw);
        assert_eq!(root.plan.physical().main_arity(), 0);
        assert_eq!(root.plan.physical().condition_arity(), 1);
        assert!(root.source.projection().is_script());
        assert_eq!(root.config.source_file(), Some("owner0.hako"));
        assert_eq!(root.module_name.as_ref(), "owner0");
    }

    #[test]
    fn package_keeps_callable_main_selection_only_in_continuation() {
        let omitted = bind(app(), RawCallableMainSelectionV1::Omitted)
            .into_root_package()
            .unwrap();
        assert_eq!(
            omitted.continuation.callable_main(),
            RawCallableMainCompatibilityDispositionV1::NotSelected
        );
        let selected = bind(app(), RawCallableMainSelectionV1::Required)
            .into_root_package()
            .unwrap();
        assert_eq!(
            selected.continuation.callable_main(),
            RawCallableMainCompatibilityDispositionV1::Selected
        );
        let crate::mir::compiler::raw_root_plan0::RawRootKindV1::App(app) = selected.plan.kind()
        else {
            panic!("expected App root plan");
        };
        assert_eq!(app.main().arity(), 2);
    }

    #[test]
    fn rejected_planning_retains_the_original_bound_package() {
        let mut compiler = MirCompiler::new();
        let package = compiler
            .bind_raw_source(
                LegacyModuleLoweringInputV1::bare_ast(ASTNode::Program {
                    statements: Vec::new(),
                    span: Span::unknown(),
                }),
                None,
                "",
                RawCallableMainSelectionV1::Omitted,
            )
            .unwrap();
        let rejected = package.into_root_package().unwrap_err();
        assert!(matches!(
            rejected.error(),
            RawRootPlanErrorV1::EmptyModuleName
        ));
        rejected.discard();
    }
}
