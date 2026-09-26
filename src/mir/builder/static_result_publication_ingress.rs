//! Effect-free ingress for the source-bound static result publication owner.
//!
//! The ingress keeps target and result states distinct.  A compatibility port
//! is `Unavailable`; an exact Cataloged site may be `Selected` or `TargetOnly`;
//! a site with no exact target is `NoExactStaticTarget`; source loss/drift is
//! a typed error.
//! No terminal, AST matcher, or target resolver lives here.
//!
//! Cataloged admission is caller-namespace-agnostic once the probed
//! (owner, method, arity) resolves to a same-module `StaticBoxMethod`
//! declaration; probes that cannot resolve that declaration (builtin owners
//! such as `Math`, the `"<source-owned>"` me-call probe) stay `Unavailable`
//! and keep their sibling lanes.

use std::fmt;

use crate::mir::builder::callable_declaration_catalog::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawSourceTransportPortV1,
};
use crate::mir::builder::recursive_child_lowering::{
    RawInvocationChildPortV1, RawLegacyChildLoweringPortV1,
};
use crate::mir::callable_result_representation::{
    StaticCallResultPublicationOwnerTakeErrorV1, StaticCallResultPublicationTakeV1,
    StaticCallResultTargetOnlyV1, VerifiedStaticCallResultPublicationHandoffV1,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;

#[derive(Debug, PartialEq, Eq)]
enum StaticResultPublicationSourceClassV1 {
    Unavailable,
    Cataloged {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum StaticResultPublicationIngressV1 {
    Unavailable,
    NoExactStaticTarget,
    TargetOnly(StaticCallResultTargetOnlyV1),
    Selected(VerifiedStaticCallResultPublicationHandoffV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum StaticResultPublicationIngressErrorV1 {
    SourceContextMissing,
    SourceLocationLost,
    ForeignLineage,
    OwnerUnavailable,
    DeclarationCatalogUnavailable,
    HandoffTake(StaticCallResultPublicationOwnerTakeErrorV1),
}

impl fmt::Display for StaticResultPublicationIngressErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceContextMissing => {
                write!(
                    formatter,
                    "[freeze:contract][static-result-ingress/source-context-missing]"
                )
            }
            Self::SourceLocationLost => {
                write!(
                    formatter,
                    "[freeze:contract][static-result-ingress/source-location-lost]"
                )
            }
            Self::ForeignLineage => {
                write!(
                    formatter,
                    "[freeze:contract][static-result-ingress/foreign-lineage]"
                )
            }
            Self::OwnerUnavailable => {
                write!(
                    formatter,
                    "[freeze:contract][static-result-ingress/owner-unavailable]"
                )
            }
            Self::DeclarationCatalogUnavailable => write!(
                formatter,
                "[freeze:contract][static-result-ingress/declaration-catalog-unavailable]"
            ),
            Self::HandoffTake(error) => {
                write!(
                    formatter,
                    "[freeze:contract][static-result-ingress/take] {error:?}"
                )
            }
        }
    }
}

pub(in crate::mir::builder) trait StaticResultPublicationIngressPortV1 {
    /// Classify and, only for an exact Cataloged source site, consume one
    /// existing publication handoff.  A non-`StaticBoxMethod` Cataloged
    /// caller is admitted only when `declarations` resolves the probed
    /// (owner, method, argument_count) to a same-module `StaticBoxMethod`
    /// declaration; `declarations == None` never upgrades such a caller and
    /// is inspected as an error only after the source context has already
    /// selected the Cataloged state — it is never a wildcard that turns
    /// source loss into `Unavailable`.
    fn take_static_result_publication_ingress_v1(
        &mut self,
        declarations: Option<&VerifiedSameModuleCallableDeclarationCatalogV1>,
        owner: &str,
        method: &str,
        argument_count: usize,
    ) -> Result<StaticResultPublicationIngressV1, StaticResultPublicationIngressErrorV1>;
}

fn classify_source_context_v1(
    source: Option<&RawInvocationSourceContextV1>,
    source_backed: bool,
    declarations: Option<&VerifiedSameModuleCallableDeclarationCatalogV1>,
    owner: &str,
    method: &str,
    argument_count: usize,
) -> Result<StaticResultPublicationSourceClassV1, StaticResultPublicationIngressErrorV1> {
    let Some(source) = source else {
        if source_backed {
            return Err(StaticResultPublicationIngressErrorV1::SourceContextMissing);
        }
        return Ok(StaticResultPublicationSourceClassV1::Unavailable);
    };
    match source {
        RawInvocationSourceContextV1::UnlocatedCompatibility { .. } => {
            if source_backed {
                Err(StaticResultPublicationIngressErrorV1::SourceLocationLost)
            } else {
                Ok(StaticResultPublicationSourceClassV1::Unavailable)
            }
        }
        RawInvocationSourceContextV1::Located {
            root:
                RawInvocationRootLineageV1::Main(_)
                | RawInvocationRootLineageV1::ScriptRoot
                | RawInvocationRootLineageV1::TopLevel(_)
                | RawInvocationRootLineageV1::InstanceConstructor(_)
                | RawInvocationRootLineageV1::NestedBoxMethod { .. },
            ..
        } => {
            if source_backed {
                Err(StaticResultPublicationIngressErrorV1::ForeignLineage)
            } else {
                Ok(StaticResultPublicationSourceClassV1::Unavailable)
            }
        }
        RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::Cataloged(caller),
            site,
            ..
        } if caller.namespace() == SameModuleCallableNamespaceV1::StaticBoxMethod
            || declarations.is_some_and(|catalog| {
                catalog
                    .declaration_for(
                        SameModuleCallableNamespaceV1::StaticBoxMethod,
                        owner,
                        method,
                        argument_count,
                    )
                    .is_some()
            }) =>
        {
            Ok(StaticResultPublicationSourceClassV1::Cataloged {
                caller: caller.clone(),
                site: SourceExprSiteV1::from_node(site.clone()),
            })
        }
        RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::Cataloged(_),
            ..
        } => Ok(StaticResultPublicationSourceClassV1::Unavailable),
    }
}

