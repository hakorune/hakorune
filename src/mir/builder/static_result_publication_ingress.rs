//! Effect-free ingress for the source-bound static result publication owner.
//!
//! The ingress keeps four states distinct.  A compatibility port is
//! `Unavailable`, an exact Cataloged site with no row is `Absent`, an owned
//! row is `Selected`, and any source-backed loss or drift is a typed error.
//! No terminal, AST matcher, or target resolver lives here.

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
    VerifiedStaticCallResultPublicationHandoffV1,
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
    Absent,
    Selected(VerifiedStaticCallResultPublicationHandoffV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum StaticResultPublicationIngressErrorV1 {
    SourceContextMissing,
    SourceLocationLost,
    ForeignLineage,
    OwnerUnavailable,
    DeclarationCatalogUnavailable,
    TargetUnavailable,
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
            Self::TargetUnavailable => {
                write!(
                    formatter,
                    "[freeze:contract][static-result-ingress/target-unavailable]"
                )
            }
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
    /// existing publication handoff.  `declarations == None` is inspected
    /// only after the source context has selected the Cataloged state; it is
    /// never a wildcard that turns source loss into `Unavailable`.
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
        } => Ok(StaticResultPublicationSourceClassV1::Cataloged {
            caller: caller.clone(),
            site: SourceExprSiteV1::from_node(site.clone()),
        }),
    }
}

fn take_cataloged_publication_v1(
    port: &mut RawInvocationChildPortV1<'_, '_>,
    source: StaticResultPublicationSourceClassV1,
    declarations: Option<&VerifiedSameModuleCallableDeclarationCatalogV1>,
    owner: &str,
    method: &str,
    argument_count: usize,
) -> Result<StaticResultPublicationIngressV1, StaticResultPublicationIngressErrorV1> {
    let StaticResultPublicationSourceClassV1::Cataloged { caller, site } = source else {
        return Ok(StaticResultPublicationIngressV1::Unavailable);
    };
    let declarations =
        declarations.ok_or(StaticResultPublicationIngressErrorV1::DeclarationCatalogUnavailable)?;
    let target = declarations
        .declaration_for(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            owner,
            method,
            argument_count,
        )
        .map(|declaration| declaration.key().clone())
        .ok_or(StaticResultPublicationIngressErrorV1::TargetUnavailable)?;
    let decision = port
        .module_port
        .take_static_result_publication_handoff(declarations, &caller, &site, &target)
        .map_err(|error| match error {
            StaticCallResultPublicationOwnerTakeErrorV1::OwnerUnavailable => {
                StaticResultPublicationIngressErrorV1::OwnerUnavailable
            }
            other => StaticResultPublicationIngressErrorV1::HandoffTake(other),
        })?;
    Ok(match decision {
        StaticCallResultPublicationTakeV1::Unselected => StaticResultPublicationIngressV1::Absent,
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

    #[test]
    fn state_vocabulary_keeps_unavailable_and_absent_distinct() {
        assert_ne!(
            StaticResultPublicationIngressV1::Unavailable,
            StaticResultPublicationIngressV1::Absent
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
        assert_eq!(
            classify_source_context_v1(None, false).unwrap(),
            StaticResultPublicationSourceClassV1::Unavailable
        );
        let compatibility = RawInvocationSourceContextV1::UnlocatedCompatibility {
            reason: super::super::raw_invocation_source_transport::RawUnlocatedPortalV1::CallObject,
            expected_lineage: None,
        };
        assert_eq!(
            classify_source_context_v1(Some(&compatibility), false).unwrap(),
            StaticResultPublicationSourceClassV1::Unavailable
        );
        let lost = RawInvocationSourceContextV1::UnlocatedCompatibility {
            reason: super::super::raw_invocation_source_transport::RawUnlocatedPortalV1::CallObject,
            expected_lineage: Some(RawInvocationRootLineageV1::Cataloged(caller())),
        };
        assert_eq!(
            classify_source_context_v1(Some(&lost), true),
            Err(StaticResultPublicationIngressErrorV1::SourceLocationLost)
        );
        let located = RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::Cataloged(caller()),
            site: site(),
            body_kind: None,
        };
        assert!(matches!(
            classify_source_context_v1(Some(&located), true).unwrap(),
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
            classify_source_context_v1(Some(&foreign), false).unwrap(),
            StaticResultPublicationSourceClassV1::Unavailable
        );
        assert_eq!(
            classify_source_context_v1(None, true),
            Err(StaticResultPublicationIngressErrorV1::SourceContextMissing)
        );
        assert_eq!(
            classify_source_context_v1(Some(&foreign), true),
            Err(StaticResultPublicationIngressErrorV1::ForeignLineage)
        );
    }
}
