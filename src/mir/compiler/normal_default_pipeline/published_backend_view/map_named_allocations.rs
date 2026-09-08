//! Copied conditional C observations bound to the original MIR graph.
//! No target-name classification, source admission, or execution-coverage proof.
use super::c_transport_v2::ProjectionAction;
use super::map_body_index::{MapBodyIndex, Producer, Site, ValueKey};
use crate::mir::{ConstructionTarget, MirInstruction, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NamedAllocationConsumer {
    Array,
    DirectArray,
    Map,
    File,
    TypedObject,
    AliasOperandZero,
    InvalidPlan,
    Unsupported,
}

fn reject(reason: &str, site: Site<'_>) -> String {
    format!(
        "[freeze:contract][map-frame/named-{reason}] function={} block={} instruction={}",
        site.0, site.1, site.2
    )
}

impl<'m> MapBodyIndex<'m> {
    /// Consume this index when binding so a partial failure cannot escape.
    /// Missing observations are diagnosed only if a Named producer is demanded.
    pub(super) fn with_named_allocations(
        mut self,
        observations: impl IntoIterator<Item = (Site<'m>, NamedAllocationConsumer)>,
    ) -> Result<Self, String> {
        for (site, consumer) in observations {
            match self.instructions.get(&site) {
                Some(MirInstruction::NewBox {
                    target: ConstructionTarget::Named(_),
                    ..
                }) => {}
                _ => return Err(reject("site-mismatch", site)),
            }
            if self.named_allocations.insert(site, consumer).is_some() {
                return Err(reject("duplicate-binding", site));
            }
            if consumer == NamedAllocationConsumer::AliasOperandZero {
                self.named_alias_operand(site)?;
            }
        }
        Ok(self)
    }

    pub(super) fn named_alias_operand(&self, site: Site<'m>) -> Result<Option<ValueId>, String> {
        match self
            .named_allocations
            .get(&site)
            .ok_or_else(|| reject("not-observed", site))?
        {
            NamedAllocationConsumer::AliasOperandZero => {
                let Some(MirInstruction::NewBox {
                    target: ConstructionTarget::Named(_),
                    args,
                    ..
                }) = self.instructions.get(&site)
                else {
                    return Err(reject("site-mismatch", site));
                };
                let source = *args
                    .first()
                    .ok_or_else(|| reject("alias-operand-missing", site))?;
                self.producer((site.0, source))?;
                Ok(Some(source))
            }
            NamedAllocationConsumer::InvalidPlan => Err(reject("invalid-plan", site)),
            NamedAllocationConsumer::Unsupported => Err(reject("unsupported", site)),
            _ => Ok(None),
        }
    }

    /// Select only the Named projection; other producers retain their own owners.
    pub(super) fn named_projection_action(
        &self,
        key: ValueKey<'m>,
    ) -> Result<ProjectionAction, String> {
        let Producer::Instruction {
            site,
            instruction:
                MirInstruction::NewBox {
                    target: ConstructionTarget::Named(_),
                    ..
                },
        } = self.producer(key)?
        else {
            return Err("[freeze:contract][map-frame/named-producer-mismatch]".into());
        };
        Ok(if self.named_alias_operand(site)?.is_some() {
            ProjectionAction::NamedAliasOperandZero
        } else {
            ProjectionAction::OriginalHandle
        })
    }
}

#[cfg(test)]
#[path = "map_named_allocations_tests.rs"]
mod tests;