fn take_cataloged_publication_v1(
    port: &mut RawInvocationChildPortV1<'_, '_>,
    source: StaticResultPublicationSourceClassV1,
    declarations: Option<&VerifiedSameModuleCallableDeclarationCatalogV1>,
    _owner: &str,
    _method: &str,
    _argument_count: usize,
) -> Result<StaticResultPublicationIngressV1, StaticResultPublicationIngressErrorV1> {
    let StaticResultPublicationSourceClassV1::Cataloged { caller, site } = source else {
        return Ok(StaticResultPublicationIngressV1::Unavailable);
    };
    let declarations =
        declarations.ok_or(StaticResultPublicationIngressErrorV1::DeclarationCatalogUnavailable)?;
    let decision = port
        .module_port
        .take_static_result_publication_handoff(declarations, &caller, &site)
        .map_err(|error| match error {
            StaticCallResultPublicationOwnerTakeErrorV1::OwnerUnavailable => {
                StaticResultPublicationIngressErrorV1::OwnerUnavailable
            }
            other => StaticResultPublicationIngressErrorV1::HandoffTake(other),
        })?;
    Ok(match decision {
        StaticCallResultPublicationTakeV1::NoExactStaticTarget => {
            StaticResultPublicationIngressV1::NoExactStaticTarget
        }
        StaticCallResultPublicationTakeV1::TargetOnly(target) => {
            StaticResultPublicationIngressV1::TargetOnly(target)
        }
        StaticCallResultPublicationTakeV1::Selected(handoff) => {
            StaticResultPublicationIngressV1::Selected(handoff)
        }
    })
}

impl StaticResultPublicationIngressPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn take_static_result_publication_ingress_v1(
        &mut self,
        declarations: Option<&VerifiedSameModuleCallableDeclarationCatalogV1>,
        owner: &str,
        method: &str,
        argument_count: usize,
    ) -> Result<StaticResultPublicationIngressV1, StaticResultPublicationIngressErrorV1> {
        let source = classify_source_context_v1(
            self.current_source_context_v1().as_ref(),
            self.callable_ledger.is_some(),
            declarations,
            owner,
            method,
            argument_count,
        )?;
        match source {
            StaticResultPublicationSourceClassV1::Unavailable => {
                Ok(StaticResultPublicationIngressV1::Unavailable)
            }
            cataloged => take_cataloged_publication_v1(
                self,
                cataloged,
                declarations,
                owner,
                method,
                argument_count,
            ),
        }
    }
}

