//! Private source acquisition rows issued only by the Script continuation seal.
//! Numeric storage owns no element Homes. Unknown source capability is explicit;
//! neither MIR nor runtime handles can manufacture an empty cleanup prefix.

use crate::mir::resolved_semantics::{
    BindingRefV1, BodyExpressionShapeV1, ResolvedInitializerRelationV1, ResolvedLexicalRefV1,
    ResolvedLiteralSourceV1, ScriptRootSemanticDispositionV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, VerifiedResolvedScriptV1,
    VerifiedScriptRootDemandWindowV1,
};
use crate::typed_array_contract_spec::{parse_annotation, ArrayElementContractSpec};
use std::collections::{BTreeMap, BTreeSet};
#[path = "normal_script_array_root_terminal.rs"]
pub(super) mod root_terminal;
use root_terminal::RootTerminalCoverage;

#[derive(Debug, PartialEq, Eq)]
enum LocalProgress {
    Pending,
    InFlight,
    Completed,
}

#[derive(Debug, PartialEq, Eq)]
enum Cutpoint {
    BeforeAllocation,
    AcquiredBeforeClaim,
    Claimed,
    ChildEvaluating(SourceExprSiteV1),
    ChildReady(SourceExprSiteV1),
    Written(SourceExprSiteV1),
    FaultUnwind,
    LocalCommit,
    CallerExit,
}

#[derive(Debug, PartialEq, Eq)]
struct ArraySourceLifecycle {
    initializer: ResolvedInitializerRelationV1,
    spec: ArrayElementContractSpec,
    cutpoints: Box<[Cutpoint]>,
    // Each row names one actual Home, in release order. Aliases add none.
    caller_homes: Box<[BindingRefV1]>,
    progress: LocalProgress,
}

