//! SA3 transport seam from sealed lexical identity to Lower value materialization.
//!
//! SA3-A keeps this state disconnected from production declarations. The
//! atomic SA3-B cutover installs one sealed function product and claims each
//! exact declaration site before publishing its current MIR value.

#![allow(dead_code)] // SA3-A transport lands behavior-neutral before SA3-B.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, SourceBindingSiteV1, VerifiedResolvedFunctionV1,
};
use crate::mir::{BindingId, ValueId};

#[derive(Debug, Default)]
pub(in crate::mir) struct ResolvedBindingLoweringStateV1 {
    product: Option<Arc<VerifiedResolvedFunctionV1>>,
    values: BTreeMap<BindingId, ValueId>,
    claimed_declarations: BTreeSet<SourceBindingSiteV1>,
    published_declarations: BTreeSet<SourceBindingSiteV1>,
}

/// One-shot proof that Lower claimed one exact declaration site.
#[derive(Debug)]
pub(in crate::mir::builder) struct ResolvedDeclarationClaimV1 {
    site: SourceBindingSiteV1,
    binding: BindingRefV1,
}

impl ResolvedDeclarationClaimV1 {
    pub(in crate::mir::builder) fn binding_id(&self) -> BindingId {
        self.binding.binding()
    }
}

impl ResolvedBindingLoweringStateV1 {
    pub(in crate::mir::builder) fn install(
        &mut self,
        product: Arc<VerifiedResolvedFunctionV1>,
    ) -> Result<(), String> {
        if self.product.is_some()
            || !self.values.is_empty()
            || !self.claimed_declarations.is_empty()
            || !self.published_declarations.is_empty()
        {
            return Err("[freeze:contract][resolved_binding/install_nonempty]".to_string());
        }
        self.product = Some(product);
        Ok(())
    }

