//! Disconnected receipt payload for one ordinary non-FastMem FieldSet site.
//!
//! FIELDSTORE-OBSERVE0-S0 retains the existing source span and resolved site
//! inputs before physical emission.  It has no Builder, metadata, instruction,
//! or commit capability; I0 may consume it only after FieldSet succeeds.

use crate::ast::Span;
use crate::mir::ValueId;

/// One exact ordinary FieldSet access-site descriptor.
///
/// This is deliberately not a general FieldStore lifecycle: weak writes,
/// typed-array contract claims, FastMem, and index routes keep their existing
/// owners outside this first profile.
#[derive(Debug, PartialEq)]
pub(super) struct PreparedOrdinaryFieldStoreAccessSiteV1 {
    source_span: Span,
    base: ValueId,
    receiver_box_name: Option<String>,
    field: String,
}

impl PreparedOrdinaryFieldStoreAccessSiteV1 {
    /// Captures only already-resolved ordinary FieldSet site inputs.
    pub(super) fn prepare(
        source_span: Span,
        base: ValueId,
        receiver_box_name: Option<&str>,
        field: &str,
    ) -> Self {
        Self {
            source_span,
            base,
            receiver_box_name: receiver_box_name.map(str::to_string),
            field: field.to_string(),
        }
    }

    /// Commits the ordinary access-site receipt after FieldSet succeeds.
    pub(super) fn commit(self, builder: &mut super::super::MirBuilder) -> Result<(), String> {
        let Self {
            source_span,
            base,
            receiver_box_name,
            field,
        } = self;
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let site_id = format!(
            "field.{}",
            function.metadata.fastmem_field_access_sites.len()
        );
        function.metadata.fastmem_field_access_sites.push(
            crate::mir::function::FastMemFieldAccessSite {
                site_id,
                source_span,
                region: None,
                base_value: base,
                receiver_box_name,
                field_id: field,
                layout_id: None,
                access_kind: "store".to_string(),
                required_route: "none".to_string(),
                fallback_policy: "allow_dynamic".to_string(),
            },
        );
        Ok(())
    }

    #[cfg(test)]
    fn source_span(&self) -> Span {
        self.source_span
    }

    #[cfg(test)]
    fn base(&self) -> ValueId {
        self.base
    }

    #[cfg(test)]
    fn receiver_box_name(&self) -> Option<&str> {
        self.receiver_box_name.as_deref()
    }

    #[cfg(test)]
    fn field(&self) -> &str {
        &self.field
    }
}

#[cfg(test)]
mod tests {
    use super::PreparedOrdinaryFieldStoreAccessSiteV1;
    use crate::ast::Span;
    use crate::mir::ValueId;

    #[test]
    fn captures_exact_resolved_ordinary_fieldset_site_inputs() {
        let span = Span::new(3, 5, 3, 17);
        let prepared = PreparedOrdinaryFieldStoreAccessSiteV1::prepare(
            span,
            ValueId::new(12),
            Some("OwnerBox"),
            "items",
        );

        assert_eq!(prepared.source_span(), span);
        assert_eq!(prepared.base(), ValueId::new(12));
        assert_eq!(prepared.receiver_box_name(), Some("OwnerBox"));
        assert_eq!(prepared.field(), "items");
    }

    #[test]
    fn allows_dynamic_receiver_identity_without_synthesizing_one() {
        let prepared = PreparedOrdinaryFieldStoreAccessSiteV1::prepare(
            Span::unknown(),
            ValueId::new(4),
            None,
            "dynamic",
        );

        assert_eq!(prepared.receiver_box_name(), None);
        assert_eq!(prepared.field(), "dynamic");
    }
}
