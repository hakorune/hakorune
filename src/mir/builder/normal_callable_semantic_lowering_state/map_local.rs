//! Exact initializer/alias correspondence for opaque local placement.
use super::*;
use crate::mir::builder::stmts::variable_stmt::LocalValuePlacement;
use crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1;
use crate::mir::resolved_semantics::{OwnedExprSiteV1, ResolvedInitializerRelationV1};

impl CallableSemanticLoweringState {
    pub(in crate::mir::builder) fn map_initializer(
        &self,
        site: &SourceNodeSiteV1,
    ) -> Result<ResolvedInitializerRelationV1, String> {
        let mut rows = self
            .initializers
            .values()
            .filter(|row| row.initializer_site().is_some_and(|s| s.node() == site));
        let row = rows
            .next()
            .ok_or_else(|| freeze("map-initializer-missing"))?;
        if rows.next().is_some() {
            return Err(freeze("map-initializer-duplicate"));
        }
        Ok(row.clone())
    }
    pub(in crate::mir::builder) fn local_placement(
        &self,
        statement: &SourceNodeSiteV1,
        ordinal: usize,
        value: ValueId,
        ledger: &OrdinaryNewClaimLedgerV1,
    ) -> Result<LocalValuePlacement, String> {
        let binding = self
            .locals
            .get(statement)
            .and_then(|rows| rows.get(ordinal))
            .ok_or_else(|| freeze("placement-local-missing"))?;
        let mut rows = self
            .initializers
            .values()
            .filter(|row| row.binding() == *binding);
        let relation = rows
            .next()
            .ok_or_else(|| freeze("placement-initializer-missing"))?;
        if rows.next().is_some() {
            return Err(freeze("placement-initializer-duplicate"));
        }
        let Some(site) = relation.initializer_site() else {
            return Ok(LocalValuePlacement::Copy);
        };
        let owned = OwnedExprSiteV1::new(self.owner, site.clone());
        let reuse = if ledger.has_map_source(&owned) {
            if !ledger.map_initializer_matches(&owned, *binding, value) {
                return Err(freeze("placement-map-result-drift"));
            }
            true
        } else if let Some(reference) = self.variables.get(site.node()) {
            if !self.consumed_variables.contains(site.node())
                || self.values.get(reference) != Some(&value)
            {
                return Err(freeze("placement-alias-unconsumed-or-value-drift"));
            }
            self.map_alias_origin(*reference, value, ledger)?
        } else {
            false
        };
        if reuse {
            let annotation = relation.declared_type_name();
            if crate::mir::type_contracts::local_slot::is_exact_numeric_local_type(annotation)
                || annotation
                    .map(crate::typed_array_contract_spec::parse_annotation)
                    .transpose()?
                    .flatten()
                    .is_some()
            {
                return Err(freeze("placement-map-annotation"));
            }
            Ok(LocalValuePlacement::ReuseInitializer)
        } else {
            Ok(LocalValuePlacement::Copy)
        }
    }
    fn map_alias_origin(
        &self,
        mut binding: BindingRefV1,
        value: ValueId,
        ledger: &OrdinaryNewClaimLedgerV1,
    ) -> Result<bool, String> {
        let mut seen = BTreeSet::new();
        loop {
            if !seen.insert(binding) {
                return Err(freeze("placement-alias-cycle"));
            }
            if self.values.get(&binding) != Some(&value) {
                return Ok(false);
            }
            if ledger.is_installed_map_binding(binding, value) {
                return Ok(true);
            }
            let mut rows = self
                .initializers
                .values()
                .filter(|row| row.binding() == binding);
            let Some(row) = rows.next() else {
                return Ok(false);
            };
            if rows.next().is_some() {
                return Err(freeze("placement-alias-duplicate"));
            }
            let Some(site) = row.initializer_site() else {
                return Ok(false);
            };
            let Some(next) = self.variables.get(site.node()) else {
                return Ok(false);
            };
            if !self.consumed_variables.contains(site.node()) {
                return Err(freeze("placement-alias-unconsumed"));
            }
            binding = *next;
        }
    }
}

pub(super) fn retain_initializers(
    input: ResolvedFunctionLoweringInputV1<'_>,
    locals: &BTreeMap<SourceNodeSiteV1, Box<[BindingRefV1]>>,
) -> Result<
    BTreeMap<crate::mir::resolved_semantics::SourceBindingSiteV1, ResolvedInitializerRelationV1>,
    String,
> {
    let mut result = BTreeMap::new();
    for row in input.function().expression_source().initializers() {
        if let crate::mir::resolved_semantics::SourceBindingSiteV1::Local { statement, ordinal } =
            row.declaration_site()
        {
            if locals
                .get(statement.node())
                .and_then(|rows| rows.get(*ordinal as usize))
                != Some(&row.binding())
            {
                return Err(freeze("initializer-declaration-drift"));
            }
            if result
                .insert(row.declaration_site().clone(), row.clone())
                .is_some()
            {
                return Err(freeze("initializer-declaration-duplicate"));
            }
        }
    }
    Ok(result)
}