#[derive(Debug, PartialEq, Eq)]
enum ArraySourceCoverage {
    Available(ArraySourceLifecycle),
    Unavailable(&'static str),
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct ArraySourceLifecycleRows {
    rows: BTreeMap<SourceNodeSiteV1, ArraySourceCoverage>,
    terminal: RootTerminalCoverage,
}

impl ArraySourceLifecycleRows {
    pub(super) fn issue(
        product: &VerifiedResolvedScriptV1,
        window: &VerifiedScriptRootDemandWindowV1,
    ) -> Result<Self, &'static str> {
        let mut result = Self::default();
        let mut homes = Vec::new();
        let mut aliases = BTreeSet::new();
        let mut prefix_available = true;
        let source = &product.core().data().expression_source;
        for (ordinal, entry) in window.entries().iter().enumerate() {
            if product
                .body_shape()
                .statements()
                .iter()
                .any(|row| row.site() == entry.site() && row.is_return())
            {
                result.terminal =
                    RootTerminalCoverage::issue(product, window, ordinal, prefix_available, &homes);
                prefix_available = false;
                continue;
            }
            if matches!(
                entry.semantic(),
                ScriptRootSemanticDispositionV1::Transparent(_)
            ) {
                continue;
            }
            let locals = source
                .initializers()
                .filter(|relation| {
                    matches!(
                        relation.declaration_site(),
                        SourceBindingSiteV1::Local { statement, .. } if statement == entry.site()
                    )
                })
                .collect::<Vec<_>>();
            let [relation] = locals.as_slice() else {
                // Includes reassignments, opaque statements, scope/control boundaries
                // and runtime work: never skip them to reuse a stale initializer.
                prefix_available = false;
                continue;
            };
            if relation.binding().owner() != product.core().data().owner {
                return Err("foreign-local-owner");
            }
            let spec = relation
                .declared_type_name()
                .map(parse_annotation)
                .transpose()
                .map_err(|_| "invalid-array-annotation")?
                .flatten();
            if let Some(spec) = spec {
                let coverage = if !prefix_available {
                    ArraySourceCoverage::Unavailable("caller-prefix-capability")
                } else {
                    match array_cutpoints(product, relation) {
                        Ok(cutpoints) => ArraySourceCoverage::Available(ArraySourceLifecycle {
                            initializer: (*relation).clone(),
                            spec,
                            cutpoints,
                            caller_homes: homes.iter().rev().copied().collect(),
                            progress: LocalProgress::Pending,
                        }),
                        Err(reason) => ArraySourceCoverage::Unavailable(reason),
                    }
                };
                let available = matches!(coverage, ArraySourceCoverage::Available(_));
                if result
                    .rows
                    .insert(entry.site().node().clone(), coverage)
                    .is_some()
                {
                    return Err("duplicate-array-local");
                }
                if available {
                    homes.push(relation.binding());
                    aliases.insert(relation.binding());
                } else {
                    prefix_available = false;
                }
                continue;
            }
            let known = relation.initializer_site().is_some_and(|site| {
                primitive(source.literal(site))
                    || matches!(
                        product.core().data().variable_uses.get(site),
                        Some(ResolvedLexicalRefV1::Local(binding)) if aliases.contains(binding)
                    )
            });
            if !known {
                prefix_available = false;
            } else if let Some(site) = relation.initializer_site() {
                if matches!(product.core().data().variable_uses.get(site),
                    Some(ResolvedLexicalRefV1::Local(binding)) if aliases.contains(binding))
                {
                    aliases.insert(relation.binding());
                }
            }
        }
        if result.rows.is_empty() {
            result.terminal = RootTerminalCoverage::NotSelected;
        }
        Ok(result)
    }

    pub(super) fn consume(
        &mut self,
        relation: &ResolvedInitializerRelationV1,
    ) -> Result<(), String> {
        let spec = relation
            .declared_type_name()
            .map(parse_annotation)
            .transpose()?
            .flatten();
        let SourceBindingSiteV1::Local {
            statement,
            ordinal: 0,
        } = relation.declaration_site()
        else {
            return Err(freeze("local-site"));
        };
        let completed_homes = self
            .rows
            .values()
            .filter_map(|row| match row {
                ArraySourceCoverage::Available(row) if row.progress == LocalProgress::Completed => {
                    Some(row.initializer.binding())
                }
                _ => None,
            })
            .collect::<BTreeSet<_>>();
        let slot = self.rows.get_mut(statement.node());
        let Some(spec) = spec else {
            return if slot.is_none() {
                Ok(())
            } else {
                Err(freeze("annotation-drift"))
            };
        };
        let Some(slot) = slot else {
            return Err(freeze("missing-row"));
        };
        match slot {
            ArraySourceCoverage::Available(row) => {
                if row.initializer != *relation || row.spec != spec {
                    return Err(freeze("source-drift"));
                }
                if row.progress != LocalProgress::Pending {
                    return Err(freeze("duplicate-consume"));
                }
                if row
                    .caller_homes
                    .iter()
                    .any(|binding| !completed_homes.contains(binding))
                {
                    return Err(freeze("prior-home-not-completed"));
                }
                row.progress = LocalProgress::InFlight;
                Ok(())
            }
            ArraySourceCoverage::Unavailable(reason) => Err(format!(
                "[freeze:contract][script-array/source-lifecycle-unavailable] {reason}"
            )),
        }
    }

    pub(super) fn require_root_for(
        &self,
        relation: &ResolvedInitializerRelationV1,
    ) -> Result<(), String> {
        if let SourceBindingSiteV1::Local { statement, .. } = relation.declaration_site() {
            if self.rows.contains_key(statement.node()) {
                return self.require_root();
            }
        }
        Ok(())
    }

    pub(super) fn require_root(&self) -> Result<(), String> {
        if matches!(self.terminal, RootTerminalCoverage::NotSelected) {
            return if self.rows.is_empty() {
                Ok(())
            } else {
                Err(freeze("root-selection-drift"))
            };
        }
        self.terminal.require().map(|_| ()).map_err(freeze)
    }

    pub(super) fn complete(
        &mut self,
        relation: &ResolvedInitializerRelationV1,
    ) -> Result<(), String> {
        let SourceBindingSiteV1::Local { statement, .. } = relation.declaration_site() else {
            return Err(freeze("local-site"));
        };
        let Some(coverage) = self.rows.get_mut(statement.node()) else {
            return Ok(());
        };
        let ArraySourceCoverage::Available(row) = coverage else {
            return Err(freeze("incomplete-local"));
        };
        if row.initializer != *relation || row.progress != LocalProgress::InFlight {
            return Err(freeze("incomplete-local"));
        }
        row.progress = LocalProgress::Completed;
        Ok(())
    }

    pub(super) fn element_sites(
        &self,
        relation: &ResolvedInitializerRelationV1,
    ) -> Result<Option<(ArrayElementContractSpec, Vec<SourceExprSiteV1>)>, String> {
        let SourceBindingSiteV1::Local { statement, .. } = relation.declaration_site() else {
            return Err(freeze("local-site"));
        };
        match self.rows.get(statement.node()) {
            None => Ok(None),
            Some(ArraySourceCoverage::Available(row)) if row.initializer == *relation => {
                Ok(Some((
                    row.spec,
                    row.cutpoints
                        .iter()
                        .filter_map(|cut| match cut {
                            Cutpoint::Written(site) => Some(site.clone()),
                            _ => None,
                        })
                        .collect(),
                )))
            }
            _ => Err(freeze("emission-source-drift")),
        }
    }

    pub(super) fn terminal(&self) -> Result<Option<&root_terminal::RootTerminal>, String> {
        match &self.terminal {
            RootTerminalCoverage::NotSelected => Ok(None),
            terminal => terminal.require().map(Some).map_err(freeze),
        }
    }

    pub(super) fn bindings(&self) -> Result<Vec<BindingRefV1>, String> {
        self.finish_root()?;
        self.rows
            .values()
            .map(|row| match row {
                ArraySourceCoverage::Available(row) => Ok(row.initializer.binding()),
                _ => Err(freeze("incomplete-local")),
            })
            .collect()
    }

    pub(super) fn finish_root(&self) -> Result<(), String> {
        self.require_root()?;
        self.finish()?;
        if matches!(self.terminal, RootTerminalCoverage::NotSelected) {
            return Ok(());
        }
        let terminal = self.terminal.require().map_err(freeze)?;
        let homes = self
            .rows
            .values()
            .rev()
            .map(|row| match row {
                ArraySourceCoverage::Available(row) => Ok(row.initializer.binding()),
                _ => Err(freeze("incomplete-local")),
            })
            .collect::<Result<Vec<_>, _>>()?;
        if terminal.homes() != homes.as_slice() {
            return Err(freeze("terminal-home-drift"));
        }
        Ok(())
    }

    fn finish(&self) -> Result<(), String> {
        if self
            .rows
            .values()
            .any(|row| !matches!(row, ArraySourceCoverage::Available(row) if row.progress == LocalProgress::Completed))
        {
            return Err(freeze("unconsumed-row"));
        }
        Ok(())
    }
}

fn primitive(literal: Option<&ResolvedLiteralSourceV1>) -> bool {
    matches!(
        literal,
        Some(
            ResolvedLiteralSourceV1::Integer(_)
                | ResolvedLiteralSourceV1::Bool(_)
                | ResolvedLiteralSourceV1::Float
        )
    )
}

fn array_cutpoints(
    product: &VerifiedResolvedScriptV1,
    relation: &ResolvedInitializerRelationV1,
) -> Result<Box<[Cutpoint]>, &'static str> {
    let site = relation.initializer_site().ok_or("missing-initializer")?;
    let shape = product.body_shape();
    let element_count = shape
        .expressions()
        .iter()
        .find_map(|expression| match expression {
            BodyExpressionShapeV1::ArrayLiteral {
                site: candidate,
                element_count,
            } if candidate == site => Some(*element_count),
            _ => None,
        })
        .ok_or("nonliteral-initializer")?;
    let mut elements = shape
        .relations()
        .iter()
        .filter(|row| row.parent() == site.node())
        .collect::<Vec<_>>();
    if elements.len() != element_count as usize {
        return Err("element-cardinality");
    }
    elements.sort_by_key(|row| row.role());
    let mut cutpoints = vec![
        Cutpoint::BeforeAllocation,
        Cutpoint::AcquiredBeforeClaim,
        Cutpoint::Claimed,
    ];
    for (index, element) in elements.iter().enumerate() {
        if element.role() != &SourcePathSegmentV1::Element(index as u32) {
            return Err("element-order");
        }
        if !primitive(
            product
                .core()
                .data()
                .expression_source
                .literal(element.child()),
        ) {
            return Err("child-capability");
        }
        // Integer/Bool/Float proves no child Home. It does not prove a numeric
        // write succeeds: range/type checking remains an explicit Fault cutpoint.
        cutpoints.extend([
            Cutpoint::ChildEvaluating(element.child().clone()),
            Cutpoint::ChildReady(element.child().clone()),
            Cutpoint::Written(element.child().clone()),
        ]);
    }
    cutpoints.extend([
        Cutpoint::FaultUnwind,
        Cutpoint::LocalCommit,
        Cutpoint::CallerExit,
    ]);
    Ok(cutpoints.into_boxed_slice())
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][script-array/source-lifecycle-{reason}]")
}

#[cfg(test)]
#[path = "normal_script_array_source_lifecycle_tests.rs"]
mod tests;
