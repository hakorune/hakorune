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
    /// source loss into `Unavailable`.  A located `Main` root admits the
    /// same way through its sealed locator: the probed
    /// (box_name, method_name, arity) must resolve to a same-module
    /// `StaticBoxMethod` declaration and the caller key comes from that
    /// declaration, never from the locator symbol.  A located `TopLevel`
    /// root projects its sealed occurrence key to `free_function(name,
    /// arity)` and admits only when the catalog holds that declaration row
    /// AND the probed (owner, method, argument_count) resolves to a
    /// `StaticBoxMethod` declaration — the same caller-membership +
    /// target-probe discipline.
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
            root: RawInvocationRootLineageV1::Main(locator),
            site,
            ..
        } => {
            if !source_backed {
                return Ok(StaticResultPublicationSourceClassV1::Unavailable);
            }
            let Some(declarations) = declarations else {
                return Err(StaticResultPublicationIngressErrorV1::DeclarationCatalogUnavailable);
            };
            let Some(declaration) = declarations.declaration_for(
                SameModuleCallableNamespaceV1::StaticBoxMethod,
                locator.box_name(),
                locator.method_name(),
                locator.arity(),
            ) else {
                return Err(StaticResultPublicationIngressErrorV1::ForeignLineage);
            };
            Ok(StaticResultPublicationSourceClassV1::Cataloged {
                caller: declaration.key().clone(),
                site: SourceExprSiteV1::from_node(site.clone()),
            })
        }
        RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::TopLevel(key),
            site,
            ..
        } => {
            if !source_backed {
                return Ok(StaticResultPublicationSourceClassV1::Unavailable);
            }
            let Some(declarations) = declarations else {
                return Err(StaticResultPublicationIngressErrorV1::DeclarationCatalogUnavailable);
            };
            let Ok(arity) = u32::try_from(key.declared_arity()) else {
                return Err(StaticResultPublicationIngressErrorV1::ForeignLineage);
            };
            let caller =
                CanonicalSameModuleCallableKeyV1::free_function(key.declared_name(), arity);
            let Some(declaration) = declarations.declaration(&caller) else {
                return Err(StaticResultPublicationIngressErrorV1::ForeignLineage);
            };
            if declarations
                .declaration_for(
                    SameModuleCallableNamespaceV1::StaticBoxMethod,
                    owner,
                    method,
                    argument_count,
                )
                .is_some()
            {
                Ok(StaticResultPublicationSourceClassV1::Cataloged {
                    caller: declaration.key().clone(),
                    site: SourceExprSiteV1::from_node(site.clone()),
                })
            } else {
                Ok(StaticResultPublicationSourceClassV1::Unavailable)
            }
        }
        RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::InstanceConstructor(key),
            site,
            ..
        } => {
            if !source_backed {
                return Ok(StaticResultPublicationSourceClassV1::Unavailable);
            }
            let Some(declarations) = declarations else {
                return Err(StaticResultPublicationIngressErrorV1::DeclarationCatalogUnavailable);
            };
            let Some(birth_key) = key.published_birth_key() else {
                return Err(StaticResultPublicationIngressErrorV1::ForeignLineage);
            };
            let Some(declaration) = declarations.declaration(birth_key) else {
                return Err(StaticResultPublicationIngressErrorV1::ForeignLineage);
            };
            if declarations
                .declaration_for(
                    SameModuleCallableNamespaceV1::StaticBoxMethod,
                    owner,
                    method,
                    argument_count,
                )
                .is_some()
            {
                Ok(StaticResultPublicationSourceClassV1::Cataloged {
                    caller: declaration.key().clone(),
                    site: SourceExprSiteV1::from_node(site.clone()),
                })
            } else {
                Ok(StaticResultPublicationSourceClassV1::Unavailable)
            }
        }
        RawInvocationSourceContextV1::Located {
            root:
                RawInvocationRootLineageV1::ScriptRoot
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
#[path = "static_result_publication_ingress_tests.rs"]
mod tests;
