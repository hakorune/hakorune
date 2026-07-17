//! Exact catalog caller/body/site co-seal for one source `MethodCall`.

use crate::ast::ASTNode;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1,
    VerifiedSameModuleCallableDeclarationV1,
};
use crate::mir::resolved_semantics::{
    project_source_body_node_v1, ProjectedSourceNodeV1, SourceExprSiteV1, SourceNodeSiteV1,
    SourcePathSegmentV1,
};

use super::SourceMethodCallSiteErrorV1;

#[derive(Debug)]
pub(crate) struct VerifiedSourceMethodCallSiteV1<'catalog> {
    caller: &'catalog CanonicalSameModuleCallableKeyV1,
    declaration: &'catalog VerifiedSameModuleCallableDeclarationV1,
    site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    expression: &'catalog ASTNode,
    receiver: &'catalog ASTNode,
    method: &'catalog str,
    arguments: &'catalog [ASTNode],
    arity: u32,
}

impl<'catalog> VerifiedSourceMethodCallSiteV1<'catalog> {
    pub(crate) fn verify(
        catalog: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    ) -> Result<Self, SourceMethodCallSiteErrorV1> {
        let Some(declaration) = catalog.declaration(caller) else {
            return Err(SourceMethodCallSiteErrorV1::CallerOutsideCatalog {
                caller: caller.clone(),
            });
        };
        if crosses_nested_callable_boundary(site.node()) {
            return Err(
                SourceMethodCallSiteErrorV1::SiteCrossesNestedCallableBoundary {
                    caller: declaration.key().clone(),
                    site,
                },
            );
        }
        let Some(ProjectedSourceNodeV1::Node(expression)) =
            project_source_body_node_v1(declaration.body(), site.node())
        else {
            return Err(SourceMethodCallSiteErrorV1::SiteOutsideCallerBody {
                caller: caller.clone(),
                site,
            });
        };
        let ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } = expression
        else {
            return Err(SourceMethodCallSiteErrorV1::MethodCallRequired {
                caller: caller.clone(),
                site,
            });
        };
        let arity = checked_method_call_arity(arguments.len()).map_err(|()| {
            SourceMethodCallSiteErrorV1::ArityOverflow {
                caller: caller.clone(),
                site: site.clone(),
                method: method.clone().into_boxed_str(),
            }
        })?;
        let mut receiver_segments = site.node().segments().to_vec();
        receiver_segments.push(SourcePathSegmentV1::Receiver);
        let receiver_site =
            SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(receiver_segments));
        Ok(Self {
            caller: declaration.key(),
            declaration,
            site,
            receiver_site,
            expression,
            receiver: object,
            method,
            arguments,
            arity,
        })
    }

    pub(crate) const fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        self.caller
    }

    pub(crate) const fn declaration(&self) -> &VerifiedSameModuleCallableDeclarationV1 {
        self.declaration
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    pub(crate) const fn expression(&self) -> &ASTNode {
        self.expression
    }

    pub(crate) fn receiver(&self) -> &ASTNode {
        self.receiver
    }

    pub(crate) fn method(&self) -> &str {
        self.method
    }

    pub(crate) fn arguments(&self) -> &[ASTNode] {
        self.arguments
    }

    pub(crate) const fn arity(&self) -> u32 {
        self.arity
    }
}

fn crosses_nested_callable_boundary(site: &SourceNodeSiteV1) -> bool {
    site.segments().iter().any(|segment| {
        matches!(
            segment,
            SourcePathSegmentV1::FunctionBody
                | SourcePathSegmentV1::LambdaBodyRoot
                | SourcePathSegmentV1::LambdaBody(_)
        )
    })
}

fn checked_method_call_arity(arity: usize) -> Result<u32, ()> {
    u32::try_from(arity).map_err(|_| ())
}

#[cfg(test)]
pub(super) fn checked_method_call_arity_for_test(arity: usize) -> Result<u32, ()> {
    checked_method_call_arity(arity)
}
