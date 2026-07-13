//! SA3 transport seam from sealed lexical identity to Lower value materialization.
//!
//! SA3-A keeps this state disconnected from production declarations. The
//! atomic SA3-B cutover installs one sealed function product and claims each
//! exact declaration site before publishing its current MIR value.

#![allow(dead_code)] // SA3-A transport lands behavior-neutral before SA3-B.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
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
    pending_declarations: VecDeque<SourceBindingSiteV1>,
}

impl ResolvedBindingLoweringStateV1 {
    pub(in crate::mir::builder) fn install(
        &mut self,
        product: Arc<VerifiedResolvedFunctionV1>,
    ) -> Result<(), String> {
        if self.product.is_some()
            || !self.values.is_empty()
            || !self.claimed_declarations.is_empty()
            || !self.pending_declarations.is_empty()
        {
            return Err("[freeze:contract][resolved_binding/install_nonempty]".to_string());
        }
        self.pending_declarations = product.declaration_order().iter().cloned().collect();
        self.product = Some(product);
        Ok(())
    }

    pub(in crate::mir::builder) fn claim_next_declaration(
        &mut self,
        expected_kind: BindingKindV1,
        expected_name: &str,
    ) -> Result<(SourceBindingSiteV1, BindingRefV1), String> {
        let site = self.pending_declarations.pop_front().ok_or_else(|| {
            "[freeze:contract][resolved_binding/declaration_order_exhausted]".to_string()
        })?;
        let binding = self.claim_declaration(&site, expected_name)?;
        let actual_kind = self
            .product
            .as_ref()
            .expect("claim requires product")
            .binding(binding)
            .expect("sealed declaration must resolve")
            .kind();
        if actual_kind != expected_kind {
            return Err(format!(
                "[freeze:contract][resolved_binding/kind_mismatch] expected={expected_kind:?} actual={actual_kind:?} site={site:?}"
            ));
        }
        Ok((site, binding))
    }

    pub(in crate::mir::builder) fn finish_claims(&self) -> Result<(), String> {
        if let Some(site) = self.pending_declarations.front() {
            return Err(format!(
                "[freeze:contract][resolved_binding/declaration_unclaimed] first={site:?} remaining={}",
                self.pending_declarations.len()
            ));
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn claim_declaration(
        &mut self,
        site: &SourceBindingSiteV1,
        expected_name: &str,
    ) -> Result<BindingRefV1, String> {
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
        if !self.claimed_declarations.insert(site.clone()) {
            return Err(format!(
                "[freeze:contract][resolved_binding/declaration_reclaimed] site={site:?}"
            ));
        }
        Ok(binding)
    }

    pub(in crate::mir::builder) fn publish_value(
        &mut self,
        binding: BindingRefV1,
        value: ValueId,
    ) -> Result<(), String> {
        let product = self
            .product
            .as_ref()
            .ok_or_else(|| "[freeze:contract][resolved_binding/product_missing]".to_string())?;
        if product.binding(binding).is_none() {
            return Err("[freeze:contract][resolved_binding/foreign_binding]".to_string());
        }
        if self.values.insert(binding.binding(), value).is_some() {
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