    pub(in crate::mir::builder) fn finish_claims(&self) -> Result<(), String> {
        let product = self
            .product
            .as_ref()
            .ok_or_else(|| "[freeze:contract][resolved_binding/product_missing]".to_string())?;
        let expected = product
            .declaration_sites()
            .cloned()
            .collect::<BTreeSet<_>>();
        if self.claimed_declarations != expected {
            return Err(format!(
                "[freeze:contract][resolved_binding/declaration_claim_set_mismatch] expected={} actual={}",
                expected.len(),
                self.claimed_declarations.len()
            ));
        }
        if self.published_declarations != expected {
            return Err(format!(
                "[freeze:contract][resolved_binding/declaration_publish_set_mismatch] expected={} actual={}",
                expected.len(),
                self.published_declarations.len()
            ));
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn claim_declaration(
        &mut self,
        site: &SourceBindingSiteV1,
        expected_kind: BindingKindV1,
        expected_name: &str,
    ) -> Result<ResolvedDeclarationClaimV1, String> {
        let product = self
            .product
            .as_ref()
            .ok_or_else(|| "[freeze:contract][resolved_binding/product_missing]".to_string())?;
        let binding = product.declaration_binding(site).ok_or_else(|| {
            format!("[freeze:contract][resolved_binding/declaration_missing] site={site:?}")
        })?;
        let record = product.binding(binding).ok_or_else(|| {
            "[freeze:contract][resolved_binding/declaration_dangling]".to_string()
        })?;
        if record.diagnostic_name() != expected_name {
            return Err(format!(
                "[freeze:contract][resolved_binding/name_mismatch] expected={} actual={}",
                expected_name,
                record.diagnostic_name()
            ));
        }
        if record.kind() != expected_kind {
            return Err(format!(
                "[freeze:contract][resolved_binding/kind_mismatch] expected={expected_kind:?} actual={:?} site={site:?}",
                record.kind()
            ));
        }
        if !self.claimed_declarations.insert(site.clone()) {
            return Err(format!(
                "[freeze:contract][resolved_binding/declaration_reclaimed] site={site:?}"
            ));
        }
        Ok(ResolvedDeclarationClaimV1 {
            site: site.clone(),
            binding,
        })
    }

    pub(in crate::mir::builder) fn publish_declared_value(
        &mut self,
        claim: ResolvedDeclarationClaimV1,
        value: ValueId,
    ) -> Result<(), String> {
        let product = self
            .product
            .as_ref()
            .ok_or_else(|| "[freeze:contract][resolved_binding/product_missing]".to_string())?;
        if product.binding(claim.binding).is_none() {
            return Err("[freeze:contract][resolved_binding/foreign_binding]".to_string());
        }
        if !self.published_declarations.insert(claim.site) {
            return Err("[freeze:contract][resolved_binding/declaration_republished]".to_string());
        }
        if self.values.insert(claim.binding.binding(), value).is_some() {
            return Err("[freeze:contract][resolved_binding/value_republished]".to_string());
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn value(&self, binding: BindingRefV1) -> Option<ValueId> {
        self.product
            .as_ref()
            .filter(|product| product.binding(binding).is_some())?;
        self.values.get(&binding.binding()).copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
    use crate::mir::resolved_semantics::{
        FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, SourceNodeSiteV1,
        SourcePathSegmentV1, SourceStmtSiteV1,
    };

    use super::*;

    fn product() -> Arc<VerifiedResolvedFunctionV1> {
        let function = ASTNode::FunctionDeclaration {
            name: "fixture".into(),
            params: vec!["arg".into()],
            param_decls: Vec::new(),
            return_type_name: None,
            body: vec![ASTNode::Local {
                variables: vec!["x".into()],
                initial_values: vec![Some(Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            }],
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: false,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        };
        let view = FunctionSyntaxViewV1::from_ast(&function).unwrap();
        FunctionSemanticResolverSessionV1::new(0)
            .unwrap()
            .resolve(view)
            .unwrap()
    }

    fn local_site() -> SourceBindingSiteV1 {
        SourceBindingSiteV1::Local {
            statement: SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
                SourcePathSegmentV1::Body(0),
            ])),
            ordinal: 0,
        }
    }

    fn claim_and_publish(
        state: &mut ResolvedBindingLoweringStateV1,
        site: &SourceBindingSiteV1,
        kind: BindingKindV1,
        name: &str,
        value: u32,
    ) {
        let claim = state.claim_declaration(site, kind, name).unwrap();
        state
            .publish_declared_value(claim, ValueId::new(value))
            .unwrap();
    }

    #[test]
    fn exact_claims_must_all_be_consumed_by_value_publication() {
        let mut state = ResolvedBindingLoweringStateV1::default();
        state.install(product()).unwrap();
        claim_and_publish(
            &mut state,
            &SourceBindingSiteV1::Receiver,
            BindingKindV1::Receiver,
            "me",
            1,
        );
        claim_and_publish(
            &mut state,
            &SourceBindingSiteV1::Parameter { index: 0 },
            BindingKindV1::Parameter { index: 0 },
            "arg",
            2,
        );
        claim_and_publish(
            &mut state,
            &local_site(),
            BindingKindV1::Local { ordinal: 0 },
            "x",
            3,
        );
        state.finish_claims().unwrap();
    }

    #[test]
    fn claimed_but_unpublished_declaration_fails_finish() {
        let mut state = ResolvedBindingLoweringStateV1::default();
        state.install(product()).unwrap();
        let _claim = state
            .claim_declaration(
                &SourceBindingSiteV1::Receiver,
                BindingKindV1::Receiver,
                "me",
            )
            .unwrap();
        assert!(state.finish_claims().is_err());
    }

    #[test]
    fn wrong_exact_site_cannot_be_recovered_by_matching_name_and_kind() {
        let mut state = ResolvedBindingLoweringStateV1::default();
        state.install(product()).unwrap();
        let wrong_site = SourceBindingSiteV1::Local {
            statement: SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
                SourcePathSegmentV1::Body(99),
            ])),
            ordinal: 0,
        };
        assert!(state
            .claim_declaration(&wrong_site, BindingKindV1::Local { ordinal: 0 }, "x",)
            .is_err());
    }
}
