//! RAW-SOURCE0-LOWER0-ROOT0-CHILDREN0-S0.
//!
//! This module consumes only the PLAN0-derived helper schedule.  It keeps the
//! physical owner in Builder and exposes one complete-all transition; no
//! sibling, retry, or root-body continuation is published.

use std::collections::VecDeque;

use super::raw_root_eligibility::{
    RawAppRootInvocationV1, RawEligibleCatalogV1, RawRootInvocationV1, RawRootPhysicalCoreV1,
    RawScriptRootInvocationV1,
};
use super::raw_root_plan0::RawRootPlanV1;
use crate::mir::builder::MirBuilder;
use crate::mir::builder::{
    CollectedDraftAdmissionReceiptV1, InvocationBranded, RawRootPhysicalChildErrorV1,
    RawRootStaticChildWorkErrorV1, RawRootStaticChildWorkV1, RawSourceLocatorV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawRootChildOrderV1 {
    LexicalMethodName,
}

#[derive(Debug)]
pub(in crate::mir) struct RawRootChildScheduleV1 {
    order: RawRootChildOrderV1,
    remaining: VecDeque<RawSourceLocatorV1>,
}

impl RawRootChildScheduleV1 {
    fn new(locators: Box<[RawSourceLocatorV1]>) -> Self {
        Self {
            order: RawRootChildOrderV1::LexicalMethodName,
            remaining: locators.into_vec().into(),
        }
    }

    fn pop(&mut self) -> Option<RawSourceLocatorV1> {
        self.remaining.pop_front()
    }
    fn len(&self) -> usize {
        self.remaining.len()
    }
}

#[derive(Debug)]
pub(in crate::mir) struct RawPreRootChildrenCompletionV1 {
    brand: crate::mir::module_invocation_identity::ModuleInvocationBrandV1,
    order: RawRootChildOrderV1,
    expected_count: usize,
    successful_count: usize,
}

#[derive(Debug)]
pub(in crate::mir) struct RawRootChildReceiptV1 {
    ordinal: usize,
    symbol: Box<str>,
    receipt: InvocationBranded<CollectedDraftAdmissionReceiptV1>,
}

#[derive(Debug)]
pub(in crate::mir) struct RawRootChildFailureSiteV1 {
    ordinal: usize,
    locator: RawSourceLocatorV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawChildrenPendingInvocationV1 {
    core: RawRootChildCoreV1,
    schedule: RawRootChildScheduleV1,
    prefix: Vec<RawRootChildReceiptV1>,
}

#[derive(Debug)]
pub(in crate::mir) enum RawChildrenCompleteInvocationV1 {
    Script(RawScriptChildrenCompleteInvocationV1),
    App(RawAppChildrenCompleteInvocationV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RawScriptChildrenCompleteInvocationV1 {
    core: RawRootChildCoreV1,
    completion: RawPreRootChildrenCompletionV1,
    receipts: Box<[RawRootChildReceiptV1]>,
}

#[derive(Debug)]
pub(in crate::mir) struct RawAppChildrenCompleteInvocationV1 {
    core: RawRootChildCoreV1,
    completion: RawPreRootChildrenCompletionV1,
    receipts: Box<[RawRootChildReceiptV1]>,
}

#[derive(Debug)]
struct RawRootChildCoreV1 {
    token: crate::mir::module_invocation_identity::ModuleInvocationTokenV1,
    source: crate::mir::builder::OwnedRawSourceV1,
    continuation: super::raw_source_binding::RawSourceContinuationV1,
    config: crate::mir::builder::BuilderInvocationConfigV1,
    module_name: Box<str>,
    plan: RawRootPlanV1,
    session: crate::mir::builder::ModuleBuilderInvocationSessionV1,
    physical: crate::mir::builder::RawRootPhysicalStateV1,
}

#[derive(Debug)]
pub(in crate::mir) enum RejectedRawRootChildrenInvocationV1 {
    Source {
        owner: RawChildrenPendingInvocationV1,
        error: RawRootStaticChildWorkErrorV1,
        failed: Option<RawRootChildFailureSiteV1>,
    },
    Physical {
        owner: RawChildrenPendingInvocationV1,
        error: RawRootPhysicalChildErrorV1,
        failed: Option<RawRootChildFailureSiteV1>,
    },
}

impl RawRootInvocationV1 {
    pub(in crate::mir) fn prepare_children(
        self,
    ) -> Result<RawChildrenPendingInvocationV1, RejectedRawRootChildrenInvocationV1> {
        let core = match self {
            Self::Script(RawScriptRootInvocationV1 { core })
            | Self::App(RawAppRootInvocationV1 { core }) => core,
        };
        let RawRootPhysicalCoreV1 {
            token,
            source,
            continuation,
            config,
            module_name,
            plan: original_plan,
            proof,
            session,
            physical,
        } = core;
        let (plan, locators) = original_plan.into_pre_root_children();
        let helper_count = match (&proof.catalog, &plan.kind()) {
            (
                RawEligibleCatalogV1::EmptyScript,
                super::raw_root_plan0::RawRootKindV1::Script(_),
            ) => 0,
            (
                RawEligibleCatalogV1::PlainStaticMain { helper_count },
                super::raw_root_plan0::RawRootKindV1::App(_),
            ) => *helper_count,
            _ => usize::MAX,
        };
        if helper_count != locators.len() || !schedule_is_lexical(&locators) {
            return Err(RejectedRawRootChildrenInvocationV1::Source {
                owner: pending_from_parts(
                    token,
                    source,
                    continuation,
                    config,
                    module_name,
                    plan,
                    session,
                    physical,
                    locators,
                ),
                error: RawRootStaticChildWorkErrorV1::ScheduleMismatch,
                failed: None,
            });
        }
        Ok(RawChildrenPendingInvocationV1 {
            core: RawRootChildCoreV1 {
                token,
                source,
                continuation,
                config,
                module_name,
                plan,
                session,
                physical,
            },
            schedule: RawRootChildScheduleV1::new(locators),
            prefix: Vec::new(),
        })
    }
}

fn pending_from_parts(
    token: crate::mir::module_invocation_identity::ModuleInvocationTokenV1,
    source: crate::mir::builder::OwnedRawSourceV1,
    continuation: super::raw_source_binding::RawSourceContinuationV1,
    config: crate::mir::builder::BuilderInvocationConfigV1,
    module_name: Box<str>,
    plan: RawRootPlanV1,
    session: crate::mir::builder::ModuleBuilderInvocationSessionV1,
    physical: crate::mir::builder::RawRootPhysicalStateV1,
    locators: Box<[RawSourceLocatorV1]>,
) -> RawChildrenPendingInvocationV1 {
    RawChildrenPendingInvocationV1 {
        core: RawRootChildCoreV1 {
            token,
            source,
            continuation,
            config,
            module_name,
            plan,
            session,
            physical,
        },
        schedule: RawRootChildScheduleV1::new(locators),
        prefix: Vec::new(),
    }
}

impl RawChildrenPendingInvocationV1 {
    pub(in crate::mir) fn complete_all(
        mut self,
    ) -> Result<RawChildrenCompleteInvocationV1, RejectedRawRootChildrenInvocationV1> {
        let expected = self.schedule.remaining.len();
        let mut ordinal = 0;
        while let Some(locator) = self.schedule.pop() {
            let failed = RawRootChildFailureSiteV1 {
                ordinal,
                locator: locator.clone(),
            };
            let work = match self.core.source.prepare_static_child(locator, ordinal) {
                Ok(work) => work,
                Err(error) => {
                    return Err(RejectedRawRootChildrenInvocationV1::Source {
                        owner: self,
                        error,
                        failed: Some(failed),
                    })
                }
            };
            let symbol = work.symbol().to_owned().into_boxed_str();
            let builder = self.core.session.builder_mut();
            let result = self.core.physical.complete_static_child(builder, work);
            match result {
                Ok(receipt) => {
                    self.prefix.push(RawRootChildReceiptV1 {
                        ordinal,
                        symbol,
                        receipt,
                    });
                }
                Err(failure) => {
                    return Err(RejectedRawRootChildrenInvocationV1::Physical {
                        owner: self,
                        error: failure,
                        failed: Some(failed),
                    });
                }
            }
            ordinal += 1;
        }
        let completion = RawPreRootChildrenCompletionV1 {
            brand: self.core.token.brand(),
            order: self.schedule.order,
            expected_count: expected,
            successful_count: self.prefix.len(),
        };
        let is_script = matches!(
            self.core.plan.kind(),
            super::raw_root_plan0::RawRootKindV1::Script(_)
        );
        if is_script {
            Ok(RawChildrenCompleteInvocationV1::Script(
                RawScriptChildrenCompleteInvocationV1 {
                    core: self.core,
                    completion,
                    receipts: self.prefix.into_boxed_slice(),
                },
            ))
        } else {
            Ok(RawChildrenCompleteInvocationV1::App(
                RawAppChildrenCompleteInvocationV1 {
                    core: self.core,
                    completion,
                    receipts: self.prefix.into_boxed_slice(),
                },
            ))
        }
    }
}

fn schedule_is_lexical(locators: &[RawSourceLocatorV1]) -> bool {
    locators.windows(2).all(|pair| {
        !pair[0].method_name().is_empty()
            && pair[0].method_name() < pair[1].method_name()
            && pair[0].method_name() != "main"
            && pair[0].symbol() != pair[1].symbol()
    }) && locators.last().map_or(true, |last| {
        !last.method_name().is_empty() && last.method_name() != "main"
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, DeclarationAttrs, Span};
    use crate::mir::builder::MirBuilder;
    use crate::mir::compiler::lowering_input::LegacyModuleLoweringInputV1;
    use crate::mir::compiler::raw_source_binding::RawCallableMainSelectionV1;
    use crate::mir::MirCompiler;
    use std::collections::HashMap;

    fn function(name: &str) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: name.into(),
            params: Vec::new(),
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

    fn bound(source: ASTNode) -> RawRootInvocationV1 {
        let mut compiler = MirCompiler::new();
        compiler
            .bind_raw_source(
                LegacyModuleLoweringInputV1::bare_ast(source),
                None,
                "children0",
                RawCallableMainSelectionV1::Omitted,
            )
            .unwrap()
            .into_root_package()
            .unwrap()
            .prepare_eligibility()
            .unwrap()
            .open_physical(&MirBuilder::new())
            .unwrap()
    }

    fn app(method_names: &[&str]) -> ASTNode {
        let mut methods = HashMap::new();
        methods.insert("main".into(), function("main"));
        for name in method_names {
            methods.insert((*name).into(), function(name));
        }
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

    #[test]
    fn script_uses_typed_zero_child_completion_without_root_tracker_activity() {
        let invocation = bound(ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        });
        let pending = invocation.prepare_children().unwrap();
        let complete = pending.complete_all().unwrap();
        let RawChildrenCompleteInvocationV1::Script(complete) = complete else {
            panic!("empty script must produce Script completion")
        };
        assert_eq!(complete.completion.expected_count, 0);
        assert_eq!(complete.completion.successful_count, 0);
        assert!(complete.receipts.is_empty());
    }

    #[test]
    fn app_consumes_the_projected_lexical_helper_schedule() {
        let invocation = bound(app(&["zeta", "alpha"]));
        let complete = invocation
            .prepare_children()
            .unwrap()
            .complete_all()
            .unwrap();
        let RawChildrenCompleteInvocationV1::App(complete) = complete else {
            panic!("static Main must produce App completion")
        };
        assert_eq!(complete.completion.expected_count, 2);
        assert_eq!(complete.completion.successful_count, 2);
        assert_eq!(complete.receipts[0].symbol.as_ref(), "Main.alpha/0");
        assert_eq!(complete.receipts[1].symbol.as_ref(), "Main.zeta/0");
    }
}
