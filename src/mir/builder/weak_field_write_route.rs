//! Builder-free declaration-backed weak-field route classification.

use super::ValueId;
use crate::ast::FieldDecl;
use crate::mir::instruction::FastMemRegionId;
use crate::mir::type_contracts::weak_field::box_schema_fingerprint;
use crate::mir::UserBoxFieldDecl;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum PreparedFieldWriteRouteV1 {
    Ordinary(PreparedOrdinaryFieldWriteRouteV1),
    KnownWeak(PreparedKnownWeakFieldWriteV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct PreparedOrdinaryFieldWriteRouteV1 {
    pub(super) region: Option<FastMemRegionId>,
    pub(super) base: ValueId,
    pub(super) field: String,
    pub(super) value: ValueId,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct PreparedKnownWeakFieldWriteV1 {
    pub(super) region: Option<FastMemRegionId>,
    pub(super) base: ValueId,
    pub(super) value: ValueId,
    pub(super) box_name: String,
    pub(super) field_name: String,
    pub(super) field_index: usize,
    pub(super) schema_fingerprint: String,
    _seal: KnownWeakFieldWriteSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct KnownWeakFieldWriteSealV1;

impl PreparedKnownWeakFieldWriteV1 {
    pub(super) fn contract_id(&self) -> String {
        format!(
            "weak-field:{}:{}",
            self.schema_fingerprint, self.field_index
        )
    }
}

/// Classifies one field write without mutating Builder or metadata state.
pub(super) fn prepare_field_write_route_v1(
    region: Option<FastMemRegionId>,
    base: ValueId,
    field_name: &str,
    value: ValueId,
    base_owner: Option<&str>,
    declarations: Option<&[FieldDecl]>,
) -> PreparedFieldWriteRouteV1 {
    let Some(owner) = base_owner else {
        return ordinary(region, base, field_name, value);
    };
    let Some(fields) = declarations else {
        return ordinary(region, base, field_name, value);
    };
    let Some((field_index, field)) = fields
        .iter()
        .enumerate()
        .find(|(_, field)| field.name == field_name)
    else {
        return ordinary(region, base, field_name, value);
    };
    if !field.is_weak {
        return ordinary(region, base, field_name, value);
    }

    let typed_fields = fields
        .iter()
        .map(|field| UserBoxFieldDecl {
            name: field.name.clone(),
            declared_type_name: field.declared_type_name.clone(),
            is_weak: field.is_weak,
        })
        .collect::<Vec<_>>();
    PreparedFieldWriteRouteV1::KnownWeak(PreparedKnownWeakFieldWriteV1 {
        region,
        base,
        value,
        box_name: owner.to_string(),
        field_name: field_name.to_string(),
        field_index,
        schema_fingerprint: box_schema_fingerprint(owner, &typed_fields),
        _seal: KnownWeakFieldWriteSealV1,
    })
}

fn ordinary(
    region: Option<FastMemRegionId>,
    base: ValueId,
    field: &str,
    value: ValueId,
) -> PreparedFieldWriteRouteV1 {
    PreparedFieldWriteRouteV1::Ordinary(PreparedOrdinaryFieldWriteRouteV1 {
        region,
        base,
        field: field.to_string(),
        value,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decl(name: &str, is_weak: bool) -> FieldDecl {
        FieldDecl {
            name: name.to_string(),
            declared_type_name: Some("MapBox".to_string()),
            is_weak,
            default_value: None,
        }
    }

    #[test]
    fn missing_or_nonweak_declaration_is_ordinary() {
        let fields = [decl("slot", false)];
        for (owner, declarations) in [(None, None), (Some("Owner"), Some(&fields[..]))] {
            let route = prepare_field_write_route_v1(
                None,
                ValueId::new(1),
                "slot",
                ValueId::new(2),
                owner,
                declarations,
            );
            assert!(matches!(route, PreparedFieldWriteRouteV1::Ordinary(_)));
        }
    }

    #[test]
    fn missing_field_is_ordinary() {
        let fields = [decl("slot", true)];
        let route = prepare_field_write_route_v1(
            None,
            ValueId::new(1),
            "other",
            ValueId::new(2),
            Some("Owner"),
            Some(&fields),
        );
        assert!(matches!(route, PreparedFieldWriteRouteV1::Ordinary(_)));
    }

    #[test]
    fn weak_route_retains_declaration_identity_and_region() {
        let fields = [decl("head", false), decl("slot", true)];
        let route = prepare_field_write_route_v1(
            Some(FastMemRegionId::new(7)),
            ValueId::new(3),
            "slot",
            ValueId::new(4),
            Some("Owner"),
            Some(&fields),
        );
        let PreparedFieldWriteRouteV1::KnownWeak(route) = route else {
            panic!("weak declaration must produce KnownWeak");
        };
        assert_eq!(route.field_index, 1);
        assert_eq!(route.box_name, "Owner");
        assert_eq!(route.field_name, "slot");
        assert_eq!(route.region, Some(FastMemRegionId::new(7)));
        assert_eq!(
            route.contract_id(),
            format!("weak-field:{}:1", route.schema_fingerprint)
        );
    }
}