impl StaticResultPublicationIngressPortV1 for RawLegacyChildLoweringPortV1 {
    fn take_static_result_publication_ingress_v1(
        &mut self,
        _declarations: Option<&VerifiedSameModuleCallableDeclarationCatalogV1>,
        _owner: &str,
        _method: &str,
        _argument_count: usize,
    ) -> Result<StaticResultPublicationIngressV1, StaticResultPublicationIngressErrorV1> {
        Ok(StaticResultPublicationIngressV1::Unavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::{CanonicalSameModuleCallableKeyV1, RawSourceLocatorV1};
    use crate::mir::resolved_semantics::{SourceNodeSiteV1, SourcePathV1};

    fn site() -> SourceNodeSiteV1 {
        SourcePathV1::function_body().node()
    }

    fn caller() -> CanonicalSameModuleCallableKeyV1 {
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Caller", "run", 0)
    }

    fn instance_caller() -> CanonicalSameModuleCallableKeyV1 {
        CanonicalSameModuleCallableKeyV1::test_instance_box_method("Caller", "run", 0)
    }

    #[test]
    fn state_vocabulary_keeps_unavailable_and_no_exact_target_distinct() {
        assert_ne!(
            StaticResultPublicationIngressV1::Unavailable,
            StaticResultPublicationIngressV1::NoExactStaticTarget
        );
    }

    #[test]
    fn source_loss_has_a_typed_error() {
        assert_eq!(
            StaticResultPublicationIngressErrorV1::SourceLocationLost.to_string(),
            "[freeze:contract][static-result-ingress/source-location-lost]"
        );
    }

    #[test]
    fn source_classification_enumerates_all_ingress_boundaries() {
        let absent = None;
        assert_eq!(
            classify_source_context_v1(None, false, absent, "Owner", "m", 0).unwrap(),
            StaticResultPublicationSourceClassV1::Unavailable
        );
        let compatibility = RawInvocationSourceContextV1::UnlocatedCompatibility {
            reason: super::super::raw_invocation_source_transport::RawUnlocatedPortalV1::CallObject,
            expected_lineage: None,
        };
        assert_eq!(
            classify_source_context_v1(Some(&compatibility), false, absent, "Owner", "m", 0)
                .unwrap(),
            StaticResultPublicationSourceClassV1::Unavailable
        );
        let lost = RawInvocationSourceContextV1::UnlocatedCompatibility {
            reason: super::super::raw_invocation_source_transport::RawUnlocatedPortalV1::CallObject,
            expected_lineage: Some(RawInvocationRootLineageV1::Cataloged(caller())),
        };
        assert_eq!(
            classify_source_context_v1(Some(&lost), true, absent, "Owner", "m", 0),
            Err(StaticResultPublicationIngressErrorV1::SourceLocationLost)
        );
        let located = RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::Cataloged(caller()),
            site: site(),
            body_kind: None,
        };
        assert!(matches!(
            classify_source_context_v1(Some(&located), true, absent, "Owner", "m", 0).unwrap(),
            StaticResultPublicationSourceClassV1::Cataloged { .. }
        ));
        let foreign = RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::Main(RawSourceLocatorV1::for_test(
                0,
                "Main",
                "main",
                "Main.main/0",
                0,
            )),
            site: site(),
            body_kind: None,
        };
        assert_eq!(
            classify_source_context_v1(Some(&foreign), false, absent, "Owner", "m", 0).unwrap(),
            StaticResultPublicationSourceClassV1::Unavailable
        );
        assert_eq!(
            classify_source_context_v1(None, true, absent, "Owner", "m", 0),
            Err(StaticResultPublicationIngressErrorV1::SourceContextMissing)
        );
        assert_eq!(
            classify_source_context_v1(Some(&foreign), true, absent, "Owner", "m", 0),
            Err(StaticResultPublicationIngressErrorV1::ForeignLineage)
        );
    }

    fn json_line_declarations() -> VerifiedSameModuleCallableDeclarationCatalogV1 {
        let root = crate::parser::NyashParser::parse_from_string(
            "static box JsonLine { stringField(a, b) { return \"x\" } }",
        )
        .expect("fixture program must parse");
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
            .expect("fixture declaration catalog must seal")
    }

    fn instance_caller_source() -> RawInvocationSourceContextV1 {
        RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::Cataloged(instance_caller()),
            site: site(),
            body_kind: None,
        }
    }

    #[test]
    fn instance_caller_is_admitted_for_declaration_resolved_static_target() {
        let declarations = json_line_declarations();
        let located = instance_caller_source();

        assert!(
            matches!(
                classify_source_context_v1(
                    Some(&located),
                    true,
                    Some(&declarations),
                    "JsonLine",
                    "stringField",
                    2,
                )
                .unwrap(),
                StaticResultPublicationSourceClassV1::Cataloged { .. }
            ),
            "a qualified same-module static call inside an instance method must reach the publication owner"
        );
    }

    #[test]
    fn instance_caller_declines_non_declaration_targets() {
        let declarations = json_line_declarations();
        let located = instance_caller_source();

        assert_eq!(
            classify_source_context_v1(
                Some(&located),
                true,
                Some(&declarations),
                "Math",
                "floor",
                1,
            )
            .unwrap(),
            StaticResultPublicationSourceClassV1::Unavailable,
            "builtin owners stay on the compatibility lane"
        );
        assert_eq!(
            classify_source_context_v1(
                Some(&located),
                true,
                Some(&declarations),
                "JsonLine",
                "undeclared",
                0,
            )
            .unwrap(),
            StaticResultPublicationSourceClassV1::Unavailable,
            "undeclared same-name methods keep the retired-fallback terminal"
        );
    }

    #[test]
    fn me_call_probe_keeps_instance_callers_outside_static_ingress() {
        let declarations = json_line_declarations();
        let located = instance_caller_source();

        assert_eq!(
            classify_source_context_v1(
                Some(&located),
                true,
                Some(&declarations),
                "<source-owned>",
                "stringField",
                2,
            )
            .unwrap(),
            StaticResultPublicationSourceClassV1::Unavailable,
            "the me.method probe never resolves <source-owned>, so the DeclaredInstance sibling keeps the site"
        );
    }

    #[test]
    fn instance_caller_declines_when_declarations_are_unavailable() {
        let located = instance_caller_source();

        assert_eq!(
            classify_source_context_v1(Some(&located), true, None, "JsonLine", "stringField", 2,)
                .unwrap(),
            StaticResultPublicationSourceClassV1::Unavailable,
            "an unresolvable target stays outside this ingress rather than guessing"
        );
    }
}
