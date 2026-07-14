//! Exact-once Builder-side consumption of one sealed trivial owner profile.
//!
//! This ledger does not discover representation facts. It only proves that
//! Lower consumed every pre-Builder claim exactly once.

use crate::mir::resolved_semantics::{
    BindingRefV1, SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
};

use super::product::{
    TrivialBindingDefinitionOriginV1, TrivialProfileCoverageSubjectV1, TrivialRepresentationV1,
    TrivialTerminalProfileV1, VerifiedTrivialCanonicalOwnerV1,
};

#[derive(Debug)]
pub(crate) struct TrivialProfileConsumptionV1 {
    product: VerifiedTrivialCanonicalOwnerV1,
    next: usize,
}

impl TrivialProfileConsumptionV1 {
    pub(crate) fn new(product: VerifiedTrivialCanonicalOwnerV1) -> Self {
        Self { product, next: 0 }
    }

    pub(crate) fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.product.owner()
    }

    pub(crate) fn claim_value(
        &mut self,
        site: &SourceExprSiteV1,
    ) -> Result<TrivialRepresentationV1, String> {
        let representation = self.product.representation_at(site).ok_or_else(|| {
            format!("[freeze:contract][trivial_profile/value_missing] site={site:?}")
        })?;
        self.claim(TrivialProfileCoverageSubjectV1::Value(site.clone()))?;
        Ok(representation)
    }

    pub(crate) fn claim_declaration(
        &mut self,
        binding: BindingRefV1,
        site: &SourceBindingSiteV1,
    ) -> Result<TrivialRepresentationV1, String> {
        self.claim_definition(
            binding,
            TrivialBindingDefinitionOriginV1::Declaration(site.clone()),
        )
    }

    pub(crate) fn claim_assignment(
        &mut self,
        binding: BindingRefV1,
        site: &SourceExprSiteV1,
    ) -> Result<TrivialRepresentationV1, String> {
        self.claim_definition(
            binding,
            TrivialBindingDefinitionOriginV1::Assignment(site.clone()),
        )
    }

    pub(crate) fn claim_if_merges(
        &mut self,
        statement: &SourceStmtSiteV1,
    ) -> Result<Box<[(BindingRefV1, TrivialRepresentationV1)]>, String> {
        let rows = self
            .product
            .merge_profiles()
            .iter()
            .filter(|row| row.statement() == statement)
            .map(|row| (row.binding(), row.representation()))
            .collect::<Vec<_>>();
        for (binding, _) in rows.iter().copied() {
            self.claim(TrivialProfileCoverageSubjectV1::IfMergeProfile {
                statement: statement.clone(),
                binding,
            })?;
        }
        Ok(rows.into_boxed_slice())
    }

    pub(crate) fn claim_terminal_explicit_value(
        &mut self,
        statement: &SourceStmtSiteV1,
        value: &SourceExprSiteV1,
    ) -> Result<TrivialRepresentationV1, String> {
        let TrivialTerminalProfileV1::ExplicitValue {
            statement: expected_statement,
            value: expected_value,
            representation,
        } = self.product.terminal()
        else {
            return Err("[freeze:contract][trivial_profile/terminal_kind_mismatch]".to_string());
        };
        if expected_statement != statement || expected_value != value {
            return Err("[freeze:contract][trivial_profile/terminal_site_mismatch]".to_string());
        }
        let representation = *representation;
        self.claim(TrivialProfileCoverageSubjectV1::ExplicitValueTerminal(
            statement.clone(),
        ))?;
        Ok(representation)
    }

    pub(crate) fn claim_terminal_explicit_no_value(
        &mut self,
        statement: &SourceStmtSiteV1,
    ) -> Result<(), String> {
        let TrivialTerminalProfileV1::ExplicitNoValue {
            statement: expected,
        } = self.product.terminal()
        else {
            return Err("[freeze:contract][trivial_profile/terminal_kind_mismatch]".to_string());
        };
        if expected != statement {
            return Err("[freeze:contract][trivial_profile/terminal_site_mismatch]".to_string());
        }
        self.claim(TrivialProfileCoverageSubjectV1::ExplicitNoValueTerminal(
            statement.clone(),
        ))
    }

    pub(crate) fn claim_terminal_implicit_no_value(
        &mut self,
        body: &crate::mir::compiler::located::SourceBodySiteV1,
        body_end: u32,
    ) -> Result<(), String> {
        let TrivialTerminalProfileV1::ImplicitNoValue {
            body: expected_body,
            body_end: expected_end,
        } = self.product.terminal()
        else {
            return Err("[freeze:contract][trivial_profile/terminal_kind_mismatch]".to_string());
        };
        if expected_body != body || *expected_end != body_end {
            return Err("[freeze:contract][trivial_profile/terminal_site_mismatch]".to_string());
        }
        self.claim(TrivialProfileCoverageSubjectV1::ImplicitNoValueTerminal {
            body: body.clone(),
            body_end,
        })
    }

    pub(crate) fn finish(self) -> Result<(), String> {
        let expected = self.product.coverage().ordered_subjects();
        if self.next != expected.len() {
            return Err(format!(
                "[freeze:contract][trivial_profile/coverage_finish_mismatch] consumed={} expected={}",
                self.next,
                expected.len(),
            ));
        }
        Ok(())
    }

    fn claim_definition(
        &mut self,
        binding: BindingRefV1,
        origin: TrivialBindingDefinitionOriginV1,
    ) -> Result<TrivialRepresentationV1, String> {
        let representation = self
            .product
            .definitions()
            .iter()
            .find(|row| row.binding() == binding && row.origin() == &origin)
            .map(|row| row.representation())
            .ok_or_else(|| {
                format!(
                    "[freeze:contract][trivial_profile/definition_missing] binding={binding:?} origin={origin:?}"
                )
            })?;
        self.claim(TrivialProfileCoverageSubjectV1::Definition { binding, origin })?;
        Ok(representation)
    }

    fn claim(&mut self, subject: TrivialProfileCoverageSubjectV1) -> Result<(), String> {
        let expected = self.product.coverage().ordered_subjects();
        if expected.get(self.next) == Some(&subject) {
            self.next += 1;
            return Ok(());
        }
        if expected[..self.next].contains(&subject) {
            return Err(format!(
                "[freeze:contract][trivial_profile/duplicate_claim] subject={subject:?}"
            ));
        }
        if expected[self.next..].contains(&subject) {
            return Err(format!(
                "[freeze:contract][trivial_profile/wrong_order] subject={subject:?} next={} ",
                self.next,
            ));
        }
        Err(format!(
            "[freeze:contract][trivial_profile/foreign_claim] subject={subject:?}"
        ))
    }
}
