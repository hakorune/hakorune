use super::{GenericMethodRoute, GenericMethodRouteKind, GenericMethodRouteSite};
use crate::mir::exact_numeric_value_facts::{
    ExactNumericValueFact, ExactNumericValueFactSource,
};
use crate::mir::{MirFunction, MirModule, ValueId};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapStoreI64KeyWitness {
    pub(crate) site: GenericMethodRouteSite,
    pub(crate) key_value: ValueId,
    pub(crate) declared_type_name: String,
    pub(crate) source: ExactNumericValueFactSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MapStoreI64KeyFactRejectionKind {
    MissingKeyValue,
    MirTypeOnly,
    NotExactI64 { declared_type_name: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapStoreI64KeyFactRejection {
    pub(crate) site: GenericMethodRouteSite,
    pub(crate) key_value: Option<ValueId>,
    pub(crate) kind: MapStoreI64KeyFactRejectionKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MapStoreI64KeyFactError {
    MissingKeyValue,
    MirTypeOnly,
    NotExactI64 { declared_type_name: String },
}

impl From<MapStoreI64KeyFactError> for MapStoreI64KeyFactRejectionKind {
    fn from(error: MapStoreI64KeyFactError) -> Self {
        match error {
            MapStoreI64KeyFactError::MissingKeyValue => Self::MissingKeyValue,
            MapStoreI64KeyFactError::MirTypeOnly => Self::MirTypeOnly,
            MapStoreI64KeyFactError::NotExactI64 { declared_type_name } => {
                Self::NotExactI64 { declared_type_name }
            }
        }
    }
}

pub(crate) fn refresh_module_mapstore_i64_key_witnesses(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_mapstore_i64_key_witnesses(function);
    }
}

pub(crate) fn refresh_function_mapstore_i64_key_witnesses(function: &mut MirFunction) {
    let mut witnesses = Vec::new();
    let mut rejections = Vec::new();
    for route in &function.metadata.generic_method_routes {
        if route.route_kind() != GenericMethodRouteKind::MapStoreI64 {
            continue;
        }
        match verify_mapstore_i64_key_route(route, &function.metadata.exact_numeric_value_facts) {
            Ok(Some(witness)) => witnesses.push(witness),
            Ok(None) => {}
            Err(kind) => rejections.push(MapStoreI64KeyFactRejection {
                site: GenericMethodRouteSite::new(route.block(), route.instruction_index()),
                key_value: route.key_value(),
                kind: kind.into(),
            }),
        }
    }
    function.metadata.mapstore_i64_key_witnesses = witnesses;
    function.metadata.mapstore_i64_key_fact_rejections = rejections;
}

pub(crate) fn verify_mapstore_i64_key_route(
    route: &GenericMethodRoute,
    facts: &BTreeMap<ValueId, ExactNumericValueFact>,
) -> Result<Option<MapStoreI64KeyWitness>, MapStoreI64KeyFactError> {
    if route.route_kind() != GenericMethodRouteKind::MapStoreI64 {
        return Ok(None);
    }
    let Some(key_value) = route.key_value() else {
        return Err(MapStoreI64KeyFactError::MissingKeyValue);
    };
    let Some(fact) = facts.get(&key_value) else {
        return Err(MapStoreI64KeyFactError::MirTypeOnly);
    };
    if fact.declared_type_name != "i64" {
        return Err(MapStoreI64KeyFactError::NotExactI64 {
            declared_type_name: fact.declared_type_name.clone(),
        });
    }
    Ok(Some(MapStoreI64KeyWitness {
        site: GenericMethodRouteSite::new(route.block(), route.instruction_index()),
        key_value,
        declared_type_name: fact.declared_type_name.clone(),
        source: fact.source.clone(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fact(name: &str) -> ExactNumericValueFact {
        ExactNumericValueFact {
            declared_type_name: name.to_string(),
            source: ExactNumericValueFactSource::Param {
                index: 0,
                name: "key".to_string(),
            },
        }
    }

    #[test]
    fn exact_i64_fact_is_eligible_for_projection() {
        let key = ValueId::new(7);
        let site = GenericMethodRouteSite::new(crate::mir::BasicBlockId::new(0), 1);
        let mut facts = BTreeMap::new();
        facts.insert(key, fact("i64"));
        let route = test_route(site, key);
        let witness = verify_mapstore_i64_key_route(&route, &facts)
            .expect("exact i64 fact should verify")
            .expect("mapstore route should produce witness");
        assert_eq!(witness.key_value, key);
        assert_eq!(witness.declared_type_name, "i64");
    }

    #[test]
    fn missing_fact_is_derived_only_rejection() {
        let route = test_route(
            GenericMethodRouteSite::new(crate::mir::BasicBlockId::new(0), 1),
            ValueId::new(7),
        );
        let error = verify_mapstore_i64_key_route(&route, &BTreeMap::new())
            .expect_err("MirType-only route must not become a hard witness");
        assert_eq!(error, MapStoreI64KeyFactError::MirTypeOnly);
    }

    #[test]
    fn non_i64_exact_fact_is_rejected() {
        let key = ValueId::new(7);
        let mut facts = BTreeMap::new();
        facts.insert(key, fact("u64"));
        let route = test_route(
            GenericMethodRouteSite::new(crate::mir::BasicBlockId::new(0), 1),
            key,
        );
        assert_eq!(
            verify_mapstore_i64_key_route(&route, &facts),
            Err(MapStoreI64KeyFactError::NotExactI64 {
                declared_type_name: "u64".to_string()
            })
        );
    }

    #[test]
    fn refresh_attaches_witness_after_route_metadata_exists() {
        let key = ValueId::new(7);
        let mut function = MirFunction::new(
            crate::mir::FunctionSignature {
                name: "mapstore_witness".to_string(),
                params: vec![],
                return_type: crate::mir::MirType::Void,
                effects: crate::mir::EffectMask::PURE,
            },
            crate::mir::BasicBlockId::new(0),
        );
        function.metadata.generic_method_routes.push(test_route(
            GenericMethodRouteSite::new(crate::mir::BasicBlockId::new(0), 1),
            key,
        ));
        function
            .metadata
            .exact_numeric_value_facts
            .insert(key, fact("i64"));

        refresh_function_mapstore_i64_key_witnesses(&mut function);

        assert_eq!(function.metadata.mapstore_i64_key_witnesses.len(), 1);
        assert!(function.metadata.mapstore_i64_key_fact_rejections.is_empty());
        assert_eq!(function.metadata.mapstore_i64_key_witnesses[0].key_value, key);
    }

    fn test_route(site: GenericMethodRouteSite, key: ValueId) -> GenericMethodRoute {
        GenericMethodRoute::new(
            site,
            super::super::GenericMethodRouteSurface::new("MapBox", "set", 2),
            super::super::GenericMethodRouteEvidence::new(None, None),
            super::super::GenericMethodRouteOperands::new(ValueId::new(1), Some(key), None),
            super::super::GenericMethodRouteDecision::new(
                GenericMethodRouteKind::MapStoreI64,
                super::super::GenericMethodRouteProof::SetSurfacePolicy,
                None,
                None,
                super::super::GenericMethodValueDemand::WriteAny,
                None,
            ),
        )
    }
}
