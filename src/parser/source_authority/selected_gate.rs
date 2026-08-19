use super::*;
use crate::parser::callable_gate_projection::MemberGateSelectionReceiptV1;

impl OpenBoxMethodSourceTransactionV1 {
    pub(in crate::parser) fn try_merge_selected_gate(
        &mut self,
        selected: Self,
        gate_site: crate::ast::BoxMemberGateSiteV1,
    ) -> Result<(), ParseError> {
        let receipt = MemberGateSelectionReceiptV1::issue_from_selected_path(
            SourceProgramDeclarationPathV1::from_parser_path(
                selected.cursor.box_site().path().clone(),
            ),
            &self.written_gate_path,
            &selected.written_gate_path,
            gate_site.box_member_ordinal(),
        )
        .map_err(|message| ParseError::BuildCfg {
            message: message.to_owned(),
            line: 0,
        })?;
        if let Some(receipt) = &receipt {
            if self
                .member_gate_selection_receipts
                .iter()
                .chain(selected.member_gate_selection_receipts.iter())
                .any(|existing| existing.same_gate_as(receipt))
            {
                return Err(ParseError::BuildCfg {
                    message: "duplicate member-gate selection receipt".to_owned(),
                    line: 0,
                });
            }
        }
        let mut entries = selected.inventory.into_selected_declaration_order();
        let mut relations = selected.method_relations;
        let mut declarations = selected.delegate_source_declarations;
        let mut constructor_relations = selected.constructor_relations;
        let mut generated_birth_triggers = selected.generated_birth_triggers;
        let selected_receipts = selected.member_gate_selection_receipts;
        if entries.len() != relations.len() {
            return Err(ParseError::BuildCfg {
                message: "selected Box source relation coverage is incomplete".to_owned(),
                line: 0,
            });
        }
        let gate_ordinal = gate_site.box_member_ordinal();
        for relation in &mut constructor_relations {
            let branch_ordinal =
                relation
                    .source_member_ordinal()
                    .ok_or_else(|| ParseError::BuildCfg {
                        message: "generated constructor cannot originate inside a selected gate"
                            .to_owned(),
                        line: 0,
                    })?;
            relation.prepend_selected_gate(gate_ordinal, branch_ordinal);
            if self
                .constructor_relations
                .iter()
                .any(|existing| existing.key() == relation.key())
            {
                return Err(ParseError::BuildCfg {
                    message: format!(
                        "selected gate duplicates constructor source key `{}`",
                        relation.key()
                    ),
                    line: 0,
                });
            }
        }
        for trigger in &mut generated_birth_triggers {
            let branch_ordinal = trigger.source_site().source_member_ordinal();
            trigger.prepend_selected_gate(gate_ordinal, branch_ordinal);
        }
        let mut rebased = Vec::with_capacity(relations.len());
        for (entry, relation) in entries.iter_mut().zip(relations.iter_mut()) {
            if entry.site() != relation.inventory_ordinal() || entry.name() != relation.name() {
                return Err(ParseError::BuildCfg {
                    message: "selected Box source relation does not match inventory".to_owned(),
                    line: 0,
                });
            }
            let branch_ordinal = relation.source_member_ordinal();
            entry
                .prepend_selected_gate(gate_site, branch_ordinal)
                .map_err(inventory_error_to_parse_error)?;
            relation.prepend_selected_gate(gate_ordinal, branch_ordinal);
            rebased.push(relation.clone());
        }
        for declaration in &mut declarations {
            declaration.prepend_selected_gate(
                gate_ordinal,
                declaration.source_site.source_member_ordinal(),
            );
        }
        let placements = self
            .inventory
            .commit_prepared_append(
                crate::ast::PreparedBoxMethodInventoryAppendV1::try_new(entries)
                    .map_err(inventory_error_to_parse_error)?,
            )
            .map_err(inventory_error_to_parse_error)?;
        for (relation, placement) in rebased.into_iter().zip(placements.iter()) {
            let relation_name = relation.name().to_owned();
            self.method_relations.push(match relation {
                MethodSourceRelationV1::Explicit(mut relation) => {
                    relation.inventory_ordinal = placement.inventory_ordinal();
                    MethodSourceRelationV1::Explicit(relation)
                }
                MethodSourceRelationV1::GeneratedProperty { source_site, .. } => {
                    if placement.name() != relation_name {
                        return Err(ParseError::BuildCfg {
                            message: "selected generated property placement/name mismatch"
                                .to_owned(),
                            line: 0,
                        });
                    }
                    MethodSourceRelationV1::GeneratedProperty {
                        source_site,
                        placement: placement.clone(),
                    }
                }
            });
        }
        self.delegate_source_declarations.extend(declarations);
        self.constructor_relations.extend(constructor_relations);
        self.generated_birth_triggers
            .extend(generated_birth_triggers);
        self.member_gate_selection_receipts
            .extend(selected_receipts);
        self.member_gate_selection_receipts.extend(receipt);
        Ok(())
    }
}
