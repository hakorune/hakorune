//! Disconnected receipt payload for one ordinary non-FastMem FieldGet.
//!
//! FIELDGET0-S0 captures only already-resolved field facts. It has no Builder,
//! ValueId, type-map, metadata, instruction, or commit capability; I0 may
//! consume it only after the physical ordinary FieldGet succeeds.

use hakorune_mir_builder::lowering_facts::{
    PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1, TypeFactDecisionV1,
};

use crate::mir::builder::MirBuilder;
use crate::mir::MirType;
use crate::mir::ValueId;

/// Immutable field-access metadata inputs prepared before ordinary FieldGet.
#[derive(Debug, Eq, PartialEq)]
struct OrdinaryFieldAccessSiteDescriptorV1 {
    receiver_box_name: Option<String>,
    field: String,
}

/// Existing field-origin result disposition, detached from Builder mutation.
#[derive(Debug, Eq, PartialEq)]
enum FieldResultOriginDispositionV1 {
    Absent,
    Publish(String),
}

/// One non-Clone, Builder-free post-success payload for ordinary FieldGet.
#[derive(Debug)]
pub(super) struct PreparedOrdinaryFieldGetPostSuccessV1 {
    type_publication: PreparedTypeFactPublicationV1,
    access_site: OrdinaryFieldAccessSiteDescriptorV1,
    field_result_origin: FieldResultOriginDispositionV1,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum OrdinaryFieldGetPostSuccessErrorV1 {
    TypeFact(TypeFactDecisionErrorV1),
}

impl std::fmt::Display for OrdinaryFieldGetPostSuccessErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TypeFact(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for OrdinaryFieldGetPostSuccessErrorV1 {}

impl PreparedOrdinaryFieldGetPostSuccessV1 {
    /// Prepares only existing declared-type, site, and origin dispositions.
    ///
    /// The future destination is fresh, so S0 supplies no existing destination
    /// fact. A declared `Unknown` is rejected by the existing exact-fact owner;
    /// absence remains a deliberate no-publication decision.
    pub(super) fn prepare(
        declared_type: Option<&MirType>,
        receiver_box_name: Option<&str>,
        field: &str,
        field_result_origin: Option<&str>,
    ) -> Result<Self, OrdinaryFieldGetPostSuccessErrorV1> {
        let type_publication = TypeFactDecisionV1::prepare(None, declared_type)
            .map_err(OrdinaryFieldGetPostSuccessErrorV1::TypeFact)?;
        let access_site = OrdinaryFieldAccessSiteDescriptorV1 {
            receiver_box_name: receiver_box_name.map(str::to_string),
            field: field.to_string(),
        };
        let field_result_origin = match field_result_origin {
            Some(origin) => FieldResultOriginDispositionV1::Publish(origin.to_string()),
            None => FieldResultOriginDispositionV1::Absent,
        };
        Ok(Self {
            type_publication,
            access_site,
            field_result_origin,
        })
    }

    /// Commits the prepared ordinary FieldGet facts after its instruction
    /// receipt.  Successful emission already proves the current function
    /// exists, so the access-site append is structurally non-fallible here.
    pub(super) fn commit(self, builder: &mut MirBuilder, destination: ValueId, base: ValueId) {
        let Self {
            type_publication,
            access_site,
            field_result_origin,
        } = self;
        if let PreparedTypeFactPublicationV1::Publish(ty) = type_publication {
            builder.function_state.type_ctx.set_type(destination, ty);
        }
        builder
            .record_field_access_site(
                None,
                base,
                access_site.receiver_box_name,
                access_site.field,
                None,
                "load",
                "none",
                "allow_dynamic",
            )
            .expect("[freeze:contract][fieldget/receipt_site_commit_failed]");
        if let FieldResultOriginDispositionV1::Publish(origin) = field_result_origin {
            builder
                .function_state
                .type_ctx
                .set_origin_box(destination, origin);
        }
    }

    #[cfg(test)]
    fn type_publication(&self) -> &PreparedTypeFactPublicationV1 {
        &self.type_publication
    }

    #[cfg(test)]
    fn access_site(&self) -> &OrdinaryFieldAccessSiteDescriptorV1 {
        &self.access_site
    }

    #[cfg(test)]
    fn field_result_origin(&self) -> &FieldResultOriginDispositionV1 {
        &self.field_result_origin
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FieldResultOriginDispositionV1, OrdinaryFieldGetPostSuccessErrorV1,
        PreparedOrdinaryFieldGetPostSuccessV1,
    };
    use crate::mir::MirType;
    use hakorune_mir_builder::lowering_facts::{
        PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1,
    };

    #[test]
    fn declared_exact_type_keeps_site_and_origin_inputs_without_builder_state() {
        let prepared = PreparedOrdinaryFieldGetPostSuccessV1::prepare(
            Some(&MirType::Box("ArrayBox".to_string())),
            Some("OwnerBox"),
            "items",
            Some("ArrayBox"),
        )
        .unwrap();

        assert_eq!(
            prepared.type_publication(),
            &PreparedTypeFactPublicationV1::Publish(MirType::Box("ArrayBox".to_string()))
        );
        assert_eq!(
            prepared.access_site().receiver_box_name.as_deref(),
            Some("OwnerBox")
        );
        assert_eq!(prepared.access_site().field, "items");
        assert_eq!(
            prepared.field_result_origin(),
            &FieldResultOriginDispositionV1::Publish("ArrayBox".to_string())
        );
    }

    #[test]
    fn absent_declared_type_is_no_publication_with_absent_origin() {
        let prepared =
            PreparedOrdinaryFieldGetPostSuccessV1::prepare(None, None, "dynamic", None).unwrap();

        assert_eq!(
            prepared.type_publication(),
            &PreparedTypeFactPublicationV1::NoPublication
        );
        assert_eq!(prepared.access_site().receiver_box_name, None);
        assert_eq!(prepared.access_site().field, "dynamic");
        assert_eq!(
            prepared.field_result_origin(),
            &FieldResultOriginDispositionV1::Absent
        );
    }

    #[test]
    fn declared_unknown_rejects_without_a_compatibility_publication() {
        assert!(matches!(
            PreparedOrdinaryFieldGetPostSuccessV1::prepare(
                Some(&MirType::Unknown),
                Some("OwnerBox"),
                "items",
                None,
            ),
            Err(OrdinaryFieldGetPostSuccessErrorV1::TypeFact(
                TypeFactDecisionErrorV1::UnknownProposal { existing: None }
            ))
        ));
    }

    #[test]
    fn void_is_an_exact_declared_field_fact() {
        let prepared = PreparedOrdinaryFieldGetPostSuccessV1::prepare(
            Some(&MirType::Void),
            None,
            "unit",
            None,
        )
        .unwrap();
        assert_eq!(
            prepared.type_publication(),
            &PreparedTypeFactPublicationV1::Publish(MirType::Void)
        );
    }
}
