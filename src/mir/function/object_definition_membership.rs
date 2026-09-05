//! One-way definition-ID projection into compatibility declaration metadata.
//! Names are output keys, never inputs used to recover canonical identity.

use super::{MirModule, UserBoxFieldDecl};
use hakorune_mir_defs::CanonicalObjectIdV1;
use std::collections::{BTreeMap, HashMap};

impl MirModule {
    pub(in crate::mir) fn prepare_object_definition_membership(
        &self,
        names: &HashMap<String, Vec<String>>,
        fields: &HashMap<String, Vec<UserBoxFieldDecl>>,
    ) -> Result<Option<BTreeMap<String, CanonicalObjectIdV1>>, String> {
        let Some(definitions) = self.canonical_object_definitions() else {
            return Ok(None);
        };
        let mut membership = BTreeMap::new();
        for (index, definition) in definitions.iter().enumerate() {
            let id = CanonicalObjectIdV1::from_declaration_index(index)
                .ok_or_else(|| fault("id-overflow"))?;
            let name = definition.diagnostic_name();
            let expected_names: Vec<_> = definition
                .fields()
                .iter()
                .map(|field| field.name.clone())
                .collect();
            if names.get(name) != Some(&expected_names)
                || fields.get(name).map(Vec::as_slice) != Some(definition.fields())
            {
                return Err(fault("declaration-projection-drift"));
            }
            if membership.insert(name.to_owned(), id).is_some() {
                return Err(fault("duplicate-projection-key"));
            }
        }
        Ok(Some(membership))
    }

    pub(crate) fn validate_object_definition_membership(&self) -> Result<(), String> {
        let expected = self.prepare_object_definition_membership(
            &self.metadata.user_box_decls,
            &self.metadata.user_box_field_decls,
        )?;
        if expected != self.metadata.canonical_object_membership {
            return Err(fault("membership-drift"));
        }
        Ok(())
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][mir/object-definition/{reason}]")
}
